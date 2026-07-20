#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const packageJson = JSON.parse(readRepoFile("package.json"));
const ciGuide = readRepoFile("docs/ci-browser-smoke.md");
const workflowTemplate = readRepoFile("docs/ci-browser-workflow-template.md");
const rfc = readRepoFile("docs/rfcs/0009-ci-browser-workflow-activation.md");
const gitignore = readRepoFile(".gitignore");
const repoHygiene = readRepoFile("scripts/repo-hygiene-verify.mjs");
const policyDoc = readRepoFile("docs/browser-artifact-policy-metadata.md");
const activeWorkflowPath = join(repoRoot, ".github/workflows/browser-smoke.yml");
const failures = [];

const requireFragment = (label, source, fragment) => {
  if (!source.includes(fragment)) {
    failures.push(`${label} missing fragment: ${fragment}`);
  }
};

const requireAbsentFragment = (label, source, fragment) => {
  if (source.includes(fragment)) {
    failures.push(`${label} must not include fragment: ${fragment}`);
  }
};

const scripts = packageJson.scripts ?? {};
if (scripts["verify:browser-artifact-policy"] !== "node scripts/browser-artifact-policy-verify.mjs") {
  failures.push("package.json missing verify:browser-artifact-policy script");
}

if (!scripts["verify:release"]?.includes("npm run verify:browser-artifact-policy")) {
  failures.push("package.json verify:release missing browser artifact policy gate");
}

if (existsSync(activeWorkflowPath)) {
  failures.push(".github/workflows/browser-smoke.yml must not be committed yet");
}

const screenshotPattern = "dioxus-ui-mobile-browser-preview-*.png";
const webScreenshotPattern = "dioxus-ui-web-preview-*.png";
const forbiddenArtifactTerms = [
  "browser profiles",
  "Playwright caches",
  "target directories",
  "temporary preview",
];

const ciGuideFragments = [
  "## Artifacts",
  screenshotPattern,
  "upload those PNG files as job artifacts",
  "not commit them.",
  "Browser profiles, Playwright caches, screenshots, and temporary preview outputs",
  "must remain outside Git",
];

for (const fragment of ciGuideFragments) {
  requireFragment("docs/ci-browser-smoke.md", ciGuide, fragment);
}

const templateFragments = [
  "actions/upload-artifact@v4",
  "name: mobile-browser-screenshots",
  `path: ${screenshotPattern}`,
  "if-no-files-found: ignore",
];

for (const fragment of templateFragments) {
  requireFragment("docs/ci-browser-workflow-template.md", workflowTemplate, fragment);
}

const broadUploadFragments = [
  "path: .",
  "path: ./",
  "path: **/*",
  "path: target/",
  "path: .playwright-mcp/",
  "path: node_modules/",
];

for (const fragment of broadUploadFragments) {
  requireAbsentFragment("docs/ci-browser-workflow-template.md", workflowTemplate, fragment);
}

const rfcFragments = [
  "## Artifact Policy",
  "Upload only screenshot PNG files matching:",
  screenshotPattern,
  "Do not upload browser profiles, full Playwright caches, target directories, or",
  "temporary preview server output",
  "Recommended retention starts short, such as 7 days",
];

for (const fragment of rfcFragments) {
  requireFragment("docs/rfcs/0009-ci-browser-workflow-activation.md", rfc, fragment);
}

const gitignoreFragments = [
  ".playwright-mcp/",
  webScreenshotPattern,
  "dioxus-ui-desktop-webview-preview-*.png",
  screenshotPattern,
];

for (const fragment of gitignoreFragments) {
  requireFragment(".gitignore", gitignore, fragment);
}

const repoHygieneFragments = [
  'file === ".github/workflows/browser-smoke.yml"',
  'path: ".github/workflows/browser-smoke.yml"',
  '/^dioxus-ui-web-preview-.*\\.png$/.test(file)',
  '/^dioxus-ui-mobile-browser-preview-.*\\.png$/.test(file)',
  'file.startsWith("target/")',
  '/^dxui-generated-fixture[./-]/.test(file)',
];

for (const fragment of repoHygieneFragments) {
  requireFragment("scripts/repo-hygiene-verify.mjs", repoHygiene, fragment);
}

const policyDocFragments = [
  "normal browser smoke uploads are screenshot PNG files only",
  screenshotPattern,
  webScreenshotPattern,
  "Web screenshot smoke writes screenshots only when",
  "DIOXUS_UI_WEB_SCREENSHOT=1",
  "runtime interaction verification writes no screenshots or traces by default",
  "browser profiles, Playwright caches, Rust target directories, and temporary",
  "preview server output stay outside the normal upload path",
  "tracing runtime interaction verification",
  "enforcing remote artifact retention settings",
  "validating screenshot pixels or PNG metadata",
];

for (const fragment of policyDocFragments) {
  requireFragment("docs/browser-artifact-policy-metadata.md", policyDoc, fragment);
}

for (const term of forbiddenArtifactTerms) {
  requireFragment("docs/browser-artifact-policy-metadata.md", policyDoc.replace(/\s+/g, " "), term);
}

if (failures.length > 0) {
  console.error("browser artifact policy metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("browser artifact policy metadata verification passed");
}
