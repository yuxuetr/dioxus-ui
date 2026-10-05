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
  "dioxus-shadcn-core",
  "dioxus-shadcn-primitives",
  "dioxus-shadcn",
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

const workspacePackage = getSection(rootCargo, "workspace.package");
const workspaceVersion = workspacePackage?.match(/^\s*version\s*=\s*"([^"]+)"\s*$/m)?.[1] ?? null;
if (workspaceVersion === null) {
  failures.push("Cargo.toml [workspace.package] is missing version");
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

    // Published crates resolve internal dependencies from crates.io by this
    // version; it must track the workspace version so path and registry agree.
    const dependencyVersion = entry.match(/\bversion\s*=\s*"([^"]+)"/)?.[1] ?? null;
    if (dependencyVersion === null) {
      failures.push(`Cargo.toml workspace dependency ${dependencyName} is missing crates.io-resolvable version metadata`);
    } else if (workspaceVersion !== null && dependencyVersion !== workspaceVersion) {
      failures.push(
        `Cargo.toml workspace dependency ${dependencyName} version "${dependencyVersion}" must match workspace version "${workspaceVersion}"`,
      );
    }
  }
}

requireIncludes("docs/workspace-dependency-publish-readiness-metadata.md", metadataDoc, [
  "Workspace Dependency Publish Readiness Metadata",
  `version = "${workspaceVersion}", path = "crates/dioxus-shadcn-core"`,
  "crates.io-resolvable version metadata",
  "after resolution",
  "must not change dependency versions, run `cargo package`, run `cargo publish`, contact crates.io, check registry ownership, inspect credentials, create package archives, or authorize a release",
]);

requireIncludes("docs/publish-readiness-blockers.md", blockerDoc, [
  "Workspace dependency publish readiness",
  `Internal workspace dependencies declare \`version = "${workspaceVersion}"\` alongside local paths`,
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
  "internal workspace dependencies declare crates.io-resolvable versions",
]);

requireIncludes("docs/publish-order-metadata.md", publishOrderDoc, [
  "internal crate dependencies declare crates.io-resolvable version metadata aligned with this publish order",
]);

requireIncludes("docs/release.md", releaseDoc, [
  "npm run verify:workspace-dependency-publish-readiness",
  "Workspace dependency publish readiness checks are read-only",
  "internal workspace dependencies declare versions matching the workspace version alongside local paths",
  "they do not change dependency versions, run `cargo package`, run `cargo publish`, contact crates.io, check registry ownership, inspect credentials, create package archives, or authorize a release",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:workspace-dependency-publish-readiness`",
  "internal workspace dependencies declare versions matching the workspace version alongside local paths",
  "does not change dependency versions, run `cargo package`, run `cargo publish`, contact crates.io, check registry ownership, inspect credentials, create package archives, or authorize a release",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M108 Workspace Dependency Publish Readiness Metadata Gate Usage",
  "npm run verify:workspace-dependency-publish-readiness",
  "internal workspace dependencies declare versions matching the workspace version alongside local paths",
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
