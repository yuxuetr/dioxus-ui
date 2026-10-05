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
