#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readText = (relativePath) => readFileSync(join(repoRoot, relativePath), "utf8");
const normalizeWhitespace = (text) => text.replace(/\s+/g, " ");

const packageJson = JSON.parse(readText("package.json"));
const coverageDoc = normalizeWhitespace(readText("docs/publish-readiness-coverage-metadata.md"));
const blockerDoc = readText("docs/publish-readiness-blockers.md");
const readme = readText("README.md");
const cargoPublishDoc = normalizeWhitespace(readText("docs/cargo-publish-metadata.md"));
const releaseDoc = normalizeWhitespace(readText("docs/release.md"));
const qualityDoc = normalizeWhitespace(readText("docs/quality-gates.md"));
const siteDoc = normalizeWhitespace(readText("docs/site.md"));
const scripts = packageJson.scripts ?? {};
const failures = [];

const mappings = [
  {
    blocker: "Placeholder repository URL",
    gate: "npm run verify:repository-identity-readiness",
    doc: "repository-identity-readiness-metadata.md",
  },
  {
    blocker: "Pre-1.0 API stability",
    gate: "npm run verify:api-stability-readiness",
    doc: "api-stability-readiness-metadata.md",
  },
  {
    blocker: "Release notes not publish-ready",
    gate: "npm run verify:release-notes-readiness",
    doc: "release-notes-readiness-metadata.md",
  },
  {
    blocker: "Root license files not committed",
    gate: "npm run verify:license-readiness",
    doc: "license-readiness-metadata.md",
  },
  {
    blocker: "CLI template packaging strategy",
    gate: "npm run verify:cli-template-packaging-readiness",
    doc: "cli-template-packaging-readiness-metadata.md",
  },
  {
    blocker: "Registry availability not checked",
    gate: "npm run verify:registry-availability-readiness",
    doc: "registry-availability-readiness-metadata.md",
  },
  {
    blocker: "Workspace dependency publish readiness",
    gate: "npm run verify:workspace-dependency-publish-readiness",
    doc: "workspace-dependency-publish-readiness-metadata.md",
  },
];

const requireIncludes = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (!text.includes(fragment)) {
      failures.push(`${name} is missing: ${fragment}`);
    }
  }
};

if (scripts["verify:publish-readiness-coverage"] !== "node scripts/publish-readiness-coverage-verify.mjs") {
  failures.push("package.json verify:publish-readiness-coverage script is missing or mismatched");
}

if (!scripts["verify:release"]?.includes("npm run verify:publish-readiness-coverage")) {
  failures.push("package.json verify:release must include npm run verify:publish-readiness-coverage");
}

for (const { blocker, gate, doc } of mappings) {
  requireIncludes("docs/publish-readiness-blockers.md", blockerDoc, [blocker]);
  requireIncludes("docs/publish-readiness-coverage-metadata.md", coverageDoc, [blocker, gate, doc]);
  requireIncludes("README.md", readme, [gate]);

  const scriptName = gate.replace("npm run ", "");
  if (typeof scripts[scriptName] !== "string") {
    failures.push(`package.json missing focused readiness script: ${scriptName}`);
  }
  if (!scripts["verify:release"]?.includes(gate)) {
    failures.push(`package.json verify:release must include focused readiness gate: ${gate}`);
  }
}

requireIncludes("docs/publish-readiness-coverage-metadata.md", coverageDoc, [
  "Publish Readiness Coverage Metadata",
  "every current blocker has a focused readiness gate",
  "must not resolve blockers, replace repository URLs, stabilize APIs, generate release notes, generate license text, embed or package CLI templates, change dependency versions, contact registries, inspect credentials, run `cargo package`, run `cargo publish`, or create package archives",
]);

requireIncludes("docs/cargo-publish-metadata.md", cargoPublishDoc, [
  "Publish readiness coverage is tracked separately",
]);

requireIncludes("docs/release.md", releaseDoc, [
  "Publish readiness coverage checks are read-only",
  "every current publish blocker has a focused readiness gate",
  "they do not resolve blockers, replace repository URLs, stabilize APIs, generate release notes, generate license text, embed or package CLI templates, change dependency versions, contact registries, inspect credentials, run `cargo package`, run `cargo publish`, or create package archives",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:publish-readiness-coverage`",
  "every current publish blocker has a focused readiness gate",
  "does not resolve blockers, replace repository URLs, stabilize APIs, generate release notes, generate license text, embed or package CLI templates, change dependency versions, contact registries, inspect credentials, run `cargo package`, run `cargo publish`, or create package archives",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M105 Publish Readiness Coverage Metadata Gate Usage",
  "npm run verify:publish-readiness-coverage",
  "every current publish blocker has a focused readiness gate",
]);

if (failures.length > 0) {
  console.error("publish readiness coverage metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("publish readiness coverage metadata verification passed");
}
