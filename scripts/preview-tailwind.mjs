import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { compile } from "@tailwindcss/node";
import { Scanner } from "@tailwindcss/oxide";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

// Compiles a preview stylesheet the way the Tailwind CLI would, so browser
// checks see the layout the component classes produce instead of an
// unresolved `@import "tailwindcss"`.
export async function compilePreviewCss(
  inputPath = join(repoRoot, "examples/web-demo/assets/preview.css"),
) {
  const base = dirname(inputPath);
  const compiler = await compile(readFileSync(inputPath, "utf8"), {
    base,
    onDependency: () => {},
  });
  const roots =
    compiler.root === "none"
      ? []
      : compiler.root === null
        ? [{ base, pattern: "**/*", negated: false }]
        : [{ ...compiler.root, negated: false }];
  const scanner = new Scanner({ sources: [...roots, ...compiler.sources] });

  return compiler.build(scanner.scan());
}

// Committed compiled stylesheets: the one PreviewSurface links for the Web,
// Desktop, and Mobile previews (RFC 0049) and the component site's
// (RFC 0052). Each ends with a newline like other committed files.
export const compiledStylesheets = {
  preview: {
    input: join(repoRoot, "examples/web-demo/assets/preview.css"),
    output: join(repoRoot, "examples/preview-states/assets/preview.generated.css"),
  },
  site: {
    input: join(repoRoot, "site/assets/site.css"),
    output: join(repoRoot, "site/assets/site.generated.css"),
  },
};

export function compiledStylesheet(name = "preview") {
  const stylesheet = compiledStylesheets[name];
  if (!stylesheet) {
    throw new Error(`unknown stylesheet "${name}"; expected one of ${Object.keys(compiledStylesheets).join(", ")}`);
  }
  return stylesheet;
}

export async function renderCompiledStylesheet(name = "preview") {
  const css = await compilePreviewCss(compiledStylesheet(name).input);
  return css.endsWith("\n") ? css : `${css}\n`;
}

const declaredPropertiesCache = new Map();
const important = (utility) => utility.endsWith("!") || utility.startsWith("!");
const variantOf = (utility) => utility.slice(0, Math.max(utility.lastIndexOf(":"), 0));

// The stylesheet `dxui init` writes, without the preview's scan roots: its
// `@theme inline` block defines the token colors, so `bg-primary` compiles
// like `bg-blue-500` instead of producing nothing.
export const themeInput = readFileSync(compiledStylesheets.preview.input, "utf8").replace(/^@source .*\n/gm, "");

async function declaredProperties(utility) {
  // A fresh compiler per utility, because build() accumulates candidates.
  const compiler = await compile(themeInput, {
    base: repoRoot,
    onDependency: () => {},
  });
  const css = compiler.build([utility]);
  // With no utility, Tailwind still emits an empty `@layer utilities;`
  // statement, and the token rules that follow it are not the utility's.
  const start = css.indexOf("@layer utilities {");
  if (start < 0) {
    return new Set();
  }
  const body = css.slice(start, css.indexOf("\n}\n", start));
  // Declarations that read a --tw-* variable are designed to compose, such as
  // text-sm deferring its line height to leading-*. Custom properties count,
  // so two ring colors conflict.
  return new Set(
    [...body.matchAll(/([a-z-]+)\s*:([^;{}]*);/g)]
      .filter(([, property, value]) => !value.includes("var(--tw-"))
      .map(([, property]) => property),
  );
}

// Finds utilities in one class list that set the same property under the same
// variant. Which one applies then depends on stylesheet order, not on the
// order of the class list.
export async function utilityConflicts(classLists) {
  const properties = declaredPropertiesCache;
  for (const classList of classLists) {
    for (const utility of classList.split(/\s+/).filter(Boolean)) {
      if (!properties.has(utility)) {
        properties.set(utility, await declaredProperties(utility));
      }
    }
  }

  const conflicts = new Set();
  for (const classList of classLists) {
    const utilities = [...new Set(classList.split(/\s+/).filter(Boolean))];
    utilities.forEach((first, index) => {
      for (const second of utilities.slice(index + 1)) {
        const shared = [...properties.get(first)].some((property) =>
          properties.get(second).has(property),
        );
        // An important utility, such as bg-blue-100!, wins over a plain one.
        const sameWeight = important(first) === important(second);
        if (shared && sameWeight && variantOf(first) === variantOf(second)) {
          conflicts.add(`${first} / ${second}`);
        }
      }
    });
  }
  return [...conflicts];
}
