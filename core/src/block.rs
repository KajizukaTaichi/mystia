use crate::*;

#[derive(Clone, Debug)]
pub struct Block(pub Vec<Stmt>);

impl Node for Block {
    fn parse(source: &str) -> Option<Block> {
        Some(Block(iter_map!(
            tokenize(source, &[";"], false, false)?,
            |line: String| if line.trim().is_empty() {
                Some(Stmt::Drop)
            } else {
                Stmt::parse(&line)
            }
        )))
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(join!(iter_map!(&self.0, |x: &Stmt| x.compile(ctx))))
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        Some(
            iter_map!(self.0.clone(), |x: Stmt| x.type_infer(ctx))
                .last()?
                .clone(),
        )
    }
}
