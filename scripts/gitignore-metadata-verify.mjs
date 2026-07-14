#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const gitignore = readRepoFile(".gitignore");
const repoHygiene = readRepoFile("scripts/repo-hygiene-verify.mjs");
const packageJson = JSON.parse(readRepoFile("package.json"));
const gitignoreLines = new Set(
  gitignore
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0 && !line.startsWith("#")),
);
const failures = [];

const requiredGitignorePatterns = [
  ".playwright-mcp/",
  "node_modules/",
  "target/",
  "dist/",
  "build/",
  "dioxus-ui-web-preview-*.png",
  "dioxus-ui-desktop-webview-preview-*.png",
  "dioxus-ui-mobile-browser-preview-*.png",
];

for (const pattern of requiredGitignorePatterns) {
  if (!gitignoreLines.has(pattern)) {
    failures.push(`.gitignore missing pattern: ${pattern}`);
  }
}

const requiredRepoHygieneFragments = [
  'file.startsWith("node_modules/")',
  'file.startsWith("target/")',
  'file.startsWith("dist/") || file.startsWith("build/")',
  '/^dioxus-ui-mobile-browser-preview-.*\\.png$/.test(file)',
  '/^dxui-generated-fixture[./-]/.test(file)',
  'file === ".github/workflows/browser-smoke.yml"',
  'path: ".github/workflows/browser-smoke.yml"',
];

for (const fragment of requiredRepoHygieneFragments) {
  if (!repoHygiene.includes(fragment)) {
    failures.push(`scripts/repo-hygiene-verify.mjs missing fragment: ${fragment}`);
  }
}

const scripts = packageJson.scripts ?? {};
if (scripts["verify:gitignore"] !== "node scripts/gitignore-metadata-verify.mjs") {
  failures.push("package.json script verify:gitignore must be node scripts/gitignore-metadata-verify.mjs");
}

if (!scripts["verify:release"]?.includes("npm run verify:gitignore")) {
  failures.push("package.json verify:release must include npm run verify:gitignore");
}

if (failures.length > 0) {
  console.error("gitignore metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log(`gitignore metadata verification passed (${requiredGitignorePatterns.length} patterns)`);
}
