mod block;
mod expr;
mod lexer;
mod node;
mod oper;
mod stmt;
mod utils;
mod value;

use {block::*, expr::*, lexer::*, node::*, oper::*, stmt::*, utils::*, value::*};

pub struct Compiler {
    declare: Vec<String>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { declare: vec![] }
    }

    pub fn build(&mut self, source: &str) -> Option<String> {
        Some(format!(
            r#"(module {1} (func (export "_start") (result i32) {0}))"#,
            Block::parse(source).map(|x| x.compile(self))?,
            join!(self.declare)
        ))
    }
}
