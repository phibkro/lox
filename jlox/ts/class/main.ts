import { Lox } from "./Lox.ts";

if (import.meta.main) {
  if (1 < Deno.args.length) {
    console.log("Usage: jlox [script]");
    Deno.exit(64);
  } else if (1 == Deno.args.length) {
    Lox.runFile(Deno.args[0]);
  } else {
    Lox.runPrompt();
  }
}
