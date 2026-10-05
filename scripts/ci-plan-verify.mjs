#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const qualityGatesPath = join(repoRoot, "docs/quality-gates.md");
const activeWorkflowPath = join(repoRoot, ".github/workflows/browser-smoke.yml");
const qualityGates = readFileSync(qualityGatesPath, "utf8");

const ciPlanStart = qualityGates.indexOf("## CI Plan");
const runtimeGateStart = qualityGates.indexOf("## Runtime Web Verification Gate");
const ciPlan =
  ciPlanStart >= 0 && runtimeGateStart > ciPlanStart
    ? qualityGates.slice(ciPlanStart, runtimeGateStart)
    : "";

const requiredSnippets = [
  {
    label: "CI Plan section",
    snippet: "## CI Plan",
  },
  {
    label: "workspace check command",
    snippet: "cargo check --workspace --all-features",
  },
  {
    label: "workspace test command",
    snippet: "cargo test --workspace --all-features",
  },
  {
    label: "CLI registry test command",
    snippet: "cargo test -p dioxus-shadcn-cli --test registry",
  },
  {
    label: "source-copy fixture smoke command",
    snippet: "scripts/generated-fixture-smoke.sh",
  },
  {
    label: "local deterministic aggregate command",
    snippet: "npm run verify",
  },
  {
    label: "release aggregate command",
    snippet: "npm run verify:release",
  },
  {
    label: "feature check command",
    snippet: "scripts/feature-check.sh",
  },
  {
    label: "browser smoke opt-in command",
    snippet: "docs/ci-browser-smoke.md",
  },
];

const missingSnippets = requiredSnippets.filter(({ snippet }) => {
  return !ciPlan.includes(snippet);
});

if (ciPlan.length === 0 || missingSnippets.length > 0 || existsSync(activeWorkflowPath)) {
  console.error("CI Plan documentation verification failed");

  if (ciPlan.length === 0) {
    console.error("- docs/quality-gates.md is missing a bounded CI Plan section");
  }

  for (const { label, snippet } of missingSnippets) {
    console.error(`- CI Plan missing ${label}: ${snippet}`);
  }

  if (existsSync(activeWorkflowPath)) {
    console.error("- active browser smoke workflow must not be committed yet");
  }

  process.exitCode = 1;
} else {
  console.log("CI Plan documentation verification passed");
}
