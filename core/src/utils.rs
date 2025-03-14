use crate::*;

pub const SPACE: [&str; 5] = [" ", "　", "\n", "\t", "\r"];
pub const OPERATOR: [&str; 12] = [
    "+", "-", "*", "/", "%", "^", "==", "!=", "<=", ">=", "<", ">",
];

pub fn include_letter(query: &str, chars: &Vec<String>, idx: usize) -> bool {
    chars
        .clone()
        .get(idx..idx + query.chars().count())
        .map(|i| query == i.concat())
        .unwrap_or(false)
}

pub fn expand_local(ctx: &mut Compiler) -> Option<String> {
    Some(join!(iter_map!(ctx.variable, |x: (String, Expr)| format!(
        "(local ${} {})",
        x.0,
        x.1.compile(ctx)?
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
macro_rules! type_check {
    ($lhs: expr, $rhs: expr, $ctx: expr) => {
        if format!("{:?}", $lhs.type_infer($ctx)?) == format!("{:?}", $rhs.type_infer($ctx)?) {
            Some($lhs.type_infer($ctx)?)
        } else {
            None
        }
    };
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
