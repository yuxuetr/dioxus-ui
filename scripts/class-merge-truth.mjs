// Ground truth for RFC 0076 class merging, shared by the gate and the merge
// table generator: what Tailwind compiles for each utility, which longhands
// Chrome expands each property into, and the physical side a logical
// longhand names in each writing direction.

// Splits `data-[state=open]:hover:px-2` into variants and the utility,
// ignoring colons inside brackets and parentheses.
export function splitVariants(token) {
  const parts = [];
  let depth = 0;
  let start = 0;
  for (let index = 0; index < token.length; index += 1) {
    const char = token[index];
    if (char === "[" || char === "(") depth += 1;
    else if (char === "]" || char === ")") depth -= 1;
    else if (char === ":" && depth === 0) {
      parts.push(token.slice(start, index));
      start = index + 1;
    }
  }
  return { variants: parts, utility: token.slice(start) };
}

// Walks Tailwind's utilities layer and returns, per class token, the
// declarations it sets with the state they apply in: enclosing at-rules
// other than @supports (a fallback for the same value) and the selector
// around the class.
export function declarationsByToken(css) {
  const start = css.indexOf("@layer utilities {");
  let order = 0;
  const byToken = new Map();
  if (start < 0) return byToken;
  const stack = [];
  let buffer = "";
  let quote = null;
  let parens = 0;
  for (let index = start + "@layer utilities {".length; index < css.length; index += 1) {
    const char = css[index];
    if (quote) {
      if (char === quote && css[index - 1] !== "\\") quote = null;
      buffer += char;
      continue;
    }
    if (char === '"' || char === "'") quote = char;
    if (char === "(") parens += 1;
    if (char === ")") parens -= 1;
    if (parens > 0 || (char !== "{" && char !== "}" && char !== ";")) {
      buffer += char;
      continue;
    }
    const text = buffer.trim();
    buffer = "";
    if (char === "{") {
      stack.push(text);
      continue;
    }
    if (text) record(text);
    if (char === "}") {
      if (stack.length === 0) break;
      stack.pop();
    }
  }
  return byToken;

  function record(declaration) {
    const colon = declaration.indexOf(":");
    const property = declaration.slice(0, colon).trim();
    const value = declaration.slice(colon + 1).trim();
    const selectorIndex = stack.findIndex((header) => !header.startsWith("@"));
    if (selectorIndex < 0) return;
    const selector = stack[selectorIndex];
    const match = selector.match(/\.((?:\\[0-9a-f]{1,6} ?|\\.|[\w-])+)/);
    if (!match) return;
    const token = match[1]
      .replace(/\\([0-9a-f]{1,6}) ?/g, (_, hex) => String.fromCodePoint(parseInt(hex, 16)))
      .replace(/\\(.)/g, "$1");
    const state = [
      ...stack.slice(0, selectorIndex).filter((header) => !header.startsWith("@supports")),
      selector.replace(match[0], "&"),
      ...stack.slice(selectorIndex + 1).filter((header) => !header.startsWith("@supports")),
    ].join(" ");
    if (!byToken.has(token)) byToken.set(token, []);
    // `order` is the declaration's place in the stylesheet: between equal
    // selectors and importance, the later one wins the cascade.
    byToken.get(token).push({ state, property, value, important: /!important$/.test(value), order: order++ });
  }
}

// Declarations of standard properties that read a --tw-* variable compose
// with other utilities by design, such as text-sm deferring its line height
// to leading-*; they are not slots. Custom properties always are.
export const isSlot = ({ property, value }) => property.startsWith("--") || !value.includes("var(--tw-");

// Expands each property into the longhands Chrome sets for it; custom and
// unknown properties stand for themselves.
export async function expandLonghands(page, names) {
  return page.evaluate((list) => {
    const element = document.createElement("div");
    return Object.fromEntries(
      list.map((name) => {
        if (name.startsWith("--")) return [name, [name]];
        element.style.cssText = "";
        element.style.setProperty(name, "initial");
        const expanded = [...element.style];
        return [name, expanded.length > 0 ? expanded : [name]];
      }),
    );
  }, names);
}

// Logical longhands name a physical side that depends on the direction.
export const physical = (longhand, rtl) =>
  longhand
    .replace("inline-start", rtl ? "right" : "left")
    .replace("inline-end", rtl ? "left" : "right")
    .replace("block-start", "top")
    .replace("block-end", "bottom")
    .replace(/^inset-(top|right|bottom|left)$/, "$1")
    .replace(/border-(start|end)-(start|end)-radius/, (_, block, inline) =>
      `border-${block === "start" ? "top" : "bottom"}-${(inline === "start") !== rtl ? "left" : "right"}-radius`,
    );
