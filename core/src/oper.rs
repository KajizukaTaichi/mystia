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
        let token = || Expr::parse(token_list.last()?);
        let operator = token_list.get(token_list.len().checked_sub(2)?)?;
        let has_lhs = |len: usize| Expr::parse(&join!(token_list.get(..token_list.len() - len)?));
        Some(match operator.as_str() {
            "+" => Oper::Add(has_lhs(2)?, token()?),
            "-" => Oper::Sub(has_lhs(2)?, token()?),
            "*" => Oper::Mul(has_lhs(2)?, token()?),
            "/" => Oper::Div(has_lhs(2)?, token()?),
            "%" => Oper::Mod(has_lhs(2)?, token()?),
            "==" => Oper::Eql(has_lhs(2)?, token()?),
            "!=" => Oper::Neq(has_lhs(2)?, token()?),
            "<" => Oper::Lt(has_lhs(2)?, token()?),
            ">" => Oper::Gt(has_lhs(2)?, token()?),
            ">=" => Oper::GtEq(has_lhs(2)?, token()?),
            "<=" => Oper::LtEq(has_lhs(2)?, token()?),
            ":" => Oper::Cast(has_lhs(2)?, Type::parse(token_list.last()?)?),
            _ => return None,
        })
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Oper::Add(lhs, rhs) => compile_arithmetic!("add", self, ctx, lhs, rhs),
            Oper::Sub(lhs, rhs) => compile_arithmetic!("sub", self, ctx, lhs, rhs),
            Oper::Mul(lhs, rhs) => compile_arithmetic!("mul", self, ctx, lhs, rhs),
            Oper::Div(lhs, rhs) => compile_compare!("div", self, ctx, lhs, rhs),
            Oper::Mod(lhs, rhs) => compile_compare!("rem", self, ctx, lhs, rhs),
            Oper::Eql(lhs, rhs) => compile_arithmetic!("eq", self, ctx, lhs, rhs),
            Oper::Neq(lhs, rhs) => compile_arithmetic!("ne", self, ctx, lhs, rhs),
            Oper::Lt(lhs, rhs) => compile_compare!("lt", self, ctx, lhs, rhs),
            Oper::LtEq(lhs, rhs) => compile_compare!("le", self, ctx, lhs, rhs),
            Oper::Gt(lhs, rhs) => compile_compare!("gt", self, ctx, lhs, rhs),
            Oper::GtEq(lhs, rhs) => compile_compare!("ge", self, ctx, lhs, rhs),
            Oper::Cast(Expr::Deref(expr), rhs) => {
                ctx.deref_type = rhs.clone();
                Expr::Deref(expr.clone()).compile(ctx)?
            }
            Oper::Cast(Expr::Access(array, expr), rhs) => {
                ctx.deref_type = rhs.clone();
                Expr::Access(array.clone(), expr.clone()).compile(ctx)?
            }
            Oper::Cast(lhs, rhs) => {
                format!(
                    "({}.{} {})",
                    rhs.compile(ctx)?,
                    match rhs {
                        Type::Float => "convert_i32_s",
                        Type::Integer => "trunc_f64_s",
                        _ => return lhs.compile(ctx),
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
                Some(rhs.clone())
            }
        }
    }
}
