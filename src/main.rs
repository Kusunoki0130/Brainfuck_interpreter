
/**Brainfuck run by Rust
 * 
 * "hello world" in Brainfuck
 * ```Brainfuck
 * ++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
 * ```
 */

use std::env;
use std::fs;

mod bfir;
use crate::bfir::BfIR;


fn main() {
    let mut args = env::args();
    args.next();
    let file_path = args.next().unwrap();

    let str_tokens: String = fs::read_to_string(&file_path).unwrap(); 

    println!("{:?}", BfIR::run(&str_tokens));

}
