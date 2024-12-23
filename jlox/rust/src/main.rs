use lox::scanning::{run_file, run_prompt};

use clap::Parser;
use std::{io, path::PathBuf};

#[derive(Parser)]
struct Cli {
    script: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    match args.script {
        None => {
            println!("Starting REPL mode");
            run_prompt();
        }
        Some(path) => {
            println!("Script: {:?}", &path);
            run_file(&path)?;
        }
    }
    Ok(())
}
