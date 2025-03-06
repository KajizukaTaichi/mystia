use crate::*;

#[derive(Debug, Clone)]
pub enum Oper {
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Eql(Expr, Expr),
    Mod(Expr, Expr),
}

impl Oper {
    pub fn parse(source: &str) -> Option<Self> {
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
            _ => return None,
        })
    }

    pub fn compile(&self) -> String {
        match self {
            Oper::Add(lhs, rhs) => format!("(i32.add {} {})", lhs.compile(), rhs.compile()),
            Oper::Sub(lhs, rhs) => format!("(i32.sub {} {})", lhs.compile(), rhs.compile()),
            Oper::Mul(lhs, rhs) => format!("(i32.mul {} {})", lhs.compile(), rhs.compile()),
            Oper::Div(lhs, rhs) => format!("(i32.div_s {} {})", lhs.compile(), rhs.compile()),
            Oper::Eql(lhs, rhs) => format!("(i32.eq {} {})", lhs.compile(), rhs.compile()),
            Oper::Mod(lhs, rhs) => format!("(i32.rem_s {} {})", lhs.compile(), rhs.compile()),
        }
    }
}
