use crate::*;

pub struct Block(Vec<Stmt>);

impl Block {
    pub fn parse(source: &str) -> Option<Block> {
        let mut result = vec![];
        for line in tokenize(source, &[";"], false)? {
            result.push(Stmt::parse(&line)?);
        }
        Some(Block(result))
    }

    pub fn compile(&self, ctx: &mut Compiler) -> String {
        format!(
            "(block (result i32) {}  (br 0))",
            join!(self.0.iter().map(|x| x.compile(ctx)).collect::<Vec<_>>())
        )
    }
}
