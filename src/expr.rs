use crate::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Value(Value),
    Oper(Box<Oper>),
}

impl Expr {
    pub fn parse(source: &str) -> Option<Expr> {
        let source = source.trim();
        let token_list: Vec<String> = tokenize(source.trim(), SPACE.as_ref(), true)?;
        if token_list.len() >= 2 {
            Some(Expr::Oper(Box::new(Oper::parse(source)?)))
        } else {
            let token = token_list.last()?.trim().to_string();
            Some(if let Ok(n) = token.parse::<i32>() {
                Expr::Value(Value::Integer(n))
            // prioritize higher than others
            } else if token.starts_with("(") && token.ends_with(")") {
                let token = token.get(1..token.len() - 1)?.trim();
                Expr::parse(token)?
            // Variable reference
            } else {
                return None;
            })
        }
    }

    pub fn compile(&self) -> String {
        match self {
            Expr::Oper(oper) => oper.compile(),
            Expr::Value(Value::Integer(n)) => format!("(i32.const {n})"),
        }
    }
}
