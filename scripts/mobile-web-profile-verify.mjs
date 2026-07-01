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
const webScreenshotDoc = readFileSync(
  join(repoRoot, "docs/components/web-preview-screenshot-verification.md"),
  "utf8",
);
const mobilePlanDoc = readFileSync(
  join(repoRoot, "docs/components/mobile-web-profile-verification.md"),
  "utf8",
);
const mobileChecklistDoc = readFileSync(
  join(repoRoot, "docs/components/runtime-mobile-verification-checklist.md"),
  "utf8",
);

const requiredSourceFragments = [
  "PreviewSurface",
  "PreviewTarget::Web",
  'data-preview-panel": "mobile-profile"',
  'data-mobile-profile": "touch-targets"',
  'data-mobile-profile": "hover-alternative"',
  'data-mobile-profile": "safe-area-owned"',
  'data-mobile-profile": "reduced-motion"',
  'data-mobile-profile": "visible-status"',
];

const missingSourceFragments = requiredSourceFragments.filter((fragment) => {
  return !previewSource.includes(fragment) && !previewSharedSource.includes(fragment);
});

if (missingSourceFragments.length > 0) {
  console.error("mobile web profile source is missing required fragments:");
  for (const fragment of missingSourceFragments) {
    console.error(`- ${fragment}`);
  }
  process.exit(1);
}

const requiredDocFragments = [
  {
    name: "web screenshot mobile viewport",
    doc: webScreenshotDoc,
    fragments: ["mobile: `390x844`"],
  },
  {
    name: "mobile web profile boundary",
    doc: mobilePlanDoc,
    fragments: [
      "not the same as native Mobile support",
      "node scripts/mobile-web-profile-verify.mjs",
      "no native iOS or Android automation",
    ],
  },
  {
    name: "mobile checklist deferred automation",
    doc: mobileChecklistDoc,
    fragments: [
      "Emulator-backed",
      "Deferred",
      "Do not add a workspace package until the target command",
    ],
  },
];

const missingDocFragments = [];
for (const entry of requiredDocFragments) {
  for (const fragment of entry.fragments) {
    if (!entry.doc.includes(fragment)) {
      missingDocFragments.push(`${entry.name}: ${fragment}`);
    }
  }
}

if (missingDocFragments.length > 0) {
  console.error("mobile web profile docs are missing required fragments:");
  for (const fragment of missingDocFragments) {
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

console.log("mobile web profile structural gate passed");
