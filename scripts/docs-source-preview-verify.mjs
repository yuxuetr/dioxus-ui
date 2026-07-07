#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { renderDocsSourcePreviewMarkdown } from "./docs-source-preview-markdown.mjs";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const manifestPath = join(repoRoot, "docs/components/source-preview.md");
const actual = readFileSync(manifestPath, "utf8");
const expected = renderDocsSourcePreviewMarkdown();

if (actual !== expected) {
  console.error("docs source preview manifest is out of date");
  console.error("Run `node scripts/docs-source-preview-markdown.mjs` and update docs/components/source-preview.md.");
  process.exitCode = 1;
} else {
  console.log("docs source preview manifest verification passed");
}
