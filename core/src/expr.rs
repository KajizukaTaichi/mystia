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
                let len = array.len();
                let inner_type = array.first()?.type_infer(ctx)?;
                let mut result: Vec<_> = vec![];
                ctx.pointer_index = ctx.alloc_index;
                for elm in array {
                    type_check!(inner_type, elm.type_infer(ctx)?, ctx)?;
                    result.push(format!(
                        "({type}.store {address} {value})",
                        r#type = inner_type.clone().compile(ctx)?,
                        address =
                            Value::Array(ctx.alloc_index, len, inner_type.clone()).compile(ctx)?,
                        value = elm.compile(ctx)?
                    ));
                    ctx.alloc_index += inner_type.bytes_length();
                }
                format!(
                    "{} {}",
                    Value::Array(ctx.pointer_index, len, inner_type).compile(ctx)?,
                    join!(result)
                )
            }
            Expr::Call(name, args) => format!(
                "(call ${name} {})",
                join!(iter_map!(args, |x: &Expr| x.compile(ctx)))
            ),
            Expr::Access(array, index) => {
                let Type::Array(typ, len) = array.type_infer(ctx)? else {
                    return None;
                };
                let addr = Oper::Add(
                    Expr::Oper(Box::new(Oper::Cast(*array.clone(), Type::Integer))),
                    Expr::Oper(Box::new(Oper::Mul(
                        Expr::Oper(Box::new(Oper::Mod(
                            *index.clone(),
                            Expr::Literal(Value::Integer(len as i32)),
                        ))),
                        Expr::Literal(Value::Integer(typ.bytes_length())),
                    ))),
                );
                format!("({}.load {})", typ.compile(ctx)?, addr.compile(ctx)?)
            }
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
            Expr::Array(e) => Type::Array(Box::new(e.first()?.type_infer(ctx)?), e.len()),
            Expr::Literal(literal) => literal.type_infer(ctx)?,
            Expr::Call(name, args) => {
                let (_, args_type, ret_type) = ctx.function_type.get(name)?.clone();
                let _ = iter_map!(args.iter().zip(args_type.values()), |(x, t): (
                    &Expr,
                    &Type
                )| type_check!(
                    x, t, ctx
                ));
                ret_type.clone()
            }
            Expr::Block(block) => block.type_infer(ctx)?,
            Expr::Access(arr, _) => {
                let Type::Array(typ, _) = arr.type_infer(ctx)? else {
                    return None;
                };
                *typ
            }
        })
    }
}
