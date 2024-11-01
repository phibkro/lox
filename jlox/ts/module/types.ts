export const singles = {
	// Single-character tokens.
	LEFT_PAREN: "(",
	RIGHT_PAREN: ")",
	LEFT_BRACE: "{",
	RIGHT_BRACE: "}",
	COMMA: ",",
	DOT: ".",
	MINUS: "-",
	PLUS: "+",
	SEMICOLON: ";",
	STAR: "*",
} as const;
export type SingleKey = keyof typeof singles;
export type SingleValue = (typeof singles)[SingleKey];

export const leading = {
	BANG: "!",
	EQUAL: "=",
	GREATER: ">",
	LESS: "<",
} as const;
export type LeadingKey = keyof typeof leading;
export const doubles = {
	BANG_EQUAL: "!=",
	EQUAL_EQUAL: "==",
	GREATER_EQUAL: ">=",
	LESS_EQUAL: "<=",
} as const;
export type DoubleKey = keyof typeof doubles;

const slash = {
	SLASH: "/",
} as const;

export const literals = {
	IDENTIFIER: "IDENTIFIER",
	STRING: "STRING",
	NUMBER: "NUMBER",
} as const;
export type LiteralKey = keyof typeof literals;

const endOfFile = {
	EOF: "EOF",
} as const;

export const keywords = {
	AND: "and",
	CLASS: "class",
	ELSE: "else",
	FALSE: "false",
	FUN: "fun",
	FOR: "for",
	IF: "if",
	NIL: "nil",
	OR: "or",
	PRINT: "print",
	RETURN: "return",
	SUPER: "super",
	THIS: "this",
	TRUE: "true",
	VAR: "var",
	WHILE: "while",
} as const;
export type KeywordKey = keyof typeof keywords;
export type KeywordValue = (typeof keywords)[KeywordKey];

export const tokenTypes = {
	...singles,
	...leading,
	...doubles,
	...slash,
	...literals,
	...keywords,
	...endOfFile,
};
export type TokenKey = keyof typeof tokenTypes;
export type TokenValue = (typeof tokenTypes)[TokenKey];

export type Token = {
	type: TokenKey;
	lexeme: string;
	literal: any;
	line: number;
};

export const whitespaces = {
	SPACE: " ",
	CARRIAGE_RETURN: "\r",
	TAB: "\t",
} as const;
export type WhitespaceKey = keyof typeof whitespaces;
