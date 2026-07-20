#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const packageJson = JSON.parse(readRepoFile("package.json"));
const qualityGates = readRepoFile("docs/quality-gates.md");
const releaseScript = packageJson.scripts?.["verify:release"];
const releaseCommandSegments =
  typeof releaseScript === "string"
    ? releaseScript.split(/\s*&&\s*/).filter((segment) => segment.length > 0)
    : [];
const qualityGateMarker = "The release aggregate expands to the required local release gates:";
const qualityGateMarkerIndex = qualityGates.indexOf(qualityGateMarker);
const qualityGateBlockMatch =
  qualityGateMarkerIndex >= 0
    ? qualityGates.slice(qualityGateMarkerIndex).match(/```bash\n([\s\S]*?)\n```/)
    : null;
const documentedQualityGateSegments =
  qualityGateBlockMatch?.[1]
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0) ?? [];

const requiredLinks = [
  {
    file: "README.md",
    content: readRepoFile("README.md"),
    links: [
      "docs/design.md",
      "docs/roadmap.md",
      "docs/workspace.md",
      "docs/component-api.md",
      "docs/release.md",
      "docs/quality-gates.md",
      "docs/ci-browser-smoke.md",
      "docs/ci-browser-workflow-template.md",
      "docs/components/README.md",
      "docs/site.md",
      "TODOs.md",
      "docs/rfcs/0001-project-architecture.md",
      "docs/rfcs/0002-cli-registry-and-code-generation.md",
      "docs/rfcs/0003-tailwind-styling-contract.md",
      "docs/rfcs/0009-ci-browser-workflow-activation.md",
    ],
  },
  {
    file: "docs/README.md",
    content: readRepoFile("docs/README.md"),
    links: [
      "design.md",
      "roadmap.md",
      "workspace.md",
      "component-api.md",
      "release.md",
      "quality-gates.md",
      "components/README.md",
      "site.md",
      "components/runtime-adapters.md",
      "components/runtime-renderer-verification.md",
      "components/runtime-web-verification-harness.md",
      "components/runtime-desktop-mobile-verification.md",
      "components/runtime-implementation-milestones.md",
      "components/runtime-web-adapter-boundaries.md",
      "components/release-screenshot-review-checklist.md",
      "components/screenshot-artifact-retention.md",
      "components/release-screenshot-review-notes-template.md",
      "ci-browser-smoke.md",
      "ci-browser-workflow-template.md",
      "rfcs/0001-project-architecture.md",
      "rfcs/0002-cli-registry-and-code-generation.md",
      "rfcs/0003-tailwind-styling-contract.md",
      "rfcs/0004-benchmark-and-css-output.md",
      "rfcs/0005-modules-and-platform-profiles.md",
      "rfcs/0006-focus-and-portal-primitives.md",
      "rfcs/0007-keyboard-navigation-primitives.md",
      "rfcs/0008-overlay-positioning-and-portals.md",
      "rfcs/0009-ci-browser-workflow-activation.md",
      "../TODOs.md",
    ],
  },
];

const missingLinks = [];
const missingQualityGateAliases = [];
const qualityGateBlockFailures = [];

for (const { file, content, links } of requiredLinks) {
  for (const link of links) {
    if (!content.includes(`](${link})`)) {
      missingLinks.push({ file, link });
    }
  }
}

const verifyAliases = Object.keys(packageJson.scripts ?? {})
  .filter((name) => name === "verify" || name.startsWith("verify:"))
  .sort();

for (const alias of verifyAliases) {
  const expectedSnippet = `npm run ${alias}`;
  if (!qualityGates.includes(expectedSnippet)) {
    missingQualityGateAliases.push(expectedSnippet);
  }
}

if (typeof releaseScript !== "string") {
  qualityGateBlockFailures.push("package.json is missing script: verify:release");
}

if (qualityGateMarkerIndex < 0) {
  qualityGateBlockFailures.push(`docs/quality-gates.md is missing release gate marker: ${qualityGateMarker}`);
}

if (qualityGateMarkerIndex >= 0 && qualityGateBlockMatch === null) {
  qualityGateBlockFailures.push("docs/quality-gates.md is missing release gate bash block");
}

const qualityGateBlockMatches =
  documentedQualityGateSegments.length === releaseCommandSegments.length &&
  documentedQualityGateSegments.every((segment, index) => {
    return segment === releaseCommandSegments[index];
  });

if (qualityGateBlockMatch !== null && !qualityGateBlockMatches) {
  qualityGateBlockFailures.push(
    "docs/quality-gates.md release gate block does not match verify:release order",
  );
}

if (missingLinks.length > 0 || missingQualityGateAliases.length > 0 || qualityGateBlockFailures.length > 0) {
  console.error("documentation index verification failed");
  for (const { file, link } of missingLinks) {
    console.error(`- ${file} missing link: ${link}`);
  }
  for (const snippet of missingQualityGateAliases) {
    console.error(`- docs/quality-gates.md missing verification alias: ${snippet}`);
  }
  for (const failure of qualityGateBlockFailures) {
    console.error(`- ${failure}`);
  }
  if (qualityGateBlockMatch !== null && !qualityGateBlockMatches) {
    console.error("  expected:");
    for (const segment of releaseCommandSegments) {
      console.error(`  - ${segment}`);
    }
    console.error("  actual:");
    for (const segment of documentedQualityGateSegments) {
      console.error(`  - ${segment}`);
    }
  }
  process.exitCode = 1;
} else {
  console.log("documentation index verification passed");
}
