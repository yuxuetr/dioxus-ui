#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readText = (relativePath) => readFileSync(join(repoRoot, relativePath), "utf8");
const normalizeWhitespace = (text) => text.replace(/\s+/g, " ");

const rootCargo = readText("Cargo.toml");
const packageJson = JSON.parse(readText("package.json"));
const workspaceDoc = normalizeWhitespace(readText("docs/workspace.md"));
const publishBlockers = readText("docs/publish-readiness-blockers.md");
const repositoryDoc = readText("docs/repository-identity-readiness-metadata.md");
const preparationPlan = readText("docs/repository-identity-decision-preparation-plan.md");
const decisionTemplate = readText("docs/repository-identity-decision-record-template.md");
const followUpMap = readText("docs/repository-identity-local-follow-up-map.md");
const cargoPublishDoc = normalizeWhitespace(readText("docs/cargo-publish-metadata.md"));
const releaseDoc = normalizeWhitespace(readText("docs/release.md"));
const qualityDoc = normalizeWhitespace(readText("docs/quality-gates.md"));
const readme = normalizeWhitespace(readText("README.md"));
const docsIndex = readText("docs/README.md");
const siteDoc = normalizeWhitespace(readText("docs/site.md"));
const scripts = packageJson.scripts ?? {};
const failures = [];

const approvedRepository = "https://github.com/yuxuetr/dioxus-ui";

const requireIncludes = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (!text.includes(fragment)) {
      failures.push(`${name} is missing: ${fragment}`);
    }
  }
};

if (scripts["verify:repository-identity-readiness"] !== "node scripts/repository-identity-readiness-verify.mjs") {
  failures.push("package.json verify:repository-identity-readiness script is missing or mismatched");
}

if (!scripts["verify:release"]?.includes("npm run verify:repository-identity-readiness")) {
  failures.push("package.json verify:release must include npm run verify:repository-identity-readiness");
}

requireIncludes("Cargo.toml", rootCargo, [
  `repository = "${approvedRepository}"`,
]);

requireIncludes("docs/repository-identity-readiness-metadata.md", repositoryDoc, [
  "Repository Identity Readiness Metadata",
  approvedRepository,
  "The repository value is the approved canonical URL.",
  "This gate must not change the URL automatically.",
  "Repository Identity Decision Preparation Plan",
  "Repository Identity Decision Record Template",
  "Repository Identity Local Follow-up Map",
  "after resolution",
]);

requireIncludes("docs/repository-identity-decision-preparation-plan.md", preparationPlan, [
  "Repository Identity Decision Preparation Plan",
  "canonical repository owner and URL",
  "remote availability evidence",
  "repository identity local follow-up map",
  "M129 must not",
  "replace the placeholder repository URL",
]);

requireIncludes("docs/repository-identity-decision-record-template.md", decisionTemplate, [
  "Repository Identity Decision Record Template",
  "Final repository owner",
  "Canonical repository URL",
  "Remote availability confirmed by",
  "Workspace metadata update approved",
  "Decision state: `approved` / `blocked` / `deferred`",
]);

requireIncludes("docs/repository-identity-local-follow-up-map.md", followUpMap, [
  "Repository Identity Local Follow-up Map",
  "Move repository identity out of current blockers only after the approved URL",
  "Keep the placeholder URL",
  "Workspace repository metadata",
  "does not apply repository URL changes",
]);

requireIncludes("docs/publish-readiness-blockers.md", publishBlockers, [
  "Placeholder repository URL",
  approvedRepository,
  "npm run verify:repository-identity-readiness",
]);

requireIncludes("docs/workspace.md", workspaceDoc, [
  "The repository URL has been approved for first publish preparation.",
  approvedRepository,
  "Do not change it as a side effect of metadata verification",
]);

requireIncludes("docs/cargo-publish-metadata.md", cargoPublishDoc, [
  `repository URL is approved as \`${approvedRepository}\``,
]);

requireIncludes("docs/release.md", releaseDoc, [
  "Repository identity readiness checks are read-only",
  "approved repository URL remains in workspace metadata",
  "they do not choose a different repository owner, change repository metadata, check crates.io availability, run `cargo package`, or run `cargo publish`",
  "Repository Identity Decision Preparation Plan",
  "Repository Identity Decision Record Template",
  "Repository Identity Local Follow-up Map",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:repository-identity-readiness`",
  "approved repository URL remains in workspace metadata",
  "does not choose a different repository owner, change repository metadata, check crates.io availability, run `cargo package`, or run `cargo publish`",
  "Repository Identity Decision Preparation Plan",
  "Repository Identity Decision Record Template",
  "Repository Identity Local Follow-up Map",
]);

requireIncludes("README.md", readme, [
  "approved repository URL remains in workspace metadata",
  "Repository Identity Decision Preparation Plan",
  "Repository Identity Decision Record Template",
  "Repository Identity Local Follow-up Map",
]);

requireIncludes("docs/README.md", docsIndex, [
  "Repository Identity Decision Preparation Plan",
  "Repository Identity Decision Record Template",
  "Repository Identity Local Follow-up Map",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M101 Repository Identity Readiness Metadata Gate Usage",
  "M129 Repository Identity Decision Preparation",
  "npm run verify:repository-identity-readiness",
  "approved repository URL remains in workspace metadata",
]);

if (failures.length > 0) {
  console.error("repository identity readiness metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("repository identity readiness metadata verification passed");
}
