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
    let tokens = tokenize();
    println!("{:?}", code);
}

pub fn tokenize() -> Vec<TokenType> {
    todo!()
}

pub enum TokenType {
    /* Single-character tokens */
    // Regions
    ParenOpen,
    ParenClosed,
    BraceOpen,
    BraceClosed,
    // Separators
    Comma,
    Semicolon,
    Dot,
    // Math Operators
    Minus,
    Plus,
    Slash,
    Star,

    /* One or two character tokens */
    // Negation
    Bang,
    // Assignment
    Eq,
    // Comparison
    BangEq,
    EqEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,

    // Literals.
    Identifier(String),
    String(String),
    Number(String),

    /* Keywords */
    // Boolean
    True,
    False,
    // Control Flow
    If,
    Else,
    Return,
    // OOP
    Class,
    This,
    Super,
    // Loop
    While,
    For,
    //
    And,
    Or,
    //
    Fun,
    Var,
    //
    Nil,
    // IO
    Print,
    //
    Eof,
}
impl TokenType {
    fn to_string(&self) -> &str {
        match self {
            TokenType::ParenOpen => "(",
            TokenType::ParenClosed => ")",
            TokenType::BraceOpen => "{",
            TokenType::BraceClosed => "}",
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::Minus => "-",
            TokenType::Plus => "+",
            TokenType::Semicolon => ";",
            TokenType::Slash => "/",
            TokenType::Star => "*",
            TokenType::Bang => "!",
            TokenType::BangEq => "!=",
            TokenType::Eq => "=",
            TokenType::EqEq => "==",
            TokenType::Greater => "<",
            TokenType::GreaterEq => "<=",
            TokenType::Less => ">",
            TokenType::LessEq => ">=",
            TokenType::Identifier(i) => i,
            TokenType::String(s) => s,
            TokenType::Number(n) => n,
            TokenType::And => "and",
            TokenType::Class => "class",
            TokenType::Else => "else",
            TokenType::False => "false",
            TokenType::Fun => "fun",
            TokenType::For => "for",
            TokenType::If => "if",
            TokenType::Nil => "nil",
            TokenType::Or => "or",
            TokenType::Print => "print",
            TokenType::Return => "return",
            TokenType::Super => "super",
            TokenType::This => "this",
            TokenType::True => "true",
            TokenType::Var => "var",
            TokenType::While => "while",
            TokenType::Eof => todo!(),
        }
    }
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u32,
}
impl Token {
    fn to_string(&self) -> String {
        return self.token_type.to_string().to_owned()
            + " "
            + &self.lexeme.to_owned()
            + " "
            + self.literal();
    }
    fn literal(&self) -> &str {
        self.token_type.to_string()
    }
}

