mod block;
mod expr;
mod lexer;
mod oper;
mod stmt;
mod utils;
mod value;

use clap::Parser;
use std::{
    fs::{File, read_to_string},
    io::Write,
    path::Path,
};
use {block::*, expr::*, lexer::*, oper::*, stmt::*, utils::*, value::*};

#[derive(Parser)]
#[command(
    name = "mystia",
    about = "Web frontend programming language outputs wasm"
)]
struct Cli {
    /// Source code file path
    path: String,
}

fn main() {
    let mut compiler = Compiler::new();
    let cli = Cli::parse();
    if let Ok(source) = read_to_string(cli.path.clone()) {
        if let Some(wat_code) = compiler.build(&source) {
            if let Ok(mut output_file) = File::create(Path::new(&cli.path).with_extension("wat")) {
                if output_file.write_all(wat_code.as_bytes()).is_err() {
                    eprintln!("Failed to write output in the file")
                }
            } else {
                eprintln!("Failed to create output file")
            }
        } else {
            eprintln!("Failed to compile Mystia code")
        }
    } else {
        eprintln!("Failed to read source file")
    }
}

struct Compiler {
    declare: Vec<String>,
}

impl Compiler {
    fn new() -> Self {
        Compiler { declare: vec![] }
    }

    fn build(&mut self, source: &str) -> Option<String> {
        Some(format!(
            r#"(module {1} (func (export "_start") (result i32) {0}))"#,
            Block::parse(source).map(|x| x.compile(self))?,
            join!(self.declare)
        ))
    }
}
