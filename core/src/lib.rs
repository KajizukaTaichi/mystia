mod block;
mod expr;
mod lexer;
mod oper;
mod stmt;
mod utils;
mod value;

use indexmap::IndexMap;
use {
    block::Block,
    expr::Expr,
    lexer::{str_escape, tokenize},
    oper::Oper,
    stmt::Stmt,
    utils::{OPERATOR, RESERVED, SPACE, expand_local, include_letter},
    value::{Type, Value},
};

pub trait Node {
    fn compile(&self, ctx: &mut Compiler) -> Option<String>;
    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type>;
    fn parse(source: &str) -> Option<Self>
    where
        Self: Node + Sized;
}

/// Context in compiling
#[derive(Debug, Clone)]
pub struct Compiler {
    /// Address for memory allocation
    pub alloc_index: i32,
    /// Address of pointer
    pub pointer_index: i32,
    /// Static string data
    pub static_data: Vec<String>,
    /// Set of function declare code
    pub declare_code: Vec<String>,
    /// Type inference for variable
    pub variable_type: IndexMap<String, Type>,
    /// Type inference for argument
    pub argument_type: IndexMap<String, Type>,
    /// Type inference for function includes local variables, arguments, and returns
    pub function_type: IndexMap<String, (IndexMap<String, Type>, IndexMap<String, Type>, Type)>,
    /// Error message if it was fault
    pub error: Option<String>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            alloc_index: 0,
            pointer_index: 0,
            static_data: vec![],
            declare_code: vec![],
            variable_type: IndexMap::new(),
            argument_type: IndexMap::new(),
            function_type: IndexMap::new(),
            error: vec![],
        }
    }

    pub fn build(&mut self, source: &str) -> Option<String> {
        let ast = Block::parse(source)?;
        let ret = ast.type_infer(self)?;
        Some(format!(
            "(module (memory $mem 1) {2} {3} (func (export \"_start\") {1} {4} {0}))",
            ast.compile(self)?,
            config_return!(ret, self)?,
            join!(self.static_data),
            join!(self.declare_code),
            expand_local(self)?,
        ))
    }
}
