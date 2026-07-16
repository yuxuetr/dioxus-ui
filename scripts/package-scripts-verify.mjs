#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const packageJsonPath = join(repoRoot, "package.json");
const packageJson = JSON.parse(readFileSync(packageJsonPath, "utf8"));
const scripts = packageJson.scripts ?? {};

const requiredScripts = {
  "verify:web-preview": "node scripts/web-preview-verify.mjs",
  "verify:desktop-preview": "node scripts/desktop-preview-verify.mjs",
  "verify:mobile-web-profile": "node scripts/mobile-web-profile-verify.mjs",
  "verify:mobile-browser": "node scripts/mobile-browser-smoke.mjs",
  "verify:css-inputs": "node scripts/css-input-metadata-verify.mjs",
  "verify:registry": "node scripts/registry-verify.mjs",
  "verify:tailwind-static": "node scripts/tailwind-static-verify.mjs",
  "verify:docs-catalog": "node scripts/docs-catalog-verify.mjs",
  "verify:docs-catalog-page": "node scripts/docs-catalog-markdown-verify.mjs",
  "verify:docs-routes": "node scripts/docs-route-manifest-verify.mjs",
  "verify:docs-source-preview": "node scripts/docs-source-preview-verify.mjs",
  "verify:docs-status": "node scripts/docs-component-status-verify.mjs",
  "verify:docs-structure": "node scripts/docs-structure-verify.mjs",
  "verify:docs-index": "node scripts/docs-index-verify.mjs",
  "verify:docs-links": "node scripts/docs-link-targets-verify.mjs",
  "verify:docs-anchors": "node scripts/docs-anchors-verify.mjs",
  "verify:rfcs": "node scripts/rfc-metadata-verify.mjs",
  "verify:readme": "node scripts/readme-verification-verify.mjs",
  "verify:examples-metadata": "node scripts/examples-metadata-verify.mjs",
  "verify:release-docs": "node scripts/release-docs-verify.mjs",
  "verify:package-scripts": "node scripts/package-scripts-verify.mjs",
  "verify:package-lock": "node scripts/package-lock-verify.mjs",
  "verify:cargo-workspace": "node scripts/cargo-workspace-verify.mjs",
  "verify:cargo-lock": "node scripts/cargo-lock-verify.mjs",
  "verify:pre-commit": "node scripts/pre-commit-verify.mjs",
  "verify:scripts": "node scripts/script-metadata-verify.mjs",
  "verify:gitignore": "node scripts/gitignore-metadata-verify.mjs",
  "verify:preview-state-metadata": "node scripts/preview-state-metadata-verify.mjs",
  "verify:mobile-browser-metadata": "node scripts/mobile-browser-metadata-verify.mjs",
  "verify:repo-hygiene": "node scripts/repo-hygiene-verify.mjs",
  "verify:ci-docs": "node scripts/ci-docs-verify.mjs",
  "verify:ci-plan": "node scripts/ci-plan-verify.mjs",
  "verify:ci-workflow-template": "node scripts/ci-workflow-template-verify.mjs",
  "verify:browser-artifact-policy": "node scripts/browser-artifact-policy-verify.mjs",
};

const aggregateScriptRequirements = {
  "verify:docs": [
    "npm run verify:docs-catalog",
    "npm run verify:docs-catalog-page",
    "npm run verify:docs-status",
    "npm run verify:docs-structure",
    "npm run verify:docs-routes",
    "npm run verify:docs-source-preview",
    "npm run verify:rfcs",
    "npm run verify:readme",
    "npm run verify:docs-index",
    "npm run verify:docs-links",
    "npm run verify:docs-anchors",
  ],
  "verify:preview": [
    "npm run verify:web-preview",
    "npm run verify:mobile-web-profile",
    "npm run verify:desktop-preview",
  ],
  "verify:smoke": [
    "npm run verify:preview",
    "npm run verify:examples",
  ],
  "verify": [
    "npm run verify:smoke",
    "npm run verify:docs",
  ],
  "verify:release": [
    "cargo check --workspace --all-features",
    "cargo test --workspace --all-features",
    "cargo test -p dioxus-ui-cli --test registry",
    "cargo run -p dioxus-ui-cli -- list",
    "npm run verify:cargo-workspace",
    "npm run verify:cargo-lock",
    "npm run verify:pre-commit",
    "npm run verify:scripts",
    "npm run verify:gitignore",
    "npm run verify:preview-state-metadata",
    "npm run verify:mobile-browser-metadata",
    "npm run verify:examples-metadata",
    "npm run verify:css-inputs",
    "npm run verify:registry",
    "npm run verify:tailwind-static",
    "npm run verify",
    "scripts/feature-check.sh",
    "scripts/generated-fixture-smoke.sh",
    "npm run verify:release-docs",
    "npm run verify:package-scripts",
    "npm run verify:package-lock",
    "npm run verify:ci-docs",
    "npm run verify:ci-plan",
    "npm run verify:ci-workflow-template",
    "npm run verify:browser-artifact-policy",
    "npm run verify:repo-hygiene",
  ],
};

const missingScripts = [];
const mismatchedScripts = [];
const missingAggregateParts = [];
const missingLocalTargets = [];

for (const [name, expectedCommand] of Object.entries(requiredScripts)) {
  const actualCommand = scripts[name];
  if (actualCommand === undefined) {
    missingScripts.push(name);
  } else if (actualCommand !== expectedCommand) {
    mismatchedScripts.push({ name, expectedCommand, actualCommand });
  }
}

for (const [name, requiredParts] of Object.entries(aggregateScriptRequirements)) {
  const actualCommand = scripts[name];
  if (actualCommand === undefined) {
    missingScripts.push(name);
    continue;
  }

  for (const requiredPart of requiredParts) {
    if (!actualCommand.includes(requiredPart)) {
      missingAggregateParts.push({ name, requiredPart });
    }
  }
}

for (const [name, command] of Object.entries(scripts)) {
  const nodeScriptMatches = command.matchAll(/\bnode\s+(scripts\/[^\s&|;]+)/g);
  for (const match of nodeScriptMatches) {
    const target = match[1];
    if (!existsSync(join(repoRoot, target))) {
      missingLocalTargets.push({ name, target });
    }
  }

  const directScriptMatches = command.matchAll(/(?:^|[\s&|;])(scripts\/[^\s&|;]+)/g);
  for (const match of directScriptMatches) {
    const target = match[1];
    if (!existsSync(join(repoRoot, target))) {
      missingLocalTargets.push({ name, target });
    }
  }
}

if (
  missingScripts.length > 0 ||
  mismatchedScripts.length > 0 ||
  missingAggregateParts.length > 0 ||
  missingLocalTargets.length > 0
) {
  console.error("package script verification failed");

  for (const name of missingScripts) {
    console.error(`- missing script: ${name}`);
  }

  for (const { name, expectedCommand, actualCommand } of mismatchedScripts) {
    console.error(`- mismatched script: ${name}`);
    console.error(`  expected: ${expectedCommand}`);
    console.error(`  actual:   ${actualCommand}`);
  }

  for (const { name, requiredPart } of missingAggregateParts) {
    console.error(`- aggregate script ${name} is missing: ${requiredPart}`);
  }

  for (const { name, target } of missingLocalTargets) {
    console.error(`- script ${name} references missing local target: ${target}`);
  }

  process.exitCode = 1;
} else {
  console.log("package script verification passed");
}
