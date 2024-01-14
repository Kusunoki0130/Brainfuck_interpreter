use std::{collections::HashMap, io::BufRead};


#[derive(Debug, thiserror::Error)]
pub enum CompileErrorType {
    #[error("Left bracket is not closed.")]
    LeftBracketError,   
    #[error("Left bracket is not expected.")]
    RightBracketError,
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeErrorType {
    #[error("Pointer overflow")]
    PointerOverFlowError,
}

#[derive(Debug)]
pub enum BfErrorType {
    CompileError(CompileErrorType),
    RuntimeError(RuntimeErrorType)
}


#[derive(Debug)]
pub struct BfError {
    pub line: u32,
    pub col: u32,
    pub error_type: BfErrorType
}

#[derive(Debug)]
pub enum BfIR {
    BfAdd(u8),
    BfSub(u8),
    BfNext(u8),
    BfPre(u8),
    BfJz,
    BfJnz,
    BfRead,
    BfWrite
}

impl BfIR {
    pub fn run(str_tokens: &str) -> Result<Vec<char>, BfError> {

        let code: Vec<(BfIR, u32, u32)>;
        let jmp_table:HashMap<usize, usize>;
        match Self::build(str_tokens) {
            Ok((res_token, res_table)) => {
                code = res_token;
                jmp_table = res_table;
            },
            Err(err) => {
                return Err(err);
            }
        };

        const MEMORY_SIZE: usize = 512;
        let mut display: Vec<char> = Vec::new();
        let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
        let mut data_p: usize = MEMORY_SIZE/2;
        let mut code_p: usize = 0;

        while code_p as usize != code.len() {
            match code[code_p].0 {
                BfIR::BfAdd(num) => {memory[data_p] = memory[data_p].wrapping_add(num);},
                BfIR::BfSub(num) => {memory[data_p] = memory[data_p].wrapping_sub(num);},
                BfIR::BfNext(num) => {
                    data_p += num as usize;
                    if data_p >= MEMORY_SIZE {
                        return Err(BfError{
                            line: code[code_p].1,
                            col: code[code_p].2,
                            error_type: BfErrorType::RuntimeError(RuntimeErrorType::PointerOverFlowError)
                        });
                    } 
                }, 
                BfIR::BfPre(num) => {
                    if let None = data_p.checked_sub(num as usize) {
                        return Err(BfError{
                            line: code[code_p].1,
                            col: code[code_p].2,
                            error_type: BfErrorType::RuntimeError(RuntimeErrorType::PointerOverFlowError)
                        });
                    }
                    data_p -= num as usize;
                },
                BfIR::BfJz => {
                    if memory[data_p] == 0 {
                        code_p = jmp_table[&code_p];
                    } 
                }
                BfIR::BfJnz => code_p = jmp_table[&code_p] - 1,
                BfIR::BfRead => {
                    let stdin = std::io::stdin();
                    match stdin.lock().lines().next().unwrap() {
                        Ok(res) => {
                            memory[data_p] = match res.chars().next() {
                                Some(ch) => ch as u8,
                                _ => 0
                            };
                        }
                        _ => {}
                    }
                },
                BfIR::BfWrite => {
                    display.push(memory[data_p] as char);
                }
            }
            code_p += 1;
        }

        Ok(display)

    }

    pub fn build(str_tokens: &str) -> Result<(Vec<(BfIR, u32, u32)>, HashMap<usize, usize>), BfError> {
        let mut line_p = 1;
        let mut col_p = 0;
        let mut tokens: Vec<(BfIR, u32, u32)> = Vec::new();
        let mut sta: Vec<(usize, u32, u32)> = Vec::new();
        let mut jmp_table: HashMap<usize, usize> = HashMap::new();

        for ch in str_tokens.chars() {
            col_p += 1;
            match ch {
                '+' => tokens.push((BfIR::BfAdd(1), line_p, col_p)),
                '-' => tokens.push((BfIR::BfSub(1), line_p, col_p)),
                '>' => tokens.push((BfIR::BfNext(1), line_p, col_p)),
                '<' => tokens.push((BfIR::BfPre(1), line_p, col_p)),
                '[' => {
                    sta.push((tokens.len(), line_p, col_p));
                    tokens.push((BfIR::BfJz, line_p, col_p));
                },
                ']' => {
                    let jz = sta.pop().ok_or(BfError{
                        line: line_p,
                        col: col_p,
                        error_type: BfErrorType::CompileError(CompileErrorType::RightBracketError)
                    })?;
                    tokens.push((BfIR::BfJnz, line_p, col_p));
                    jmp_table.insert(jz.0, tokens.len() - 1);
                    jmp_table.insert(tokens.len() - 1, jz.0);
                },
                '.' => tokens.push((BfIR::BfWrite, line_p, col_p)),
                ',' => tokens.push((BfIR::BfRead, line_p, col_p)),
                '\n' => {
                    col_p = 0;
                    line_p += 1;
                },
                _ => {}
            }
        }

        if sta.len() > 0 {
            return Err(BfError{
                line: sta[0].1,
                col: sta[0].2,
                error_type: BfErrorType::CompileError(CompileErrorType::LeftBracketError)
            });
        }    

        Ok((tokens, jmp_table))
    }
}
