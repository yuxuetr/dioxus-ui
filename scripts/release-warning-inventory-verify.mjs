#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const packageJson = JSON.parse(readRepoFile("package.json"));
const cargoLock = readRepoFile("Cargo.lock");
const inventoryDoc = readRepoFile("docs/release-warning-inventory-metadata.md");
const releaseDocs = readRepoFile("docs/release.md");
const qualityGates = readRepoFile("docs/quality-gates.md");
const normalizedQualityGates = qualityGates.replace(/\s+/g, " ");
const docsSite = readRepoFile("docs/site.md");
const failures = [];

const requireFragment = (label, source, fragment) => {
  if (!source.includes(fragment)) {
    failures.push(`${label} missing fragment: ${fragment}`);
  }
};

const requireBlockPackage = (cargoLockText, name, version) => {
  const packageBlocks = cargoLockText.split(/\n\[\[package\]\]\n/g).slice(1);
  const hasPackage = packageBlocks.some((block) => {
    return block.includes(`name = "${name}"`) && block.includes(`version = "${version}"`);
  });

  if (!hasPackage) {
    failures.push(`Cargo.lock missing package ${name} ${version}`);
  }
};

const scripts = packageJson.scripts ?? {};
if (scripts["verify:release-warning-inventory"] !== "node scripts/release-warning-inventory-verify.mjs") {
  failures.push("package.json missing verify:release-warning-inventory script");
}

if (!scripts["verify:release"]?.includes("npm run verify:release-warning-inventory")) {
  failures.push("package.json verify:release missing release warning inventory gate");
}

requireBlockPackage(cargoLock, "block", "0.1.6");

const warningFragments = [
  "`block`",
  "`0.1.6`",
  "Rust future-incompatibility warning",
  "cargo check --workspace --all-features",
  "cargo test --workspace --all-features",
  "npm run verify:release",
];

for (const fragment of warningFragments) {
  requireFragment("docs/release-warning-inventory-metadata.md", inventoryDoc, fragment);
}

const inventoryBoundaryFragments = [
  "running Cargo commands",
  "parsing live compiler output",
  "executing `cargo report future-incompatibilities`",
  "upgrading, patching, or removing dependencies",
  "suppressing warnings",
  "contacting crates.io or upstream repositories",
];

for (const fragment of inventoryBoundaryFragments) {
  requireFragment("docs/release-warning-inventory-metadata.md", inventoryDoc, fragment);
}

const releaseFragments = [
  "Release warning inventory metadata checks are also read-only",
  "`block` `0.1.6`",
  "Rust future-incompatibility warning",
  "they do not run Cargo, parse compiler output, upgrade dependencies, or suppress warnings",
];

for (const fragment of releaseFragments) {
  requireFragment("docs/release.md", releaseDocs, fragment);
}

const qualityFragments = [
  "`npm run verify:release-warning-inventory`",
  "`block` `0.1.6`",
  "future-incompatibility warning inventory",
  "does not run Cargo, parse live compiler output, execute `cargo report`, upgrade dependencies, or suppress warnings",
];

for (const fragment of qualityFragments) {
  requireFragment("docs/quality-gates.md", normalizedQualityGates, fragment);
}

const docsSiteFragments = [
  "M95 Release Warning Inventory Metadata Gate Usage",
  "npm run verify:release-warning-inventory",
  "`block` `0.1.6`",
  "future-incompatibility warning",
];

for (const fragment of docsSiteFragments) {
  requireFragment("docs/site.md", docsSite, fragment);
}

if (failures.length > 0) {
  console.error("release warning inventory metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("release warning inventory metadata verification passed");
}
