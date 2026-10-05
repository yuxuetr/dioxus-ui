#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readText = (relativePath) => readFileSync(join(repoRoot, relativePath), "utf8");
const normalizeWhitespace = (text) => text.replace(/\s+/g, " ");

const packageJson = JSON.parse(readText("package.json"));
const runbookDoc = normalizeWhitespace(readText("docs/publish-readiness-resolution-runbook.md"));
const blockerDoc = readText("docs/publish-readiness-blockers.md");
const coverageDoc = normalizeWhitespace(readText("docs/publish-readiness-coverage-metadata.md"));
const cargoPublishDoc = normalizeWhitespace(readText("docs/cargo-publish-metadata.md"));
const releaseDoc = normalizeWhitespace(readText("docs/release.md"));
const qualityDoc = normalizeWhitespace(readText("docs/quality-gates.md"));
const siteDoc = normalizeWhitespace(readText("docs/site.md"));
const scripts = packageJson.scripts ?? {};
const failures = [];

const blockers = [
  "Placeholder repository URL",
  "Root license files not committed",
  "Pre-1.0 API stability",
  "Release notes not publish-ready",
  "Registry availability not checked",
  "Workspace dependency publish readiness",
];

const followUpTargets = [
  "publish blockers",
  "Cargo publish metadata",
  "publish order metadata",
  "release docs",
  "quality gates",
  "README",
  "TODO planning",
];

const requireIncludes = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (!text.includes(fragment)) {
      failures.push(`${name} is missing: ${fragment}`);
    }
  }
};

if (scripts["verify:publish-readiness-runbook"] !== "node scripts/publish-readiness-runbook-verify.mjs") {
  failures.push("package.json verify:publish-readiness-runbook script is missing or mismatched");
}

if (!scripts["verify:release"]?.includes("npm run verify:publish-readiness-runbook")) {
  failures.push("package.json verify:release must include npm run verify:publish-readiness-runbook");
}

for (const blocker of blockers) {
  requireIncludes("docs/publish-readiness-blockers.md", blockerDoc, [blocker]);
  requireIncludes("docs/publish-readiness-resolution-runbook.md", runbookDoc, [blocker]);
}

for (const target of followUpTargets) {
  requireIncludes("docs/publish-readiness-resolution-runbook.md", runbookDoc, [target]);
}

requireIncludes("docs/publish-readiness-resolution-runbook.md", runbookDoc, [
  "Publish Readiness Resolution Runbook",
  "manual resolution runbook",
  "Manual Evidence Required",
  "Follow-up Updates",
  "Resolved items",
  "CLI template packaging strategy",
  "`dioxus-shadcn-cli` embeds registry and template assets at compile time",
  "npm run verify:cli-template-packaging-readiness",
  "must not resolve blockers, replace repository URLs, stabilize APIs, generate release notes, generate license text, change embedded CLI template delivery, change dependency versions, contact registries, inspect credentials, run `cargo package`, run `cargo publish`, or create package archives",
]);

requireIncludes("docs/publish-readiness-coverage-metadata.md", coverageDoc, [
  "runbook",
]);

requireIncludes("docs/cargo-publish-metadata.md", cargoPublishDoc, [
  "Publish readiness resolution is tracked separately",
]);

requireIncludes("docs/release.md", releaseDoc, [
  "Publish readiness runbook checks are read-only",
  "manual resolution evidence for every current publish blocker",
  "they do not resolve blockers, replace repository URLs, stabilize APIs, generate release notes, generate license text, change embedded CLI template delivery, change dependency versions, contact registries, inspect credentials, run `cargo package`, run `cargo publish`, or create package archives",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:publish-readiness-runbook`",
  "manual resolution evidence for every current publish blocker",
  "does not resolve blockers, replace repository URLs, stabilize APIs, generate release notes, generate license text, change embedded CLI template delivery, change dependency versions, contact registries, inspect credentials, run `cargo package`, run `cargo publish`, or create package archives",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M106 Publish Readiness Resolution Runbook Metadata Gate Usage",
  "npm run verify:publish-readiness-runbook",
  "manual resolution evidence for every current publish blocker",
]);

if (failures.length > 0) {
  console.error("publish readiness runbook metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("publish readiness runbook metadata verification passed");
}
