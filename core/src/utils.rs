use crate::*;

pub const SPACE: [&str; 5] = [" ", "　", "\n", "\t", "\r"];
pub const OPERATOR: [&str; 12] = [
    "+", "-", "*", "/", "%", "^", "==", "!=", "<=", ">=", "<", ">",
];
pub const IMPORT_WRITE: &str = r#"(import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))"#;

pub fn include_letter(query: &str, chars: &Vec<String>, idx: usize) -> bool {
    chars
        .clone()
        .get(idx..idx + query.chars().count())
        .map(|i| query == i.concat())
        .unwrap_or(false)
}

pub fn expand_local(ctx: &mut Compiler) -> String {
    join!(
        ctx.variable
            .clone()
            .iter()
            .map(|x| format!("(local ${} {})", x.0, x.1.compile(ctx)))
            .collect::<Vec<_>>()
    )
}

#[macro_export]
macro_rules! config_return {
    ($ret: expr, $ctx: expr) => {
        if let Type::Void = $ret {
            String::new()
        } else {
            format!("(result {})", $ret.compile($ctx))
        }
    };
}

#[macro_export]
macro_rules! join {
    ($x:expr) => {
        $x.join(&SPACE[0].to_string())
    };
}
