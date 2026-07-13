#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const readme = readRepoFile("README.md");
const packageJson = JSON.parse(readRepoFile("package.json"));
const scripts = packageJson.scripts ?? {};
const docsScript = scripts["verify:docs"];
const docsCommandSegments =
  typeof docsScript === "string"
    ? docsScript.split(/\s*&&\s*/).filter((segment) => segment.length > 0)
    : [];

const requiredSnippets = [
  {
    label: "verification shortcut section",
    snippet: "## Verification Shortcuts",
  },
  {
    label: "default local gate",
    snippet: "npm run verify",
  },
  {
    label: "docs aggregate gate",
    snippet: "npm run verify:docs",
  },
  {
    label: "smoke aggregate gate",
    snippet: "npm run verify:smoke",
  },
  {
    label: "release aggregate gate",
    snippet: "npm run verify:release",
  },
  {
    label: "package script focused gate",
    snippet: "npm run verify:package-scripts",
  },
  {
    label: "release docs focused gate",
    snippet: "npm run verify:release-docs",
  },
  {
    label: "Cargo workspace focused gate",
    snippet: "npm run verify:cargo-workspace",
  },
  {
    label: "Cargo lock focused gate",
    snippet: "npm run verify:cargo-lock",
  },
  {
    label: "pre-commit focused gate",
    snippet: "npm run verify:pre-commit",
  },
  {
    label: "script metadata focused gate",
    snippet: "npm run verify:scripts",
  },
  {
    label: "repository hygiene focused gate",
    snippet: "npm run verify:repo-hygiene",
  },
  {
    label: "README verification gate",
    snippet: "npm run verify:readme",
  },
  {
    label: "release summary Cargo workspace metadata",
    snippet: "Cargo workspace metadata",
  },
  {
    label: "release summary Cargo lock metadata",
    snippet: "Cargo lock metadata",
  },
  {
    label: "release summary pre-commit metadata",
    snippet: "pre-commit metadata",
  },
  {
    label: "release summary script metadata",
    snippet: "script metadata",
  },
  {
    label: "release summary repository hygiene",
    snippet: "repository hygiene",
  },
];

const failures = [];

if (typeof docsScript !== "string") {
  failures.push("package.json is missing script: verify:docs");
}

for (const { label, snippet } of requiredSnippets) {
  if (!readme.includes(snippet)) {
    failures.push(`README.md missing ${label}: ${snippet}`);
  }
}

for (const segment of docsCommandSegments) {
  if (!readme.includes(segment)) {
    failures.push(`README.md missing verify:docs command segment: ${segment}`);
  }
}

if (failures.length > 0) {
  console.error("README verification summary failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("README verification summary passed");
}
