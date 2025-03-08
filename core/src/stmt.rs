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
    Declare {
        name: String,
        annotation: Type,
    },
    Assign(String, Expr),
    Expr(Expr),
}

impl Node for Stmt {
    fn parse(source: &str) -> Option<Self> {
        let source = source.trim();
        if let Some(source) = source.strip_prefix("fn ") {
            let (name, source) = source.split_once("(")?;
            let (args, source) = source.split_once("):")?;
            let (ret, body) = source.split_once("=")?;
            Some(Stmt::Defun {
                name: name.trim().to_string(),
                args: {
                    let mut result = vec![];
                    for arg in args.split(",") {
                        let (arg, annotation) = arg.split_once(":")?;
                        result.push((arg.trim().to_string(), Type::parse(annotation)?));
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
        } else if let Some(source) = source.strip_prefix("declare ") {
            let (name, annotation) = source.split_once(":")?;
            Some(Stmt::Declare {
                name: name.trim().to_string(),
                annotation: Type::parse(annotation)?,
            })
        } else if let Some(source) = source.strip_prefix("let ") {
            let (name, source) = source.split_once("=")?;
            Some(Stmt::Assign(name.trim().to_string(), Expr::parse(source)?))
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
                let code = format!(
                    "(func ${name} {} (result {}) {})",
                    join!(
                        args.iter()
                            .map(|x| format!("(param ${} {})", x.0, x.1.compile(ctx)))
                            .collect::<Vec<_>>()
                    ),
                    ret.compile(ctx),
                    body.compile(ctx)
                );
                ctx.declare.push(code);
                String::new()
            }
            Stmt::If { cond, then, r#else } => {
                format!(
                    "(if (result {}) {} (then {}) (else {}))",
                    self.type_infer(ctx).compile(ctx),
                    cond.compile(ctx),
                    then.compile(ctx),
                    r#else.compile(ctx)
                )
            }
            Stmt::While { cond, body } => {
                format!(
                    "(loop $while_start {} (br_if $while_start {}))",
                    body.compile(ctx),
                    cond.compile(ctx),
                )
            }
            Stmt::Declare { name, annotation } => {
                format!("(local ${name} {})", annotation.compile(ctx))
            }
            Stmt::Assign(name, expr) => format!("(local.set ${name} {})", expr.compile(ctx)),
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
                    ctx.variable.insert(arg.to_string(), anno.clone());
                }
                body.type_infer(ctx);
                ctx.function.insert(name.to_string(), ret.clone());
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
            Stmt::Declare { name, annotation } => {
                ctx.variable.insert(name.to_string(), annotation.clone());
                Type::Void
            }
            Stmt::Assign(_, expr) => expr.type_infer(ctx),
        }
    }
}
