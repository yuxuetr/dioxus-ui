#!/usr/bin/env node
import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const roots = [
  "crates/dioxus-ui/src",
  "crates/dioxus-ui-core/src",
  "templates",
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

  for (const [index, line] of lines.entries()) {
    dynamicTokenPattern.lastIndex = 0;
    const matches = [...line.matchAll(dynamicTokenPattern)];
    for (const match of matches) {
      failures.push(`${relativePath}:${index + 1}: dynamic Tailwind token "${match[0].trim()}"`);
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
