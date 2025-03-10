mod block;
mod expr;
mod lexer;
mod node;
mod oper;
mod stmt;
mod utils;
mod value;

use std::collections::HashMap;
use {
    block::Block,
    expr::Expr,
    lexer::tokenize,
    node::Node,
    oper::Oper,
    stmt::Stmt,
    utils::{OPERATOR, SPACE, expand_local, include_letter},
    value::{Type, Value},
};

#[derive(Debug, Clone)]
pub struct Compiler {
    index: usize,
    array: Vec<String>,
    declare: Vec<String>,
    variable: HashMap<String, Type>,
    function: HashMap<String, Type>,
    argument: HashMap<String, Type>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            index: 0,
            declare: vec![],
            array: vec![],
            variable: HashMap::new(),
            function: HashMap::from([
                ("array.set".to_string(), Type::Void),
                ("array.get".to_string(), Type::Integer),
            ]),
            argument: HashMap::new(),
        }
    }

    pub fn build(&mut self, source: &str) -> Option<String> {
        let ast = Block::parse(source)?;
        let ret = ast.type_infer(self);
        Some(format!(
            r#"(module (memory $mem 1) {2} (func (export "_start") (result {1}) {3} {0}))"#,
            ast.compile(self),
            ret.compile(self),
            join!(self.declare),
            expand_local(self)
        ))
    }
}
