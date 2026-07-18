#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readText = (relativePath) => readFileSync(join(repoRoot, relativePath), "utf8");
const normalizeWhitespace = (text) => text.replace(/\s+/g, " ");

const packageJson = JSON.parse(readText("package.json"));
const publishOrderDoc = normalizeWhitespace(readText("docs/publish-order-metadata.md"));
const releaseDoc = normalizeWhitespace(readText("docs/release.md"));
const cargoPublishDoc = normalizeWhitespace(readText("docs/cargo-publish-metadata.md"));
const registryAvailabilityDoc = normalizeWhitespace(readText("docs/registry-availability-readiness-metadata.md"));
const runbookDoc = normalizeWhitespace(readText("docs/publish-readiness-resolution-runbook.md"));
const qualityDoc = normalizeWhitespace(readText("docs/quality-gates.md"));
const siteDoc = normalizeWhitespace(readText("docs/site.md"));
const scripts = packageJson.scripts ?? {};
const failures = [];

const publishOrder = [
  "dioxus-ui-core",
  "dioxus-ui-primitives",
  "dioxus-ui",
  "dioxus-ui-cli",
];

const requireIncludes = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (!text.includes(fragment)) {
      failures.push(`${name} is missing: ${fragment}`);
    }
  }
};

const requireOrder = (name, text, orderedItems) => {
  let lastIndex = -1;
  for (const item of orderedItems) {
    const index = text.indexOf(item, lastIndex + 1);
    if (index < 0) {
      failures.push(`${name} is missing ordered item: ${item}`);
      return;
    }
    if (index <= lastIndex) {
      failures.push(`${name} has publish order drift around: ${item}`);
      return;
    }
    lastIndex = index;
  }
};

if (scripts["verify:publish-order"] !== "node scripts/publish-order-verify.mjs") {
  failures.push("package.json verify:publish-order script is missing or mismatched");
}

if (!scripts["verify:release"]?.includes("npm run verify:publish-order")) {
  failures.push("package.json verify:release must include npm run verify:publish-order");
}

for (const [name, text] of [
  ["docs/publish-order-metadata.md", publishOrderDoc],
  ["docs/release.md", releaseDoc],
  ["docs/cargo-publish-metadata.md", cargoPublishDoc],
  ["docs/registry-availability-readiness-metadata.md", registryAvailabilityDoc],
]) {
  requireOrder(name, text, publishOrder);
}

requireIncludes("docs/publish-order-metadata.md", publishOrderDoc, [
  "Publish Order Metadata",
  "dependency crates come first",
  "must not create package archives, run `cargo package`, run `cargo publish`, contact crates.io, check registry ownership, inspect credentials, change dependency versions, or authorize a release",
]);

requireIncludes("docs/publish-readiness-resolution-runbook.md", runbookDoc, [
  "publish order",
  "Release owner confirms crates.io names, ownership, credentials, and publish order",
]);

requireIncludes("docs/release.md", releaseDoc, [
  "Publish order checks are read-only",
  "planned crate publish order",
  "they do not create package archives, run `cargo package`, run `cargo publish`, contact crates.io, check registry ownership, inspect credentials, change dependency versions, or authorize a release",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:publish-order`",
  "planned crate publish order",
  "does not create package archives, run `cargo package`, run `cargo publish`, contact crates.io, check registry ownership, inspect credentials, change dependency versions, or authorize a release",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M107 Publish Order Metadata Gate Usage",
  "npm run verify:publish-order",
  "planned crate publish order",
]);

if (failures.length > 0) {
  console.error("publish order metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("publish order metadata verification passed");
}
