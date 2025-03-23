use crate::*;

#[derive(Clone, Debug)]
pub struct Block(pub Vec<Stmt>);

impl Node for Block {
    fn parse(source: &str) -> Option<Block> {
        let mut result = vec![];
        for line in tokenize(source, &[";"], false, false)? {
            result.push(if line.trim().is_empty() {
                Stmt::Drop
            } else {
                Stmt::parse(&line)?
            })
        }
        Some(Block(result))
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

    fn addr_infer(&self, ctx: &mut Compiler) -> Option<i32> {
        Some(*iter_map!(self.0.clone(), |x: Stmt| x.addr_infer(ctx)).last()?)
    }
}
