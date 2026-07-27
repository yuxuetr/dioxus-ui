#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const packageJson = JSON.parse(readRepoFile("package.json"));
const rootCargo = readRepoFile("Cargo.toml");
const blockerDoc = readRepoFile("docs/publish-readiness-blockers.md");
const publishMetadataDoc = readRepoFile("docs/cargo-publish-metadata.md");
const workspaceDocs = readRepoFile("docs/workspace.md");
const releaseDocs = readRepoFile("docs/release.md");
const qualityGates = readRepoFile("docs/quality-gates.md");
const docsSite = readRepoFile("docs/site.md");
const normalizedPublishMetadataDoc = publishMetadataDoc.replace(/\s+/g, " ");
const normalizedReleaseDocs = releaseDocs.replace(/\s+/g, " ");
const normalizedQualityGates = qualityGates.replace(/\s+/g, " ");
const normalizedDocsSite = docsSite.replace(/\s+/g, " ");
const failures = [];

const requireFragment = (label, source, fragment) => {
  if (!source.includes(fragment)) {
    failures.push(`${label} missing fragment: ${fragment}`);
  }
};

const rejectFragment = (label, source, fragment) => {
  if (source.includes(fragment)) {
    failures.push(`${label} should not include fragment: ${fragment}`);
  }
};

const staleCliBlockerFragment =
  "| CLI template packaging strategy | " +
  "CLI release notes still " +
  "say " +
  "templates are read from the repository layout |";

const scripts = packageJson.scripts ?? {};
if (scripts["verify:publish-readiness-blockers"] !== "node scripts/publish-readiness-blockers-verify.mjs") {
  failures.push("package.json missing verify:publish-readiness-blockers script");
}

if (!scripts["verify:release"]?.includes("npm run verify:publish-readiness-blockers")) {
  failures.push("package.json verify:release missing publish readiness blocker gate");
}

requireFragment("Cargo.toml", rootCargo, 'repository = "https://github.com/yuxuetr/dioxus-ui"');

const blockerFragments = [
  "Placeholder repository URL",
  "Pre-1.0 API stability",
  "Release notes not publish-ready",
  "Root license files not committed",
  "Registry availability not checked",
  "Workspace dependency publish readiness",
  "Resolved publish readiness items",
  "https://github.com/yuxuetr/dioxus-ui",
  "`dioxus-ui-cli` embeds registry and template assets at compile time",
  "does not resolve the blockers",
];

for (const fragment of blockerFragments) {
  requireFragment("docs/publish-readiness-blockers.md", blockerDoc, fragment);
}

const blockerBoundaryFragments = [
  "changing repository URLs",
  "checking crates.io name availability",
  "running `cargo package`",
  "running `cargo publish`",
  "stabilizing component APIs",
  "generating changelogs or release notes",
  "generating license text",
  "changing embedded CLI template delivery",
  "changing dependency versions",
];

for (const fragment of blockerBoundaryFragments) {
  requireFragment("docs/publish-readiness-blockers.md", blockerDoc, fragment);
}

rejectFragment(
  "docs/publish-readiness-blockers.md",
  blockerDoc,
  staleCliBlockerFragment,
);

const publishMetadataFragments = [
  "does not claim the crates",
  "repository URL is approved",
  "APIs remain pre-1.0",
  "CLI template delivery now uses embedded registry/template assets",
];

for (const fragment of publishMetadataFragments) {
  requireFragment("docs/cargo-publish-metadata.md", normalizedPublishMetadataDoc, fragment);
}

const workspaceFragments = [
  "repository URL has been approved",
  "APIs are still pre-1.0",
];

for (const fragment of workspaceFragments) {
  requireFragment("docs/workspace.md", workspaceDocs, fragment);
}

const releaseFragments = [
  "Publish readiness blocker checks are read-only",
  "placeholder repository URL, pre-1.0 API stability, release notes readiness, root license file readiness, crates.io review, and workspace dependency publish readiness blockers",
  "they do not replace repository URLs, check registries, run `cargo package`, run `cargo publish`, stabilize APIs, generate changelogs, generate license text, change embedded CLI template delivery, or change dependency versions",
  "CLI embeds registry and template assets at compile time",
];

for (const fragment of releaseFragments) {
  requireFragment("docs/release.md", normalizedReleaseDocs, fragment);
}

const qualityFragments = [
  "`npm run verify:publish-readiness-blockers`",
  "placeholder repository URL, pre-1.0 API stability, release notes readiness, root license file readiness, crates.io review, and workspace dependency publish readiness blockers",
  "does not replace repository URLs, check registries, run `cargo package`, run `cargo publish`, stabilize APIs, generate changelogs, generate license text, change embedded CLI template delivery, or change dependency versions",
];

for (const fragment of qualityFragments) {
  requireFragment("docs/quality-gates.md", normalizedQualityGates, fragment);
}

const siteFragments = [
  "M97 Publish Readiness Blocker Metadata Gate Usage",
  "npm run verify:publish-readiness-blockers",
  "placeholder repository URL",
];

for (const fragment of siteFragments) {
  requireFragment("docs/site.md", normalizedDocsSite, fragment);
}

if (failures.length > 0) {
  console.error("publish readiness blocker metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("publish readiness blocker metadata verification passed");
}
