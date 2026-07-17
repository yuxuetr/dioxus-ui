#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readText = (relativePath) => readFileSync(join(repoRoot, relativePath), "utf8");
const normalizeWhitespace = (text) => text.replace(/\s+/g, " ");

const packageJson = JSON.parse(readText("package.json"));
const cliSource = readText("crates/dioxus-ui-cli/src/main.rs");
const readinessDoc = normalizeWhitespace(readText("docs/cli-template-packaging-readiness-metadata.md"));
const publishBlockers = readText("docs/publish-readiness-blockers.md");
const cargoPublishDoc = normalizeWhitespace(readText("docs/cargo-publish-metadata.md"));
const releaseDoc = normalizeWhitespace(readText("docs/release.md"));
const qualityDoc = normalizeWhitespace(readText("docs/quality-gates.md"));
const siteDoc = normalizeWhitespace(readText("docs/site.md"));
const scripts = packageJson.scripts ?? {};
const failures = [];

const requireIncludes = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (!text.includes(fragment)) {
      failures.push(`${name} is missing: ${fragment}`);
    }
  }
};

if (
  scripts["verify:cli-template-packaging-readiness"] !==
  "node scripts/cli-template-packaging-readiness-verify.mjs"
) {
  failures.push("package.json verify:cli-template-packaging-readiness script is missing or mismatched");
}

if (!scripts["verify:release"]?.includes("npm run verify:cli-template-packaging-readiness")) {
  failures.push("package.json verify:release must include npm run verify:cli-template-packaging-readiness");
}

requireIncludes("crates/dioxus-ui-cli/src/main.rs", cliSource, [
  'env!("CARGO_MANIFEST_DIR")',
  'join("registry")',
  "workspace.join(&file.source)",
  "workspace.join(&asset.source)",
  "fs::read_to_string",
]);

for (const forbidden of ["include_str!", "include_bytes!"]) {
  if (cliSource.includes(forbidden)) {
    failures.push(`crates/dioxus-ui-cli/src/main.rs should not use ${forbidden} before packaging readiness is resolved`);
  }
}

requireIncludes("docs/cli-template-packaging-readiness-metadata.md", readinessDoc, [
  "CLI Template Packaging Readiness Metadata",
  "source-tree template loading",
  "publish-ready CLI binary",
  "embed templates at compile time or package templates in a stable install location",
  "must not embed templates, package templates, change CLI runtime path lookup, run `cargo package`, run `cargo publish`, install the CLI, or create package archives",
]);

requireIncludes("docs/publish-readiness-blockers.md", publishBlockers, [
  "CLI template packaging strategy",
  "CLI release notes still say templates are read from the repository layout",
  "CLI owner embeds templates or packages them in a stable install location",
  "packaging CLI templates",
]);

requireIncludes("docs/cargo-publish-metadata.md", cargoPublishDoc, [
  "CLI template packaging remains unresolved",
]);

requireIncludes("docs/release.md", releaseDoc, [
  "CLI template packaging readiness checks are read-only",
  "CLI template source is still repository-layout based",
  "they do not embed templates, package templates, change CLI runtime path lookup, run `cargo package`, run `cargo publish`, install the CLI, or create package archives",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:cli-template-packaging-readiness`",
  "CLI template source is still repository-layout based",
  "does not embed templates, package templates, change CLI runtime path lookup, run `cargo package`, run `cargo publish`, install the CLI, or create package archives",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M103 CLI Template Packaging Readiness Metadata Gate Usage",
  "npm run verify:cli-template-packaging-readiness",
  "CLI template source is still repository-layout based",
]);

if (failures.length > 0) {
  console.error("CLI template packaging readiness metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("CLI template packaging readiness metadata verification passed");
}
