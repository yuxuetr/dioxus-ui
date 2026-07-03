#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { renderDocsRouteManifestMarkdown } from "./docs-route-manifest-markdown.mjs";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const manifestPath = join(repoRoot, "docs/components/routes.md");
const actual = readFileSync(manifestPath, "utf8");
const expected = renderDocsRouteManifestMarkdown();

if (actual !== expected) {
  console.error("docs route manifest is out of date");
  console.error("Run `node scripts/docs-route-manifest-markdown.mjs` and update docs/components/routes.md.");
  process.exitCode = 1;
} else {
  console.log("docs route manifest verification passed");
}
