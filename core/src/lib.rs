mod block;
mod expr;
mod lexer;
mod oper;
mod stmt;
mod r#type;
mod utils;
mod value;

use indexmap::IndexMap;
use {
    block::Block,
    expr::Expr,
    lexer::{str_escape, tokenize},
    oper::Oper,
    stmt::Stmt,
    r#type::Type,
    utils::{OPERATOR, RESERVED, SPACE, expand_local, include_letter},
    value::Value,
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
    /// Type inference for returns of main program
    pub program_return: Type,
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
            program_return: Type::Void,
        }
    }

    pub fn build(&mut self, source: &str) -> Option<String> {
        let ast = Block::parse(source)?;
        self.program_return = ast.type_infer(self)?;
        Some(format!(
            "(module (memory $mem (export \"mem\") 1) {strings} {declare} (func (export \"_start\") {returns} {locals} {code}))",
            code = ast.compile(self)?,
            returns = config_return!(self.program_return.clone(), self)?,
            strings = join!(self.static_data),
            declare = join!(self.declare_code),
            locals = expand_local(self)?,
        ))
    }
}
