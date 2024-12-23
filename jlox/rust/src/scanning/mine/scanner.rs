use super::tokens::{self, Token, TokenType};

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
            '(' => self.add_token(tokens::PAREN_OPEN),
            ')' => self.add_token(tokens::PAREN_CLOSED),
            '{' => self.add_token(tokens::BRACE_OPEN),
            '}' => self.add_token(tokens::BRACE_CLOSED),
            ',' => self.add_token(tokens::COMMA),
            ';' => self.add_token(tokens::SEMICOLON),
            '.' => self.add_token(tokens::DOT),
            '-' => self.add_token(tokens::MINUS),
            '+' => self.add_token(tokens::PLUS),
            '*' => self.add_token(tokens::STAR),
            '!' => {
                if self.match_char('=') {
                    self.add_token(tokens::BANG_EQ);
                } else {
                    self.add_token(tokens::BANG);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(tokens::EQ_EQ);
                } else {
                    self.add_token(tokens::EQ);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(tokens::LESS_EQ);
                } else {
                    self.add_token(tokens::LESS);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(tokens::GREATER_EQ);
                } else {
                    self.add_token(tokens::GREATER);
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
                    _ => self.add_token(tokens::SLASH),
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
        match lexeme {
            "and" => self.add_token(tokens::AND),
            "class" => self.add_token(tokens::CLASS),
            "else" => self.add_token(tokens::ELSE),
            "false" => self.add_token(tokens::FALSE),
            "for" => self.add_token(tokens::FOR),
            "if" => self.add_token(tokens::IF),
            "nil" => self.add_token(tokens::NIL),
            "or" => self.add_token(tokens::OR),
            "return" => self.add_token(tokens::RETURN),
            "super" => self.add_token(tokens::SUPER),
            "this" => self.add_token(tokens::THIS),
            "true" => self.add_token(tokens::TRUE),
            "var" => self.add_token(tokens::VAR),
            "while" => self.add_token(tokens::WHILE),
            _ => self.add_token(TokenType::Identifier(lexeme.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_declaration() {
        let source = String::from("var x = 1;");
        let mut scanner = Scanner::from(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].token_type, tokens::VAR);
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier(String::from("x"))
        );
        assert_eq!(tokens[2].token_type, tokens::EQ);
        assert_eq!(tokens[3].token_type, TokenType::Number(String::from("1")));
        assert_eq!(tokens[4].token_type, tokens::SEMICOLON);
        assert_eq!(tokens[5].token_type, TokenType::Eof);
    }

    #[test]
    fn string_literal() {
        let source = String::from("\"Hello, World!\"");
        let mut scanner = Scanner::from(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0].token_type,
            TokenType::String(String::from("Hello, World!"))
        );
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }
}
