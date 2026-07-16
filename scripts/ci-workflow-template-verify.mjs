#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const packageJson = JSON.parse(readRepoFile("package.json"));
const workflowTemplate = readRepoFile("docs/ci-browser-workflow-template.md");
const ciGuide = readRepoFile("docs/ci-browser-smoke.md");
const rfc = readRepoFile("docs/rfcs/0009-ci-browser-workflow-activation.md");
const activeWorkflowPath = join(repoRoot, ".github/workflows/browser-smoke.yml");
const failures = [];

const requireFragment = (label, source, fragment) => {
  if (!source.includes(fragment)) {
    failures.push(`${label} missing fragment: ${fragment}`);
  }
};

const scripts = packageJson.scripts ?? {};
if (scripts["verify:ci-workflow-template"] !== "node scripts/ci-workflow-template-verify.mjs") {
  failures.push("package.json missing verify:ci-workflow-template script");
}

if (!scripts["verify:release"]?.includes("npm run verify:ci-workflow-template")) {
  failures.push("package.json verify:release missing CI workflow template gate");
}

if (existsSync(activeWorkflowPath)) {
  failures.push(".github/workflows/browser-smoke.yml must not be committed yet");
}

const templateFragments = [
  "workflow_dispatch:",
  "continue-on-error: true",
  "permissions:",
  "contents: read",
  "timeout-minutes: 30",
  "npm ci",
  "cargo install dioxus-cli --locked",
  "run: npm run verify",
  "cargo test --workspace --all-features -q",
  "npx playwright install chromium",
  "DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT",
  "run: npm run verify:mobile-browser",
  "actions/upload-artifact@v4",
  "mobile-browser-screenshots",
  "dioxus-ui-mobile-browser-preview-*.png",
  "if-no-files-found: ignore",
  "DIOXUS_UI_BROWSER_EXECUTABLE",
];

for (const fragment of templateFragments) {
  requireFragment("docs/ci-browser-workflow-template.md", workflowTemplate, fragment);
}

const guideFragments = [
  "docs/ci-browser-workflow-template.md",
  "RFC 0009",
  "manual or non-blocking CI job",
  "npm run verify:release",
  "npx playwright install chromium",
  "DIOXUS_UI_BROWSER_EXECUTABLE",
  "dioxus-ui-mobile-browser-preview-*.png",
  "not part of the release gate",
];

for (const fragment of guideFragments) {
  requireFragment("docs/ci-browser-smoke.md", ciGuide, fragment);
}

const rfcFragments = [
  "Phase 0: Documentation Only",
  "Phase 1: Manual Non-blocking Workflow",
  "workflow_dispatch",
  "continue-on-error: true",
  "permissions: `contents: read`",
  "timeout: 30 minutes",
  "artifact upload: screenshot PNG files only",
  "Playwright-managed Chromium first",
  "Required Merge Gate",
  "deterministic Rust and structural preview gates remain the required checks",
  "dioxus-ui-mobile-browser-preview-*.png",
];

for (const fragment of rfcFragments) {
  requireFragment("docs/rfcs/0009-ci-browser-workflow-activation.md", rfc, fragment);
}

if (failures.length > 0) {
  console.error("CI workflow template metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("CI workflow template metadata verification passed");
}
