const parentheses = {
	OPEN: "(",
	CLOSED: ")",
};
const curlyBraces = {
	OPEN: "{",
	CLOSED: "}",
};
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
export type TokenInstance = {
	line: number;
	token: Token;
};
export type Token = {
	type: TokenKey;
	lexeme: TokenValue;
	literal?: string | number;
};

export function createToken<T extends TokenKey>(
	type: T,
	literal?: T extends LiteralKey // if the type is a literal:
		? T extends "NUMBER" ? number // and type is "NUMBER", then literal should be a number
		: string // but not "NUMBER", then literal should be a string
		: undefined, // if type isn't a literal, then literal should be undefined
): Token {
	return {
		type,
		lexeme: tokenTypes[type],
		literal,
	};
}

export const whitespaces = {
	SPACE: " ",
	CARRIAGE_RETURN: "\r",
	TAB: "\t",
} as const;
export type WhitespaceKey = keyof typeof whitespaces;

export type Bool = "true" | "false";
export type Nil = "nil";
export type Literal = {
	value: number | string | boolean | undefined;
};
export type Expression = {
	pre?: Expression | Literal;
	post?: Expression | Literal;
	operator: Token;
};
export type Binary = Required<Expression>;
export type Group = Pick<Expression, "operator">;
// export type Unary = Required<Pick<Expression, "post" | "operator">>;
/* export type Group = {
	operator: Token;
};
export type Unary = {
	operator: Token;
	post: Literal | Expression;
};
export type Binary = {
	operator: Token;
	pre: Literal | Expression;
	post: Literal | Expression;
};
export type Expression = Literal | Group | Unary | Binary; */

export type Primary = Bool | Nil;
type UnaryOperator = "!" | "-";
export type Unary = `${UnaryOperator}${Primary}`;
