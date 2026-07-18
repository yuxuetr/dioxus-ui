#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readText = (relativePath) => readFileSync(join(repoRoot, relativePath), "utf8");
const normalizeWhitespace = (text) => text.replace(/\s+/g, " ");

const getSection = (toml, sectionName) => {
  const sectionHeader = `[${sectionName}]`;
  const lines = toml.split("\n");
  const startIndex = lines.findIndex((line) => line.trim() === sectionHeader);
  if (startIndex < 0) {
    return null;
  }

  const sectionLines = [];
  for (const line of lines.slice(startIndex + 1)) {
    if (/^\s*\[/.test(line)) {
      break;
    }
    sectionLines.push(line);
  }
  return sectionLines.join("\n");
};

const workspaceDependencyEntry = (section, dependencyName) => {
  const escapedName = dependencyName.replaceAll("-", "\\-");
  const match = section.match(new RegExp(`^\\s*${escapedName}\\s*=\\s*\\{([^}]*)\\}\\s*$`, "m"));
  return match?.[1] ?? null;
};

const packageJson = JSON.parse(readText("package.json"));
const rootCargo = readText("Cargo.toml");
const metadataDoc = normalizeWhitespace(readText("docs/workspace-dependency-publish-readiness-metadata.md"));
const blockerDoc = readText("docs/publish-readiness-blockers.md");
const coverageDoc = normalizeWhitespace(readText("docs/publish-readiness-coverage-metadata.md"));
const runbookDoc = normalizeWhitespace(readText("docs/publish-readiness-resolution-runbook.md"));
const cargoPublishDoc = normalizeWhitespace(readText("docs/cargo-publish-metadata.md"));
const publishOrderDoc = normalizeWhitespace(readText("docs/publish-order-metadata.md"));
const releaseDoc = normalizeWhitespace(readText("docs/release.md"));
const qualityDoc = normalizeWhitespace(readText("docs/quality-gates.md"));
const siteDoc = normalizeWhitespace(readText("docs/site.md"));
const scripts = packageJson.scripts ?? {};
const failures = [];

const internalDependencies = [
  "dioxus-ui-core",
  "dioxus-ui-primitives",
  "dioxus-ui",
];

const requireIncludes = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (!text.includes(fragment)) {
      failures.push(`${name} is missing: ${fragment}`);
    }
  }
};

if (
  scripts["verify:workspace-dependency-publish-readiness"] !==
  "node scripts/workspace-dependency-publish-readiness-verify.mjs"
) {
  failures.push("package.json verify:workspace-dependency-publish-readiness script is missing or mismatched");
}

if (!scripts["verify:release"]?.includes("npm run verify:workspace-dependency-publish-readiness")) {
  failures.push("package.json verify:release must include npm run verify:workspace-dependency-publish-readiness");
}

const workspaceDependencies = getSection(rootCargo, "workspace.dependencies");
if (workspaceDependencies === null) {
  failures.push("Cargo.toml is missing [workspace.dependencies]");
} else {
  for (const dependencyName of internalDependencies) {
    const entry = workspaceDependencyEntry(workspaceDependencies, dependencyName);
    if (entry === null) {
      failures.push(`Cargo.toml is missing workspace dependency ${dependencyName}`);
      continue;
    }

    if (!entry.includes(`path = "crates/${dependencyName}"`)) {
      failures.push(`Cargo.toml workspace dependency ${dependencyName} must document the local path`);
    }

    if (/\bversion\s*=/.test(entry)) {
      failures.push(`Cargo.toml workspace dependency ${dependencyName} unexpectedly has publish-ready version metadata`);
    }
  }
}

requireIncludes("docs/workspace-dependency-publish-readiness-metadata.md", metadataDoc, [
  "Workspace Dependency Publish Readiness Metadata",
  "path-only internal workspace dependencies",
  "crates.io-resolvable version metadata",
  "must not change dependency versions, run `cargo package`, run `cargo publish`, contact crates.io, check registry ownership, inspect credentials, create package archives, or authorize a release",
]);

requireIncludes("docs/publish-readiness-blockers.md", blockerDoc, [
  "Workspace dependency publish readiness",
  "Path-only internal workspace dependencies",
]);

requireIncludes("docs/publish-readiness-coverage-metadata.md", coverageDoc, [
  "Workspace dependency publish readiness",
  "npm run verify:workspace-dependency-publish-readiness",
  "workspace-dependency-publish-readiness-metadata.md",
]);

requireIncludes("docs/publish-readiness-resolution-runbook.md", runbookDoc, [
  "Workspace dependency publish readiness",
  "internal crate dependencies have crates.io-resolvable version metadata",
]);

requireIncludes("docs/cargo-publish-metadata.md", cargoPublishDoc, [
  "Workspace dependency publish readiness is tracked separately",
]);

requireIncludes("docs/publish-order-metadata.md", publishOrderDoc, [
  "workspace dependency publish readiness",
]);

requireIncludes("docs/release.md", releaseDoc, [
  "npm run verify:workspace-dependency-publish-readiness",
  "Workspace dependency publish readiness checks are read-only",
  "path-only internal workspace dependencies",
  "they do not change dependency versions, run `cargo package`, run `cargo publish`, contact crates.io, check registry ownership, inspect credentials, create package archives, or authorize a release",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:workspace-dependency-publish-readiness`",
  "path-only internal workspace dependencies",
  "does not change dependency versions, run `cargo package`, run `cargo publish`, contact crates.io, check registry ownership, inspect credentials, create package archives, or authorize a release",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M108 Workspace Dependency Publish Readiness Metadata Gate Usage",
  "npm run verify:workspace-dependency-publish-readiness",
  "path-only internal workspace dependencies",
]);

if (failures.length > 0) {
  console.error("workspace dependency publish readiness metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("workspace dependency publish readiness metadata verification passed");
}
