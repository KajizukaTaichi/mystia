use crate::*;

#[derive(Debug, Clone)]
pub enum Oper {
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Mod(Expr, Expr),
    Eql(Expr, Expr),
    Neq(Expr, Expr),
    Lt(Expr, Expr),
    Gt(Expr, Expr),
    LtEq(Expr, Expr),
    GtEq(Expr, Expr),
    Cast(Expr, Type),
}

impl Node for Oper {
    fn parse(source: &str) -> Option<Self> {
        let token_list: Vec<String> = tokenize(source, SPACE.as_ref(), true, true)?;
        let operator = token_list.get(token_list.len().checked_sub(2)?)?;
        let lhs = &join!(token_list.get(..token_list.len() - 2)?);
        let rhs = token_list.last()?;
        Some(match operator.as_str() {
            "+" => Oper::Add(Expr::parse(lhs)?, Expr::parse(rhs)?),
            "-" => Oper::Sub(Expr::parse(lhs)?, Expr::parse(rhs)?),
            "*" => Oper::Mul(Expr::parse(lhs)?, Expr::parse(rhs)?),
            "/" => Oper::Div(Expr::parse(lhs)?, Expr::parse(rhs)?),
            "%" => Oper::Mod(Expr::parse(lhs)?, Expr::parse(rhs)?),
            "==" => Oper::Eql(Expr::parse(lhs)?, Expr::parse(rhs)?),
            "!=" => Oper::Neq(Expr::parse(lhs)?, Expr::parse(rhs)?),
            "<" => Oper::Lt(Expr::parse(lhs)?, Expr::parse(rhs)?),
            ">" => Oper::Gt(Expr::parse(lhs)?, Expr::parse(rhs)?),
            ">=" => Oper::GtEq(Expr::parse(lhs)?, Expr::parse(rhs)?),
            "<=" => Oper::LtEq(Expr::parse(lhs)?, Expr::parse(rhs)?),
            ":" => Oper::Cast(Expr::parse(lhs)?, Type::parse(rhs)?),
            _ => return None,
        })
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Oper::Add(lhs, rhs) => compile_arithmetic!("add", self, ctx, lhs, rhs),
            Oper::Sub(lhs, rhs) => compile_arithmetic!("sub", self, ctx, lhs, rhs),
            Oper::Mul(lhs, rhs) => compile_arithmetic!("mul", self, ctx, lhs, rhs),
            Oper::Div(lhs, rhs) => compile_compare!("div", ctx, lhs, rhs),
            Oper::Mod(lhs, rhs) => compile_compare!("rem", ctx, lhs, rhs),
            Oper::Eql(lhs, rhs) => compile_arithmetic!("eq", self, ctx, lhs, rhs),
            Oper::Neq(lhs, rhs) => compile_arithmetic!("ne", self, ctx, lhs, rhs),
            Oper::Lt(lhs, rhs) => compile_compare!("lt", ctx, lhs, rhs),
            Oper::Gt(lhs, rhs) => compile_compare!("gt", ctx, lhs, rhs),
            Oper::LtEq(lhs, rhs) => compile_compare!("le", ctx, lhs, rhs),
            Oper::GtEq(lhs, rhs) => compile_compare!("ge", ctx, lhs, rhs),
            Oper::Cast(lhs, rhs) => {
                let rhs = rhs.type_infer(ctx)?;
                if lhs.type_infer(ctx)?.compile(ctx)? == rhs.compile(ctx)? {
                    return lhs.compile(ctx);
                }

                format!(
                    "({}.{} {})",
                    rhs.compile(ctx)?,
                    match rhs.compile(ctx)?.as_str() {
                        "f64" => "convert_i32_s",
                        "i32" => "trunc_f64_s",
                        _ => return None,
                    },
                    lhs.compile(ctx)?,
                )
            }
        })
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        match self {
            Oper::Add(lhs, rhs)
            | Oper::Sub(lhs, rhs)
            | Oper::Mul(lhs, rhs)
            | Oper::Div(lhs, rhs)
            | Oper::Mod(lhs, rhs) => {
                type_check!(lhs, rhs, ctx)
            }
            Oper::Eql(lhs, rhs)
            | Oper::Neq(lhs, rhs)
            | Oper::Lt(lhs, rhs)
            | Oper::Gt(lhs, rhs)
            | Oper::LtEq(lhs, rhs)
            | Oper::GtEq(lhs, rhs) => {
                type_check!(lhs, rhs, ctx)?;
                Some(Type::Bool)
            }
            Oper::Cast(lhs, rhs) => {
                lhs.type_infer(ctx)?;
                Some(rhs.type_infer(ctx)?)
            }
        }
    }
}
