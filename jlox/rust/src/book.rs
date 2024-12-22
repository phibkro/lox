use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

pub mod tokens;
pub mod scanner;
use scanner::Scanner;

pub fn run_prompt() {
    let mut buffer = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    while io::stdin().read_line(&mut buffer).is_ok() {
        if buffer == "\n" {
            break;
        }
        run(&buffer);
        buffer.clear();
        print!("> ");
        io::stdout().flush().unwrap();
    }
    println!("Bye bye!");
}

pub fn run_file(path: &PathBuf) -> io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }
        println!("{}", line.trim());
        line.clear();
    }

    Ok(())
}

fn run(code: &str) {
    let mut scanner = Scanner::from(code.to_string());
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
}

static mut HAD_ERROR: bool = false;

pub fn error (line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
    unsafe {
        HAD_ERROR = true;
    }
}