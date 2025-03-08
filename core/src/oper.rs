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
}

impl Node for Oper {
    fn parse(source: &str) -> Option<Self> {
        let token_list: Vec<String> = tokenize(source, SPACE.as_ref(), true)?;
        let token = Expr::parse(token_list.last()?)?;
        let operator = token_list.get(token_list.len().checked_sub(2)?)?;
        let has_lhs = |len: usize| Expr::parse(&join!(token_list.get(..token_list.len() - len)?));
        Some(match operator.as_str() {
            "+" => Oper::Add(has_lhs(2)?, token),
            "-" => Oper::Sub(has_lhs(2)?, token),
            "*" => Oper::Mul(has_lhs(2)?, token),
            "/" => Oper::Div(has_lhs(2)?, token),
            "%" => Oper::Mod(has_lhs(2)?, token),
            "==" => Oper::Eql(has_lhs(2)?, token),
            "!=" => Oper::Neq(has_lhs(2)?, token),
            "<" => Oper::Lt(has_lhs(2)?, token),
            ">" => Oper::Gt(has_lhs(2)?, token),
            ">=" => Oper::GtEq(has_lhs(2)?, token),
            "<=" => Oper::LtEq(has_lhs(2)?, token),
            _ => return None,
        })
    }

    fn compile(&self, ctx: &mut Compiler) -> String {
        match self {
            Oper::Add(lhs, rhs) => format!("(i32.add {} {})", lhs.compile(ctx), rhs.compile(ctx)),
            Oper::Sub(lhs, rhs) => format!("(i32.sub {} {})", lhs.compile(ctx), rhs.compile(ctx)),
            Oper::Mul(lhs, rhs) => format!("(i32.mul {} {})", lhs.compile(ctx), rhs.compile(ctx)),
            Oper::Div(lhs, rhs) => format!("(i32.div_s {} {})", lhs.compile(ctx), rhs.compile(ctx)),
            Oper::Mod(lhs, rhs) => format!("(i32.rem_s {} {})", lhs.compile(ctx), rhs.compile(ctx)),
            Oper::Eql(lhs, rhs) => format!("(i32.eq {} {})", lhs.compile(ctx), rhs.compile(ctx)),
            Oper::Neq(lhs, rhs) => format!("(i32.ne {} {})", lhs.compile(ctx), rhs.compile(ctx)),
            Oper::Lt(lhs, rhs) => format!("(i32.lt_s {} {})", lhs.compile(ctx), rhs.compile(ctx)),
            Oper::Gt(lhs, rhs) => format!("(i32.gt_s {} {})", lhs.compile(ctx), rhs.compile(ctx)),
            Oper::LtEq(lhs, rhs) => format!("(i32.le_s {} {})", lhs.compile(ctx), rhs.compile(ctx)),
            Oper::GtEq(lhs, rhs) => format!("(i32.ge_s {} {})", lhs.compile(ctx), rhs.compile(ctx)),
        }
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Type {
        match self {
            Oper::Add(lhs, _) => lhs.type_infer(ctx),
            Oper::Sub(lhs, _) => lhs.type_infer(ctx),
            Oper::Mul(lhs, _) => lhs.type_infer(ctx),
            Oper::Div(lhs, _) => lhs.type_infer(ctx),
            Oper::Mod(lhs, _) => lhs.type_infer(ctx),
            Oper::Eql(lhs, _) => lhs.type_infer(ctx),
            Oper::Neq(lhs, _) => lhs.type_infer(ctx),
            Oper::Lt(lhs, _) => lhs.type_infer(ctx),
            Oper::Gt(lhs, _) => lhs.type_infer(ctx),
            Oper::LtEq(lhs, _) => lhs.type_infer(ctx),
            Oper::GtEq(lhs, _) => lhs.type_infer(ctx),
        }
    }
}
