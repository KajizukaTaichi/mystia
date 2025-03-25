use crate::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Array(Vec<Expr>),
    Variable(String),
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
                // Literal value
                if let Some(literal) = Value::parse(&token) {
                    Expr::Literal(literal)
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
                    Expr::Variable(token)
                } else {
                    return None;
                },
            )
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Expr::Oper(oper) => oper.compile(ctx)?,
            Expr::Variable(to) => format!("(local.get ${to})"),
            Expr::Literal(literal) => literal.compile(ctx)?,
            Expr::Array(array) => {
                let index = Expr::Literal(Value::Pointer(ctx.alloc_index.clone()));
                let mut result: Vec<_> = vec![];
                for elm in array {
                    let elm_type = elm.type_infer(ctx)?;
                    result.push(format!(
                        "({type}.store {address} {value})",
                        r#type = elm_type.compile(ctx)?,
                        address = Value::Integer(ctx.alloc_index).compile(ctx)?,
                        value = elm.compile(ctx)?
                    ));
                    match elm_type {
                        Type::Integer | Type::Pointer | Type::Bool => ctx.alloc_index += 4,
                        Type::Float => ctx.alloc_index += 8,
                        Type::Void => {}
                    }
                }
                format!("{} {}", index.compile(ctx)?, join!(result))
            }
            Expr::Deref(expr) => {
                let addr = &expr.addr_infer(ctx)?.clone();
                let typ = ctx.address_type.get(addr)?.clone();
                format!("({}.load {})", typ.compile(ctx)?, addr)
            }
            Expr::Call(name, args) => format!(
                "(call ${name} {})",
                join!(iter_map!(args, |x: &Expr| x.compile(ctx)))
            ),
            Expr::Access(array, index) => Expr::Deref(Box::new(Expr::Oper(Box::new(Oper::Add(
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
            Expr::Variable(to) => {
                let mut locals = ctx.variable_type.clone();
                locals.extend(ctx.argument_type.clone());
                locals.get(to)?.clone()
            }
            Expr::Array(_) => Type::Pointer,
            Expr::Literal(literal) => literal.type_infer(ctx)?,
            Expr::Deref(_) => Type::Integer,
            Expr::Call(name, args) => {
                let (args_type, ret_type) = ctx.function_type.get(name)?.clone();
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

    fn addr_infer(&self, ctx: &mut Compiler) -> Option<i32> {
        Some(match self {
            Expr::Variable(to) => ctx.variable_addr.get(to)?.clone(),
            Expr::Literal(val) => val.addr_infer(ctx)?,
            Expr::Deref(to) => to.addr_infer(ctx)?,
            Expr::Block(block) => block.addr_infer(ctx)?,
            Expr::Oper(oper) => oper.addr_infer(ctx)?,
            Expr::Call(_, args) => *iter_map!(args, |x: &Expr| x.addr_infer(ctx)).last()?,
            Expr::Access(arr, idx) => {
                *iter_map!([arr, idx], |x: &Expr| x.addr_infer(ctx)).last()?
            }
            Expr::Array(arr) => *iter_map!(arr, |x: &Expr| x.addr_infer(ctx)).last()?,
        })
    }
}
