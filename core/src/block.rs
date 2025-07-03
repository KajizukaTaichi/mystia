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
                if !matches!(line.type_infer(ctx)?, Type::Void) {
                    output.push_str("(drop)");
                }
            }
            result.push(output);
        }
        Some(join!(result))
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        let var_ctx = ctx.variable_type.clone();
        let typ_ctx = ctx.type_alias.clone();
        let fun_ctx = ctx.function_type.clone();
        let mcr_ctx = ctx.macro_code.clone();

        let Block(block) = self.clone();
        let mut result = Type::Void;
        for line in block {
            result = line.type_infer(ctx)?;
        }

        ctx.variable_type = var_ctx;
        ctx.type_alias = typ_ctx;
        ctx.function_type = fun_ctx;
        ctx.macro_code = mcr_ctx;
        Some(result)
    }
}
