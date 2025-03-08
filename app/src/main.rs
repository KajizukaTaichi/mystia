use clap::Parser;
use mystia_core::Compiler;
use std::{
    fs::{File, read_to_string},
    io::Write,
    path::Path,
};

#[derive(Parser)]
#[command(
    name = "Mystia",
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
