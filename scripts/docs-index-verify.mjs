#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

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
      "docs/ci-browser-smoke.md",
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
      "rfcs/0001-project-architecture.md",
      "rfcs/0002-cli-registry-and-code-generation.md",
      "rfcs/0003-tailwind-styling-contract.md",
      "rfcs/0004-benchmark-and-css-output.md",
      "rfcs/0005-modules-and-platform-profiles.md",
      "rfcs/0006-focus-and-portal-primitives.md",
      "rfcs/0007-keyboard-navigation-primitives.md",
      "rfcs/0008-overlay-positioning-and-portals.md",
      "../TODOs.md",
    ],
  },
];

const missingLinks = [];

for (const { file, content, links } of requiredLinks) {
  for (const link of links) {
    if (!content.includes(`](${link})`)) {
      missingLinks.push({ file, link });
    }
  }
}

if (missingLinks.length > 0) {
  console.error("documentation index verification failed");
  for (const { file, link } of missingLinks) {
    console.error(`- ${file} missing link: ${link}`);
  }
  process.exitCode = 1;
} else {
  console.log("documentation index verification passed");
}
