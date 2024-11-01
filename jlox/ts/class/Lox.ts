import { Scanner } from "./Scanner.ts";

export class Lox {
    static hadError = false;

    public static async runFile(path: string) {
        try {
            const file = await Deno.readTextFile(path);
            Lox.run(file);
            if (Lox.hadError) {
                Deno.exit(65);
            }
        } catch (error) {
            console.error(error);
        }
    }

    public static runPrompt() {
        while (true) {
            const line = prompt("> ");
            if (!line) {
                Deno.exit();
            }
            Lox.run(line);
            Lox.hadError = false;
        }
    }

    private static run(source: string) {
        const scanner = new Scanner(source);
        const tokens = scanner.scanTokens();

        for (const token of tokens) {
            console.log(token);
        }
        // tokens.forEach((token) => console.log(token));
    }

    static error(line: number, message: string) {
        this.report(line, "", message);
    }

    private static report(line: number, where: string, message: string) {
        console.error(`[line ${line}] Error ${where}: ${message}`);
        this.hadError = true;
    }
}
