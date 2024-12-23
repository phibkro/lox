pub const PAREN_OPEN: TokenType = TokenType::ParenOpen("(");
pub const PAREN_CLOSED: TokenType = TokenType::ParenClosed(")");
pub const BRACE_OPEN: TokenType = TokenType::BraceOpen("{");
pub const BRACE_CLOSED: TokenType = TokenType::BraceClosed("}");
pub const COMMA: TokenType = TokenType::Comma(",");
pub const SEMICOLON: TokenType = TokenType::Semicolon(";");
pub const DOT: TokenType = TokenType::Dot(".");
pub const MINUS: TokenType = TokenType::Minus("-");
pub const PLUS: TokenType = TokenType::Plus("+");
pub const STAR: TokenType = TokenType::Star("*");
pub const BANG: TokenType = TokenType::Bang("!");
pub const BANG_EQ: TokenType = TokenType::BangEq("!=");
pub const EQ: TokenType = TokenType::Eq("=");
pub const EQ_EQ: TokenType = TokenType::EqEq("==");
pub const GREATER: TokenType = TokenType::Greater(">");
pub const GREATER_EQ: TokenType = TokenType::GreaterEq(">=");
pub const LESS: TokenType = TokenType::Less("<");
pub const LESS_EQ: TokenType = TokenType::LessEq("<=");
pub const TRUE: TokenType = TokenType::True("true");
pub const FALSE: TokenType = TokenType::False("false");
pub const IF: TokenType = TokenType::If("if");
pub const ELSE: TokenType = TokenType::Else("else");
pub const RETURN: TokenType = TokenType::Return("return");
pub const CLASS: TokenType = TokenType::Class("class");
pub const THIS: TokenType = TokenType::This("this");
pub const SUPER: TokenType = TokenType::Super("super");
pub const WHILE: TokenType = TokenType::While("while");
pub const FOR: TokenType = TokenType::For("for");
pub const AND: TokenType = TokenType::And("and");
pub const OR: TokenType = TokenType::Or("or");
pub const FUN: TokenType = TokenType::Fun("fun");
pub const VAR: TokenType = TokenType::Var("var");
pub const NIL: TokenType = TokenType::Nil("nil");
pub const PRINT: TokenType = TokenType::Print("print");


#[derive(Debug, PartialEq)]
pub enum TokenType {
    /* Single-character tokens */
    // Regions
    ParenOpen(&'static str),
    ParenClosed(&'static str),
    BraceOpen(&'static str),
    BraceClosed(&'static str),
    // Separators
    Comma(&'static str),
    Semicolon(&'static str),
    Dot(&'static str),
    // Math Operators
    Minus(&'static str),
    Plus(&'static str),
    Slash(&'static str),
    Star(&'static str),

    /* One or two character tokens */
    // Negation
    Bang(&'static str),
    // Assignment
    Eq(&'static str),
    // Comparison
    BangEq(&'static str),
    EqEq(&'static str),
    Greater(&'static str),
    GreaterEq(&'static str),
    Less(&'static str),
    LessEq(&'static str),

    // Literals.
    Identifier(String),
    String(String),
    Number(String),

    /* Keywords */
    // Boolean
    True(&'static str),
    False(&'static str),
    // Control Flow
    If(&'static str),
    Else(&'static str),
    Return(&'static str),
    // OOP
    Class(&'static str),
    This(&'static str),
    Super(&'static str),
    // Loop
    While(&'static str),
    For(&'static str),
    //
    And(&'static str),
    Or(&'static str),
    //
    Fun(&'static str),
    Var(&'static str),
    //
    Nil(&'static str),
    // IO
    Print(&'static str),
    //
    Eof,
}