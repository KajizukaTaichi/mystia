use crate::*;

pub const SPACE: [&str; 5] = [" ", "　", "\n", "\t", "\r"];
pub const OPERATOR: [&str; 23] = [
    "+", "-", "*", "/", "%", "==", "=", "!=", "<<", ">>", "<=", ">=", "<", ">", "&&", "||", "&",
    "|", "^", ":", "!", "?", "~",
];
pub const RESERVED: [&str; 16] = [
    "pub", "let", "type", "if", "then", "else", "while", "loop", "break", "next", "return", "load",
    "as", "try", "catch", "use",
];

pub fn is_identifier(name: &str) -> bool {
    name.split_whitespace().count() == 1
        && !RESERVED.contains(&name)
        && !OPERATOR.contains(&name)
        && name.is_ascii()
}

pub fn include_letter(query: &str, chars: &Vec<String>, idx: usize) -> bool {
    chars
        .clone()
        .get(idx..idx + query.chars().count())
        .map(|i| query == i.concat())
        .unwrap_or(false)
}

pub fn expand_local(ctx: &mut Compiler) -> Option<String> {
    Some(join!(
        ctx.variable_type
            .clone()
            .iter()
            .map(|(name, typ)| { Some(format!("(local ${name} {})", typ.compile(ctx)?)) })
            .collect::<Option<Vec<String>>>()?
    ))
}

#[macro_export]
macro_rules! compile_return {
    ($ret: expr, $ctx: expr) => {{
        let ret = $ret.type_infer($ctx)?;
        if let Type::Void = ret {
            String::new()
        } else {
            format!("(result {})", ret.compile($ctx)?)
        }
    }};
}

#[macro_export]
macro_rules! compile_args_type {
    ($function: expr, $ctx: expr) => {
        format!(
            "(param {})",
            join!(
                $function
                    .arguments
                    .iter()
                    .map(|(_, typ)| typ.compile($ctx))
                    .collect::<Option<Vec<_>>>()?
            )
        )
    };
}

#[macro_export]
macro_rules! if_ptr {
    ($typ: expr => $block: block) => {
        if let Type::String | Type::Array(_) | Type::Dict(_) = $typ {
            $block;
        }
    };
    ($typ: expr => $block: block else $els: block) => {
        if let Type::String | Type::Array(_) | Type::Dict(_) = $typ {
            $block;
        } else {
            $els;
        }
    };
}

#[macro_export]
macro_rules! type_check {
    ($lhs: expr, $rhs: expr, $ctx: expr) => {{
        let lhs = $lhs.type_infer($ctx)?.type_infer($ctx)?;
        let rhs = $rhs.type_infer($ctx)?.type_infer($ctx)?;
        if lhs.format() == rhs.format() {
            Some(lhs.clone())
        } else {
            $ctx.occurred_error = Some(format!(
                "type mismatch between {} and {}",
                lhs.format(),
                rhs.format()
            ));
            None
        }
    }};
}

#[macro_export]
macro_rules! compile_compare {
    ($oper: expr, $ctx: expr, $lhs: expr, $rhs: expr) => {{
        let ret = type_check!($lhs, $rhs, $ctx)?.compile($ctx)?;
        format!(
            "({ret}.{}{} {} {})",
            $oper,
            if ret == "i32" { "_s" } else { "" },
            $lhs.compile($ctx)?,
            $rhs.compile($ctx)?
        )
    }};
    ($oper: expr, $ctx: expr, $lhs: expr) => {{
        let ret = $lhs.type_infer($ctx)?.compile($ctx)?;
        format!("({}.{} {})", ret, $oper, $lhs.compile($ctx)?)
    }};
}

#[macro_export]
macro_rules! address_calc {
    ($array: expr, $index: expr, $typ: expr) => {
        Op::Add(
            Expr::Operator(Box::new(Op::Add(
                Expr::Literal(Value::Integer(4)),
                Expr::Operator(Box::new(Op::Transmute(*$array.clone(), Type::Integer))),
            ))),
            Expr::Operator(Box::new(Op::Mul(
                Expr::Operator(Box::new(Op::Mod(
                    *$index.clone(),
                    Expr::MemLoad($array.clone(), Type::Integer),
                ))),
                Expr::Literal(Value::Integer($typ.pointer_length()?)),
            ))),
        )
    };
}

#[macro_export]
macro_rules! compile_args {
    ($args: expr, $ctx: expr) => {
        for arg in $args {
            let Expr::Operator(oper) = arg else {
                let msg = "function argument definition needs type annotation";
                $ctx.occurred_error = Some(msg.to_string());
                return None;
            };
            let Op::Cast(Expr::Variable(name), typ) = *oper.clone() else {
                let msg = "function argument name should be identifier";
                $ctx.occurred_error = Some(msg.to_string());
                return None;
            };
            if let Some(typ) = typ.type_infer($ctx) {
                $ctx.argument_type.insert(name.to_string(), typ);
            } else {
                $ctx.argument_type.insert(name.to_string(), typ);
            }
        }
    };
}

#[macro_export]
macro_rules! offset_calc {
    ($dict: expr, $offset: expr) => {
        Expr::Operator(Box::new(Op::Add(
            Expr::Operator(Box::new(Op::Transmute(*$dict.clone(), Type::Integer))),
            Expr::Literal(Value::Integer($offset.clone())),
        )))
    };
}

#[macro_export]
macro_rules! compile_arithmetic {
    ($oper: expr, $self: expr, $ctx: expr, $lhs: expr, $rhs: expr) => {{
        type_check!($lhs, $rhs, $ctx)?;
        format!(
            "({}.{} {} {})",
            $lhs.type_infer($ctx)?.compile($ctx)?,
            $oper,
            $lhs.compile($ctx)?,
            $rhs.compile($ctx)?
        )
    }};
}

#[macro_export]
macro_rules! correct {
    ($lhs: expr, $rhs: expr , $ctx: expr, $pat: pat) => {{
        let ret = type_check!($lhs, $rhs, $ctx)?;
        if let $pat = ret {
            Some(ret)
        } else {
            let msg = format!(
                "can't mathematical operation between {} and {}",
                $lhs.type_infer($ctx)?.format(),
                $rhs.type_infer($ctx)?.format()
            );
            $ctx.occurred_error = Some(msg);
            None
        }
    }};
}

#[macro_export]
macro_rules! ok {
    ($result:expr) => {
        if let Ok(val) = $result {
            Some(val)
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! join {
    ($x:expr) => {
        $x.join(&SPACE[0].to_string())
    };
}
