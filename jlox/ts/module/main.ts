import { scanTokens } from "./lexing.ts";

let hadError = false;

if (import.meta.main) {
	if (1 < Deno.args.length) {
		console.log("Usage: jlox [script]");
		Deno.exit(64);
	} else if (1 == Deno.args.length) {
		// runFile()
		try {
			const file = await Deno.readTextFile(Deno.args[0]);
			run(file);
			if (hadError) {
				Deno.exit(65);
			}
		} catch (error) {
			console.error(error);
		}
	} else {
		// runPrompt()
		while (true) {
			const line = prompt("> ");
			if (!line) {
				Deno.exit();
			}
			run(line);
			hadError = false;
		}
	}
}

function run(source: string) {
	const tokens = scanTokens(source);

	for (const token of tokens) {
		console.log(token);
	}
	// tokens.forEach((token) => console.log(token));
}

export function generateError(line: number, message: string) {
	report(line, "", message);
}

function report(line: number, where: string, message: string) {
	console.error(`[line ${line}] Error ${where}: ${message}`);
	hadError = true;
}
