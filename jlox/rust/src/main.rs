use clap::Parser;
use std::io::{self, Write};
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
            run_prompt();
        }
        Some(path) => {
            println!("Script: {:?}", &path);
            run_file(&path);
        }
    }
}

fn run_prompt() {
    let mut buffer = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    while io::stdin().read_line(&mut buffer).is_ok() {
        run(&buffer);
        buffer.clear();
        print!("> ");
        io::stdout().flush().unwrap();
    }
    println!("Bye bye!");
}

fn run_file(path: &PathBuf) {
    todo!();
}

fn run(command: &String) {
    println!("{:?}", command);
}
