use crate::*;

pub const SPACE: [&str; 5] = [" ", "　", "\n", "\t", "\r"];
pub const OPERATOR: [&str; 13] = [
    "+", "-", "*", "/", "%", "^", "==", "!=", "<=", ">=", "<", ">", ":",
];
pub const RESERVED: [&str; 9] = [
    "let", "if", "then", "else", "while", "loop", "break", "next", "return",
];

pub fn include_letter(query: &str, chars: &Vec<String>, idx: usize) -> bool {
    chars
        .clone()
        .get(idx..idx + query.chars().count())
        .map(|i| query == i.concat())
        .unwrap_or(false)
}

pub fn expand_local(ctx: &mut Compiler) -> Option<String> {
    Some(join!(iter_map!(ctx.variable_type.clone(), |x: (
        String,
        Type
    )| Some(
        format!("(local ${} {})", x.0, x.1.compile(ctx)?)
    ))))
}

#[macro_export]
macro_rules! config_return {
    ($ret: expr, $ctx: expr) => {
        if let Type::Void = $ret {
            Some(String::new())
        } else {
            Some(format!("(result {})", $ret.compile($ctx)?))
        }
    };
}

#[macro_export]
macro_rules! if_ptr {
    ($typ: expr, $block: block) => {
        if let Type::String | Type::Array(_, _) | Type::Dict(_) = $typ {
            $block;
        }
    };
    ($typ: expr, $block: block else $els: block) => {
        if let Type::String | Type::Array(_, _) | Type::Dict(_) = $typ {
            $block;
        } else {
            $els;
        }
    };
}

#[macro_export]
macro_rules! type_check {
    ($lhs: expr, $rhs: expr, $ctx: expr) => {{
        let lhs = $lhs.type_infer($ctx);
        let rhs = $rhs.type_infer($ctx);
        if let (Some(lhs), Some(rhs)) = (&lhs, &rhs) {
            if lhs.format() == rhs.format() {
                Some(lhs.clone())
            } else {
                None
            }
        } else if let (Some(typ), _) | (_, Some(typ)) = (lhs, rhs) {
            $ctx.expect_type = Some(typ);
            let lhs = $lhs.type_infer($ctx)?;
            let rhs = $rhs.type_infer($ctx)?;
            if lhs.format() == rhs.format() {
                Some(lhs.clone())
            } else {
                None
            }
        } else {
            None
        }
    }};
}

#[macro_export]
macro_rules! compile_compare {
    ($oper: expr, $ctx: expr, $lhs: expr, $rhs: expr) => {{
        let ret = type_check!($lhs, $rhs, $ctx)?.compile($ctx)?;
        format!(
            "({}.{}{} {} {})",
            ret,
            $oper,
            if ret == "i32" { "_s" } else { "" },
            $lhs.compile($ctx)?,
            $rhs.compile($ctx)?
        )
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
macro_rules! iter_map {
    ($iter: expr, $proc: expr) => {{
        let mut result = vec![];
        for i in $iter {
            result.push($proc(i)?);
        }
        result
    }};
}

#[macro_export]
macro_rules! join {
    ($x:expr) => {
        $x.join(&SPACE[0].to_string())
    };
}
