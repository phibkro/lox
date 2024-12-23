use super::tokens::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn from(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        let lexeme = self.source.get(self.start..self.current).unwrap();
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: lexeme.to_string(),
            line: self.line,
        });
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::ParenOpen),
            ')' => self.add_token(TokenType::ParenClosed),
            '{' => self.add_token(TokenType::BraceOpen),
            '}' => self.add_token(TokenType::BraceClosed),
            ',' => self.add_token(TokenType::Comma),
            ';' => self.add_token(TokenType::Semicolon),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEq);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqEq);
                } else {
                    self.add_token(TokenType::Eq);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEq);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEq);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                match self.peek() {
                    '/' => {
                        // A comment goes until the end of the line.
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    }
                    '*' => {
                        // A block comment goes until the end of the block.
                        while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
                            if self.peek() == '\n' {
                                self.line += 1;
                            }
                            self.advance();
                        }
                        // Consume the closing */
                        self.advance();
                        self.advance();
                    }
                    _ => self.add_token(TokenType::Slash),
                    
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphabetic() {
                    self.identifier();
                } else {
                    panic!("Unexpected character: {}", c);
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source.get(self.start..self.current).unwrap();
        self.tokens.push(Token {
            token_type,
            lexeme: lexeme.to_string(),
            line: self.line,
        });
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string.");
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes
        let lexeme = self.source.get(self.start + 1..self.current - 1).unwrap();
        self.tokens.push(Token {
            token_type: TokenType::String(lexeme.to_string()),
            lexeme: lexeme.to_string(),
            line: self.line,
        });
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme = self.source.get(self.start..self.current).unwrap();
        self.tokens.push(Token {
            token_type: TokenType::Number(lexeme.to_string()),
            lexeme: lexeme.to_string(),
            line: self.line,
        });
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let lexeme = self.source.get(self.start..self.current).unwrap();
        let token_type = match lexeme {
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier(lexeme.to_string()),
        };
        self.tokens.push(Token {
            token_type,
            lexeme: lexeme.to_string(),
            line: self.line,
        });
    }
}
