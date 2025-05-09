use crate::*;

#[derive(Clone, Debug)]
pub enum Stmt {
    If(Expr, Expr, Option<Box<Stmt>>),
    While(Expr, Expr),
    Let(Expr, Expr),
    Type(String, Type),
    Import(Oper),
    Expr(Expr),
    Return(Option<Expr>),
    Next,
    Break,
    Drop,
}

impl Node for Stmt {
    fn parse(source: &str) -> Option<Self> {
        let source = source.trim();
        if let Some(source) = source.strip_prefix("if ") {
            let code = tokenize(source, SPACE.as_ref(), false, true)?;
            let then_pos = code.iter().position(|i| i == "then")?;
            if let Some(else_pos) = code.iter().position(|i| i == "else") {
                let cond_sec = join!(code.get(0..then_pos)?);
                let then_sec = join!(code.get(then_pos + 1..else_pos)?);
                let else_sec = join!(code.get(else_pos + 1..)?);
                Some(Stmt::If(
                    Expr::parse(&cond_sec)?,
                    Expr::parse(&then_sec)?,
                    Some(Box::new(Stmt::parse(&else_sec)?)),
                ))
            } else {
                let cond_sec = join!(code.get(0..then_pos)?);
                let then_sec = join!(code.get(then_pos + 1..)?);
                Some(Stmt::If(
                    Expr::parse(&cond_sec)?,
                    Expr::parse(&then_sec)?,
                    None,
                ))
            }
        } else if let Some(source) = source.strip_prefix("while ") {
            let code = tokenize(source, SPACE.as_ref(), false, true)?;
            let loop_pos = code.iter().position(|i| i == "loop")?;
            let cond_sec = join!(code.get(0..loop_pos)?);
            let body_sec = join!(code.get(loop_pos + 1..)?);
            Some(Stmt::While(
                Expr::parse(&cond_sec)?,
                Expr::parse(&body_sec)?,
            ))
        } else if let Some(source) = source.strip_prefix("let ") {
            if let Some((name, value)) = source.split_once("=") {
                Some(Stmt::Let(Expr::parse(name)?, Expr::parse(value)?))
            } else {
                let source = Oper::parse(source)?;
                if let Oper::Add(name, value) = source {
                    let value = Expr::Oper(Box::new(Oper::Add(name.clone(), value)));
                    Some(Stmt::Let(name, value))
                } else if let Oper::Sub(name, value) = source {
                    let value = Expr::Oper(Box::new(Oper::Sub(name.clone(), value)));
                    Some(Stmt::Let(name, value))
                } else if let Oper::Mul(name, value) = source {
                    let value = Expr::Oper(Box::new(Oper::Mul(name.clone(), value)));
                    Some(Stmt::Let(name, value))
                } else if let Oper::Div(name, value) = source {
                    let value = Expr::Oper(Box::new(Oper::Div(name.clone(), value)));
                    Some(Stmt::Let(name, value))
                } else {
                    None
                }
            }
        } else if let Some(source) = source.strip_prefix("type ") {
            let (name, value) = source.split_once("=")?;
            Some(Stmt::Type(name.trim().to_string(), Type::parse(value)?))
        } else if let Some(source) = source.strip_prefix("return ") {
            Some(Stmt::Return(Some(Expr::parse(source)?)))
        } else if let Some(source) = source.strip_prefix("load ") {
            Some(Stmt::Import(Oper::parse(source)?))
        } else if source == "return" {
            Some(Stmt::Return(None))
        } else if source == "next" {
            Some(Stmt::Next)
        } else if source == "break" {
            Some(Stmt::Break)
        } else {
            Some(Stmt::Expr(Expr::parse(source)?))
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Stmt::Expr(expr) => expr.compile(ctx)?,
            Stmt::If(cond, then, r#else) => {
                format!(
                    "(if {} {} (then {}) {})",
                    config_return!(self.type_infer(ctx)?, ctx)?,
                    cond.compile(ctx)?,
                    then.compile(ctx)?,
                    if let Some(r#else) = r#else {
                        format!("(else {})", r#else.compile(ctx)?)
                    } else {
                        String::new()
                    },
                )
            }
            Stmt::While(cond, body) => {
                format!(
                    "(block $outer (loop $while_start (br_if $outer (i32.eqz {})) {} {}))",
                    cond.compile(ctx)?,
                    body.compile(ctx)?,
                    Stmt::Next.compile(ctx)?
                )
            }
            Stmt::Next => "(br $while_start)".to_string(),
            Stmt::Break => "(br $outer)".to_string(),
            Stmt::Let(name, value) => match name {
                Expr::Variable(name) => {
                    let typ = value.type_infer(ctx)?;
                    if !ctx.argument_type.contains_key(name) {
                        ctx.variable_type.insert(name.to_string(), typ);
                    }
                    format!("(local.set ${name} {})", value.compile(ctx)?)
                }
                Expr::Access(array, index) => {
                    let Type::Array(typ, len) = array.type_infer(ctx)? else {
                        return None;
                    };
                    type_check!(typ, value.type_infer(ctx)?, ctx)?;
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
                    format!(
                        "({}.store {} {})",
                        typ.compile(ctx)?,
                        addr.compile(ctx)?,
                        value.compile(ctx)?
                    )
                }
                Expr::Field(expr, key) => {
                    let Type::Dict(dict) = expr.type_infer(ctx)? else {
                        return None;
                    };
                    let (offset, typ) = dict.get(key)?.clone();
                    type_check!(typ, value.type_infer(ctx)?, ctx)?;
                    let addr = Oper::Add(
                        Expr::Oper(Box::new(Oper::Cast(*expr.clone(), Type::Integer))),
                        Expr::Literal(Value::Integer(offset)),
                    );
                    format!(
                        "({}.store {} {})",
                        typ.compile(ctx)?,
                        addr.compile(ctx)?,
                        value.compile(ctx)?
                    )
                }
                Expr::Call(name, _) => {
                    let function = ctx.function_type.get(name)?.clone();
                    ctx.variable_type = function.variables.clone();
                    ctx.argument_type = function.arguments.clone();
                    let code = format!(
                        "(func ${name} (export \"{name}\") {args} {ret} {locals} {body})",
                        args =
                            join!(iter_map!(
                                &function.arguments,
                                |(name, typ): (&String, &Type)| Some(format!(
                                    "(param ${name} {})",
                                    typ.type_infer(ctx)?.compile(ctx)?
                                ))
                            )),
                        ret = config_return!(function.returns, ctx)?,
                        body = value.compile(ctx)?,
                        locals = expand_local(ctx)?
                    );
                    ctx.variable_type.clear();
                    ctx.argument_type.clear();
                    ctx.declare_code.push(code);
                    String::new()
                }
                _ => return None,
            },
            Stmt::Import(func) => {
                let Oper::Cast(Expr::Call(name, _), _) = func else {
                    return None;
                };
                let function = ctx.function_type.get(name)?.clone();
                let code = format!(
                    "(import \"env\" \"{name}\" (func ${name} {} {}))",
                    if function.arguments.is_empty() {
                        String::new()
                    } else {
                        format!(
                            "(param {})",
                            join!(iter_map!(function.arguments, |(_, typ): (_, Type)| typ
                                .compile(ctx)))
                        )
                    },
                    config_return!(function.returns, ctx)?,
                );
                ctx.import_code.push(code);
                String::new()
            }
            Stmt::Drop => "(drop)".to_string(),
            Stmt::Return(Some(expr)) => format!("(return {})", expr.compile(ctx)?),
            Stmt::Return(_) => "(return)".to_string(),
            Stmt::Type(_, _) => String::new(),
        })
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        Some(match self {
            Stmt::Expr(expr) => expr.type_infer(ctx)?,
            Stmt::If(cond, then, r#else) => {
                type_check!(cond, Type::Bool, ctx)?;
                if let Some(r#else) = r#else {
                    type_check!(then, r#else, ctx)?
                } else {
                    then.type_infer(ctx)?
                }
            }
            Stmt::While(cond, body) => {
                type_check!(cond, Type::Bool, ctx)?;
                body.type_infer(ctx)?;
                Type::Void
            }
            Stmt::Break => Type::Void,
            Stmt::Next => Type::Void,
            Stmt::Let(name, value) => {
                match name {
                    Expr::Variable(name) if !ctx.argument_type.contains_key(name) => {
                        let value_type = value.type_infer(ctx)?;
                        if let Some(exist_val) = ctx.clone().variable_type.get(name) {
                            type_check!(exist_val, value_type, ctx)?;
                        } else {
                            ctx.variable_type.insert(name.to_string(), value_type);
                        }
                    }
                    Expr::Call(name, args) => {
                        for arg in args {
                            let Expr::Oper(oper) = arg else { return None };
                            let Oper::Cast(Expr::Variable(name), typ) = *oper.clone() else {
                                return None;
                            };
                            let typ = typ.type_infer(ctx)?;
                            ctx.argument_type.insert(name.to_string(), typ);
                        }
                        let ret = value.type_infer(ctx)?;
                        ctx.function_type.insert(
                            name.to_owned(),
                            Function {
                                variables: ctx.variable_type.clone(),
                                arguments: ctx.argument_type.clone(),
                                returns: ret,
                            },
                        );
                        ctx.variable_type.clear();
                        ctx.argument_type.clear();
                    }
                    _ => {
                        value.type_infer(ctx);
                    }
                }
                Type::Void
            }
            Stmt::Type(name, value) => {
                let value = value.type_infer(ctx)?;
                ctx.type_alias.insert(name.to_string(), value);
                Type::Void
            }
            Stmt::Import(func) => {
                let Oper::Cast(Expr::Call(name, args), ret_typ) = func else {
                    return None;
                };
                let mut args_typ = IndexMap::new();
                for arg in args {
                    let Expr::Oper(arg) = arg else { return None };
                    let Oper::Cast(Expr::Variable(name), arg_typ) = *arg.clone() else {
                        return None;
                    };
                    args_typ.insert(name, arg_typ.type_infer(ctx)?);
                }
                ctx.function_type.insert(
                    name.to_owned(),
                    Function {
                        variables: IndexMap::new(),
                        arguments: args_typ.clone(),
                        returns: ret_typ.clone(),
                    },
                );
                Type::Void
            }
            Stmt::Drop => Type::Void,
            Stmt::Return(_) => Type::Void,
        })
    }
}
