#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const packageJson = JSON.parse(readRepoFile("package.json"));
const smokeScript = readRepoFile("scripts/mobile-browser-smoke.mjs");
const readme = readRepoFile("README.md");
const releaseDocs = readRepoFile("docs/release.md");
const qualityGates = readRepoFile("docs/quality-gates.md");
const ciGuide = readRepoFile("docs/ci-browser-smoke.md");
const workflowTemplate = readRepoFile("docs/ci-browser-workflow-template.md");

const failures = [];

const requireFragment = (label, source, fragment) => {
  if (!source.includes(fragment)) {
    failures.push(`${label} missing fragment: ${fragment}`);
  }
};

const scripts = packageJson.scripts ?? {};
const expectedScript = "node scripts/mobile-browser-metadata-verify.mjs";
if (scripts["verify:mobile-browser"] !== "node scripts/mobile-browser-smoke.mjs") {
  failures.push("package.json missing verify:mobile-browser script");
}

if (scripts["verify:mobile-browser-metadata"] !== expectedScript) {
  failures.push("package.json missing verify:mobile-browser-metadata script");
}

if (!scripts["verify:release"]?.includes("npm run verify:mobile-browser-metadata")) {
  failures.push("package.json verify:release missing mobile browser metadata gate");
}

const smokeFragments = [
  'const host = "127.0.0.1";',
  "const port = 45237;",
  "const mobileViewport = { width: 390, height: 844 };",
  'const installHint = "npx playwright install chromium";',
  "DIOXUS_UI_BROWSER_EXECUTABLE",
  "DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT",
  "dioxus-ui-mobile-browser-preview-",
  "dx",
  "serve",
  "--web",
  "--package",
  "dioxus-ui-web-demo",
  "--bin",
  "preview",
  "devices[\"iPhone 12\"]",
  "data-preview-root=\"web\"",
  "data-preview-panel=\"mobile-profile\"",
  "data-mobile-profile=\"touch-targets\"",
  "data-mobile-profile=\"hover-alternative\"",
  "data-mobile-profile=\"safe-area-owned\"",
  "data-mobile-profile=\"reduced-motion\"",
  "data-mobile-profile=\"visible-status\"",
  "data-preview-panel=\"form\"",
  "data-preview-panel=\"message\"",
  "data-preview-panel=\"chart\"",
  "data-preview-panel=\"overlay-open\"",
  "mobile browser screenshot saved:",
  "mobile browser screenshot metadata:",
  "mobile browser smoke passed",
];

for (const fragment of smokeFragments) {
  requireFragment("scripts/mobile-browser-smoke.mjs", smokeScript, fragment);
}

const docs = [
  ["README.md", readme],
  ["docs/release.md", releaseDocs],
  ["docs/quality-gates.md", qualityGates],
  ["docs/ci-browser-smoke.md", ciGuide],
];

const requiredDocFragments = [
  "npm run verify:mobile-browser",
  "npx playwright install chromium",
  "DIOXUS_UI_BROWSER_EXECUTABLE",
  "DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT",
  "dioxus-ui-mobile-browser-preview-*.png",
  "390x844",
  "not part of the release gate",
];

for (const [label, source] of docs) {
  for (const fragment of requiredDocFragments) {
    requireFragment(label, source, fragment);
  }
}

const workflowFragments = [
  "npx playwright install chromium",
  "DIOXUS_UI_BROWSER_EXECUTABLE",
  "DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT",
  "npm run verify:mobile-browser",
  "dioxus-ui-mobile-browser-preview-*.png",
  "continue-on-error: true",
  "workflow_dispatch:",
];

for (const fragment of workflowFragments) {
  requireFragment("docs/ci-browser-workflow-template.md", workflowTemplate, fragment);
}

if (failures.length > 0) {
  console.error("mobile browser smoke metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("mobile browser smoke metadata verification passed");
}
