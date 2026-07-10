#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const releaseDocsPath = join(repoRoot, "docs/release.md");
const packageJsonPath = join(repoRoot, "package.json");
const releaseDocs = readFileSync(releaseDocsPath, "utf8");
const packageJson = JSON.parse(readFileSync(packageJsonPath, "utf8"));

const releaseScript = packageJson.scripts?.["verify:release"];
const releaseCommandSegments =
  typeof releaseScript === "string"
    ? releaseScript.split(/\s*&&\s*/).filter((segment) => segment.length > 0)
    : [];

const requiredSnippets = [
  {
    label: "release aggregate verification alias",
    snippet: "npm run verify:release",
  },
  {
    label: "local aggregate verification alias",
    snippet: "npm run verify",
  },
  {
    label: "Rust workspace check release gate",
    snippet: "cargo check --workspace --all-features",
  },
  {
    label: "Rust workspace test release gate",
    snippet: "cargo test --workspace --all-features",
  },
  {
    label: "source-copy fixture release gate",
    snippet: "scripts/generated-fixture-smoke.sh",
  },
  {
    label: "component feature release gate",
    snippet: "scripts/feature-check.sh",
  },
  {
    label: "opt-in browser smoke command",
    snippet: "npm run verify:mobile-browser",
  },
  {
    label: "browser smoke opt-in boundary",
    snippet: "not part of the release gate",
  },
];

const missingSnippets = requiredSnippets.filter(({ snippet }) => {
  return !releaseDocs.includes(snippet);
});
const missingReleaseSegments = releaseCommandSegments.filter((segment) => {
  return !releaseDocs.includes(segment);
});

if (
  typeof releaseScript !== "string" ||
  missingSnippets.length > 0 ||
  missingReleaseSegments.length > 0
) {
  console.error("release documentation verification failed");
  if (typeof releaseScript !== "string") {
    console.error("- package.json is missing script: verify:release");
  }
  for (const { label, snippet } of missingSnippets) {
    console.error(`- missing ${label}: ${snippet}`);
  }
  for (const segment of missingReleaseSegments) {
    console.error(`- docs/release.md missing release command segment: ${segment}`);
  }
  process.exitCode = 1;
} else {
  console.log("release documentation verification passed");
}
