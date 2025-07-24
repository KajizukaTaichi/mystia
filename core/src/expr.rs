use crate::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Variable(String),
    Operator(Box<Op>),
    Call(String, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Field(Box<Expr>, String),
    Block(Block),
    MemCpy(Box<Expr>),
    MemLoad(Box<Expr>, Type),
}

impl Node for Expr {
    fn parse(source: &str) -> Option<Expr> {
        let token = source.trim();
        // Operator
        if let Some(literal) = Op::parse(&token) {
            Some(Expr::Operator(Box::new(literal)))
        // Literal value
        } else if let Some(literal) = Value::parse(&token) {
            Some(Expr::Literal(literal))
        // Prioritize expression `(expr)`
        } else if token.starts_with("(") && token.ends_with(")") {
            let token = token.get(1..token.len() - 1)?.trim();
            Some(Expr::parse(token)?)
        // Code block `{ stmt; ... }`
        } else if token.starts_with("{") && token.ends_with("}") {
            let token = token.get(1..token.len() - 1)?.trim();
            Some(Expr::Block(Block::parse(token)?))
        // Index access `array[index]`
        } else if token.contains("[") && token.ends_with("]") {
            let token = token.get(..token.len() - 1)?.trim();
            let (array, index) = token.rsplit_once("[")?;
            let (array, index) = (Expr::parse(array)?, Expr::parse(index)?);
            Some(Expr::Index(Box::new(array), Box::new(index)))
        // Function call `name(args, ...)`
        } else if token.contains("(") && token.ends_with(")") {
            let token = tokenize(token, &["("], false, true, true)?;
            let name = token.get(0..token.len() - 1)?.concat();
            let args = token.last()?.get(1..token.last()?.len() - 1)?;
            let args = tokenize(args, &[","], false, true, false)?;
            let args = args.iter().map(|i| Expr::parse(&i));
            let args = args.collect::<Option<Vec<_>>>()?;
            match Expr::parse(&name)? {
                Expr::Variable(name) if name == "memcpy" => {
                    Some(Expr::MemCpy(Box::new(args.first()?.clone())))
                }
                Expr::Variable(name) => Some(Expr::Call(name, args)),
                Expr::Field(obj, name) if name == "memcpy" => Some(Expr::MemCpy(obj)),
                Expr::Field(obj, name) => Some(Expr::Call(name, [vec![*obj], args].concat())),
                _ => None,
            }
        // Dictionary access `dict.field`
        } else if token.contains(".") {
            let (dict, field) = token.rsplit_once(".")?;
            if !is_identifier(field) {
                return None;
            };
            Some(Expr::Field(Box::new(Expr::parse(dict)?), field.to_owned()))
        // Enumerate access `( a | b )#a`
        } else if source.contains("#") {
            let (typ, enum_) = source.rsplit_once("#")?;
            let enum_ = Value::Enum(Type::parse(typ)?, enum_.to_owned());
            Some(Expr::Literal(enum_))
        // Variable reference
        } else if is_identifier(token) {
            Some(Expr::Variable(token.to_string()))
        } else {
            None
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Expr::Operator(oper) => oper.compile(ctx)?,
            Expr::Variable(name) if ctx.global_type.contains_key(name) => {
                format!("(global.get ${name})")
            }
            Expr::Variable(name) => format!("(local.get ${name})"),
            Expr::Literal(literal) => literal.compile(ctx)?,
            Expr::Call(name, args) => {
                if ctx.function_type.contains_key(name) || ctx.export_type.contains_key(name) {
                    let args = args
                        .iter()
                        .map(|x| x.compile(ctx))
                        .collect::<Option<Vec<_>>>()?;
                    format!("(call ${name} {})", join!(args))
                } else if let Some((params, expr)) = ctx.macro_code.get(name).cloned() {
                    for (param, arg) in params.iter().zip(args) {
                        let typ = arg.type_infer(ctx)?;
                        ctx.variable_type.insert(param.to_owned(), typ);
                    }
                    let mut body = expr.compile(ctx)?;
                    for (param, arg) in params.iter().zip(args) {
                        let var = Expr::Variable(param.to_owned()).compile(ctx)?;
                        body = body.replace(&var, &arg.compile(ctx)?);
                    }
                    body
                } else {
                    return None;
                }
            }
            Expr::Index(array, index) => {
                let Type::Array(typ) = array.type_infer(ctx)?.type_infer(ctx)? else {
                    return None;
                };
                let addr = Box::new(address_calc!(array, index, typ));
                Expr::MemLoad(Box::new(Expr::Operator(addr)), *typ).compile(ctx)?
            }
            Expr::Field(expr, key) => {
                let typ = expr.type_infer(ctx)?.type_infer(ctx)?;
                let Type::Dict(dict) = typ else {
                    return None;
                };
                let (offset, typ) = dict.get(key)?.clone();
                let addr = offset_calc!(expr, offset);
                Expr::MemLoad(Box::new(addr), typ).compile(ctx)?
            }
            Expr::Block(block) => block.compile(ctx)?,
            Expr::MemCpy(from) => {
                let typ = from.type_infer(ctx)?;
                let size = from.bytes_length(ctx)?.compile(ctx)?;
                if is_ptr!(typ) {
                    return Some(format!(
                        "(global.get $allocator) (memory.copy (global.get $allocator) {object} {size}) {}",
                        format!("(global.set $allocator (i32.add (global.get $allocator) {size}))"),
                        object = from.compile(ctx)?,
                    ));
                } else {
                    ctx.occurred_error =
                        Some("can't memory copy primitive typed value".to_string());
                    return None;
                }
            }
            Expr::MemLoad(expr, typ) => {
                format!("({}.load {})", typ.compile(ctx)?, expr.compile(ctx)?)
            }
        })
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        Some(match self {
            Expr::Operator(oper) => oper.type_infer(ctx)?,
            Expr::Variable(name) => {
                if let Some(global) = ctx.global_type.get(name) {
                    global.clone()
                } else if let Some(local) = ctx.variable_type.get(name) {
                    local.clone()
                } else if let Some(arg) = ctx.argument_type.get(name) {
                    arg.clone()
                } else {
                    ctx.occurred_error = Some(format!("undefined variable `{name}`"));
                    return None;
                }
            }
            Expr::Literal(literal) => literal.type_infer(ctx)?,
            Expr::Call(name, args) => {
                macro_rules! arglen_check {
                    ($params: expr, $typ: literal) => {
                        if args.len() != $params.len() {
                            let (typ, paramlen, arglen) = ($typ, $params.len(), args.len());
                            let errmsg = format!("arguments of {typ} `{name}` length should be {paramlen}, but passed {arglen} values");
                            ctx.occurred_error = Some(errmsg);
                            return None;
                        }
                    };
                }
                if let Some(function) = ctx
                    .function_type
                    .get(name)
                    .or(ctx.export_type.get(name))
                    .cloned()
                {
                    arglen_check!(function.arguments, "function");
                    let func = |(arg, typ): (&Expr, &Type)| type_check!(arg, typ, ctx);
                    let ziped = args.iter().zip(function.arguments.values());
                    ziped.map(func).collect::<Option<Vec<_>>>()?;
                    function.returns.type_infer(ctx)?
                } else if let Some((params, expr)) = ctx.macro_code.get(name).cloned() {
                    arglen_check!(params, "macro");
                    let var_ctx = ctx.variable_type.clone();
                    for (params, arg) in params.iter().zip(args) {
                        let typ = arg.type_infer(ctx)?;
                        ctx.variable_type.insert(params.to_owned(), typ);
                    }
                    let typ = expr.type_infer(ctx)?;
                    ctx.variable_type = var_ctx;
                    typ
                } else {
                    ctx.occurred_error = Some(format!(
                        "function or macro `{name}` you want to call is not defined"
                    ));
                    return None;
                }
            }
            Expr::Index(arr, _) => {
                let infered = arr.type_infer(ctx)?;
                let Some(Type::Array(typ)) = infered.type_infer(ctx) else {
                    let error_message = format!("can't index access to {}", infered.format());
                    ctx.occurred_error = Some(error_message);
                    return None;
                };
                typ.type_infer(ctx)?
            }
            Expr::Field(dict, key) => {
                let infered = dict.type_infer(ctx)?.type_infer(ctx)?;
                if let Type::Dict(dict) = infered.clone() {
                    let Some((_offset, typ)) = dict.get(key) else {
                        let error_message = format!("{} haven't field `{key}`", infered.format());
                        ctx.occurred_error = Some(error_message);
                        return None;
                    };
                    typ.type_infer(ctx)?
                } else {
                    let error_message = format!("can't field access to {}", infered.format());
                    ctx.occurred_error = Some(error_message);
                    return None;
                }
            }
            Expr::Block(block) => block.type_infer(ctx)?,
            Expr::MemCpy(from) => from.type_infer(ctx)?,
            Expr::MemLoad(_, typ) => typ.clone(),
        })
    }
}

impl Expr {
    pub fn bytes_length(&self, ctx: &mut Compiler) -> Option<Expr> {
        match self.type_infer(ctx)? {
            Type::Dict(dict) => Some(Expr::Literal(Value::Integer(dict.len() as i32 * BYTES))),
            Type::Array(_) => Some(Expr::Operator(Box::new(Op::Add(
                Expr::Operator(Box::new(Op::Mul(
                    Expr::Literal(Value::Integer(BYTES)),
                    Expr::MemLoad(Box::new(self.clone()), Type::Integer),
                ))),
                Expr::Literal(Value::Integer(BYTES)),
            )))),
            _ => None,
        }
    }
}
