use crate::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Value(Value),
    Ref(String),
    Oper(Box<Oper>),
    Call(String, Vec<Expr>),
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
            } else if token.contains("(") && token.ends_with(")") {
                let token = token.get(..token.len() - 1)?.trim();
                let (name, args) = token.split_once("(")?;
                let args = {
                    let mut result = vec![];
                    for i in tokenize(args, &[","], false)? {
                        result.push(Expr::parse(&i)?)
                    }
                    result
                };
                Expr::Call(name.to_string(), args)
            // Variable reference
            } else {
                Expr::Ref(token)
            })
        }
    }

    pub fn compile(&self) -> String {
        match self {
            Expr::Oper(oper) => oper.compile(),
            Expr::Ref(to) => format!("(local.get ${to})"),
            Expr::Value(Value::Integer(n)) => format!("(i32.const {n})"),
            Expr::Call(name, args) => format!(
                "(call ${name} {}",
                join!(args.iter().map(|x| x.compile()).collect::<Vec<_>>())
            ),
        }
    }
}
