use crate::*;

#[derive(Clone, Debug)]
pub enum Stmt {
    If {
        cond: Expr,
        then: Expr,
        r#else: Option<Box<Stmt>>,
    },
    While {
        cond: Expr,
        body: Expr,
    },
    Let {
        name: Expr,
        value: Expr,
    },
    MemCpy {
        from: Expr,
    },
    Expr(Expr),
    Next,
    Break,
    Return(Option<Expr>),
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
                Some(Stmt::If {
                    cond: Expr::parse(&cond_sec)?,
                    then: Expr::parse(&then_sec)?,
                    r#else: Some(Box::new(Stmt::parse(&else_sec)?)),
                })
            } else {
                let cond_sec = join!(code.get(0..then_pos)?);
                let then_sec = join!(code.get(then_pos + 1..)?);
                Some(Stmt::If {
                    cond: Expr::parse(&cond_sec)?,
                    then: Expr::parse(&then_sec)?,
                    r#else: None,
                })
            }
        } else if let Some(source) = source.strip_prefix("while ") {
            let code = tokenize(source, SPACE.as_ref(), false, true)?;
            let loop_pos = code.iter().position(|i| i == "loop")?;
            let cond_sec = join!(code.get(0..loop_pos)?);
            let body_sec = join!(code.get(loop_pos + 1..)?);
            Some(Stmt::While {
                cond: Expr::parse(&cond_sec)?,
                body: Expr::parse(&body_sec)?,
            })
        } else if let Some(source) = source.strip_prefix("let ") {
            let (name, value) = source.split_once("=")?;
            Some(Stmt::Let {
                name: Expr::parse(name)?,
                value: Expr::parse(value)?,
            })
        } else if let Some(source) = source.strip_prefix("return ") {
            Some(Stmt::Return(Some(Expr::parse(source)?)))
        } else if let Some(source) = source.strip_prefix("memcpy ") {
            Some(Stmt::MemCpy {
                from: Expr::parse(source)?,
            })
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
            Stmt::If { cond, then, r#else } => {
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
            Stmt::While { cond, body } => {
                format!(
                    "(block $outer (loop $while_start (br_if $outer (i32.eqz {})) {} {}))",
                    cond.compile(ctx)?,
                    body.compile(ctx)?,
                    Stmt::Next.compile(ctx)?
                )
            }
            Stmt::Next => "(br $while_start)".to_string(),
            Stmt::Break => "(br $outer)".to_string(),
            Stmt::Let { name, value } => match name {
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
                Expr::Property(expr, key) => {
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
                    let (var_inf, arg_inf, ret_inf) = ctx.function_type.get(name)?.clone();
                    (ctx.variable_type, ctx.argument_type) = (var_inf, arg_inf.clone());
                    let code = format!(
                        "(func ${name} (export \"{name}\") {args} {ret} {locals} {body})",
                        args = join!(iter_map!(&arg_inf, |(name, typ): (&String, &Type)| Some(
                            format!("(param ${name} {})", typ.compile(ctx)?)
                        ))),
                        ret = config_return!(ret_inf, ctx)?,
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
            Stmt::MemCpy { from } => {
                let size = from.type_infer(ctx)?.bytes_length()?;
                let size = Value::Integer(size as i32);
                format!(
                    "(global.get $alloc_index) (memory.copy (global.get $alloc_index) {} {size}) (global.set $alloc_index (i32.add (global.get $alloc_index) {size}))",
                    from.compile(ctx)?,
                    size = size.compile(ctx)?
                )
            }
            Stmt::Drop => "(drop)".to_string(),
            Stmt::Return(Some(expr)) => format!("(return {})", expr.compile(ctx)?),
            Stmt::Return(_) => "(return)".to_string(),
        })
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        Some(match self {
            Stmt::Expr(expr) => expr.type_infer(ctx)?,
            Stmt::If { cond, then, r#else } => {
                type_check!(cond, Type::Bool, ctx)?;
                if let Some(r#else) = r#else {
                    type_check!(then, r#else, ctx)?
                } else {
                    then.type_infer(ctx)?
                }
            }
            Stmt::While { cond, body } => {
                type_check!(cond, Type::Bool, ctx)?;
                body.type_infer(ctx)?;
                Type::Void
            }
            Stmt::Break => Type::Void,
            Stmt::Next => Type::Void,
            Stmt::Let { name, value } => {
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
                        let ret = value.type_infer(ctx)?;
                        if args.len() != ctx.argument_type.len() {
                            return None;
                        };
                        for (arg, (infered, _)) in args.iter().zip(ctx.argument_type.clone()) {
                            let Expr::Variable(arg) = arg else {
                                return None;
                            };
                            if infered != *arg {
                                return None;
                            }
                        }
                        ctx.function_type.insert(
                            name.to_owned(),
                            (ctx.variable_type.clone(), ctx.argument_type.clone(), ret),
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
            Stmt::MemCpy { from } => {
                ctx.is_memory_copied = true;
                from.type_infer(ctx)?
            }
            Stmt::Drop => Type::Void,
            Stmt::Return(_) => Type::Void,
        })
    }
}
