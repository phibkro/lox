use std::io::{self, Write};
use std::path::PathBuf;

pub fn run_prompt() {
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

pub fn run_file(path: &PathBuf) {
    todo!();
}

fn run(command: &str) {
    println!("{:?}", command);
}
