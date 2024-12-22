import type { Binary, Expression, Group, Literal, Unary } from "./types.ts";

export function astPrint(expression: Literal): string;
export function astPrint(expression: Group): string;
export function astPrint(expression: Unary): string;
export function astPrint(expression: Binary): string;
export function astPrint<T extends Expression>(expression: T) {
  // Base case
  if (!expression.pre && !expression.post) {
    // Group
    return wrap(expression.operator.lexeme);
  }
  if (!expression.pre && expression.post) {
    // Unary
    if (expression.pre) {
      return;
    }
  }
  if (expression.pre && expression.post) {
    // Binary
    if (typeof expression.pre === "number") {
      astPrint(expression.pre);
    }
  }
  if (expression.pre && expression.post) {
    // Binary
    return;
  } else if (!expression.pre && expression.post) {
    // Unary
    return;
  } else if (!expression.pre && !expression.post) {
    // Group
    return;
  }
  return wrap(expression.operator.lexeme);
  function wrap(str: string, open?: "(", closed?: ")"): string {
    return `${open}${str}${closed}`;
  }
}
