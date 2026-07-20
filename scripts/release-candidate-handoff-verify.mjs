#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const packageJson = JSON.parse(readRepoFile("package.json"));
const scripts = packageJson.scripts ?? {};
const checklist = readRepoFile("docs/release-candidate-handoff-checklist.md");
const metadata = readRepoFile("docs/release-candidate-handoff-metadata.md");
const readme = readRepoFile("README.md");
const docsIndex = readRepoFile("docs/README.md");
const releaseDocs = readRepoFile("docs/release.md");
const qualityGates = readRepoFile("docs/quality-gates.md");
const siteDocs = readRepoFile("docs/site.md");
const triageRunbook = readRepoFile("docs/release-gate-failure-triage-runbook.md");
const browserReviewRunbook = readRepoFile("docs/components/release-candidate-browser-review-runbook.md");
const publishReadinessRunbook = readRepoFile("docs/publish-readiness-resolution-runbook.md");
const failures = [];

const requireFragment = (label, source, fragment) => {
  if (!source.includes(fragment)) {
    failures.push(`${label} missing fragment: ${fragment}`);
  }
};

const handoffPath = "docs/release-candidate-handoff-checklist.md";
const handoffRelativePath = "release-candidate-handoff-checklist.md";
const handoffMetadataPath = "docs/release-candidate-handoff-metadata.md";

if (
  scripts["verify:release-candidate-handoff"] !==
  "node scripts/release-candidate-handoff-verify.mjs"
) {
  failures.push("package.json missing verify:release-candidate-handoff script");
}

if (!scripts["verify:release"]?.includes("npm run verify:release-candidate-handoff")) {
  failures.push("package.json verify:release missing release candidate handoff gate");
}

const checklistFragments = [
  "Release Candidate Handoff Checklist",
  "Related Documents",
  "Candidate Identity",
  "Required Gate Evidence",
  "Optional Browser Review Evidence",
  "Publish Readiness Evidence",
  "Warning Inventory Evidence",
  "Artifact And Repository Hygiene",
  "Handoff Decision",
  "Non-goals",
  "npm run verify:release",
  "npm run verify:release-docs",
  "npm run verify:package-scripts",
  "npm run verify:package-lock",
  "npm run verify:browser-artifact-policy",
  "npm run verify:repo-hygiene",
  "git diff --check",
  "git status --short",
  "components/release-candidate-browser-review-runbook.md",
  "components/release-screenshot-review-notes-template.md",
  "components/screenshot-artifact-retention.md",
  "publish-readiness-blockers.md",
  "publish-readiness-resolution-runbook.md",
  "release-warning-inventory-metadata.md",
  "no screenshot PNG files staged or committed",
  "no trace files staged or committed",
  "no generated docs output staged or committed",
  "no release artifacts staged or committed",
  "no CI workflow activation staged or committed",
  "no Git tag created by this checklist",
  "no package publishing",
  "no `cargo publish`",
  "no Git tag creation",
  "no CI workflow activation",
  "no browser smoke promotion into default or release gates",
];

for (const fragment of checklistFragments) {
  requireFragment("docs/release-candidate-handoff-checklist.md", checklist, fragment);
}

const metadataFragments = [
  "Release Candidate Handoff Metadata",
  "handoff checklist sections and required evidence fields",
  "release gate command references",
  "optional browser review runbook references",
  "publish readiness blocker references",
  "release warning inventory references",
  "repository hygiene and artifact boundary references",
  "README.md",
  "docs/README.md",
  "docs/release.md",
  "docs/quality-gates.md",
  "docs/site.md",
  handoffPath,
  "docs/release-gate-failure-triage-runbook.md",
  "docs/components/release-candidate-browser-review-runbook.md",
  "docs/publish-readiness-resolution-runbook.md",
  "release wiring",
  "run `npm run verify:release`",
  "execute release aggregate command segments",
  "launch browser automation",
  "capture screenshots",
  "create Git tags",
  "run `cargo publish`",
  "activate CI workflows",
  "npm run verify:release-candidate-handoff",
];

for (const fragment of metadataFragments) {
  requireFragment("docs/release-candidate-handoff-metadata.md", metadata, fragment);
}

const discoverabilityTargets = [
  {
    label: "README.md",
    source: readme,
    fragment: handoffPath,
  },
  {
    label: "docs/README.md",
    source: docsIndex,
    fragment: handoffRelativePath,
  },
  {
    label: "docs/release.md",
    source: releaseDocs,
    fragment: handoffPath,
  },
  {
    label: "docs/quality-gates.md",
    source: qualityGates,
    fragment: handoffPath,
  },
  {
    label: "docs/site.md",
    source: siteDocs,
    fragment: handoffPath,
  },
  {
    label: "docs/release-gate-failure-triage-runbook.md",
    source: triageRunbook,
    fragment: "release-candidate-handoff-checklist.md",
  },
  {
    label: "docs/release-gate-failure-triage-runbook.md",
    source: triageRunbook,
    fragment: "npm run verify:release",
  },
  {
    label: "docs/components/release-candidate-browser-review-runbook.md",
    source: browserReviewRunbook,
    fragment: "../release-candidate-handoff-checklist.md",
  },
  {
    label: "docs/publish-readiness-resolution-runbook.md",
    source: publishReadinessRunbook,
    fragment: handoffRelativePath,
  },
];

for (const { label, source, fragment } of discoverabilityTargets) {
  requireFragment(label, source, fragment);
}

if (failures.length > 0) {
  console.error("release candidate handoff metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("release candidate handoff metadata verification passed");
}
