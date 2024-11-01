import type { TokenType } from "./TokenType.ts";

export class Token {
    type: TokenType;
    lexeme: string;
    literal: any;
    line: number;

    constructor(
        type: TokenType,
        lexeme: string,
        literal: any,
        line: number,
    ) {
        this.type = type;
        this.lexeme = lexeme;
        this.literal = literal;
        this.line = line;
    }

    toString() {
        return `${this.type} ${this.lexeme} ${this.literal} ${this.line}`;
    }
}
