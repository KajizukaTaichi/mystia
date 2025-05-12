use crate::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Variable(String),
    Oper(Box<Oper>),
    Call(String, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Field(Box<Expr>, String),
    Block(Block),
    MemCpy(Box<Expr>),
}

impl Node for Expr {
    fn parse(source: &str) -> Option<Expr> {
        let source = source.trim();
        let token_list: Vec<String> = tokenize(source, SPACE.as_ref(), true, true)?;
        if token_list.len() >= 2 {
            return Some(Expr::Oper(Box::new(Oper::parse(source)?)));
        };
        let token = token_list.last()?.trim();

        // Literal value
        if let Some(literal) = Value::parse(&token) {
            Some(Expr::Literal(literal))
        // Code block `{ stmt; ... }`
        } else if token.starts_with("{") && token.ends_with("}") {
            let token = token.get(1..token.len() - 1)?.trim();
            Some(Expr::Block(Block::parse(token)?))
        // Prioritize higher than others
        } else if token.starts_with("(") && token.ends_with(")") {
            let token = token.get(1..token.len() - 1)?.trim();
            Some(Expr::parse(token)?)
        // syntax sugar of memcpy statement
        } else if token.starts_with("memcpy(") && token.ends_with(")") {
            let token = token.get("memcpy(".len()..token.len() - 1)?.trim();
            Some(Expr::MemCpy(Box::new(Expr::parse(token)?)))
        // Index access `array[index]`
        } else if token.contains("[") && token.ends_with("]") {
            let token = token.get(..token.len() - 1)?.trim();
            let (array, index) = token.rsplit_once("[")?;
            Some(Expr::Index(
                Box::new(Expr::parse(array)?),
                Box::new(Expr::parse(index)?),
            ))
        // Function call `name(args, ...)`
        } else if token.contains("(") && token.ends_with(")") {
            let token = token.get(..token.len() - 1)?.trim();
            let (name, args) = token.split_once("(")?;
            let args = tokenize(args, &[","], false, true)?;
            let args = args.iter().map(|i| Expr::parse(&i));
            let args = args.collect::<Option<Vec<_>>>()?;
            Some(Expr::Call(name.to_string(), args))
        // Dictionary access `dict.field`
        } else if token.contains(".") {
            let (dict, field) = token.rsplit_once(".")?;
            Some(Expr::Field(Box::new(Expr::parse(dict)?), field.to_owned()))
        // Variable reference
        } else if !RESERVED.contains(&token) && token.is_ascii() {
            Some(Expr::Variable(token.to_string()))
        } else {
            None
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Expr::Oper(oper) => oper.compile(ctx)?,
            Expr::Variable(name) => format!("(local.get ${name})"),
            Expr::Literal(literal) => literal.compile(ctx)?,
            Expr::Call(name, args) => format!(
                "(call ${name} {})",
                join!(
                    args.iter()
                        .map(|x| x.compile(ctx))
                        .collect::<Option<Vec<_>>>()?
                )
            ),
            Expr::Index(array, index) => {
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
                let typ = expr.type_infer(ctx)?;
                let Type::Dict(dict) = typ else {
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
            Expr::MemCpy(from) => {
                let typ = from.type_infer(ctx)?;
                let size = typ.bytes_length()?;
                let size = Value::Integer(size as i32).compile(ctx)?;
                if_ptr!(typ => {
                    return Some(format!(
                        "(global.get $allocator) (memory.copy (global.get $allocator) {object} {size}) {}",
                        format!("(global.set $allocator (i32.add (global.get $allocator) {size}))"),
                        object = from.compile(ctx)?,
                    ))
                } else {
                    ctx.occurred_error = Some("can't memory copy primitive typed value".to_string());
                    return None
                });
            }
        })
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        Some(match self {
            Expr::Oper(oper) => oper.type_infer(ctx)?,
            Expr::Variable(name) => {
                if let Some(local) = ctx.variable_type.get(name) {
                    local.clone()
                } else if let Some(arg) = ctx.argument_type.get(name) {
                    arg.clone()
                } else {
                    ctx.occurred_error = Some(format!("undefined variable `{name}`"));
                    return None;
                }
            }
            Expr::Literal(literal) => literal.type_infer(ctx)?,
            Expr::Call(name, args) => {
                let function = ctx.function_type.get(name)?.clone();
                if args.len() != function.arguments.len() {
                    let errmsg = format!(
                        "arguments of function `{name}` length should be {}, but passed {} values",
                        function.arguments.len(),
                        args.len()
                    );
                    ctx.occurred_error = Some(errmsg);
                    return None;
                }
                let func = |(arg, typ): (&Expr, &Type)| type_check!(arg, typ, ctx);
                let ziped = args.iter().zip(function.arguments.values());
                ziped.map(func).collect::<Option<Vec<_>>>()?;
                function.returns.clone()
            }
            Expr::Index(arr, _) => {
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
                if let Type::Dict(dict) = infered.clone() {
                    let Some((_offset, typ)) = dict.get(key) else {
                        let error_message = format!("{} haven't field `{key}`", infered.format());
                        ctx.occurred_error = Some(error_message);
                        return None;
                    };
                    typ.clone()
                } else {
                    let error_message = format!("can't field access to {}", infered.format());
                    ctx.occurred_error = Some(error_message);
                    return None;
                }
            }
            Expr::Block(block) => block.type_infer(ctx)?,
            Expr::MemCpy(from) => from.type_infer(ctx)?,
        })
    }
}
