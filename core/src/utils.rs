use crate::*;

pub const SPACE: [&str; 5] = [" ", "　", "\n", "\t", "\r"];
pub const OPERATOR: [&str; 12] = [
    "+", "-", "*", "/", "%", "^", "==", "!=", "<=", ">=", "<", ">",
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
    for key in ctx.argument.keys() {
        ctx.variable.remove(key);
    }
    Some(join!(iter_map!(ctx.variable.clone(), |x: (
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
macro_rules! type_check {
    ($lhs: expr, $rhs: expr, $ctx: expr) => {{
        let lhs = $lhs.type_infer($ctx)?;
        let rhs = $rhs.type_infer($ctx)?;
        if format!("{lhs:?}") == format!("{rhs:?}") {
            Some(lhs)
        } else {
            None
        }
    }};
}

#[macro_export]
macro_rules! compile_compare {
    ($oper: expr, $self: expr, $ctx: expr, $lhs: expr, $rhs: expr) => {{
        let ret = $self.type_infer($ctx)?.compile($ctx)?;
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
macro_rules! compile_arithmetic {
    ($oper: expr, $self: expr, $ctx: expr, $lhs: expr, $rhs: expr) => {
        format!(
            "({}.{} {} {})",
            $self.type_infer($ctx)?.compile($ctx)?,
            $oper,
            $lhs.compile($ctx)?,
            $rhs.compile($ctx)?
        )
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
