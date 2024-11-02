import { generateError } from "./main.ts";
import {
	createToken,
	type DoubleKey,
	keywords,
	leading,
		singles,
	type TokenInstance,
	type TokenKey,
	tokenTypes,
	whitespaces,
} from "./types.ts";
import { getKeyFromValue } from "./utils.ts";

export function scanTokens(
	source: string,
): TokenInstance[] {
	const tokens: TokenInstance[] = [];
	// Points to first character in the lexeme being scanned
	let start = 0;
	// Points at the character currently being considered
	let current = 0;
	let line = 1;

	while (!isAtEnd()) {
		start = current;
		scanToken();
	}

	tokens.push(
		{
		line,
			token: createToken(tokenTypes.EOF, null),
		},
	);

	return tokens;

	function scanToken() {
		const char = advance();

		const singleKey = getKeyFromValue(singles, char);

		if (singleKey) {
			addToken(singleKey, null);
			return;
		}

		const leadingKey = getKeyFromValue(leading, char);

		if (leadingKey) {
			addToken(
				match("=") ? leadingKey + "_EQUAL" as DoubleKey : leadingKey,
				null,
			);
			return;
		}

		if (getKeyFromValue(whitespaces, char)) {
			return;
		}

		switch (char) {
			case "/":
				if (match("/")) {
					// Comments go to end of line
					while (peek() != "\n" && !isAtEnd()) {
						advance();
					}
				} else {
					addToken("SLASH", null);
				}
				break;
			case "\n":
				line++;
				break;
			// String literals
			case '"': {
				/* Scan string literal */
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
				addToken(tokenTypes.STRING, value);
				break;
			}

			default:
				if (isDigit(char)) {
					/* Scan number literal */
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

						addToken(
							"NUMBER",
							Number.parseFloat(source.substring(start, current)),
						);
					}
				} else if (isAlpha(char)) {
					/* Scan identifier */
					while (isAlphaNumeric(peek())) {
						advance();
					}
					const text = source.slice(start, current);
					let type: TokenKey | undefined = getKeyFromValue(keywords, text);
					if (!type) {
						type = "IDENTIFIER";
						addToken(type, text);
					} else {
						addToken(type, null); // KeywordKey or IDENTIFIER
					}
				} else {
					generateError(line, `Unexpected character: ${char}`);
				}
				break;
		}
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

	function isAtEnd(): boolean {
		return source.length <= current;
	}

	function advance(): string {
		return source.charAt(current++);
	}

	function addToken(type: TokenKey, literal: string | number | null) {
		tokens.push(
			{
			line,
				token: createToken(type, literal),
			},
		);
	}
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

function isAlphaNumeric(c: string) {
	return isAlpha(c) || isDigit(c);
}

function isDigit(c: string) {
	const code = c.charCodeAt(0);
	if (47 < code && 58 > code) {
		return true;
	}
	return false;
	// return !Number.isNaN(c);
}
