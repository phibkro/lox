use lox;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    script: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();
    match args.script {
        None => {
            println!("Starting REPL mode");
            lox::run_prompt();
        }
        Some(path) => {
            println!("Script: {:?}", &path);
            lox::run_file(&path);
        }
    }
}
