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
    r#type::{Dict, Enum, Type},
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

/// Function includes local variables, arguments, and returns
type Function = (IndexMap<String, Type>, IndexMap<String, Type>, Type);
/// Context in compiling
#[derive(Debug, Clone)]
pub struct Compiler {
    /// Address for memory allocation
    pub alloc_index: i32,
    /// The code will copies memory?
    pub is_memory_copied: bool,
    /// Code that imports external module
    pub import_code: Vec<String>,
    /// Static string data
    pub static_data: Vec<String>,
    /// Set of function declare code
    pub declare_code: Vec<String>,
    /// Type alias that's defined by user
    pub type_alias: IndexMap<String, Type>,
    /// Type inference, hypothesis for unknown
    pub expect_type: Option<Type>,
    /// Errors that occurred during compilation
    pub occurred_error: Option<String>,
    /// Type environment for variable
    pub variable_type: IndexMap<String, Type>,
    /// Type environment for argument
    pub argument_type: IndexMap<String, Type>,
    /// Type environment for function
    pub function_type: IndexMap<String, Function>,
    /// Type of main program returns
    pub program_return: Type,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            alloc_index: 0,
            is_memory_copied: false,
            import_code: vec![],
            static_data: vec![],
            declare_code: vec![],
            expect_type: None,
            occurred_error: None,
            type_alias: IndexMap::new(),
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
            "(module {import} (memory $mem (export \"mem\") 1) {memcpy} {strings} {declare} (func (export \"_start\") {ret} {locals} {code}))",
            code = ast.compile(self)?,
            ret = config_return!(self.program_return.clone(), self)?,
            import = join!(self.import_code),
            strings = join!(self.static_data),
            declare = join!(self.declare_code),
            memcpy = if self.is_memory_copied {
                format!(
                    "(global $alloc_index (export \"alloc_index\") (mut i32) (i32.const {}))",
                    self.alloc_index
                )
            } else {
                String::new()
            },
            locals = expand_local(self)?,
        ))
    }
}
