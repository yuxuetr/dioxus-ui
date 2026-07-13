#!/usr/bin/env node
import { existsSync } from "node:fs";
import { join } from "node:path";
import { execFileSync } from "node:child_process";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const trackedFiles = execFileSync("git", ["ls-files"], {
  cwd: repoRoot,
  encoding: "utf8",
})
  .split("\n")
  .filter(Boolean);

const forbiddenTrackedPatterns = [
  {
    label: "Node dependency directory",
    test: (file) => {
      return file.startsWith("node_modules/");
    },
  },
  {
    label: "Rust target directory",
    test: (file) => {
      return file.startsWith("target/");
    },
  },
  {
    label: "distribution output directory",
    test: (file) => {
      return file.startsWith("dist/") || file.startsWith("build/");
    },
  },
  {
    label: "inactive browser smoke workflow",
    test: (file) => {
      return file === ".github/workflows/browser-smoke.yml";
    },
  },
  {
    label: "mobile browser screenshot artifact",
    test: (file) => {
      return /^dioxus-ui-mobile-browser-preview-.*\.png$/.test(file);
    },
  },
  {
    label: "repository-local generated fixture artifact",
    test: (file) => {
      return /^dxui-generated-fixture[./-]/.test(file);
    },
  },
];

const forbiddenExistingPaths = [
  {
    label: "inactive browser smoke workflow",
    path: ".github/workflows/browser-smoke.yml",
  },
];

const trackedViolations = [];
const existingViolations = [];

for (const file of trackedFiles) {
  for (const { label, test } of forbiddenTrackedPatterns) {
    if (test(file)) {
      trackedViolations.push({ label, file });
    }
  }
}

for (const { label, path } of forbiddenExistingPaths) {
  if (existsSync(join(repoRoot, path))) {
    existingViolations.push({ label, path });
  }
}

if (trackedViolations.length > 0 || existingViolations.length > 0) {
  console.error("repository hygiene verification failed");

  for (const { label, file } of trackedViolations) {
    console.error(`- tracked ${label}: ${file}`);
  }

  for (const { label, path } of existingViolations) {
    console.error(`- present ${label}: ${path}`);
  }

  process.exitCode = 1;
} else {
  console.log(`repository hygiene verification passed (${trackedFiles.length} tracked files)`);
}
