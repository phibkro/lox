#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn literal(&self) -> &str {
        self.token_type.literal()
    }
}

#[derive(Debug)]
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
    pub fn literal(&self) -> &str {
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
