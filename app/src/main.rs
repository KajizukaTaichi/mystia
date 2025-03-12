use clap::Parser;
use mystia_core::Compiler;
use std::{
    fs::{File, read_to_string},
    io::Write,
    path::Path,
};
use wat::parse_bytes;

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
    let cli = Cli::parse();
    let mut compiler = Compiler::new();

    let Ok(source) = read_to_string(cli.path.clone()) else {
        eprintln!("Failed to read source file");
        return;
    };
    let Some(wat_code) = compiler.build(&source) else {
        eprintln!("Failed to compile Mystia code");
        return;
    };
    let Ok(mut output_file) = File::create(Path::new(&cli.path).with_extension("wat")) else {
        eprintln!("Failed to create output WAT file");
        return;
    };
    let code = wat_code.as_bytes();
    let Ok(_) = output_file.write_all(code) else {
        eprintln!("Failed to write output in the WAT file");
        return;
    };
    let Ok(binary) = parse_bytes(code) else {
        eprintln!("Failed to compile WAT file");
        return;
    };
    let Ok(mut output_file) = File::create(Path::new(&cli.path).with_extension("wasm")) else {
        eprintln!("Failed to create output WASM file");
        return;
    };
    let Ok(_) = output_file.write_all(&binary) else {
        eprintln!("Failed to write output in the WASM file");
        return;
    };
}
