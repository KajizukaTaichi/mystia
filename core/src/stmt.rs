use crate::*;

#[derive(Clone, Debug)]
pub enum Stmt {
    Defun {
        name: String,
        args: Vec<(String, Type)>,
        body: Expr,
        ret: Type,
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
            let (args, source) = source.split_once(") as ")?;
            let (ret, body) = source.split_once("=")?;
            Some(Stmt::Defun {
                name: name.trim().to_string(),
                args: {
                    let mut result = vec![];
                    if !args.trim().is_empty() {
                        for arg in args.split(",") {
                            let (arg, annotation) = arg.split_once(" as ")?;
                            result.push((arg.trim().to_string(), Type::parse(annotation)?));
                        }
                    }
                    result
                },
                body: Expr::parse(body)?,
                ret: Type::parse(ret)?,
            })
        } else if let Some(source) = source.strip_prefix("if ") {
            let code = tokenize(source, SPACE.as_ref(), false)?;
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
            let code = tokenize(source, SPACE.as_ref(), false)?;
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

    fn compile(&self, ctx: &mut Compiler) -> String {
        match self {
            Stmt::Expr(expr) => expr.compile(ctx),
            Stmt::Defun {
                name,
                args,
                body,
                ret,
            } => {
                ctx.variable.clear();
                let code = format!(
                    "(func ${name} {0} {1} {3} {2})",
                    join!(
                        args.iter()
                            .map(|x| format!("(param ${} {})", x.0, x.1.compile(ctx)))
                            .collect::<Vec<_>>()
                    ),
                    config_return!(ret, ctx),
                    body.compile(ctx),
                    expand_local(ctx)
                );
                ctx.variable.clear();
                ctx.declare.push(code);
                String::new()
            }
            Stmt::If { cond, then, r#else } => {
                format!(
                    "(if (result {}) (i32.eqz {}) (then {}) (else {}))",
                    self.type_infer(ctx).compile(ctx),
                    cond.compile(ctx),
                    r#else.compile(ctx),
                    then.compile(ctx)
                )
            }
            Stmt::While { cond, body } => {
                format!(
                    "(block $outer (loop $while_start (br_if $outer (i32.eqz {})) {} (br $while_start)))",
                    cond.compile(ctx),
                    body.compile(ctx),
                )
            }
            Stmt::Let { name, value } => {
                let value_type = value.type_infer(ctx);
                match name {
                    Expr::Refer(name) => {
                        ctx.variable.insert(name.to_string(), value_type);
                        let result = format!(
                            "{1} (local.set ${name} {0})",
                            value.compile(ctx),
                            join!(ctx.array)
                        );
                        ctx.array.clear();
                        result
                    }
                    Expr::Pointer(addr) => {
                        format!(
                            "(i32.store {} {})",
                            Oper::Mul(*addr.clone(), Expr::Value(Value::Integer(4))).compile(ctx),
                            value.compile(ctx)
                        )
                    }
                    Expr::Access(array, index) => {
                        format!(
                            "(i32.store {} {})",
                            Oper::Mul(
                                Expr::Oper(Box::new(Oper::Add(*array.clone(), *index.clone()))),
                                Expr::Value(Value::Integer(4))
                            )
                            .compile(ctx),
                            value.compile(ctx)
                        )
                    }
                    _ => todo!(),
                }
            }
            Stmt::Drop => "drop".to_string(),
        }
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Type {
        match self {
            Stmt::Expr(expr) => expr.type_infer(ctx),
            Stmt::Defun {
                name,
                args,
                body,
                ret,
            } => {
                for (arg, anno) in args {
                    ctx.argument.insert(arg.to_string(), anno.clone());
                }
                ctx.function.insert(name.to_string(), ret.clone());
                body.type_infer(ctx);
                Type::Void
            }
            Stmt::If { cond, then, r#else } => {
                cond.type_infer(ctx);
                then.type_infer(ctx);
                r#else.type_infer(ctx)
            }
            Stmt::While { cond, body } => {
                cond.type_infer(ctx);
                body.type_infer(ctx)
            }
            Stmt::Let {
                name: Expr::Refer(name),
                value,
            } if !ctx.argument.contains_key(name) => {
                let value_type = value.type_infer(ctx);
                ctx.variable.insert(name.to_string(), value_type);
                Type::Void
            }
            Stmt::Let { name: _, value: _ } => Type::Void,
            Stmt::Drop => Type::Void,
        }
    }
}
