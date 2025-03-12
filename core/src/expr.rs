use crate::{lexer::str_escape, *};

#[derive(Debug, Clone)]
pub enum Expr {
    Value(Value),
    Refer(String),
    Pointer(Box<Expr>),
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
                // Pointer access
                } else if token.starts_with("@") {
                    let token = token.get(1..)?.trim();
                    Expr::Pointer(Box::new(Expr::parse(token)?))
                // String literal
                } else if token.starts_with("\"") && token.ends_with("\"") {
                    let token = token.get(1..token.len() - 1)?.trim();
                    Expr::Value(Value::String(str_escape(token)))
                // Array
                } else if token.starts_with("[") && token.ends_with("]") {
                    let token = token.get(1..token.len() - 1)?.trim();
                    Expr::Value(Value::Array(
                        tokenize(token, &[","], false)?
                            .iter()
                            .map(|x| x.trim().parse::<i32>().unwrap_or(0))
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
                // Index access `array[index]`
                } else if token.contains("[") && token.ends_with("]") {
                    let token = token.get(..token.len() - 1)?.trim();
                    let (array, index) = token.split_once("[")?;
                    Expr::Access(Box::new(Expr::parse(array)?), Box::new(Expr::parse(index)?))
                // Function call `name(args, ...)`
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
                    Expr::Refer(token)
                },
            )
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> String {
        match self {
            Expr::Oper(oper) => oper.compile(ctx),
            Expr::Refer(to) => format!("(local.get ${to})"),
            Expr::Value(Value::Integer(n)) => format!("(i32.const {n})"),
            Expr::Value(Value::Float(n)) => format!("(f64.const {n})"),
            Expr::Value(Value::Array(x)) => {
                let result = Expr::Value(Value::Integer(ctx.index.clone()));
                for i in x {
                    let code = Stmt::Let {
                        name: Expr::Pointer(Box::new(Expr::Value(Value::Integer(ctx.index)))),
                        value: Expr::Value(Value::Integer(*i)),
                    }
                    .compile(ctx);
                    ctx.array.push(code);
                    ctx.index += 1;
                }
                result.compile(ctx)
            }
            Expr::Value(Value::String(x)) => {
                let result = Expr::Value(Value::Integer(ctx.index.clone())).compile(ctx);
                ctx.data.push(format!(r#"(data {} "{x}")"#, result));
                result
            }
            Expr::Pointer(expr) => {
                format!(
                    "(i32.load {})",
                    Expr::Oper(Box::new(Oper::Mul(
                        *expr.clone(),
                        Expr::Value(Value::Integer(4))
                    )))
                    .compile(ctx)
                )
            }
            Expr::Call(name, args) => format!(
                "(call ${name} {})",
                join!(args.iter().map(|x| x.compile(ctx)).collect::<Vec<_>>())
            ),
            Expr::Access(array, index) => Expr::Pointer(Box::new(Expr::Oper(Box::new(Oper::Add(
                *array.clone(),
                *index.clone(),
            )))))
            .compile(ctx),
            Expr::Block(block) => block.compile(ctx),
        }
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Type {
        match self {
            Expr::Oper(oper) => oper.type_infer(ctx),
            Expr::Refer(to) => {
                let mut locals = ctx.variable.clone();
                locals.extend(ctx.argument.clone());
                locals[to].clone()
            }
            Expr::Value(Value::Integer(_)) => Type::Integer,
            Expr::Value(Value::Float(_)) => Type::Float,
            Expr::Value(Value::Array(_)) | Expr::Value(Value::String(_)) => Type::Pointer,
            Expr::Pointer(_) => Type::Pointer,
            Expr::Call(name, args) => {
                if name == "fd_write" {
                    ctx.stdout = true;
                    Type::Integer
                } else {
                    let _ = args.iter().map(|i| i.type_infer(ctx));
                    ctx.function[name].clone()
                }
            }
            Expr::Block(block) => block.type_infer(ctx),
            Expr::Access(_, _) => Type::Integer,
        }
    }
}
