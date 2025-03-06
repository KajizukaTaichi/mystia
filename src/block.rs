use crate::*;

#[derive(Clone, Debug)]
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
        let block = self.0.clone();
        let last = block.len() - 1;
        format!(
            "(block (result i32) {} (br 0 {}))",
            join!(
                block[..last]
                    .iter()
                    .map(|x| x.compile(ctx))
                    .collect::<Vec<_>>()
            ),
            block[last].compile(ctx)
        )
    }
}
