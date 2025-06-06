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
        let mut result = vec![];
        for (n, line) in self.0.iter().enumerate() {
            let mut output = line.compile(ctx)?;
            if n != self.0.len() - 1 {
                if let Type::Void = line.type_infer(ctx)? {
                } else {
                    output.push_str("(drop)");
                }
            }
            result.push(output);
        }
        Some(join!(result))
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
