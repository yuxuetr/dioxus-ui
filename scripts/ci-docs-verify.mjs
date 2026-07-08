#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const ciGuidePath = join(repoRoot, "docs/ci-browser-smoke.md");
const workflowTemplatePath = join(repoRoot, "docs/ci-browser-workflow-template.md");
const activeWorkflowPath = join(repoRoot, ".github/workflows/browser-smoke.yml");

const ciGuide = readFileSync(ciGuidePath, "utf8");
const workflowTemplate = readFileSync(workflowTemplatePath, "utf8");

const requiredSnippets = [
  {
    file: "docs/ci-browser-smoke.md",
    content: ciGuide,
    label: "local deterministic aggregate gate",
    snippet: "npm run verify",
  },
  {
    file: "docs/ci-browser-smoke.md",
    content: ciGuide,
    label: "release aggregate gate",
    snippet: "npm run verify:release",
  },
  {
    file: "docs/ci-browser-smoke.md",
    content: ciGuide,
    label: "workspace test gate",
    snippet: "cargo test --workspace --all-features -q",
  },
  {
    file: "docs/ci-browser-smoke.md",
    content: ciGuide,
    label: "mobile browser smoke command",
    snippet: "npm run verify:mobile-browser",
  },
  {
    file: "docs/ci-browser-smoke.md",
    content: ciGuide,
    label: "workflow documentation-only boundary",
    snippet: "It does not add a repository workflow by itself.",
  },
  {
    file: "docs/ci-browser-workflow-template.md",
    content: workflowTemplate,
    label: "manual workflow trigger",
    snippet: "workflow_dispatch:",
  },
  {
    file: "docs/ci-browser-workflow-template.md",
    content: workflowTemplate,
    label: "non-blocking workflow mode",
    snippet: "continue-on-error: true",
  },
  {
    file: "docs/ci-browser-workflow-template.md",
    content: workflowTemplate,
    label: "local deterministic aggregate gate",
    snippet: "run: npm run verify",
  },
  {
    file: "docs/ci-browser-workflow-template.md",
    content: workflowTemplate,
    label: "workspace test gate",
    snippet: "run: cargo test --workspace --all-features -q",
  },
  {
    file: "docs/ci-browser-workflow-template.md",
    content: workflowTemplate,
    label: "mobile browser smoke command",
    snippet: "run: npm run verify:mobile-browser",
  },
];

const missingSnippets = requiredSnippets.filter(({ content, snippet }) => {
  return !content.includes(snippet);
});

if (missingSnippets.length > 0 || existsSync(activeWorkflowPath)) {
  console.error("CI browser documentation verification failed");

  for (const { file, label, snippet } of missingSnippets) {
    console.error(`- ${file} missing ${label}: ${snippet}`);
  }

  if (existsSync(activeWorkflowPath)) {
    console.error("- active browser smoke workflow must not be committed yet");
  }

  process.exitCode = 1;
} else {
  console.log("CI browser documentation verification passed");
}
