use crate::*;

#[derive(Clone, Debug)]
pub struct Block(pub Vec<Stmt>);

impl Node for Block {
    fn parse(source: &str) -> Option<Block> {
        Some(Block(
            tokenize(source, &[";"], false, false, false)?
                .iter()
                .map(|line| Stmt::parse(&line))
                .collect::<Option<Vec<_>>>()?,
        ))
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(join!(
            &self
                .0
                .iter()
                .map(|x| x.compile(ctx))
                .collect::<Option<Vec<_>>>()?
        ))
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        self.0
            .clone()
            .iter()
            .map(|x| x.type_infer(ctx))
            .collect::<Option<Vec<_>>>()?
            .last()
            .cloned()
    }
}
