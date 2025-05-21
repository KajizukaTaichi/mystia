use crate::*;

#[derive(Clone, Debug)]
pub enum Stmt {
    If(Expr, Expr, Option<Box<Stmt>>),
    While(Expr, Expr),
    Let(Scope, Expr, Expr),
    Type(String, Type),
    Import(Oper),
    Expr(Expr),
    Return(Option<Expr>),
    Next,
    Break,
}

#[derive(Clone, Debug)]
pub enum Scope {
    Global,
    Local,
}

impl Node for Stmt {
    fn parse(source: &str) -> Option<Self> {
        let source = source.trim();
        if let Some(source) = source.strip_prefix("if ") {
            let tokens = tokenize(source, SPACE.as_ref(), false, true, false)?;
            let then = tokens.iter().position(|i| i == "then")?;
            if let Some(r#else) = tokens.iter().position(|i| i == "else") {
                let cond = Expr::parse(&join!(tokens.get(0..then)?))?;
                let then = Expr::parse(&join!(tokens.get(then + 1..r#else)?))?;
                let r#else = Stmt::parse(&join!(tokens.get(r#else + 1..)?))?;
                Some(Stmt::If(cond, then, Some(Box::new(r#else))))
            } else {
                let cond = Expr::parse(&join!(tokens.get(0..then)?))?;
                let then = Expr::parse(&join!(tokens.get(then + 1..)?))?;
                Some(Stmt::If(cond, then, None))
            }
        } else if let Some(source) = source.strip_prefix("while ") {
            let tokens = tokenize(source, SPACE.as_ref(), false, true, false)?;
            let r#loop = tokens.iter().position(|i| i == "loop")?;
            let cond = Expr::parse(&join!(tokens.get(0..r#loop)?))?;
            let body = Expr::parse(&join!(tokens.get(r#loop + 1..)?))?;
            Some(Stmt::While(cond, body))
        } else if let Some(token) = source.strip_prefix("let ") {
            if let Some((name, value)) = token.split_once("=") {
                let (name, value) = (Expr::parse(name)?, Expr::parse(value)?);
                Some(Stmt::Let(Scope::Local, name, value))
            } else {
                let source = Oper::parse(token)?;
                macro_rules! assign_with {
                    ($op: ident) => {
                        if let Oper::$op(name, value) = source {
                            let value = Expr::Oper(Box::new(Oper::$op(name.clone(), value)));
                            return Some(Stmt::Let(Scope::Local, name, value));
                        }
                    };
                }
                assign_with!(Add);
                assign_with!(Sub);
                assign_with!(Mul);
                assign_with!(Div);
                assign_with!(Mod);
                None
            }
        } else if let Some(token) = source.strip_prefix("pub ") {
            match Stmt::parse(token)? {
                Stmt::Let(Scope::Local, name, value) => Some(Stmt::Let(Scope::Global, name, value)),
                _ => None,
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
                    compile_return!(self.type_infer(ctx)?, ctx),
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
            Stmt::Let(scope, name, value) => match name {
                Expr::Variable(name) => match scope {
                    Scope::Local => {
                        let typ = value.type_infer(ctx)?;
                        if !ctx.argument_type.contains_key(name) {
                            ctx.variable_type.insert(name.to_string(), typ);
                        }
                        format!("(local.set ${name} {})", value.compile(ctx)?)
                    }
                    Scope::Global => {
                        let typ = value.type_infer(ctx)?;
                        if !ctx.global_type.contains_key(name) {
                            ctx.global_type.insert(name.to_string(), typ);
                        }
                        format!("(global.set ${name} {})", value.compile(ctx)?)
                    }
                },
                Expr::Call(name, _) => {
                    let function = ctx.function_type.get(name)?.clone();
                    let [var_typ, arg_typ] = [ctx.variable_type.clone(), ctx.argument_type.clone()];
                    ctx.variable_type = function.variables.clone();
                    ctx.argument_type = function.arguments.clone();
                    let code = format!(
                        "(func ${name} {pub} {args} {ret} {locals} {body})",
                        args = join!(
                            &function
                                .arguments
                                .iter()
                                .map(|(name, typ)| Some(format!(
                                    "(param ${name} {})",
                                    typ.type_infer(ctx)?.compile(ctx)?
                                )))
                                .collect::<Option<Vec<_>>>()?
                        ),
                        ret = compile_return!(function.returns, ctx),
                        pub = if let Scope::Global = scope { format!("(export \"{name}\")") } else { String::new() },
                        body = value.compile(ctx)?, locals = expand_local(ctx)?
                    );
                    [ctx.variable_type, ctx.argument_type] = [var_typ, arg_typ];
                    ctx.declare_code.push(code);
                    String::new()
                }
                Expr::Index(array, index) => {
                    let Type::Array(typ, len) = array.type_infer(ctx)? else {
                        return None;
                    };
                    type_check!(typ, value.type_infer(ctx)?, ctx)?;
                    let addr = address_calc!(array, index, len, typ);
                    let [typ, addr] = [typ.compile(ctx)?, addr.compile(ctx)?];
                    format!("({typ}.store {addr} {})", value.compile(ctx)?)
                }
                Expr::Field(expr, key) => {
                    let Type::Dict(dict) = expr.type_infer(ctx)? else {
                        return None;
                    };
                    let (offset, typ) = dict.get(key)?.clone();
                    type_check!(typ, value.type_infer(ctx)?, ctx)?;
                    let addr = offset_calc!(expr, offset);
                    let [typ, addr] = [typ.compile(ctx)?, addr.compile(ctx)?];
                    format!("({typ}.store {addr} {})", value.compile(ctx)?)
                }
                _ => return None,
            },
            Stmt::Import(func) => {
                let Oper::Cast(Expr::Call(name, _), _) = func else {
                    return None;
                };
                let func = ctx.function_type.get(name)?.clone();
                let code = format!(
                    "(import \"env\" \"{name}\" (func ${name} {} {}))",
                    if func.arguments.is_empty() {
                        String::new()
                    } else {
                        compile_args_type!(func, ctx)
                    },
                    compile_return!(func.returns, ctx),
                );
                ctx.import_code.push(code);
                String::new()
            }
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
            Stmt::Let(scope, name, value) => {
                match name {
                    Expr::Variable(name) => match scope {
                        Scope::Local => {
                            if !ctx.argument_type.contains_key(name) {
                                let value_type = value.type_infer(ctx)?;
                                if let Some(exist_val) = ctx.clone().variable_type.get(name) {
                                    type_check!(exist_val, value_type, ctx)?;
                                } else {
                                    ctx.variable_type.insert(name.to_string(), value_type);
                                }
                            }
                        }
                        Scope::Global => {
                            let value_type = value.type_infer(ctx)?;
                            if let Some(exist_val) = ctx.clone().global_type.get(name) {
                                type_check!(exist_val, value_type, ctx)?;
                            } else {
                                ctx.global_type.insert(name.to_string(), value_type);
                            }
                        }
                    },
                    Expr::Call(name, args) => {
                        let var_typ = ctx.variable_type.clone();
                        let arg_typ = ctx.argument_type.clone();
                        for arg in args {
                            let Expr::Oper(oper) = arg else {
                                let msg = "function argument definition needs type annotation";
                                ctx.occurred_error = Some(msg.to_string());
                                return None;
                            };
                            let Oper::Cast(Expr::Variable(name), typ) = *oper.clone() else {
                                let msg = "function argument name should be identifier";
                                ctx.occurred_error = Some(msg.to_string());
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
                        ctx.variable_type = var_typ;
                        ctx.argument_type = arg_typ;
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
            Stmt::Return(_) => Type::Void,
        })
    }
}
