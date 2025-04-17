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
    r#type::{Dict, Type},
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
    /// Static string data
    pub static_data: Vec<String>,
    /// Set of function declare code
    pub declare_code: Vec<String>,
    /// Type inference for unknown
    pub expect_type: Option<Type>,
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
    const MEMCPY: &str = r#"
        (global $alloc_index (mut i32) (i32.const {LAST_STATIC_ALLOC}))
        (func $memcpy (param $src i32) (param $size i32) (result i32)
            (local $idx i32) (local $dst i32)
            (local.set $dst (global.get $alloc_index))
            (block $exit
                (loop $loop
                    (i64.store (local.get $dst) (i64.load (local.get $src)))
                    (local.set $src (i32.add (local.get $src) (i32.const 8)))
                    (local.set $dst (i32.add (local.get $dst) (i32.const 8)))
                    (local.set $idx (i32.add (local.get $idx) (i32.const 1)))
                    (br_if $loop (i32.lt_s (local.get $idx) (local.get $size)))
                )
            )
            (global.set $alloc_index (i32.add (global.get $alloc_index) (i32.mul (local.get $size) (i32.const 8))))
            (global.get $alloc_index)
        )
    "#;

    pub fn new() -> Self {
        Compiler {
            alloc_index: 0,
            is_memory_copied: false,
            static_data: vec![],
            declare_code: vec![],
            expect_type: None,
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
            "(module (memory $mem (export \"mem\") 1) {memcpy} {strings} {declare} (func (export \"_start\") {ret} {locals} {code}))",
            code = ast.compile(self)?,
            ret = config_return!(self.program_return.clone(), self)?,
            strings = join!(self.static_data),
            declare = join!(self.declare_code),
            memcpy = if self.is_memory_copied {
                Compiler::MEMCPY.replace("{LAST_STATIC_ALLOC}", &self.alloc_index.to_string())
            } else {
                String::new()
            },
            locals = expand_local(self)?,
        ))
    }
}
