use crate::*;

/// Import function signature: name, arguments, return, alias
type FuncSig = (String, Vec<(String, Type)>, Type, Option<String>);
#[derive(Clone, Debug)]
pub enum Stmt {
    If(Expr, Expr, Option<Box<Stmt>>),
    While(Expr, Expr),
    Let(Scope, Expr, Expr),
    Type(String, Type),
    Import(Option<String>, Vec<FuncSig>),
    Macro(String, Vec<String>, Expr),
    Expr(Expr),
    Return(Option<Expr>),
    Next,
    Break,
}

#[derive(Clone, Copy, Debug)]
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
            let Stmt::Let(Scope::Local, name, value) = Stmt::parse(token)? else {
                return None;
            };
            Some(Stmt::Let(Scope::Global, name, value))
        } else if let Some(source) = source.strip_prefix("type ") {
            let (name, value) = source.split_once("=")?;
            let Some(Expr::Variable(name)) = Expr::parse(name) else {
                return None;
            };
            Some(Stmt::Type(name, Type::parse(value)?))
        } else if let Some(source) = source.strip_prefix("macro ") {
            let (head, value) = source.split_once("=")?;
            let Expr::Call(name, args) = Expr::parse(head)? else {
                return None;
            };
            let func = |x: &Expr| {
                let Expr::Variable(x) = x else { return None };
                Some(x.clone())
            };
            let args = args.iter().map(func).collect::<Option<Vec<_>>>()?;
            Some(Stmt::Macro(name, args, Expr::parse(value)?))
        } else if let Some(after) = source.strip_prefix("load") {
            let parse_sigs = |sigs: &str| {
                let mut result = Vec::new();
                for part in tokenize(sigs, &[","], false, true, false)? {
                    let part = part.trim();
                    let (sig, alias) = part
                        .rsplit_once("as")
                        .map(|(sig, alias)| (sig, Some(alias.to_string())))
                        .unwrap_or((part, None));
                    let Oper::Cast(Expr::Call(name, args), ret_typ) = Oper::parse(sig)? else {
                        return None;
                    };
                    let mut args_typ = vec![];
                    for arg in args {
                        let Expr::Oper(arg) = arg else { return None };
                        let Oper::Cast(Expr::Variable(arg_name), arg_typ) = *arg.clone() else {
                            return None;
                        };
                        args_typ.push((arg_name, arg_typ));
                    }
                    result.push((name, args_typ, ret_typ, alias));
                }
                Some(result)
            };
            let rest = after.trim_start();
            if let Some((module, sigs)) = rest.split_once("::") {
                let sigs = sigs
                    .strip_prefix('{')
                    .and_then(|s| s.strip_suffix('}'))
                    .unwrap_or(sigs);
                let sigs = parse_sigs(sigs.trim())?;
                let module = module.trim().to_string();
                Some(Stmt::Import(Some(module), sigs))
            } else {
                Some(Stmt::Import(None, parse_sigs(&rest.trim())?))
            }
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
                    let var_typ = ctx.variable_type.clone();
                    let arg_typ = ctx.argument_type.clone();
                    let function = ctx.function_type.get(name)?.clone();
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
                    if !ctx.declare_code.contains(&code) {
                        ctx.declare_code.push(code);
                    }
                    ctx.variable_type = var_typ;
                    ctx.argument_type = arg_typ;
                    String::new()
                }
                Expr::Oper(oper) => {
                    let Oper::Cast(func, _) = *oper.clone() else {
                        return None;
                    };
                    Stmt::Let(*scope, func, value.clone()).compile(ctx)?
                }
                Expr::Index(array, index) => {
                    let Type::Array(typ) = array.type_infer(ctx)? else {
                        return None;
                    };
                    type_check!(typ, value.type_infer(ctx)?, ctx)?;
                    let addr = address_calc!(array, index, typ);
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
            Stmt::Import(module, funcs) => {
                for (name, args, ret_typ, alias) in funcs.clone() {
                    let name = alias.unwrap_or(name.clone());
                    let mut export = name.clone();
                    if let Some(module) = module {
                        export = format!("{module}.{name}")
                    };
                    let sig = join!(
                        args.iter()
                            .map(|(_, t)| t
                                .type_infer(ctx)?
                                .compile(ctx)
                                .map(|s| format!("(param {})", s)))
                            .collect::<Option<Vec<_>>>()?
                    );
                    let ret = compile_return!(ret_typ, ctx);
                    ctx.import_code.push(format!(
                        "(import \"env\" \"{export}\" (func ${name} {sig} {ret}))"
                    ));
                }
                String::new()
            }
            Stmt::Return(Some(expr)) => format!("(return {})", expr.compile(ctx)?),
            Stmt::Return(_) => "(return)".to_string(),
            Stmt::Type(_, _) | Stmt::Macro(_, _, _) => String::new(),
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
                        ctx.variable_type.clear();
                        ctx.argument_type.clear();
                        compile_args!(args, ctx);
                        let frame = Function {
                            returns: value.type_infer(ctx)?,
                            variables: ctx.variable_type.clone(),
                            arguments: ctx.argument_type.clone(),
                        };
                        ctx.function_type.insert(name.to_owned(), frame);
                        ctx.variable_type = var_typ;
                        ctx.argument_type = arg_typ;
                    }
                    Expr::Oper(oper) => match *oper.clone() {
                        Oper::Cast(Expr::Call(name, args), ret) => {
                            let var_typ = ctx.variable_type.clone();
                            let arg_typ = ctx.argument_type.clone();
                            compile_args!(args.clone(), ctx);
                            ctx.function_type.insert(
                                name.to_owned(),
                                Function {
                                    variables: ctx.variable_type.clone(),
                                    arguments: ctx.argument_type.clone(),
                                    returns: ret.clone(),
                                },
                            );
                            type_check!(value.type_infer(ctx)?, ret, ctx);
                            let frame = ctx.function_type.get_mut(&name)?;
                            frame.variables = ctx.variable_type.clone();
                            ctx.variable_type = var_typ;
                            ctx.argument_type = arg_typ;
                        }
                        _ => return None,
                    },
                    _ => {
                        value.type_infer(ctx);
                    }
                }
                Type::Void
            }
            Stmt::Type(name, value) => {
                ctx.type_alias.insert(name.to_string(), value.clone());
                Type::Void
            }
            Stmt::Import(_module, funcs) => {
                for (fn_name, args, ret_typ, alias) in funcs {
                    let import_name = alias.as_ref().unwrap_or(fn_name).clone();
                    let mut arg_map = IndexMap::new();
                    for (name, typ) in args.iter() {
                        arg_map.insert(name.to_string(), typ.clone());
                    }
                    ctx.function_type.insert(
                        import_name,
                        Function {
                            variables: IndexMap::new(),
                            arguments: arg_map,
                            returns: ret_typ.clone(),
                        },
                    );
                }
                Type::Void
            }
            Stmt::Macro(name, args, expr) => {
                ctx.macro_code
                    .insert(name.to_owned(), (args.clone(), expr.clone()));
                Type::Void
            }
            Stmt::Return(_) => Type::Void,
        })
    }
}
