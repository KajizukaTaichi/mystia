use crate::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Value(Value),
    Ref(String),
    Oper(Box<Oper>),
    Call(String, Vec<Expr>),
    Access(Box<Expr>, Box<Expr>),
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
                // Array
                } else if token.starts_with("[") && token.ends_with("]") {
                    let token = token.get(..token.len() - 1)?.trim();
                    Expr::Value(Value::Array(
                        tokenize(token, &[","], false)?
                            .iter()
                            .map(|x| x.parse::<i32>().unwrap_or(0))
                            .collect(),
                    ))
                // Code block
                } else if token.starts_with("{") && token.ends_with("}") {
                    let token = token.get(1..token.len() - 1)?.trim();
                    Expr::Block(Block::parse(token)?)
                // prioritize higher than others
                } else if token.starts_with("(") && token.ends_with(")") {
                    let token = token.get(1..token.len() - 1)?.trim();
                    Expr::parse(token)?
                // Index access
                } else if token.starts_with("[") && token.ends_with("]") {
                    let token = token.get(..token.len() - 1)?.trim();
                    let (array, index) = token.split_once("[")?;
                    Expr::Access(Box::new(Expr::parse(array)?), Box::new(Expr::parse(index)?))
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
            Expr::Value(Value::Array(x)) => format!(
                "{1} (i32.const {0})",
                ctx.index.clone(),
                join!(
                    x.iter()
                        .map(|i| format!(
                            "(i32.store (i32.mul {} (i32.const 4)) (i32.const {i}))",
                            {
                                let index = ctx.index;
                                ctx.index += 1;
                                index
                            },
                        ))
                        .collect::<Vec<_>>()
                ),
            ),
            Expr::Call(name, args) => format!(
                "(call ${name} {})",
                join!(args.iter().map(|x| x.compile(ctx)).collect::<Vec<_>>())
            ),
            Expr::Access(array, index) => {
                format!(
                    "(i32.load (i32.mul (i32.add {} {}) (i32.const 4)))",
                    array.compile(ctx),
                    index.compile(ctx)
                )
            }
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
            Expr::Value(Value::Array(_)) => Type::Array,
            Expr::Call(name, args) => {
                let _ = args.iter().map(|i| i.type_infer(ctx));
                ctx.function[name].clone()
            }
            Expr::Block(block) => block.type_infer(ctx),
            Expr::Access(_, _) => Type::Integer,
        }
    }
}
