use crate::*;

#[derive(Clone, Debug)]
pub struct Block(pub Vec<Stmt>);

impl Node for Block {
    fn parse(source: &str) -> Option<Block> {
        let mut result = vec![];
        for line in tokenize(source, &[";"], false, false)? {
            let (line, _) = line.split_once("<--").unwrap_or((&line, ""));
            if line.trim().is_empty() {
                continue;
            }
            result.push(Stmt::parse(&line)?)
        }
        Some(Block(result))
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
