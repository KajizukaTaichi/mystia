use crate::*;

#[derive(Clone, Debug)]
pub struct Block(Vec<Stmt>);

impl Node for Block {
    fn parse(source: &str) -> Option<Block> {
        let mut result = vec![];
        for line in tokenize(source, &[";"], false)? {
            result.push(Stmt::parse(&line)?);
        }
        Some(Block(result))
    }

    fn compile(&self, ctx: &mut Compiler) -> String {
        join!(self.0.iter().map(|x| x.compile(ctx)).collect::<Vec<_>>())
    }
}
