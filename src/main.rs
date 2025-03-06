mod block;
mod expr;
mod lexer;
mod oper;
mod stmt;
mod utils;
mod value;

use block::*;
use expr::*;
use lexer::*;
use oper::*;
use stmt::*;
use utils::*;
use value::*;

fn main() {
    println!(
        r#"(module (func (export "_start") (result i32) {}))"#,
        Block::parse("1+2*3-10")
            .map(|x| x.compile(&mut Compiler { declare: vec![] }))
            .unwrap()
    );
}

struct Compiler {
    declare: Vec<String>,
}
