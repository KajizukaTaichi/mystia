use crate::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Array(Vec<Expr>),
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
        let token_list: Vec<String> = tokenize(source.trim(), SPACE.as_ref(), true, true)?;
        if token_list.len() >= 2 {
            Some(Expr::Oper(Box::new(Oper::parse(source)?)))
        } else {
            let token = token_list.last()?.trim().to_string();
            Some(
                // Integer literal
                if let Ok(n) = token.parse::<i32>() {
                    Expr::Literal(Value::Integer(n))
                // Float number literal
                } else if let Ok(n) = token.parse::<f64>() {
                    Expr::Literal(Value::Float(n))
                // Boolean literal
                } else if let Ok(n) = token.parse::<bool>() {
                    Expr::Literal(Value::Bool(n))
                // Pointer access
                } else if token.starts_with("@") {
                    let token = token.get(1..)?.trim();
                    Expr::Pointer(Box::new(Expr::parse(token)?))
                // String literal
                } else if token.starts_with("\"") && token.ends_with("\"") {
                    let token = token.get(1..token.len() - 1)?.trim();
                    Expr::Literal(Value::String(str_escape(token)))
                // Array `[expr, ...]`
                } else if token.starts_with("[") && token.ends_with("]") {
                    let token = token.get(1..token.len() - 1)?.trim();
                    let mut result = vec![];
                    for i in tokenize(token, &[","], false, true)? {
                        result.push(Expr::parse(&i)?);
                    }
                    Expr::Array(result)
                // Code block `{ stmt; ... }`
                } else if token.starts_with("{") && token.ends_with("}") {
                    let token = token.get(1..token.len() - 1)?.trim();
                    Expr::Block(Block::parse(token)?)
                // Prioritize higher than others
                } else if token.starts_with("(") && token.ends_with(")") {
                    let token = token.get(1..token.len() - 1)?.trim();
                    Expr::parse(token)?
                // Index access `array[index]`
                } else if token.contains("[") && token.ends_with("]") {
                    let token = token.get(..token.len() - 1)?.trim();
                    let (array, index) = token.rsplit_once("[")?;
                    Expr::Access(Box::new(Expr::parse(array)?), Box::new(Expr::parse(index)?))
                // Function call `name(args, ...)`
                } else if token.contains("(") && token.ends_with(")") {
                    let token = token.get(..token.len() - 1)?.trim();
                    let (name, args) = token.split_once("(")?;
                    let args = {
                        let mut result = vec![];
                        for i in tokenize(args, &[","], false, true)? {
                            result.push(Expr::parse(&i)?)
                        }
                        result
                    };
                    Expr::Call(name.to_string(), args)
                // Variable reference
                } else if !RESERVED.contains(&token.as_str()) {
                    Expr::Refer(token)
                } else {
                    return None;
                },
            )
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Expr::Oper(oper) => oper.compile(ctx)?,
            Expr::Refer(to) => format!("(local.get ${to})"),
            Expr::Literal(Value::Integer(n)) => format!("(i32.const {n})"),
            Expr::Literal(Value::Float(n)) => format!("(f64.const {n})"),
            Expr::Literal(Value::Bool(n)) => {
                Expr::Literal(Value::Integer(if *n { 1 } else { 0 })).compile(ctx)?
            }
            Expr::Array(array) => {
                let result = Expr::Literal(Value::Integer(ctx.index.clone()));
                for elm in array {
                    let code = Stmt::Let {
                        name: Expr::Pointer(Box::new(Expr::Literal(Value::Integer(ctx.index)))),
                        value: elm.clone(),
                    }
                    .compile(ctx)?;
                    ctx.array.push(code);
                    ctx.index += 1;
                }
                result.compile(ctx)?
            }
            Expr::Literal(Value::String(str)) => {
                let result = Expr::Literal(Value::Integer(ctx.index.clone())).compile(ctx)?;
                ctx.data.push(format!(r#"(data {} "{str}")"#, result));
                ctx.index += str.len() as i32;
                result
            }
            Expr::Pointer(expr) => {
                format!(
                    "(i32.load {})",
                    Expr::Oper(Box::new(Oper::Mul(
                        *expr.clone(),
                        Expr::Literal(Value::Integer(4))
                    )))
                    .compile(ctx)?
                )
            }
            Expr::Call(name, args) => format!(
                "(call ${name} {})",
                join!(iter_map!(args, |x: &Expr| x.compile(ctx)))
            ),
            Expr::Access(array, index) => Expr::Pointer(Box::new(Expr::Oper(Box::new(Oper::Add(
                *array.clone(),
                *index.clone(),
            )))))
            .compile(ctx)?,
            Expr::Block(block) => block.compile(ctx)?,
        })
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        Some(match self {
            Expr::Oper(oper) => oper.type_infer(ctx)?,
            Expr::Refer(to) => {
                let mut locals = ctx.variable.clone();
                locals.extend(ctx.argument.clone());
                locals.get(to)?.clone()
            }
            Expr::Array(_) => Type::Pointer,
            Expr::Literal(Value::Integer(_)) => Type::Integer,
            Expr::Literal(Value::Bool(_)) => Type::Bool,
            Expr::Literal(Value::Float(_)) => Type::Float,
            Expr::Literal(Value::String(_)) => Type::Pointer,
            Expr::Pointer(_) => Type::Integer,
            Expr::Call(name, args) => {
                let (args_type, ret_type) = ctx.function.get(name)?.clone();
                let _ = iter_map!(
                    args.iter().zip(args_type),
                    |(x, t): (&Expr, Type)| type_check!(x, t, ctx)
                );
                ret_type.clone()
            }
            Expr::Block(block) => block.type_infer(ctx)?,
            Expr::Access(_, _) => Type::Integer,
        })
    }
}
