use crate::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Value(Value),
    Ref(String),
    Oper(Box<Oper>),
    Call(String, Vec<Expr>),
    Block(Block),
}

impl Node for Expr {
    fn parse(source: &str) -> Option<Expr> {
        let source = source.trim();
        let token_list: Vec<String> = tokenize(source.trim(), SPACE.as_ref(), true)?;
        if token_list.len() >= 2 {
            Some(Expr::Oper(Box::new(Oper::parse(source)?)))
        } else {
            let token = token_list.last()?.trim().to_string();
            Some(
                // Integer literal
                if let Ok(n) = token.parse::<i32>() {
                    Expr::Value(Value::Integer(n))
                // Float number literal
                } else if let Ok(n) = token.parse::<f64>() {
                    Expr::Value(Value::Float(n))
                // Code block
                } else if token.starts_with("{") && token.ends_with("}") {
                    let token = token.get(1..token.len() - 1)?.trim();
                    Expr::Block(Block::parse(token)?)
                // prioritize higher than others
                } else if token.starts_with("(") && token.ends_with(")") {
                    let token = token.get(1..token.len() - 1)?.trim();
                    Expr::parse(token)?
                // Function call
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
                },
            )
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> String {
        match self {
            Expr::Oper(oper) => oper.compile(ctx),
            Expr::Ref(to) => format!("(local.get ${to})"),
            Expr::Value(Value::Integer(n)) => format!("(i32.const {n})"),
            Expr::Value(Value::Float(n)) => format!("(f64.const {n})"),
            Expr::Call(name, args) => match name.as_str() {
                "array.get" => {
                    format!(
                        "(i32.load (i32.mul {} (i32.const 4)))",
                        args[0].compile(ctx)
                    )
                }
                "array.set" => {
                    format!(
                        "(i32.store (i32.mul {} (i32.const 4)) {})",
                        args[0].compile(ctx),
                        args[1].compile(ctx),
                    )
                }
                _ => format!(
                    "(call ${name} {})",
                    join!(args.iter().map(|x| x.compile(ctx)).collect::<Vec<_>>())
                ),
            },

            Expr::Block(block) => block.compile(ctx),
        }
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Type {
        match self {
            Expr::Oper(oper) => oper.type_infer(ctx),
            Expr::Ref(to) => {
                let mut locals = ctx.variable.clone();
                locals.extend(ctx.argument.clone());
                locals[to].clone()
            }
            Expr::Value(Value::Integer(_)) => Type::Integer,
            Expr::Value(Value::Float(_)) => Type::Float,
            Expr::Call(name, args) => {
                let _ = args.iter().map(|i| i.type_infer(ctx));
                ctx.function[name].clone()
            }
            Expr::Block(block) => block.type_infer(ctx),
        }
    }
}
