use crate::*;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Number(f64),
    Bool(bool),
    Array(Vec<Expr>),
    Dict(IndexMap<String, Expr>),
    Enum(i32, Enum),
    String(String),
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
            let source = source.get(1..source.len() - 1)?.trim();
            Some(Value::String(str_escape(source)))
        // Array `[expr, ...]`
        } else if source.starts_with("[") && source.ends_with("]") {
            let source = source.get(1..source.len() - 1)?.trim();
            let elms = tokenize(source, &[","], false, true)?;
            let elms = elms.iter().map(|i| Expr::parse(&i));
            Some(Value::Array(elms.collect::<Option<Vec<_>>>()?))
        // Dict `{ field = value, ... }`
        } else if source.starts_with("{") && source.ends_with("}") {
            let token = source.get(1..source.len() - 1)?.trim();
            let mut result = IndexMap::new();
            for line in tokenize(token, &[","], false, true)? {
                let (name, value) = line.split_once(" ")?;
                result.insert(name.trim().to_string(), Expr::parse(value)?);
            }
            Some(Value::Dict(result))
        } else {
            None
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Value::Number(n) => format!("(f64.const {n})"),
            Value::Integer(n) => format!("(i32.const {n})"),
            Value::Bool(n) => Value::Integer(if *n { 1 } else { 0 }).compile(ctx)?,
            Value::Enum(tag, _) => Value::Integer(*tag).compile(ctx)?,
            Value::String(str) => {
                let len = str.len() + 1;
                let result = Value::Integer(ctx.allocator).compile(ctx)?;
                ctx.static_data
                    .push(format!(r#"(data {} "{str}\00")"#, result));
                ctx.allocator += len as i32;
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
                        for elm in array {
                            type_check!(inner_type, elm.type_infer(ctx)?, ctx)?;
                            inner_codes.push(elm.compile(ctx)?)
                        }
                        pointer = ctx.allocator;
                        for code in inner_codes {
                            result.push(format!(
                                "({type}.store {address} {code})",
                                r#type = &inner_type.compile(ctx)?,
                                address = Value::Integer(ctx.allocator).compile(ctx)?,
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
                                address = Value::Integer(ctx.allocator).compile(ctx)?,
                                value = elm.compile(ctx)?
                            ));
                            ctx.allocator += inner_type.pointer_length();
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
                    ctx.allocator += typ.pointer_length();
                }

                format!(
                    "{} {}",
                    Value::Integer(pointer).compile(ctx)?,
                    join!(result)
                )
            }
        })
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        Some(match self {
            Value::Number(_) => Type::Number,
            Value::Integer(_) => Type::Integer,
            Value::Bool(_) => Type::Bool,
            Value::String(_) => Type::String,
            Value::Array(e) => Type::Array(Box::new(e.first()?.type_infer(ctx)?), e.len()),
            Value::Dict(dict) => {
                let mut result = IndexMap::new();
                let mut index: i32 = 0;
                for (name, elm) in dict {
                    let typ = elm.type_infer(ctx)?;
                    result.insert(name.to_string(), (index, typ.clone()));
                    index += typ.pointer_length();
                }
                Type::Dict(result)
            }
            Value::Enum(_, typ) => Type::Enum(typ.clone()),
        })
    }
}
