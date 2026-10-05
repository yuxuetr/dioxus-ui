#!/usr/bin/env node
import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const roots = [
  "crates/dioxus-shadcn/src",
  "crates/dioxus-shadcn-core/src",
  "crates/dioxus-shadcn-primitives/src",
  "crates/dioxus-shadcn-cli/templates",
];
const utilityPrefixes = [
  "bg",
  "text",
  "border",
  "ring",
  "from",
  "via",
  "to",
  "fill",
  "stroke",
  "p",
  "px",
  "py",
  "pt",
  "pr",
  "pb",
  "pl",
  "m",
  "mx",
  "my",
  "mt",
  "mr",
  "mb",
  "ml",
  "h",
  "w",
  "min-h",
  "min-w",
  "max-h",
  "max-w",
  "gap",
  "gap-x",
  "gap-y",
  "grid-cols",
  "col-span",
  "row-span",
  "rounded",
];
const dynamicTokenPattern = new RegExp(
  `(?:^|[^A-Za-z0-9_-])(${utilityPrefixes
    .map((prefix) => prefix.replace("-", "\\-"))
    .join("|")})-\\{[^}\\n]+\\}`,
  "g",
);
// Tailwind's bare `data-name:` variant matches when the attribute exists, but
// components render boolean flags as "true" or "false" and enumerations as
// `data-orientation="vertical"`. Only attributes set without a value, such as
// the listbox's `data-highlighted`, may use the bare form.
const presenceDataAttributes = new Set(["highlighted"]);
const bareDataVariantPattern = /(?:^|[\s"])((?:[a-z-]+:)*data-([a-z-]+)):/g;
// Components color through the RFC 0051 semantic tokens, so a theme that
// redefines the tokens restyles them. Palette colors are allowed only where
// shadcn/ui keeps them too.
const paletteColors =
  "white|black|slate|gray|zinc|neutral|stone|red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose";
const paletteUtilityPattern = new RegExp(
  `(?:^|[\\s"':])!?((?:bg|text|border(?:-[xytrblse])?|ring(?:-offset)?|outline|divide|fill|stroke|from|via|to|accent|caret|decoration|shadow)-(?:${paletteColors})(?:-\\d{2,3})?(?:/\\d+)?)!?(?=$|[\\s"'])`,
  "g",
);
const allowedPaletteUtilities = new Set(["bg-black/50"]);

const rustFiles = [];

const walk = (dir) => {
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    const stat = statSync(path);

    if (stat.isDirectory()) {
      walk(path);
    } else if (entry.endsWith(".rs")) {
      rustFiles.push(path);
    }
  }
};

for (const root of roots) {
  walk(join(repoRoot, root));
}

const failures = [];

for (const file of rustFiles.sort()) {
  const source = readFileSync(file, "utf8");
  const relativePath = relative(repoRoot, file);
  const lines = source.split("\n");
  // Tests may pass app palette classes through `class`.
  const testStart = lines.findIndex((line) => line.startsWith("#[cfg(test)]"));

  for (const [index, line] of lines.entries()) {
    if (testStart < 0 || index < testStart) {
      for (const match of line.matchAll(paletteUtilityPattern)) {
        if (!allowedPaletteUtilities.has(match[1])) {
          failures.push(`${relativePath}:${index + 1}: palette color "${match[1]}"; use an RFC 0051 token`);
        }
      }
    }

    dynamicTokenPattern.lastIndex = 0;
    const matches = [...line.matchAll(dynamicTokenPattern)];
    for (const match of matches) {
      failures.push(`${relativePath}:${index + 1}: dynamic Tailwind token "${match[0].trim()}"`);
    }

    for (const match of line.matchAll(bareDataVariantPattern)) {
      if (!presenceDataAttributes.has(match[2])) {
        failures.push(
          `${relativePath}:${index + 1}: bare data variant "${match[1]}:" matches any value; use data-[attribute=value]:`,
        );
      }
    }
  }
}

if (failures.length > 0) {
  console.error("Tailwind static token verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log(`Tailwind static token verification passed (${rustFiles.length} files)`);
}
