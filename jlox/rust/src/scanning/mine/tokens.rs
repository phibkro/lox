#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

/* Single-character tokens */

// Regions
pub const PAREN_OPEN: TokenType = TokenType::ParenOpen("(");
pub const PAREN_CLOSED: TokenType = TokenType::ParenClosed(")");

// Block
pub const BRACE_OPEN: TokenType = TokenType::BlockOpen("{");
pub const BRACE_CLOSED: TokenType = TokenType::BlockClosed("}");

// Separators
pub const COMMA: TokenType = TokenType::Comma(",");
pub const SEMICOLON: TokenType = TokenType::Semicolon(";");
pub const DOT: TokenType = TokenType::Dot(".");

// Math Operators
pub const MINUS: TokenType = TokenType::Subtraction("-");
pub const PLUS: TokenType = TokenType::Addition("+");
pub const SLASH: TokenType = TokenType::Division("/");
pub const STAR: TokenType = TokenType::Multiplication("*");

// Negation
pub const BANG: TokenType = TokenType::Not("!");

// Assignment
pub const EQ: TokenType = TokenType::Assignment("=");

// Comparison
pub const GREATER: TokenType = TokenType::GreaterThan(">");
pub const LESS: TokenType = TokenType::LessThan("<");

/* Two character tokens */

pub const BANG_EQ: TokenType = TokenType::NotEqual("!=");
pub const EQ_EQ: TokenType = TokenType::DoubleEqual("==");
pub const GREATER_EQ: TokenType = TokenType::GreaterThanOrEqual(">=");
pub const LESS_EQ: TokenType = TokenType::LessThanOrEqual("<=");

/* Keywords */

// Loop
pub const WHILE: TokenType = TokenType::While("while");
pub const FOR: TokenType = TokenType::For("for");

// Boolean
pub const TRUE: TokenType = TokenType::True("true");
pub const FALSE: TokenType = TokenType::False("false");

// Control Flow
pub const IF: TokenType = TokenType::If("if");
pub const ELSE: TokenType = TokenType::Else("else");
pub const RETURN: TokenType = TokenType::Return("return");

// Logical
pub const AND: TokenType = TokenType::And("and");
pub const OR: TokenType = TokenType::Or("or");

// Declaration
pub const VAR: TokenType = TokenType::Variable("var");
pub const FUN: TokenType = TokenType::Function("fun");

// OOP
pub const CLASS: TokenType = TokenType::Class("class");
pub const THIS: TokenType = TokenType::This("this");
pub const SUPER: TokenType = TokenType::Super("super");

// Nil
pub const NIL: TokenType = TokenType::Nil("nil");

// IO
pub const PRINT: TokenType = TokenType::Print("print");

#[derive(Debug, PartialEq)]
pub enum TokenType {
    /* Primitives */
    // Regions
    ParenOpen(&'static str),
    ParenClosed(&'static str),

    // Block
    BlockOpen(&'static str),
    BlockClosed(&'static str),

    // Separators
    Comma(&'static str),
    Semicolon(&'static str),
    Dot(&'static str),

    // Math Operators
    Subtraction(&'static str),
    Addition(&'static str),
    Division(&'static str),
    Multiplication(&'static str),

    // Comparison
    NotEqual(&'static str),
    DoubleEqual(&'static str),
    GreaterThan(&'static str),
    GreaterThanOrEqual(&'static str),
    LessThan(&'static str),
    LessThanOrEqual(&'static str),

    /* Types */
    // Literals.
    Identifier(String),
    String(String),
    Number(String),

    // Boolean
    True(&'static str),
    False(&'static str),

    // Nil
    Nil(&'static str),

    /* Control Flow */
    Return(&'static str),

    // Conditional
    If(&'static str),
    Else(&'static str),

    // Iteration
    While(&'static str),
    For(&'static str),

    // Logical
    Not(&'static str),
    And(&'static str),
    Or(&'static str),

    /* */
    // Assignment
    Assignment(&'static str),

    // Declaration
    Variable(&'static str),
    Function(&'static str),

    // Object Orientation
    Class(&'static str),
    This(&'static str),
    Super(&'static str),

    /* Extras */
    // IO
    Print(&'static str),

    // End of file
    Eof,
}
