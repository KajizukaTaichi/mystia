use crate::*;

#[derive(Clone, Debug)]
pub enum Stmt {
    Defun {
        name: String,
        args: Vec<String>,
        body: Expr,
    },
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
        if let Some(source) = source.strip_prefix("fn ") {
            let (name, source) = source.split_once("(")?;
            let (args, body) = source.split_once(")")?;
            Some(Stmt::Defun {
                name: name.trim().to_string(),
                args: {
                    let mut result = vec![];
                    if !args.trim().is_empty() {
                        for arg in args.split(",") {
                            result.push(arg.trim().to_string());
                        }
                    }
                    result
                },
                body: Expr::parse(body)?,
            })
        } else if let Some(source) = source.strip_prefix("if ") {
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
            Stmt::Defun { name, args, body } => {
                ctx.variable.clear();
                let inf = ctx.function.get(name)?;
                let code = format!(
                    "(func ${name} {0} {1} {3} {2})",
                    join!({
                        let mut result = vec![];
                        for (k, v) in args.iter().zip(inf.0.clone()) {
                            result.push(format!("(param ${} {})", k, v.compile(ctx)?));
                        }
                        result
                    }),
                    config_return!(inf.1.clone(), ctx)?,
                    body.compile(ctx)?,
                    expand_local(ctx)?
                );
                ctx.variable.clear();
                ctx.declare.push(code);
                String::new()
            }
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
                            Oper::Mul(*addr.clone(), Expr::Value(Value::Integer(4)))
                                .compile(ctx)?,
                            value.compile(ctx)?
                        )
                    }
                    Expr::Access(array, index) => {
                        format!(
                            "(i32.store {} {})",
                            Oper::Mul(
                                Expr::Oper(Box::new(Oper::Add(*array.clone(), *index.clone()))),
                                Expr::Value(Value::Integer(4))
                            )
                            .compile(ctx)?,
                            value.compile(ctx)?
                        )
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
            Stmt::Defun {
                name,
                args: _,
                body,
            } => {
                let ret = body.type_infer(ctx)?;
                let inf = ctx.function.get_mut(name)?;
                inf.1 = ret;
                body.type_infer(ctx);
                Type::Void
            }
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
            Stmt::Let { name: _, value } => {
                value.type_infer(ctx);
                Type::Void
            }
            Stmt::Drop => Type::Void,
        })
    }

    fn func_scan(&self, ctx: &mut Compiler) -> Option<()> {
        match self {
            Stmt::Expr(expr) => expr.func_scan(ctx),
            Stmt::If { cond, then, r#else } => {
                cond.func_scan(ctx)?;
                then.func_scan(ctx)?;
                r#else.func_scan(ctx)
            }
            Stmt::While { cond, body } => {
                cond.func_scan(ctx)?;
                body.func_scan(ctx)
            }
            Stmt::Let { name: _, value } => value.func_scan(ctx),
            _ => Some(()),
        }
    }
}
