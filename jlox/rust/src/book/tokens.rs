#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
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
