use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

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
    let _tokens = tokenize();
    println!("{:?}", code);
}
pub enum TokenType {
    // Boolean
    True,
    False,
}

/* Token */

pub fn tokenize() -> Vec<TokenType> {
    todo!()
}

pub enum Literal {
    Identifier(String),
    String(String),
    Number(String),
}
pub enum Operator {
    Minus,
    Plus,
    Slash,
    Star,
    Bang,
}

/* Expression */

#[allow(dead_code)]
enum Primary {
    String(String),
    Number(i32),
    Bool(Boolean),
    Expression(Literal),
}
#[allow(dead_code)]
enum Boolean {
    True,
    False,
}
