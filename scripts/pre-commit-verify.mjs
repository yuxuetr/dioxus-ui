#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const preCommitPath = join(repoRoot, ".pre-commit-config.yaml");
const failures = [];

const requiredLocalHooks = [
  {
    id: "cargo-fmt",
    entry: "cargo fmt -- --check",
    configFiles: ["rustfmt.toml"],
  },
  {
    id: "cargo-deny",
    entry: "cargo deny --offline check",
    configFiles: ["deny.toml"],
  },
  {
    id: "typos",
    entry: "typos",
    configFiles: ["_typos.toml"],
  },
  {
    id: "cargo-check",
    entry: "cargo check --all",
    configFiles: [],
  },
  {
    id: "cargo-clippy",
    entry: "cargo clippy --all-targets --all-features --tests --benches -- -D warnings",
    configFiles: [],
  },
  {
    id: "cargo-test",
    entry: "cargo nextest run --all-features",
    configFiles: [],
  },
];

const extractLocalRepoBlock = (config) => {
  const marker = "  - repo: local";
  const startIndex = config.indexOf(marker);
  if (startIndex < 0) {
    return null;
  }

  const afterStart = config.slice(startIndex + marker.length);
  const nextRepoMatch = afterStart.match(/\n  - repo: /);
  if (nextRepoMatch === null || nextRepoMatch.index === undefined) {
    return config.slice(startIndex);
  }
  return config.slice(startIndex, startIndex + marker.length + nextRepoMatch.index);
};

const extractHookBlock = (localRepoBlock, id) => {
  const marker = `      - id: ${id}`;
  const startIndex = localRepoBlock.indexOf(marker);
  if (startIndex < 0) {
    return null;
  }

  const afterStart = localRepoBlock.slice(startIndex + marker.length);
  const nextHookMatch = afterStart.match(/\n      - id: /);
  if (nextHookMatch === null || nextHookMatch.index === undefined) {
    return localRepoBlock.slice(startIndex);
  }
  return localRepoBlock.slice(startIndex, startIndex + marker.length + nextHookMatch.index);
};

if (!existsSync(preCommitPath)) {
  failures.push(".pre-commit-config.yaml is missing");
} else {
  const config = readFileSync(preCommitPath, "utf8");
  const localRepoBlock = extractLocalRepoBlock(config);

  if (localRepoBlock === null) {
    failures.push(".pre-commit-config.yaml is missing repo: local hook block");
  } else {
    for (const hook of requiredLocalHooks) {
      const hookBlock = extractHookBlock(localRepoBlock, hook.id);
      if (hookBlock === null) {
        failures.push(`missing local pre-commit hook: ${hook.id}`);
        continue;
      }

      if (!hookBlock.includes(`entry: bash -c '${hook.entry}'`)) {
        failures.push(`local pre-commit hook ${hook.id} is missing entry: ${hook.entry}`);
      }

      if (!hookBlock.includes("language: rust")) {
        failures.push(`local pre-commit hook ${hook.id} must use language: rust`);
      }
    }
  }
}

for (const hook of requiredLocalHooks) {
  for (const configFile of hook.configFiles) {
    if (!existsSync(join(repoRoot, configFile))) {
      failures.push(`local pre-commit hook ${hook.id} requires missing config file: ${configFile}`);
    }
  }
}

if (failures.length > 0) {
  console.error("pre-commit metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("pre-commit metadata verification passed");
}
