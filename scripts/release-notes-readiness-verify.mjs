#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readText = (relativePath) => readFileSync(join(repoRoot, relativePath), "utf8");
const normalizeWhitespace = (text) => text.replace(/\s+/g, " ");

const packageJson = JSON.parse(readText("package.json"));
const publishBlockers = readText("docs/publish-readiness-blockers.md");
const releaseNotesDoc = normalizeWhitespace(readText("docs/release-notes-readiness-metadata.md"));
const changelog = readText("CHANGELOG.md");
const changelogDoc = normalizeWhitespace(readText("docs/changelog-metadata.md"));
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

const requireExcludes = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (text.includes(fragment)) {
      failures.push(`${name} must not include stale wording: ${fragment}`);
    }
  }
};

if (scripts["verify:release-notes-readiness"] !== "node scripts/release-notes-readiness-verify.mjs") {
  failures.push("package.json verify:release-notes-readiness script is missing or mismatched");
}

if (!scripts["verify:release"]?.includes("npm run verify:release-notes-readiness")) {
  failures.push("package.json verify:release must include npm run verify:release-notes-readiness");
}

requireIncludes("docs/publish-readiness-blockers.md", publishBlockers, [
  "Release notes not publish-ready",
  "`CHANGELOG.md` has project-owned structure",
  "records first publish included scope, excluded scope, and known warnings",
  "npm run verify:release-notes-readiness",
]);

requireExcludes("docs/publish-readiness-blockers.md", publishBlockers, [
  "Changelog not yet release-owned",
  "Maintainers define and maintain release notes before publishing",
]);

requireIncludes("CHANGELOG.md", changelog, [
  "## [Unreleased]",
  "first publish (`0.1.0`) release notes",
  "`dioxus-ui-core`",
  "`dioxus-ui-primitives`",
  "`dioxus-ui`:",
  "`dioxus-ui-cli`",
  "### Excluded From First Publish",
  "### Known Warnings",
  "not derived from Git history",
]);

requireIncludes("docs/release-notes-readiness-metadata.md", releaseNotesDoc, [
  "Release Notes Readiness Metadata",
  "`CHANGELOG.md` is project-owned and structurally checked.",
  "The Unreleased section carries the first publish (`0.1.0`) release notes.",
  "not derived from Git history",
  "release notes readiness",
  "after resolution",
]);

requireIncludes("docs/changelog-metadata.md", changelogDoc, [
  "validates changelog ownership and structure, not publish-ready release note completeness",
]);

requireIncludes("docs/release.md", releaseDoc, [
  "Release notes readiness checks are read-only",
  "project-owned changelog structure exists and its Unreleased section records first publish included scope, excluded scope, and known warnings",
  "they do not generate release notes, run git-cliff, derive changes from Git history, create tags, or publish releases",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:release-notes-readiness`",
  "project-owned changelog structure exists and its Unreleased section records first publish included scope, excluded scope, and known warnings",
  "does not generate release notes, run git-cliff, derive changes from Git history, create tags, or publish releases",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M99 Release Notes Readiness Metadata Gate Usage",
  "npm run verify:release-notes-readiness",
  "project-owned changelog structure exists and its Unreleased section records first publish included scope, excluded scope, and known warnings",
]);

if (failures.length > 0) {
  console.error("release notes readiness metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("release notes readiness metadata verification passed");
}
