use crate::*;

#[derive(Clone, Debug)]
pub enum Stmt {
    If {
        cond: Expr,
        then: Expr,
        r#else: Box<Stmt>,
    },
    While {
        cond: Expr,
        body: Expr,
    },
    Let {
        name: Expr,
        value: Expr,
    },
    Expr(Expr),
    Drop,
}

impl Node for Stmt {
    fn parse(source: &str) -> Option<Self> {
        let source = source.trim();
        if let Some(source) = source.strip_prefix("if ") {
            let code = tokenize(source, SPACE.as_ref(), false, true)?;
            let then_pos = code.iter().position(|i| i == "then")?;
            let else_pos = code.iter().position(|i| i == "else")?;
            let cond_sec = join!(code.get(0..then_pos)?);
            let then_sec = join!(code.get(then_pos + 1..else_pos)?);
            let else_sec = join!(code.get(else_pos + 1..)?);
            Some(Stmt::If {
                cond: Expr::parse(&cond_sec)?,
                then: Expr::parse(&then_sec)?,
                r#else: Box::new(Stmt::parse(&else_sec)?),
            })
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
        } else {
            Some(Stmt::Expr(Expr::parse(source)?))
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Stmt::Expr(expr) => expr.compile(ctx)?,
            Stmt::If { cond, then, r#else } => {
                format!(
                    "(if {} (i32.eqz {}) (then {}) (else {}))",
                    config_return!(self.type_infer(ctx)?, ctx)?,
                    cond.compile(ctx)?,
                    r#else.compile(ctx)?,
                    then.compile(ctx)?
                )
            }
            Stmt::While { cond, body } => {
                format!(
                    "(block $outer (loop $while_start (br_if $outer (i32.eqz {})) {} (br $while_start)))",
                    cond.compile(ctx)?,
                    body.compile(ctx)?,
                )
            }
            Stmt::Let { name, value } => {
                let value_type = value.type_infer(ctx)?;
                match name {
                    Expr::Refer(name) => {
                        ctx.variable.insert(name.to_string(), value_type);
                        let result = format!(
                            "{1} (local.set ${name} {0})",
                            value.compile(ctx)?,
                            join!(ctx.array)
                        );
                        ctx.array.clear();
                        result
                    }
                    Expr::Pointer(addr) => {
                        format!(
                            "(i32.store {} {})",
                            Oper::Mul(*addr.clone(), Expr::Literal(Value::Integer(4)))
                                .compile(ctx)?,
                            value.compile(ctx)?
                        )
                    }
                    Expr::Access(array, index) => {
                        format!(
                            "(i32.store {} {})",
                            Oper::Mul(
                                Expr::Oper(Box::new(Oper::Add(*array.clone(), *index.clone()))),
                                Expr::Literal(Value::Integer(4))
                            )
                            .compile(ctx)?,
                            value.compile(ctx)?
                        )
                    }
                    Expr::Call(name, args) => {
                        ctx.variable.clear();
                        let inf = ctx.function.get(name)?.clone();
                        let code = format!(
                            "(func ${name} {0} {1} {3} {2})",
                            join!({
                                let mut result = vec![];
                                for arg in args {
                                    if let Expr::Oper(oper) = arg.clone() {
                                        if let Oper::Cast(Expr::Refer(arg), t) = *oper.clone() {
                                            result.push(format!(
                                                "(param ${} {})",
                                                arg,
                                                t.compile(ctx)?
                                            ));
                                        }
                                    } else if let Expr::Refer(arg) = arg.clone() {
                                        result.push(format!(
                                            "(param ${} {})",
                                            arg,
                                            Type::Pointer.compile(ctx)?
                                        ));
                                    } else {
                                        return None;
                                    }
                                }
                                result
                            }),
                            config_return!(inf.1, ctx)?,
                            value.compile(ctx)?,
                            expand_local(ctx)?
                        );
                        ctx.variable.clear();
                        ctx.declare.push(code);
                        String::new()
                    }
                    _ => todo!(),
                }
            }
            Stmt::Drop => "drop".to_string(),
        })
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        Some(match self {
            Stmt::Expr(expr) => expr.type_infer(ctx)?,
            Stmt::If { cond, then, r#else } => {
                cond.type_infer(ctx);
                type_check!(then, r#else, ctx)?
            }
            Stmt::While { cond, body } => {
                cond.type_infer(ctx)?;
                body.type_infer(ctx)?;
                Type::Void
            }
            Stmt::Let {
                name: Expr::Refer(name),
                value,
            } if !ctx.argument.contains_key(name) => {
                let value_type = value.type_infer(ctx)?;
                if let Some(exist_val) = ctx.clone().variable.get(name) {
                    type_check!(exist_val, value_type, ctx)?;
                } else {
                    ctx.variable.insert(name.to_string(), value_type);
                }
                Type::Void
            }
            Stmt::Let {
                name: Expr::Call(name, args),
                value,
            } => {
                for (arg, (_, inf)) in args.iter().zip(ctx.argument.clone()) {
                    if let Expr::Refer(arg) = arg {
                        ctx.argument.insert(arg.to_string(), inf);
                    } else if let Expr::Oper(oper) = arg {
                        if let Oper::Cast(Expr::Refer(arg), typed) = *oper.clone() {
                            ctx.argument.insert(arg.to_string(), typed);
                        }
                    } else {
                        return None;
                    };
                }
                let ret = value.type_infer(ctx)?;
                let inf = ctx.function.get(name)?;
                ctx.function.insert(name.to_owned(), (inf.0.clone(), ret));
                value.type_infer(ctx);
                Type::Void
            }
            Stmt::Let { name: _, value } => {
                value.type_infer(ctx);
                Type::Void
            }
            Stmt::Drop => Type::Void,
        })
    }
}
