#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const sharedPreview = readRepoFile("examples/preview-states/src/lib.rs");
const webPreview = readRepoFile("examples/web-demo/src/bin/preview.rs");
const desktopPreview = readRepoFile("examples/desktop-demo/src/bin/preview.rs");
const webPreviewVerify = readRepoFile("scripts/web-preview-verify.mjs");
const desktopPreviewVerify = readRepoFile("scripts/desktop-preview-verify.mjs");
const webCss = readRepoFile("examples/web-demo/assets/preview.css");
const desktopCss = readRepoFile("examples/desktop-demo/assets/preview.css");
const packageJson = JSON.parse(readRepoFile("package.json"));

const requiredStateLabels = [
  "button group class",
  "input group class",
  "input otp helper",
  "attachment class",
  "bubble class",
  "message class",
  "message scroller helper",
  "marker class",
  "chart helper",
  "direction class/attr",
  "collapsible class",
];

const requiredPanels = [
  "overview",
  "actions",
  "mobile-profile",
  "form",
  "message",
  "chart",
  "overlay-open",
  "interactions",
  "inventory",
];

const requiredInteractionFragments = [
  'data-preview-panel": "interactions"',
  'data-interaction-root": "runtime"',
  'data-interaction-target": "disclosure"',
  'data-interaction-target": "overlay"',
  'data-interaction-target": "selection"',
  'data-interaction-target": "keyboard"',
  'data-interaction-target": "scroll-status"',
  'data-interaction-control": "disclosure-trigger"',
  'data-interaction-control": "overlay-trigger"',
  'data-interaction-control": "keyboard-listbox"',
  'data-interaction-control": "scroll-jump"',
];

const requiredCssFragments = [
  '@import "tailwindcss";',
  '@source "../../../crates";',
  '@source "../src";',
  '@source "../../preview-states/src";',
];

const failures = [];

const requireFragment = (label, source, fragment) => {
  if (!source.includes(fragment)) {
    failures.push(`${label} missing fragment: ${fragment}`);
  }
};

for (const target of ["Web", "Desktop"]) {
  requireFragment("shared preview targets", sharedPreview, `PreviewTarget::${target}`);
}

requireFragment("shared preview surface", sharedPreview, "pub fn PreviewSurface");
requireFragment("web preview binary", webPreview, "PreviewTarget::Web");
requireFragment("web preview binary", webPreview, "PreviewSurface");
requireFragment("desktop preview binary", desktopPreview, "PreviewTarget::Desktop");
requireFragment("desktop preview binary", desktopPreview, "PreviewSurface");

for (const state of requiredStateLabels) {
  requireFragment("shared preview state inventory", sharedPreview, `label: "${state}"`);
  requireFragment("web preview verifier state list", webPreviewVerify, `"${state}"`);
  requireFragment("desktop preview verifier state list", desktopPreviewVerify, `"${state}"`);
}

for (const panel of requiredPanels) {
  requireFragment("shared preview panels", sharedPreview, `"data-preview-panel": "${panel}"`);
}

for (const fragment of requiredInteractionFragments) {
  requireFragment("shared preview interaction fixtures", sharedPreview, fragment);
}

for (const fragment of requiredCssFragments) {
  requireFragment("web preview CSS", webCss, fragment);
  requireFragment("desktop preview CSS", desktopCss, fragment);
}

const scripts = packageJson.scripts ?? {};
const expectedScript = "node scripts/preview-state-metadata-verify.mjs";
if (scripts["verify:preview-state-metadata"] !== expectedScript) {
  failures.push("package.json missing verify:preview-state-metadata script");
}

if (!scripts["verify:release"]?.includes("npm run verify:preview-state-metadata")) {
  failures.push("package.json verify:release missing preview state metadata gate");
}

if (!scripts["verify:preview"]?.includes("npm run verify:web-preview")) {
  failures.push("package.json verify:preview missing web preview gate");
}

if (!scripts["verify:preview"]?.includes("npm run verify:desktop-preview")) {
  failures.push("package.json verify:preview missing desktop preview gate");
}

if (failures.length > 0) {
  console.error("preview state metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log(
    `preview state metadata verification passed (${requiredStateLabels.length} states, ${requiredPanels.length} panels)`,
  );
}
