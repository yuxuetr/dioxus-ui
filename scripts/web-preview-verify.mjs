#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { execFileSync } from "node:child_process";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, "..");

const previewSource = readFileSync(
  join(repoRoot, "examples/web-demo/src/bin/preview.rs"),
  "utf8",
);
const previewSharedSource = readFileSync(
  join(repoRoot, "examples/preview-states/src/lib.rs"),
  "utf8",
);
const previewCss = readFileSync(
  join(repoRoot, "examples/web-demo/assets/preview.css"),
  "utf8",
);

const requiredSourceFragments = [
  "data-preview-root",
  "data-preview-panel",
  "overview",
  "actions",
  "form",
  "message",
  "chart",
  "overlay-open",
  "inventory",
  "data-preview-state",
  "PreviewSurface",
  "PreviewTarget::Web",
];

const missingSourceFragments = requiredSourceFragments.filter((fragment) => {
  return !previewSource.includes(fragment) && !previewSharedSource.includes(fragment);
});

if (missingSourceFragments.length > 0) {
  console.error("web preview source is missing screenshot target fragments:");
  for (const fragment of missingSourceFragments) {
    console.error(`- ${fragment}`);
  }
  process.exit(1);
}

const requiredCssFragments = [
  '@import "tailwindcss";',
  "@source \"../../../crates\";",
  "@source \"../src\";",
  "@source \"../../preview-states/src\";",
];

const missingCssFragments = requiredCssFragments.filter((fragment) => {
  return !previewCss.includes(fragment);
});

if (missingCssFragments.length > 0) {
  console.error("web preview CSS is missing Tailwind CSS v4 source fragments:");
  for (const fragment of missingCssFragments) {
    console.error(`- ${fragment}`);
  }
  process.exit(1);
}

execFileSync(
  "cargo",
  ["check", "-q", "-p", "dioxus-ui-web-demo", "--bin", "preview"],
  {
    cwd: repoRoot,
    stdio: "inherit",
  },
);

const output = execFileSync(
  "cargo",
  ["run", "-q", "-p", "dioxus-ui-web-demo"],
  {
    cwd: repoRoot,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  },
);

const requiredStates = [
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

const missingStates = requiredStates.filter((state) => {
  return !output.includes(`dioxus-ui web demo ${state}`);
});

if (missingStates.length > 0) {
  console.error("web preview inventory is missing representative states:");
  for (const state of missingStates) {
    console.error(`- ${state}`);
  }
  process.exit(1);
}

console.log("web preview screenshot prerequisites passed");
