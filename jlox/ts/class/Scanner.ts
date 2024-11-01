import { Lox } from "./Lox.ts";
import { Token } from "./Token.ts";
import { TokenType } from "./TokenType.ts";

const keywords: {
    [key: string]: TokenType;
} = {
    and: TokenType.AND,
    class: TokenType.CLASS,
    else: TokenType.ELSE,
    false: TokenType.FALSE,
    for: TokenType.FOR,
    fun: TokenType.FUN,
    if: TokenType.IF,
    nil: TokenType.NIL,
    or: TokenType.OR,
    print: TokenType.PRINT,
    return: TokenType.RETURN,
    super: TokenType.SUPER,
    this: TokenType.THIS,
    true: TokenType.TRUE,
    var: TokenType.VAR,
    while: TokenType.WHILE,
};

export class Scanner {
    source: string;
    tokens: Token[];
    // Points to first character in the lexeme being scanned
    start = 0;
    // Points at the character currently being considered
    current = 0;
    line = 1;
    constructor(source: string) {
        this.source = source;
        this.tokens = [];
    }

    scanTokens() {
        while (!this.isAtEnd()) {
            this.start = this.current;
            this.scanToken();
        }
        this.tokens.push(
            new Token(
                TokenType.EOF,
                "",
                null,
                this.line,
            ),
        );
        return this.tokens;
    }

    scanToken() {
        const c = this.advance();
        switch (c) {
            // Single-character lexems
            case "(":
                this.addToken(TokenType.LEFT_PAREN);
                break;
            case ")":
                this.addToken(TokenType.RIGHT_PAREN);
                break;
            case "{":
                this.addToken(TokenType.LEFT_BRACE);
                break;
            case "}":
                this.addToken(TokenType.RIGHT_BRACE);
                break;
            case ",":
                this.addToken(TokenType.COMMA);
                break;
            case ".":
                this.addToken(TokenType.DOT);
                break;
            case "-":
                this.addToken(TokenType.MINUS);
                break;
            case "+":
                this.addToken(TokenType.PLUS);
                break;
            case ";":
                this.addToken(TokenType.SEMICOLON);
                break;
            case "*":
                this.addToken(TokenType.STAR);
                break;
            // Single to double character lexemes
            case "!":
                this.addToken(
                    this.match("=") ? TokenType.BANG_EQUAL : TokenType.BANG,
                );
                break;
            case "<":
                this.addToken(
                    this.match("=") ? TokenType.LESS_EQUAL : TokenType.LESS,
                );
                break;
            case ">":
                this.addToken(
                    this.match("=")
                        ? TokenType.GREATER_EQUAL
                        : TokenType.GREATER,
                );
                break;
            // Longer lexemes
            case "/":
                if (this.match("/")) {
                    // Comments go to end of line
                    while (this.peek() != "\n" && !this.isAtEnd()) {
                        this.advance();
                    }
                } else {
                    this.addToken(TokenType.SLASH);
                }
                break;
            // Newlines and whitespace
            case " ":
            case "\r":
            case "\t":
                // Ignore whitespace.
                break;
            case "\n":
                this.line++;
                break;
            // String literals
            case '"':
                this.scanString();
                break;

            default:
                if (this.isDigit(c)) {
                    this.scanNumber();
                } else if (this.isAlpha(c)) {
                    this.identifier();
                } else {
                    Lox.error(this.line, "Unexpected character.");
                }
                break;
        }
    }

    identifier() {
        while (this.isAlphaNumeric(this.peek())) {
            this.advance();
        }
        const text = this.source.slice(this.start, this.current);
        let type = keywords[text];
        if (type == null) {
            type = TokenType.IDENTIFIER;
        }
        this.addToken(TokenType.IDENTIFIER);
    }

    scanNumber() {
        while (this.isDigit(this.peek())) {
            this.advance();
        }

        // Look for a fractional part.
        if (this.peek() == "." && this.isDigit(this.peekNext())) {
            // Consume the "."
            this.advance();

            while (this.isDigit(this.peek())) {
                this.advance();
            }
        }

        this.pushToken(
            TokenType.NUMBER,
            Number.parseFloat(this.source.substring(this.start, this.current)),
        );
    }

    scanString() {
        while (this.peek() != '"' && !this.isAtEnd()) {
            if (this.peek() == "\n") {
                this.line++;
            }
            this.advance();
        }

        if (this.isAtEnd()) {
            Lox.error(this.line, "Unterminated string");
            return;
        }

        // The closing "
        this.advance();

        // Trim the surrounding quotes
        const value = this.source.slice(this.start + 1, this.current - 1);
        this.pushToken(TokenType.STRING, value);
    }

    isAlpha(c: string): boolean {
        const code = c.charCodeAt(0);
        if (
            (64 < code && code < 91) || // upper alpha (A-Z)
            (96 < code && code < 123) || // lower alpha (a-z)
            (code == 95) // underscore _
        ) {
            return true;
        }
        return false;
    }

    isDigit(c: string) {
        const code = c.charCodeAt(0);
        if (47 < code && 58 > code) {
            return true;
        }
        return false;
        // return !Number.isNaN(c);
    }

    isAlphaNumeric(c: string) {
        return this.isAlpha(c) || this.isDigit(c);
    }

    advance(): string {
        return this.source.charAt(this.current++);
    }

    peek() {
        if (this.isAtEnd()) {
            return "\0";
        }
        return this.source.charAt(this.current);
    }

    peekNext(): string {
        if (this.current + 1 >= this.source.length) {
            return "\0";
        }
        return this.source.charAt(this.current + 1);
    }

    match(expected: string): boolean {
        if (this.isAtEnd()) {
            return false;
        }
        if (this.source.charAt(this.current) != expected) {
            return false;
        }
        this.current++;
        return true;
    }

    addToken(type: TokenType) {
        this.pushToken(type, null);
    }

    pushToken(type: TokenType, literal: any) {
        const text = this.source.slice(this.start, this.current);
        this.tokens.push(
            new Token(
                type,
                text,
                literal,
                this.line,
            ),
        );
    }
    isAtEnd(): boolean {
        return this.source.length <= this.current;
    }
}
