use crate::*;

#[derive(Debug, Clone)]
pub enum Oper {
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Mod(Expr, Expr),
    Shr(Expr, Expr),
    Shl(Expr, Expr),
    Eql(Expr, Expr),
    Neq(Expr, Expr),
    Lt(Expr, Expr),
    Gt(Expr, Expr),
    LtEq(Expr, Expr),
    GtEq(Expr, Expr),
    BAnd(Expr, Expr),
    BOr(Expr, Expr),
    BNot(Expr),
    XOr(Expr, Expr),
    LAnd(Expr, Expr),
    LOr(Expr, Expr),
    LNot(Expr),
    Cast(Expr, Type),
    NullCheck(Expr),
    Nullable(Type),
    Transmute(Expr, Type),
}

impl Node for Oper {
    fn parse(source: &str) -> Option<Self> {
        let token_list: Vec<String> = tokenize(source, SPACE.as_ref(), true, true, false)?;
        // Parsing is from right to left because operator is left-associative
        let binopergen = |n: usize| {
            let operator = token_list.get(n)?;
            let lhs = &join!(token_list.get(..n)?);
            let rhs = &join!(token_list.get(n + 1..)?);
            Some(match operator.as_str() {
                "+" => Oper::Add(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "-" => Oper::Sub(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "*" => Oper::Mul(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "/" => Oper::Div(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "%" => Oper::Mod(Expr::parse(lhs)?, Expr::parse(rhs)?),
                ">>" => Oper::Shr(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "<<" => Oper::Shl(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "==" => Oper::Eql(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "!=" => Oper::Neq(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "<" => Oper::Lt(Expr::parse(lhs)?, Expr::parse(rhs)?),
                ">" => Oper::Gt(Expr::parse(lhs)?, Expr::parse(rhs)?),
                ">=" => Oper::GtEq(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "<=" => Oper::LtEq(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "&" => Oper::BAnd(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "|" => Oper::BOr(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "^" => Oper::XOr(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "&&" => Oper::LAnd(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "||" => Oper::LOr(Expr::parse(lhs)?, Expr::parse(rhs)?),
                ":" => Oper::Cast(Expr::parse(lhs)?, Type::parse(rhs)?),
                _ => return None,
            })
        };
        let unaryopergen = || {
            let oper = token_list.first()?.trim();
            let token = &join!(token_list.get(1..)?);
            Some(match oper {
                "~" => Oper::BNot(Expr::parse(token)?),
                "!" => Oper::LNot(Expr::parse(token)?),
                "-" => Oper::Sub(
                    Expr::Oper(Box::new(Oper::Sub(
                        Expr::parse(token)?,
                        Expr::parse(token)?,
                    ))),
                    Expr::parse(token)?,
                ),
                _ => return None,
            })
        };
        let suffixopergen = || {
            let oper = token_list.last()?.trim();
            let token = &join!(token_list.get(..token_list.len() - 1)?);
            Some(match oper {
                "?" => Oper::NullCheck(Expr::parse(token)?),
                "!" => Oper::Nullable(Type::parse(token)?),
                _ => return None,
            })
        };
        if let Some(op) = unaryopergen() {
            return Some(op);
        }
        if let Some(op) = suffixopergen() {
            return Some(op);
        }
        for i in 2..token_list.len() {
            if let Some(op) = binopergen(token_list.len().checked_sub(i)?) {
                return Some(op);
            }
        }
        None
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Oper::Sub(lhs, rhs) => compile_arithmetic!("sub", self, ctx, lhs, rhs),
            Oper::Mul(lhs, rhs) => compile_arithmetic!("mul", self, ctx, lhs, rhs),
            Oper::Div(lhs, rhs) => compile_compare!("div", ctx, lhs, rhs),
            Oper::Shr(lhs, rhs) => compile_compare!("shr", ctx, lhs, rhs),
            Oper::Shl(lhs, rhs) => compile_arithmetic!("shl", self, ctx, lhs, rhs),
            Oper::BAnd(lhs, rhs) => compile_arithmetic!("and", self, ctx, lhs, rhs),
            Oper::BOr(lhs, rhs) => compile_arithmetic!("or", self, ctx, lhs, rhs),
            Oper::XOr(lhs, rhs) => compile_arithmetic!("xor", self, ctx, lhs, rhs),
            Oper::LNot(lhs) => compile_compare!("eqz", ctx, lhs),
            Oper::Neq(lhs, rhs) => compile_arithmetic!("ne", self, ctx, lhs, rhs),
            Oper::Lt(lhs, rhs) => compile_compare!("lt", ctx, lhs, rhs),
            Oper::Gt(lhs, rhs) => compile_compare!("gt", ctx, lhs, rhs),
            Oper::LtEq(lhs, rhs) => compile_compare!("le", ctx, lhs, rhs),
            Oper::GtEq(lhs, rhs) => compile_compare!("ge", ctx, lhs, rhs),
            Oper::LAnd(lhs, rhs) => compile_arithmetic!("and", self, ctx, lhs, rhs),
            Oper::LOr(lhs, rhs) => compile_arithmetic!("or", self, ctx, lhs, rhs),
            Oper::Add(lhs, rhs) => {
                let typ = self.type_infer(ctx)?;
                if let Type::String = typ {
                    Expr::Call(String::from("concat"), vec![lhs.clone(), rhs.clone()])
                        .compile(ctx)?
                } else if let Type::Number | Type::Integer = typ {
                    compile_arithmetic!("add", self, ctx, lhs, rhs)
                } else {
                    return None;
                }
            }
            Oper::Eql(lhs, rhs) => {
                if let (Type::String, Type::String) = (lhs.type_infer(ctx)?, rhs.type_infer(ctx)?) {
                    Expr::Call(String::from("strcmp"), vec![lhs.clone(), rhs.clone()])
                        .compile(ctx)?
                } else {
                    compile_arithmetic!("eq", self, ctx, lhs, rhs)
                }
            }
            Oper::Mod(lhs, rhs) => {
                let typ = lhs.type_infer(ctx)?.compile(ctx)?;
                let (lhs, rhs) = (lhs.compile(ctx)?, rhs.compile(ctx)?);
                if typ == "i32" {
                    format!("(i32.rem_s (i32.add (i32.rem_s {lhs} {rhs}) {rhs}) {rhs})")
                } else {
                    format!("(f64.sub {lhs} (f64.mul (f64.floor (f64.div {lhs} {rhs})) {rhs}))")
                }
            }
            Oper::BNot(lhs) => {
                let minus_one = Expr::Literal(Value::Integer(-1));
                compile_arithmetic!("xor", self, ctx, lhs, minus_one)
            }
            Oper::Cast(lhs, rhs) => {
                let rhs = rhs.type_infer(ctx)?;
                if let (Type::Number | Type::Integer, Type::String) = (lhs.type_infer(ctx)?, &rhs) {
                    let numized = Expr::Oper(Box::new(Oper::Cast(lhs.clone(), Type::Number)));
                    Expr::Call(String::from("to_str"), vec![numized]).compile(ctx)?
                } else if let (Type::String, Type::Number | Type::Integer) =
                    (lhs.type_infer(ctx)?, &rhs)
                {
                    Oper::Cast(Expr::Call(String::from("to_num"), vec![lhs.clone()]), rhs)
                        .compile(ctx)?
                } else if let (Type::Integer, Type::Number) = (lhs.type_infer(ctx)?, &rhs) {
                    format!("(f64.convert_i32_s {})", lhs.compile(ctx)?,)
                } else if let (Type::Number, Type::Integer) = (lhs.type_infer(ctx)?, &rhs) {
                    format!("(i32.trunc_f64_s {})", lhs.compile(ctx)?,)
                } else if matches!(lhs.type_infer(ctx)?, Type::Any)
                    || lhs.type_infer(ctx)?.type_infer(ctx)?.format() == rhs.format()
                {
                    lhs.compile(ctx)?
                } else {
                    let lhs = lhs.type_infer(ctx)?.format();
                    let msg = format!("type {lhs} can't convert to {}", rhs.format());
                    ctx.occurred_error = Some(msg);
                    return None;
                }
            }
            Oper::Transmute(lhs, _) => lhs.compile(ctx)?,
            Oper::NullCheck(expr) => Oper::Neq(
                Expr::Oper(Box::new(Oper::Transmute(expr.clone(), Type::Integer))),
                Expr::Literal(Value::Integer(-1)),
            )
            .compile(ctx)?,
            Oper::Nullable(_) => Value::Null.compile(ctx)?,
        })
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        match self {
            Oper::Add(lhs, rhs) => {
                correct!(lhs, rhs, ctx, Type::Number | Type::Integer | Type::String)
            }
            Oper::Sub(lhs, rhs)
            | Oper::Mul(lhs, rhs)
            | Oper::Div(lhs, rhs)
            | Oper::Mod(lhs, rhs)
            | Oper::Shr(lhs, rhs)
            | Oper::Shl(lhs, rhs)
            | Oper::BAnd(lhs, rhs)
            | Oper::BOr(lhs, rhs)
            | Oper::XOr(lhs, rhs) => correct!(lhs, rhs, ctx, Type::Number | Type::Integer),
            Oper::Eql(lhs, rhs) | Oper::Neq(lhs, rhs) => {
                correct!(
                    lhs,
                    rhs,
                    ctx,
                    Type::Number | Type::Integer | Type::String | Type::Enum(_)
                )?;
                Some(Type::Bool)
            }
            Oper::Lt(lhs, rhs)
            | Oper::Gt(lhs, rhs)
            | Oper::LtEq(lhs, rhs)
            | Oper::GtEq(lhs, rhs) => {
                correct!(lhs, rhs, ctx, Type::Number | Type::Integer)?;
                Some(Type::Bool)
            }
            Oper::LAnd(lhs, rhs) | Oper::LOr(lhs, rhs) => {
                type_check!(lhs, Type::Bool, ctx)?;
                type_check!(rhs, Type::Bool, ctx)?;
                Some(Type::Bool)
            }
            Oper::LNot(lhs) => {
                type_check!(lhs, Type::Bool, ctx)?;
                Some(Type::Bool)
            }
            Oper::Cast(lhs, rhs) => {
                lhs.type_infer(ctx)?;
                rhs.type_infer(ctx)
            }
            Oper::BNot(lhs) => {
                type_check!(lhs, Type::Integer, ctx)?;
                Some(Type::Integer)
            }
            Oper::Transmute(lhs, rhs) => {
                lhs.type_infer(ctx)?;
                rhs.type_infer(ctx)
            }
            Oper::NullCheck(expr) => {
                expr.type_infer(ctx)?;
                Some(Type::Bool)
            }
            Oper::Nullable(typ) => Some(typ.clone()),
        }
    }
}
