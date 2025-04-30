use crate::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Array(Vec<Expr>),
    Dict(IndexMap<String, Expr>),
    Variable(String),
    Oper(Box<Oper>),
    Call(String, Vec<Expr>),
    Field(Box<Expr>, String),
    Access(Box<Expr>, Box<Expr>),
    Block(Block),
}

impl Node for Expr {
    fn parse(source: &str) -> Option<Expr> {
        let source = source.trim();
        let token_list: Vec<String> = tokenize(source, SPACE.as_ref(), true, true)?;
        if token_list.len() >= 3 {
            Some(Expr::Oper(Box::new(Oper::parse(source)?)))
        } else if token_list.len() == 2 {
            let oper = token_list.first()?.trim();
            let token = token_list.last()?.trim();
            Some(
                // Bit NOT operation
                if oper == "~" {
                    Expr::Oper(Box::new(Oper::BNot(Expr::parse(token)?)))
                    // Bit NOT operation
                } else if oper == "!" {
                    Expr::Oper(Box::new(Oper::LNot(Expr::parse(token)?)))
                } else {
                    return None;
                },
            )
        } else {
            let token = token_list.last()?.trim();
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
                    if let Some(block) = Block::parse(token) {
                        Expr::Block(block)
                    } else {
                        // If not a block, parse as dictionary
                        let mut result = IndexMap::new();
                        for line in tokenize(token, &[","], false, true)? {
                            let (name, value) = line.split_once("=")?;
                            result.insert(name.trim().to_string(), Expr::parse(value)?);
                        }
                        Expr::Dict(result)
                    }
                // Prioritize higher than others
                } else if token.starts_with("(") && token.ends_with(")") {
                    let token = token.get(1..token.len() - 1)?.trim();
                    Expr::parse(token)?
                // syntax sugar of memcpy statement
                } else if token.starts_with("memcpy(") && token.ends_with(")") {
                    let token = token.get("memcpy(".len()..token.len() - 1)?.trim();
                    Expr::Block(Block(vec![Stmt::MemCpy {
                        from: Expr::parse(token)?,
                    }]))
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
                // Dictionary access `dict.key`
                } else if token.contains(".") {
                    let (name, key) = token.rsplit_once(".")?;
                    Expr::Field(Box::new(Expr::parse(name)?), key.to_string())
                // Variable reference
                } else if !RESERVED.contains(&token) && token.is_ascii() {
                    Expr::Variable(token.to_string())
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
                let array = array.clone();
                let mut result: Vec<_> = vec![];
                let pointer;

                if_ptr!(
                    inner_type,
                    // if inner type is pointer (not primitive)
                    {
                        let mut inner_codes = vec![];
                        for elm in array {
                            type_check!(inner_type, elm.type_infer(ctx)?, ctx)?;
                            inner_codes.push(elm.compile(ctx)?)
                        }
                        pointer = ctx.allocator;
                        for code in inner_codes {
                            result.push(format!(
                                "({type}.store {address} {code})",
                                r#type = &inner_type.compile(ctx)?,
                                address = Value::Array(ctx.allocator, len, inner_type.clone())
                                    .compile(ctx)?,
                            ));
                            ctx.allocator += inner_type.pointer_length();
                        }
                    } else {
                        pointer = ctx.allocator;
                        for elm in array {
                            type_check!(inner_type, elm.type_infer(ctx)?, ctx)?;
                            result.push(format!(
                                "({type}.store {address} {value})",
                                r#type = &inner_type.compile(ctx)?,
                                address = Value::Array(ctx.allocator, len, inner_type.clone())
                                    .compile(ctx)?,
                                value = elm.compile(ctx)?
                            ));
                            ctx.allocator += inner_type.pointer_length();
                        }
                    }
                );
                format!(
                    "{} {}",
                    Value::Array(pointer, len, inner_type).compile(ctx)?,
                    join!(result)
                )
            }
            Expr::Dict(dict) => {
                let mut result: Vec<_> = vec![];
                let Type::Dict(infered) = self.type_infer(ctx)? else {
                    return None;
                };

                let mut prestore = IndexMap::new();
                for (name, elm) in dict {
                    let typ = elm.type_infer(ctx)?;
                    if_ptr!(typ, { prestore.insert(name, elm.compile(ctx)?) });
                }

                let pointer = ctx.allocator;
                for (name, elm) in dict {
                    let typ = elm.type_infer(ctx)?;
                    result.push(format!(
                        "({type}.store {address} {value})",
                        r#type = typ.clone().compile(ctx)?,
                        address = Value::Dict(ctx.allocator, infered.clone()).compile(ctx)?,
                        value = if let Some(precode) = prestore.get(name) {
                            precode.clone()
                        } else {
                            elm.compile(ctx)?
                        }
                    ));
                    ctx.allocator += typ.pointer_length();
                }

                format!(
                    "{} {}",
                    Value::Dict(pointer, infered).compile(ctx)?,
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
                        Expr::Literal(Value::Integer(typ.pointer_length())),
                    ))),
                );
                format!("({}.load {})", typ.compile(ctx)?, addr.compile(ctx)?)
            }
            Expr::Field(expr, key) => {
                let Type::Dict(dict) = expr.type_infer(ctx)? else {
                    return None;
                };
                let (offset, typ) = dict.get(key)?.clone();
                let addr = Oper::Add(
                    Expr::Oper(Box::new(Oper::Cast(*expr.clone(), Type::Integer))),
                    Expr::Literal(Value::Integer(offset.clone())),
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
                if let Some(local) = ctx.variable_type.get(to) {
                    local.clone()
                } else if let Some(arg) = ctx.argument_type.get(to) {
                    arg.clone()
                } else if let Some(expect) = ctx.expect_type.clone() {
                    ctx.expect_type = None;
                    ctx.argument_type.insert(to.to_owned(), expect.clone());
                    expect.clone()
                } else {
                    return None;
                }
            }
            Expr::Array(e) => Type::Array(Box::new(e.first()?.type_infer(ctx)?), e.len()),
            Expr::Dict(dict) => {
                let mut result = IndexMap::new();
                let mut index: i32 = 0;
                for (name, elm) in dict {
                    let typ = elm.type_infer(ctx)?;
                    result.insert(name.to_string(), (index, typ.clone()));
                    index += typ.pointer_length();
                }
                Type::Dict(result)
            }
            Expr::Literal(literal) => literal.type_infer(ctx)?,
            Expr::Call(name, args) => {
                let (_, args_type, ret_type) = ctx.function_type.get(name)?.clone();
                let _ = iter_map!(args.iter().zip(args_type.values()), |(arg, typ): (
                    &Expr,
                    &Type
                )| type_check!(
                    arg, typ, ctx
                ));
                ret_type.clone()
            }
            Expr::Block(block) => block.type_infer(ctx)?,
            Expr::Access(arr, _) => {
                let infered = arr.type_infer(ctx)?;
                let Some(Type::Array(typ, _)) = infered.type_infer(ctx) else {
                    let error_message = format!("can't index access to {}", infered.format());
                    ctx.occurred_error = Some(error_message);
                    return None;
                };
                *typ
            }
            Expr::Field(dict, key) => {
                let infered = dict.type_infer(ctx)?;
                let Type::Dict(dict) = infered.clone() else {
                    let error_message = format!("can't field access to {}", infered.format());
                    ctx.occurred_error = Some(error_message);
                    return None;
                };
                let Some((_offset, typ)) = dict.get(key) else {
                    let error_message = format!("{} haven't property \"{key}\"", infered.format());
                    ctx.occurred_error = Some(error_message);
                    return None;
                };
                typ.clone()
            }
        })
    }
}
