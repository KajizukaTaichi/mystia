use crate::*;

pub enum Stmt {
    Defun {
        name: String,
        args: Vec<String>,
        body: Expr,
    },
    Expr(Expr),
}

impl Stmt {
    pub fn parse(source: &str) -> Option<Self> {
        if let Some(source) = source.strip_prefix("fn ") {
            let (name, source) = source.split_once("(")?;
            let (args, body) = source.split_once(")")?;
            Some(Stmt::Defun {
                name: name.trim().to_string(),
                args: args.split(",").map(|x| x.trim().to_string()).collect(),
                body: Expr::parse(body)?,
            })
        } else {
            Some(Stmt::Expr(Expr::parse(source)?))
        }
    }

    pub fn compile(&self, ctx: &mut Compiler) -> String {
        match self {
            Stmt::Expr(expr) => expr.compile(),
            Stmt::Defun { name, args, body } => {
                ctx.declare.push(format!(
                    "(func ${name} {} (result i32) {})",
                    join!(
                        args.iter()
                            .map(|x| format!("(param ${x} i32)"))
                            .collect::<Vec<_>>()
                    ),
                    body.compile()
                ));
                String::new()
            }
        }
    }
}
