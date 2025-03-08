use crate::*;

pub trait Node {
    fn compile(&self, ctx: &mut Compiler) -> String;
    fn parse(source: &str) -> Option<Self>
    where
        Self: Node + Sized;
}
