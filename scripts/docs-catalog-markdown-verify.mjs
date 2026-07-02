#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { renderDocsCatalogMarkdown } from "./docs-catalog-markdown.mjs";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const catalogPath = join(repoRoot, "docs/components/catalog.md");
const actual = readFileSync(catalogPath, "utf8");
const expected = renderDocsCatalogMarkdown();

if (actual !== expected) {
  console.error("docs catalog Markdown is out of date");
  console.error("Run `node scripts/docs-catalog-markdown.mjs` and update docs/components/catalog.md.");
  process.exitCode = 1;
} else {
  console.log("docs catalog Markdown verification passed");
}
