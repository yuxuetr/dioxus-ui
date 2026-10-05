#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readText = (relativePath) => readFileSync(join(repoRoot, relativePath), "utf8");
const normalizeWhitespace = (text) => text.replace(/\s+/g, " ");

const packageJson = JSON.parse(readText("package.json"));
const cliSource = readText("crates/dioxus-shadcn-cli/src/main.rs");
const cliBuild = readText("crates/dioxus-shadcn-cli/build.rs");
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

const requireAbsent = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (text.includes(fragment)) {
      failures.push(`${name} should not include: ${fragment}`);
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

requireIncludes("crates/dioxus-shadcn-cli/src/main.rs", cliSource, [
  'include!(concat!(env!("OUT_DIR"), "/embedded_assets.rs"))',
  "EMBEDDED_REGISTRY_JSON",
  "EMBEDDED_ASSETS",
  "embedded_asset_content",
]);

requireAbsent("crates/dioxus-shadcn-cli/src/main.rs", cliSource, [
  "workspace.join(&file.source)",
  "workspace.join(&asset.source)",
  'join("registry")',
]);

requireIncludes("crates/dioxus-shadcn-cli/build.rs", cliBuild, [
  "embedded_assets.rs",
  "include_str!",
  "registry_sources",
  "serde_json::from_str::<Value>",
]);

requireIncludes("docs/cli-template-packaging-readiness-metadata.md", readinessDoc, [
  "CLI Template Packaging Readiness Metadata",
  "compile-time embedded registry and template assets",
  "no longer require the repository `registry/` and `templates/` directories at runtime",
  "CLI build script generates embedded registry/template assets",
  "CLI runtime reads embedded registry/template content",
  "must not run `cargo package`, run `cargo publish`, install the CLI, contact crates.io, create package archives, or change embedded template contents",
]);

requireIncludes("docs/publish-readiness-blockers.md", publishBlockers, [
  "Resolved publish readiness items",
  "`dioxus-shadcn-cli` embeds registry and template assets at compile time",
  "npm run verify:cli-template-packaging-readiness",
]);

requireIncludes("docs/cargo-publish-metadata.md", cargoPublishDoc, [
  "CLI template delivery now uses embedded registry/template assets",
]);

requireIncludes("docs/release.md", releaseDoc, [
  "CLI template packaging readiness checks are read-only",
  "CLI embeds registry and template assets at compile time",
  "they do not run `cargo package`, run `cargo publish`, install the CLI, contact crates.io, create package archives, or change embedded template contents",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:cli-template-packaging-readiness`",
  "CLI embeds registry and template assets at compile time",
  "does not run `cargo package`, run `cargo publish`, install the CLI, contact crates.io, create package archives, or change embedded template contents",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M103 CLI Template Packaging Readiness Metadata Gate Usage",
  "npm run verify:cli-template-packaging-readiness",
  "CLI registry and template assets are embedded at compile time",
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
