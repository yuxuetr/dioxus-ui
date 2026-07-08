#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const releaseDocsPath = join(repoRoot, "docs/release.md");
const releaseDocs = readFileSync(releaseDocsPath, "utf8");

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

if (missingSnippets.length > 0) {
  console.error("release documentation verification failed");
  for (const { label, snippet } of missingSnippets) {
    console.error(`- missing ${label}: ${snippet}`);
  }
  process.exitCode = 1;
} else {
  console.log("release documentation verification passed");
}
