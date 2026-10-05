#!/usr/bin/env node
import { readdirSync, readFileSync } from "node:fs";
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

const getStringField = (section, field) => {
  const match = section.match(new RegExp(`^\\s*${field}\\s*=\\s*"([^"]+)"\\s*$`, "m"));
  return match?.[1] ?? null;
};

const hasWorkspaceInheritance = (section, field) => {
  return new RegExp(`^\\s*${field}\\.workspace\\s*=\\s*true\\s*$`, "m").test(section);
};

const packageJson = JSON.parse(readText("package.json"));
const rootCargo = readText("Cargo.toml");
const apiDoc = normalizeWhitespace(readText("docs/api-stability-readiness-metadata.md"));
const publishBlockers = readText("docs/publish-readiness-blockers.md");
const cargoPublishDoc = normalizeWhitespace(readText("docs/cargo-publish-metadata.md"));
const releaseDoc = normalizeWhitespace(readText("docs/release.md"));
const qualityDoc = normalizeWhitespace(readText("docs/quality-gates.md"));
const siteDoc = normalizeWhitespace(readText("docs/site.md"));
const scripts = packageJson.scripts ?? {};
const failures = [];

const requireExcludes = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (text.includes(fragment)) {
      failures.push(`${name} must not include stale wording: ${fragment}`);
    }
  }
};

const requireIncludes = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (!text.includes(fragment)) {
      failures.push(`${name} is missing: ${fragment}`);
    }
  }
};

if (scripts["verify:api-stability-readiness"] !== "node scripts/api-stability-readiness-verify.mjs") {
  failures.push("package.json verify:api-stability-readiness script is missing or mismatched");
}

if (!scripts["verify:release"]?.includes("npm run verify:api-stability-readiness")) {
  failures.push("package.json verify:release must include npm run verify:api-stability-readiness");
}

const workspacePackage = getSection(rootCargo, "workspace.package");
if (workspacePackage === null) {
  failures.push("Cargo.toml is missing [workspace.package]");
} else if (getStringField(workspacePackage, "version") !== "0.1.0") {
  failures.push('Cargo.toml [workspace.package] must keep version = "0.1.0" until API readiness docs are updated');
}

const crateNames = readdirSync(join(repoRoot, "crates"), { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => entry.name);
for (const crateName of crateNames) {
  const manifest = readText(`crates/${crateName}/Cargo.toml`);
  const packageSection = getSection(manifest, "package");
  if (packageSection === null) {
    failures.push(`crates/${crateName}/Cargo.toml is missing [package]`);
  } else if (!hasWorkspaceInheritance(packageSection, "version")) {
    failures.push(`crates/${crateName}/Cargo.toml must inherit version.workspace = true`);
  }
}

requireIncludes("docs/api-stability-readiness-metadata.md", apiDoc, [
  "API Stability Readiness Metadata",
  "workspace is at `0.1.0`",
  "current `0.1.x` API surface is accepted for first publish",
  "Approved Publish Blocker Resolution Plan",
  "Must not break public crate-mode APIs",
  "Allowed before `1.0` only in a minor bump (`0.1` to `0.2`)",
  "documented in `CHANGELOG.md` with a migration note",
  "Breaking API changes remain allowed before `1.0`.",
  "This gate must not freeze APIs or change versions automatically.",
  "after resolution",
]);

requireIncludes("docs/publish-readiness-blockers.md", publishBlockers, [
  "Pre-1.0 API stability",
  "Current `0.1.x` API surface is accepted for first publish",
  "npm run verify:api-stability-readiness",
]);

requireExcludes("docs/publish-readiness-blockers.md", publishBlockers, [
  "Maintainers decide crate-mode stability and versioning policy",
]);

requireIncludes("docs/cargo-publish-metadata.md", cargoPublishDoc, [
  "APIs remain pre-1.0",
  "current `0.1.x` API surface is accepted for first publish",
]);

requireIncludes("docs/release.md", releaseDoc, [
  "Before `1.0`, API changes are allowed but should still be documented in the changelog.",
  "API stability readiness checks are read-only",
  "workspace version `0.1.0` and the accepted `0.1.x` first-publish API policy",
  "they do not stabilize component APIs, change crate versions, change the pre-`1.0` breaking-change policy, generate migration guides, run `cargo package`, or run `cargo publish`",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:api-stability-readiness`",
  "workspace version `0.1.0` and the accepted `0.1.x` first-publish API policy",
  "does not stabilize component APIs, change crate versions, change the pre-`1.0` breaking-change policy, generate migration guides, run `cargo package`, or run `cargo publish`",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M102 API Stability Readiness Metadata Gate Usage",
  "npm run verify:api-stability-readiness",
  "workspace version `0.1.0` and the accepted `0.1.x` first-publish API policy",
]);

if (failures.length > 0) {
  console.error("API stability readiness metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("API stability readiness metadata verification passed");
}
