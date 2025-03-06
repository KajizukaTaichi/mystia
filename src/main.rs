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
    let mut compiler = Compiler::new();
    println!(
        "{}",
        compiler.build("fn inc(n) n + 1; inc(inc(1))").unwrap()
    );
}

struct Compiler {
    declare: Vec<String>,
}

impl Compiler {
    fn new() -> Self {
        Compiler { declare: vec![] }
    }

    fn build(&mut self, source: &str) -> Option<String> {
        Some(format!(
            r#"(module {1} (func (export "_start") (result i32) {0}))"#,
            Block::parse(source).map(|x| x.compile(self))?,
            join!(self.declare)
        ))
    }
}
