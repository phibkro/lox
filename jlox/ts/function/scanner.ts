import { generateError } from "./main.ts";
import { Token, TokenType } from "./token.ts";

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

export function scanTokens(source: string): Token[] {
	const tokens: Token[] = [];
	// Points to first character in the lexeme being scanned
	let start = 0;
	// Points at the character currently being considered
	let current = 0;
	let line = 1;

	while (!isAtEnd()) {
		start = current;
		scanToken();
	}

	tokens.push({
		type: TokenType.EOF,
		lexeme: "",
		literal: null,
		line,
	});

	return tokens;

	function scanToken() {
		const c = advance();
		switch (c) {
			// Single-character lexems
			case "(":
				addToken(TokenType.LEFT_PAREN);
				break;
			case ")":
				addToken(TokenType.RIGHT_PAREN);
				break;
			case "{":
				addToken(TokenType.LEFT_BRACE);
				break;
			case "}":
				addToken(TokenType.RIGHT_BRACE);
				break;
			case ",":
				addToken(TokenType.COMMA);
				break;
			case ".":
				addToken(TokenType.DOT);
				break;
			case "-":
				addToken(TokenType.MINUS);
				break;
			case "+":
				addToken(TokenType.PLUS);
				break;
			case ";":
				addToken(TokenType.SEMICOLON);
				break;
			case "*":
				addToken(TokenType.STAR);
				break;
			// Single to double character lexemes
			case "!":
				addToken(match("=") ? TokenType.BANG_EQUAL : TokenType.BANG);
				break;
			case "<":
				addToken(match("=") ? TokenType.LESS_EQUAL : TokenType.LESS);
				break;
			case ">":
				addToken(
					match("=") ? TokenType.GREATER_EQUAL : TokenType.GREATER,
				);
				break;
			// Longer lexemes
			case "/":
				if (match("/")) {
					// Comments go to end of line
					while (peek() != "\n" && !isAtEnd()) {
						advance();
					}
				} else {
					addToken(TokenType.SLASH);
				}
				break;
			// Newlines and whitespace
			case " ":
			case "\r":
			case "\t":
				// Ignore whitespace.
				break;
			case "\n":
				line++;
				break;
			// String literals
			case '"':
				scanString();
				break;

			default:
				if (isDigit(c)) {
					scanNumber();
				} else if (isAlpha(c)) {
					identifier();
				} else {
					generateError(line, "Unexpected character.");
				}
				break;
		}
	}

	function identifier() {
		while (isAlphaNumeric(peek())) {
			advance();
		}
		const text = source.slice(start, current);
		let type = keywords[text];
		if (type == null) {
			type = TokenType.IDENTIFIER;
		}
		addToken(TokenType.IDENTIFIER);
	}

	function scanNumber() {
		while (isDigit(peek())) {
			advance();
		}

		// Look for a fractional part.
		if (peek() == "." && isDigit(peekNext())) {
			// Consume the "."
			advance();

			while (isDigit(peek())) {
				advance();
			}
		}

		pushToken(
			TokenType.NUMBER,
			Number.parseFloat(source.substring(start, current)),
		);
	}

	function scanString() {
		while (peek() != '"' && !isAtEnd()) {
			if (peek() == "\n") {
				line++;
			}
			advance();
		}

		if (isAtEnd()) {
			generateError(line, "Unterminated string");
			return;
		}

		// The closing "
		advance();

		// Trim the surrounding quotes
		const value = source.slice(start + 1, current - 1);
		pushToken(TokenType.STRING, value);
	}

	function isAlpha(c: string): boolean {
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

	function isDigit(c: string) {
		const code = c.charCodeAt(0);
		if (47 < code && 58 > code) {
			return true;
		}
		return false;
		// return !Number.isNaN(c);
	}

	function isAlphaNumeric(c: string) {
		return isAlpha(c) || isDigit(c);
	}

	function advance(): string {
		return source.charAt(current++);
	}

	function peek() {
		if (isAtEnd()) {
			return "\0";
		}
		return source.charAt(current);
	}

	function peekNext(): string {
		if (current + 1 >= source.length) {
			return "\0";
		}
		return source.charAt(current + 1);
	}

	function match(expected: string): boolean {
		if (isAtEnd()) {
			return false;
		}
		if (source.charAt(current) != expected) {
			return false;
		}
		current++;
		return true;
	}

	function addToken(type: TokenType) {
		pushToken(type, null);
	}

	function pushToken(type: TokenType, literal: any) {
		const text = source.slice(start, current);
		tokens.push({
			type,
			lexeme: text,
			literal,
			line,
		});
	}
	function isAtEnd(): boolean {
		return source.length <= current;
	}
}
