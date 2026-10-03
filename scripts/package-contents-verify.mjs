#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const cliRoot = join(repoRoot, "crates/dioxus-ui-cli");

const readText = (relativePath) => readFileSync(join(repoRoot, relativePath), "utf8");
const normalizeWhitespace = (text) => text.replace(/\s+/g, " ");

const packageJson = JSON.parse(readText("package.json"));
const releaseDoc = normalizeWhitespace(readText("docs/release.md"));
const qualityDoc = normalizeWhitespace(readText("docs/quality-gates.md"));
const scripts = packageJson.scripts ?? {};
const failures = [];

// Files every published crate must ship, beyond its entry point.
const publishableCrates = [
  { name: "dioxus-ui-core", entry: "src/lib.rs" },
  { name: "dioxus-ui-primitives", entry: "src/lib.rs" },
  { name: "dioxus-ui", entry: "src/lib.rs" },
  { name: "dioxus-ui-cli", entry: "src/main.rs" },
];

const requireIncludes = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (!text.includes(fragment)) {
      failures.push(`${name} is missing: ${fragment}`);
    }
  }
};

// `cargo package --list` reports exactly what `cargo publish` would upload,
// without building, compressing, or contacting crates.io.
const packageFiles = (crateName) => {
  const result = spawnSync(
    "cargo",
    ["package", "-p", crateName, "--list", "--allow-dirty", "--offline"],
    { cwd: repoRoot, encoding: "utf8" },
  );
  if (result.status !== 0) {
    failures.push(`cargo package -p ${crateName} --list failed: ${result.stderr.trim()}`);
    return null;
  }
  return new Set(result.stdout.split("\n").filter(Boolean));
};

// The CLI build script embeds every registry entry and every file it maps,
// so each of them must be inside the CLI package.
const cliEmbeddedAssets = () => {
  const assets = [];
  for (const file of readdirSync(join(cliRoot, "registry")).sort()) {
    if (!file.endsWith(".json") || file === "schema.json") {
      continue;
    }
    assets.push(`registry/${file}`);
    const entry = JSON.parse(readFileSync(join(cliRoot, "registry", file), "utf8"));
    for (const mapping of [...(entry.files ?? []), ...(entry.assets ?? [])]) {
      assets.push(mapping.source);
    }
  }
  return assets;
};

if (scripts["verify:package-contents"] !== "node scripts/package-contents-verify.mjs") {
  failures.push("package.json verify:package-contents script is missing or mismatched");
}

if (!scripts["verify:release"]?.includes("npm run verify:package-contents")) {
  failures.push("package.json verify:release must include npm run verify:package-contents");
}

for (const { name, entry } of publishableCrates) {
  const files = packageFiles(name);
  if (files === null) {
    continue;
  }
  for (const required of ["Cargo.toml", "README.md", entry]) {
    if (!files.has(required)) {
      failures.push(`${name} package is missing ${required}`);
    }
  }
  if (name === "dioxus-ui-cli") {
    const missing = cliEmbeddedAssets().filter((asset) => !files.has(asset));
    for (const asset of missing) {
      failures.push(`dioxus-ui-cli package is missing embedded asset ${asset}`);
    }
  }
}

requireIncludes("docs/release.md", releaseDoc, [
  "npm run verify:package-contents",
  "every registry entry and template the CLI embeds",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:package-contents`",
  "every registry entry and template the CLI embeds",
]);

if (failures.length > 0) {
  console.error("package contents verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log(`package contents verification passed (${publishableCrates.length} crates)`);
}
