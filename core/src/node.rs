use crate::*;

pub trait Node {
    fn compile(&self, ctx: &mut Compiler) -> String;
    fn type_infer(&self, ctx: &mut Compiler) -> Type;
    fn parse(source: &str) -> Option<Self>
    where
        Self: Node + Sized;
}
