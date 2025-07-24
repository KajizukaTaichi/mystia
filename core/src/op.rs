use crate::*;

#[derive(Debug, Clone)]
pub enum Op {
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

impl Node for Op {
    fn parse(source: &str) -> Option<Self> {
        let token_list: Vec<String> = tokenize(source, SPACE.as_ref(), true, true, false)?;
        // Parsing is from right to left because operator is left-associative
        let binopergen = |n: usize| {
            let operator = token_list.get(n)?;
            let lhs = &join!(token_list.get(..n)?);
            let rhs = &join!(token_list.get(n + 1..)?);
            Some(match operator.as_str() {
                "+" => Op::Add(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "-" => Op::Sub(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "*" => Op::Mul(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "/" => Op::Div(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "%" => Op::Mod(Expr::parse(lhs)?, Expr::parse(rhs)?),
                ">>" => Op::Shr(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "<<" => Op::Shl(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "==" => Op::Eql(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "!=" => Op::Neq(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "<" => Op::Lt(Expr::parse(lhs)?, Expr::parse(rhs)?),
                ">" => Op::Gt(Expr::parse(lhs)?, Expr::parse(rhs)?),
                ">=" => Op::GtEq(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "<=" => Op::LtEq(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "&" => Op::BAnd(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "|" => Op::BOr(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "^" => Op::XOr(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "&&" => Op::LAnd(Expr::parse(lhs)?, Expr::parse(rhs)?),
                "||" => Op::LOr(Expr::parse(lhs)?, Expr::parse(rhs)?),
                ":" => Op::Cast(Expr::parse(lhs)?, Type::parse(rhs)?),
                _ => return None,
            })
        };
        let unaryopergen = || {
            let oper = token_list.first()?.trim();
            let token = &join!(token_list.get(1..)?);
            Some(match oper {
                "~" => Op::BNot(Expr::parse(token)?),
                "!" => Op::LNot(Expr::parse(token)?),
                "-" => Op::Sub(
                    Expr::Operator(Box::new(Op::Sub(Expr::parse(token)?, Expr::parse(token)?))),
                    Expr::parse(token)?,
                ),
                _ => return None,
            })
        };
        let suffixopergen = || {
            let oper = token_list.last()?.trim();
            let token = &join!(token_list.get(..token_list.len() - 1)?);
            Some(match oper {
                "?" => Op::NullCheck(Expr::parse(token)?),
                "!" => Op::Nullable(Type::parse(token)?),
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
            Op::Sub(lhs, rhs) => compile_arithmetic!("sub", self, ctx, lhs, rhs),
            Op::Mul(lhs, rhs) => compile_arithmetic!("mul", self, ctx, lhs, rhs),
            Op::Div(lhs, rhs) => compile_compare!("div", ctx, lhs, rhs),
            Op::Shr(lhs, rhs) => compile_compare!("shr", ctx, lhs, rhs),
            Op::Shl(lhs, rhs) => compile_arithmetic!("shl", self, ctx, lhs, rhs),
            Op::BAnd(lhs, rhs) => compile_arithmetic!("and", self, ctx, lhs, rhs),
            Op::BOr(lhs, rhs) => compile_arithmetic!("or", self, ctx, lhs, rhs),
            Op::XOr(lhs, rhs) => compile_arithmetic!("xor", self, ctx, lhs, rhs),
            Op::LNot(lhs) => compile_compare!("eqz", ctx, lhs),
            Op::Neq(lhs, rhs) => compile_arithmetic!("ne", self, ctx, lhs, rhs),
            Op::Lt(lhs, rhs) => compile_compare!("lt", ctx, lhs, rhs),
            Op::Gt(lhs, rhs) => compile_compare!("gt", ctx, lhs, rhs),
            Op::LtEq(lhs, rhs) => compile_compare!("le", ctx, lhs, rhs),
            Op::GtEq(lhs, rhs) => compile_compare!("ge", ctx, lhs, rhs),
            Op::LAnd(lhs, rhs) => compile_arithmetic!("and", self, ctx, lhs, rhs),
            Op::LOr(lhs, rhs) => compile_arithmetic!("or", self, ctx, lhs, rhs),
            Op::Add(lhs, rhs) => {
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
            Op::Eql(lhs, rhs) => {
                if let (Type::String, Type::String) = (lhs.type_infer(ctx)?, rhs.type_infer(ctx)?) {
                    Expr::Call(String::from("strcmp"), vec![lhs.clone(), rhs.clone()])
                        .compile(ctx)?
                } else {
                    compile_arithmetic!("eq", self, ctx, lhs, rhs)
                }
            }
            Op::Mod(lhs, rhs) => {
                let typ = lhs.type_infer(ctx)?.compile(ctx)?;
                let (lhs, rhs) = (lhs.compile(ctx)?, rhs.compile(ctx)?);
                if typ == "i32" {
                    format!("(i32.rem_s (i32.add (i32.rem_s {lhs} {rhs}) {rhs}) {rhs})")
                } else {
                    format!("(f32.sub {lhs} (f32.mul (f32.floor (f32.div {lhs} {rhs})) {rhs}))")
                }
            }
            Op::BNot(lhs) => {
                let minus_one = Expr::Literal(Value::Integer(-1));
                compile_arithmetic!("xor", self, ctx, lhs, minus_one)
            }
            Op::Cast(lhs, rhs) => {
                let rhs = rhs.type_infer(ctx)?;
                if let (Type::Number | Type::Integer, Type::String) = (lhs.type_infer(ctx)?, &rhs) {
                    let numized = Expr::Operator(Box::new(Op::Cast(lhs.clone(), Type::Number)));
                    Expr::Call(String::from("to_str"), vec![numized]).compile(ctx)?
                } else if let (Type::String, Type::Number | Type::Integer) =
                    (lhs.type_infer(ctx)?, &rhs)
                {
                    Op::Cast(Expr::Call(String::from("to_num"), vec![lhs.clone()]), rhs)
                        .compile(ctx)?
                } else if let (Type::Integer, Type::Number) = (lhs.type_infer(ctx)?, &rhs) {
                    format!("(f32.convert_i32_s {})", lhs.compile(ctx)?,)
                } else if let (Type::Number, Type::Integer) = (lhs.type_infer(ctx)?, &rhs) {
                    format!("(i32.trunc_f32_s {})", lhs.compile(ctx)?,)
                } else if matches!(lhs.type_infer(ctx)?, Type::Any)
                    || lhs.type_infer(ctx)?.type_infer(ctx)?.format() == rhs.format()
                {
                    lhs.compile(ctx)?
                } else {
                    let [lhs, rhs] = [lhs.type_infer(ctx)?.format(), rhs.format()];
                    let msg = format!("type {lhs} can't convert to {rhs}");
                    ctx.occurred_error = Some(msg);
                    return None;
                }
            }
            Op::Transmute(lhs, _) => lhs.compile(ctx)?,
            Op::NullCheck(expr) => Op::Neq(
                Expr::Operator(Box::new(Op::Transmute(expr.clone(), Type::Integer))),
                Expr::Literal(Value::Integer(-1)),
            )
            .compile(ctx)?,
            Op::Nullable(_) => Value::Null.compile(ctx)?,
        })
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        match self {
            Op::Add(lhs, rhs) => {
                correct!(lhs, rhs, ctx, Type::Number | Type::Integer | Type::String)
            }
            Op::Sub(lhs, rhs)
            | Op::Mul(lhs, rhs)
            | Op::Div(lhs, rhs)
            | Op::Mod(lhs, rhs)
            | Op::Shr(lhs, rhs)
            | Op::Shl(lhs, rhs)
            | Op::BAnd(lhs, rhs)
            | Op::BOr(lhs, rhs)
            | Op::XOr(lhs, rhs) => correct!(lhs, rhs, ctx, Type::Number | Type::Integer),
            Op::Eql(lhs, rhs) | Op::Neq(lhs, rhs) => {
                correct!(
                    lhs,
                    rhs,
                    ctx,
                    Type::Number | Type::Integer | Type::String | Type::Enum(_)
                )?;
                Some(Type::Bool)
            }
            Op::Lt(lhs, rhs) | Op::Gt(lhs, rhs) | Op::LtEq(lhs, rhs) | Op::GtEq(lhs, rhs) => {
                correct!(lhs, rhs, ctx, Type::Number | Type::Integer)?;
                Some(Type::Bool)
            }
            Op::LAnd(lhs, rhs) | Op::LOr(lhs, rhs) => {
                type_check!(lhs, Type::Bool, ctx)?;
                type_check!(rhs, Type::Bool, ctx)?;
                Some(Type::Bool)
            }
            Op::LNot(lhs) => {
                type_check!(lhs, Type::Bool, ctx)?;
                Some(Type::Bool)
            }
            Op::Cast(lhs, rhs) => {
                lhs.type_infer(ctx)?;
                rhs.type_infer(ctx)
            }
            Op::BNot(lhs) => {
                type_check!(lhs, Type::Integer, ctx)?;
                Some(Type::Integer)
            }
            Op::Transmute(lhs, rhs) => {
                lhs.type_infer(ctx)?;
                rhs.type_infer(ctx)
            }
            Op::NullCheck(expr) => {
                expr.type_infer(ctx)?;
                Some(Type::Bool)
            }
            Op::Nullable(typ) => Some(typ.clone()),
        }
    }
}
