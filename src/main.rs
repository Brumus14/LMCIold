pub mod data;
mod executor;
mod parser;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let source_code = fs::read_to_string(&args[1]).unwrap();
    let program = parser::parse_source_code(source_code);

    executor::execute_program(program);
}
