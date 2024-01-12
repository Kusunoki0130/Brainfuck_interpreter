use std::{collections::HashMap, io::BufRead};


#[derive(Debug, thiserror::Error)]
pub enum CompileErrorType {
    #[error("Left bracket is not closed.")]
    LeftBracketError,   
    #[error("Left bracket is not expected.")]
    RightBracketError,
}

#[derive(Debug)]
pub struct CompileError {
    pub line: u32,
    pub col: u32,
    pub error_type: CompileErrorType
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
    pub fn run(str_tokens: &str) -> Result<Vec<char>, CompileError> {

        let code: Vec<BfIR>;
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

        let mut display: Vec<char> = Vec::new();
        let mut memory: [u8; 511] = [0; 511];
        let mut data_p: usize = 256;
        let mut code_p: usize = 0;

        while code_p as usize != code.len() {
            match code[code_p] {
                BfIR::BfAdd(num) => memory[data_p] += num,
                BfIR::BfSub(num) => memory[data_p] -= num,
                BfIR::BfNext(num) => data_p += num as usize, 
                BfIR::BfPre(num) => data_p -= num as usize,
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

    pub fn build(str_tokens: &str) -> Result<(Vec<BfIR>, HashMap<usize, usize>), CompileError> {
        let mut line_p = 1;
        let mut col_p = 0;
        let mut tokens: Vec<BfIR> = Vec::new();
        let mut sta: Vec<(usize, u32, u32)> = Vec::new();
        let mut jmp_table: HashMap<usize, usize> = HashMap::new();

        for ch in str_tokens.chars() {
            match ch {
                '+' => tokens.push(BfIR::BfAdd(1)),
                '-' => tokens.push(BfIR::BfSub(1)),
                '>' => tokens.push(BfIR::BfNext(1)),
                '<' => tokens.push(BfIR::BfPre(1)),
                '[' => {
                    sta.push((tokens.len(), line_p, col_p));
                    tokens.push(BfIR::BfJz);
                },
                ']' => {
                    let jz = sta.pop().ok_or(CompileError{
                        line: line_p,
                        col: col_p,
                        error_type: CompileErrorType::RightBracketError
                    })?;
                    tokens.push(BfIR::BfJnz);
                    jmp_table.insert(jz.0, tokens.len() - 1);
                    jmp_table.insert(tokens.len() - 1, jz.0);
                },
                '.' => tokens.push(BfIR::BfWrite),
                ',' => tokens.push(BfIR::BfRead),
                '\n' => {
                    col_p = 0;
                    line_p += 1;
                },
                _ => {}
            }
        }

        if sta.len() > 0 {
            return Err(CompileError{
                line: sta[0].1,
                col: sta[0].2,
                error_type: CompileErrorType::LeftBracketError
            });
        }    

        Ok((tokens, jmp_table))
    }
}
