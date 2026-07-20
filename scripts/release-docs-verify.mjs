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
const releaseGateMarker = "expanded gate set is:";
const releaseGateMarkerIndex = releaseDocs.indexOf(releaseGateMarker);
const releaseGateBlockMatch =
  releaseGateMarkerIndex >= 0
    ? releaseDocs.slice(releaseGateMarkerIndex).match(/```bash\n([\s\S]*?)\n```/)
    : null;
const documentedReleaseSegments =
  releaseGateBlockMatch?.[1]
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0) ?? [];

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
  {
    label: "release candidate handoff checklist",
    snippet: "docs/release-candidate-handoff-checklist.md",
  },
  {
    label: "handoff checklist boundary",
    snippet: "no package publishing",
  },
  {
    label: "release candidate handoff focused gate",
    snippet: "npm run verify:release-candidate-handoff",
  },
  {
    label: "release candidate handoff metadata boundary",
    snippet: "Release candidate handoff metadata checks are also read-only",
  },
];

const missingSnippets = requiredSnippets.filter(({ snippet }) => {
  return !releaseDocs.includes(snippet);
});
const missingReleaseSegments = releaseCommandSegments.filter((segment) => {
  return !releaseDocs.includes(segment);
});
const releaseGateOrderMatches =
  documentedReleaseSegments.length === releaseCommandSegments.length &&
  documentedReleaseSegments.every((segment, index) => {
    return segment === releaseCommandSegments[index];
  });

if (
  typeof releaseScript !== "string" ||
  releaseGateMarkerIndex < 0 ||
  releaseGateBlockMatch === null ||
  missingSnippets.length > 0 ||
  missingReleaseSegments.length > 0 ||
  !releaseGateOrderMatches
) {
  console.error("release documentation verification failed");
  if (typeof releaseScript !== "string") {
    console.error("- package.json is missing script: verify:release");
  }
  if (releaseGateMarkerIndex < 0) {
    console.error(`- docs/release.md is missing release gate marker: ${releaseGateMarker}`);
  }
  if (releaseGateMarkerIndex >= 0 && releaseGateBlockMatch === null) {
    console.error("- docs/release.md is missing expanded release gate bash block");
  }
  for (const { label, snippet } of missingSnippets) {
    console.error(`- missing ${label}: ${snippet}`);
  }
  for (const segment of missingReleaseSegments) {
    console.error(`- docs/release.md missing release command segment: ${segment}`);
  }
  if (releaseGateBlockMatch !== null && !releaseGateOrderMatches) {
    console.error("- docs/release.md expanded release gate block does not match verify:release order");
    console.error("  expected:");
    for (const segment of releaseCommandSegments) {
      console.error(`  - ${segment}`);
    }
    console.error("  actual:");
    for (const segment of documentedReleaseSegments) {
      console.error(`  - ${segment}`);
    }
  }
  process.exitCode = 1;
} else {
  console.log("release documentation verification passed");
}
