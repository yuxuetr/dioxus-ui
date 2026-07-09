#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { renderComponentStatusMarkdown } from "./docs-component-status.mjs";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const statusPath = join(repoRoot, "docs/components/status.md");
const actual = readFileSync(statusPath, "utf8");
const expected = renderComponentStatusMarkdown();

if (actual !== expected) {
  console.error("component status Markdown is out of date");
  console.error("Run `node scripts/docs-component-status.mjs` and update docs/components/status.md.");
  process.exitCode = 1;
} else {
  console.log("component status Markdown verification passed");
}
