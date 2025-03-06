mod expr;
mod lexer;
mod oper;
mod utils;
mod value;

use expr::*;
use lexer::*;
use oper::*;
use utils::*;
use value::*;

fn main() {
    println!(
        r#"(module (func (export "_start") (result i32) {}))"#,
        Expr::parse("1+2*3").map(|x| x.compile()).unwrap()
    );
}
