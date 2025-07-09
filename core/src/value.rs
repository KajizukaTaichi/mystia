use crate::*;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Number(f64),
    Bool(bool),
    Array(Vec<Expr>),
    Dict(IndexMap<String, Expr>),
    Enum(Type, String),
    String(String),
    Null,
}

impl Node for Value {
    fn parse(source: &str) -> Option<Self> {
        // Integer literal
        if let Ok(n) = source.parse::<i32>() {
            Some(Value::Integer(n))
        // Number literal
        } else if let Ok(n) = source.parse::<f64>() {
            Some(Value::Number(n))
        // Boolean literal
        } else if let Ok(n) = source.parse::<bool>() {
            Some(Value::Bool(n))
        // String literal
        } else if source.starts_with("\"") && source.ends_with("\"") {
            let source = source.get(1..source.len() - 1)?;
            Some(Value::String(source.to_string()))
        // Array `[expr, ...]`
        } else if source.starts_with("[") && source.ends_with("]") {
            let source = source.get(1..source.len() - 1)?.trim();
            let elms = tokenize(source, &[","], false, true, false)?;
            let elms = elms.iter().map(|i| Expr::parse(&i));
            Some(Value::Array(elms.collect::<Option<Vec<_>>>()?))
        // Dict `{ field expr, ... }`
        } else if source.starts_with("@{") && source.ends_with("}") {
            let token = source.get(2..source.len() - 1)?.trim();
            let mut result = IndexMap::new();
            for line in tokenize(token, &[","], false, true, false)? {
                let (name, value) = line.trim().split_once(":")?;
                result.insert(name.trim().to_string(), Expr::parse(value)?);
            }
            Some(Value::Dict(result))
        } else if source == "null" {
            Some(Value::Null)
        } else {
            None
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Value::Number(n) => format!("(f64.const {n})"),
            Value::Integer(n) => format!("(i32.const {n})"),
            Value::Bool(n) => Value::Integer(if *n { 1 } else { 0 }).compile(ctx)?,
            Value::String(str) => {
                let result = Value::Integer(ctx.allocator).compile(ctx)?;
                ctx.static_data
                    .push(format!(r#"(data {} "{str}\00")"#, result));
                ctx.allocator += str.len() as i32 + 1;
                result
            }
            Value::Array(array) => {
                let inner_type = array.first()?.type_infer(ctx)?;
                let array = array.clone();
                let mut result: Vec<_> = vec![];
                let pointer;

                if_ptr!(
                    inner_type =>
                    // if inner type is pointer (not primitive)
                    {
                        let mut inner_codes = vec![];
                        for elm in array.clone() {
                            type_check!(inner_type, elm.type_infer(ctx)?, ctx)?;
                            inner_codes.push(elm.compile(ctx)?)
                        }
                        pointer = ctx.allocator;
                        result.push(format!(
                            "(i32.store {address} {length})",
                            address = Value::Integer(ctx.allocator).compile(ctx)?,
                            length = Value::Integer(array.len() as i32).compile(ctx)?
                        ));
                        ctx.allocator += 4;
                        for code in inner_codes {
                            result.push(format!(
                                "({type}.store {address} {code})",
                                r#type = &inner_type.compile(ctx)?,
                                address = Value::Integer(ctx.allocator).compile(ctx)?,
                            ));
                            ctx.allocator += inner_type.pointer_length(ctx)?;
                        }
                    } else {
                        pointer = ctx.allocator;
                        result.push(format!(
                            "(i32.store {address} {length})",
                            address = Value::Integer(ctx.allocator).compile(ctx)?,
                            length = Value::Integer(array.len() as i32).compile(ctx)?
                        ));
                        ctx.allocator += 4;
                        for elm in array {
                            type_check!(inner_type, elm.type_infer(ctx)?, ctx)?;
                            result.push(format!(
                                "({type}.store {address} {value})",
                                r#type = &inner_type.compile(ctx)?,
                                address = Value::Integer(ctx.allocator).compile(ctx)?,
                                value = elm.compile(ctx)?
                            ));
                            ctx.allocator += inner_type.pointer_length(ctx)?;
                        }
                    }
                );
                format!(
                    "{} {}",
                    Value::Integer(pointer,).compile(ctx)?,
                    join!(result)
                )
            }
            Value::Dict(dict) => {
                let mut result: Vec<_> = vec![];
                let Type::Dict(_) = self.type_infer(ctx)? else {
                    return None;
                };

                let mut prestore = IndexMap::new();
                for (name, elm) in dict {
                    let typ = elm.type_infer(ctx)?;
                    if_ptr!(typ => { prestore.insert(name, elm.compile(ctx)?) });
                }

                let pointer = ctx.allocator;
                for (name, elm) in dict {
                    let typ = elm.type_infer(ctx)?;
                    result.push(format!(
                        "({type}.store {address} {value})",
                        r#type = typ.clone().compile(ctx)?,
                        address = Value::Integer(ctx.allocator).compile(ctx)?,
                        value = prestore.get(name).cloned().or_else(|| elm.compile(ctx))?
                    ));
                    ctx.allocator += typ.pointer_length(ctx)?;
                }

                format!(
                    "{} {}",
                    Value::Integer(pointer).compile(ctx)?,
                    join!(result)
                )
            }
            Value::Enum(typ, key) => {
                let typ = typ.type_infer(ctx)?;
                let Type::Enum(enum_type) = typ.clone() else {
                    let error_message = format!("can't access enumerator to {}", typ.format());
                    ctx.occurred_error = Some(error_message);
                    return None;
                };
                let Some(value) = enum_type.iter().position(|item| item == key) else {
                    let error_message = format!("`{key}` is invalid variant of {}", typ.format());
                    ctx.occurred_error = Some(error_message);
                    return None;
                };
                Value::Integer(value as i32).compile(ctx)?
            }
            Value::Null => Value::Integer(-1).compile(ctx)?,
        })
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        Some(match self {
            Value::Number(_) => Type::Number,
            Value::Integer(_) => Type::Integer,
            Value::Bool(_) => Type::Bool,
            Value::String(_) => Type::String,
            Value::Array(e) => Type::Array(Box::new(e.first()?.type_infer(ctx)?)),
            Value::Dict(dict) => {
                let mut result = IndexMap::new();
                let mut index: i32 = 0;
                for (name, elm) in dict {
                    let typ = elm.type_infer(ctx)?;
                    result.insert(name.to_string(), (index, typ.clone()));
                    index += typ.pointer_length(ctx)?;
                }
                Type::Dict(result)
            }
            Value::Enum(typ, _) => typ.type_infer(ctx)?,
            Value::Null => Type::Any,
        })
    }
}
