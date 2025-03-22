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
            Stmt::Let { name, value } => {
                let value_type = value.type_infer(ctx)?;
                match name {
                    Expr::Variable(name) => {
                        ctx.variable_type.insert(name.to_string(), value_type);
                        let result = format!("(local.set ${name} {0})", value.compile(ctx)?);
                        result
                    }
                    Expr::Deref(addr) => {
                        format!(
                            "({}.store {} {})",
                            value.type_infer(ctx)?.compile(ctx)?,
                            addr.clone().compile(ctx)?,
                            value.compile(ctx)?
                        )
                    }
                    Expr::Access(array, index) => Stmt::Let {
                        name: Expr::Deref(Box::new(Expr::Oper(Box::new(Oper::Add(
                            *array.clone(),
                            *index.clone(),
                        ))))),
                        value: value.clone(),
                    }
                    .compile(ctx)?,
                    Expr::Call(name, args) => {
                        ctx.variable_type.clear();
                        let inf = ctx.function_type.get(name)?.clone();
                        let code = format!(
                            "(func ${name} (export \"{name}\") {0} {1} {3} {2})",
                            join!({
                                let mut result = vec![];
                                for arg in args {
                                    if let Expr::Oper(oper) = arg.clone() {
                                        if let Oper::Cast(Expr::Variable(arg), t) = *oper.clone() {
                                            result.push(format!(
                                                "(param ${} {})",
                                                arg,
                                                t.compile(ctx)?
                                            ));
                                        }
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
                        ctx.variable_type.clear();
                        ctx.argument_type.clear();
                        ctx.declare_code.push(code);
                        String::new()
                    }
                    _ => todo!(),
                }
            }
            Stmt::Drop => "(drop)".to_string(),
            Stmt::Return(Some(expr)) => format!("(return {})", expr.compile(ctx)?),
            Stmt::Return(None) => "(return)".to_string(),
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
            Stmt::Let {
                name: Expr::Variable(name),
                value,
            } if !ctx.argument_type.contains_key(name) => {
                let value_type = value.type_infer(ctx)?;
                if let Some(exist_val) = ctx.clone().variable_type.get(name) {
                    type_check!(exist_val, value_type, ctx)?;
                } else {
                    ctx.variable_type.insert(name.to_string(), value_type);
                }
                Type::Void
            }
            Stmt::Let {
                name: Expr::Call(name, args),
                value,
            } => {
                for arg in args {
                    if let Expr::Oper(oper) = arg {
                        if let Oper::Cast(Expr::Variable(arg), typed) = *oper.clone() {
                            ctx.argument_type.insert(arg.to_string(), typed);
                        }
                    } else {
                        return None;
                    };
                }
                let ret = value.type_infer(ctx)?;
                ctx.function_type.insert(
                    name.to_owned(),
                    (ctx.argument_type.values().cloned().collect(), ret),
                );
                value.type_infer(ctx);
                Type::Void
            }
            Stmt::Let { name: _, value } => {
                value.type_infer(ctx);
                Type::Void
            }
            Stmt::Drop => Type::Void,
            Stmt::Return(_) => Type::Void,
        })
    }
}
