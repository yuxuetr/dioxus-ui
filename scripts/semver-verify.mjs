#!/usr/bin/env node
// Compares the three library crates with the last release tag (RFC 0079).
// cargo-semver-checks takes the release type from the version bump, so a
// breaking change fails until the version says so: before 1.0, a minor bump.
// The baseline is built from the tag in this repository, so the check needs
// no network and runs against what was published from that tag.
//
// Usage: node scripts/semver-verify.mjs
import { spawnSync } from "node:child_process";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const crates = ["dioxus-shadcn-core", "dioxus-shadcn-primitives", "dioxus-shadcn"];

const run = (command, args, options = {}) =>
  spawnSync(command, args, { cwd: repoRoot, encoding: "utf8", ...options });

const version = run("cargo", ["semver-checks", "--version"]);
if (version.status !== 0) {
  console.error("cargo-semver-checks is not installed. Run `cargo binstall cargo-semver-checks` or `cargo install cargo-semver-checks --locked`.");
  process.exit(1);
}

const tag = run("git", ["describe", "--tags", "--abbrev=0", "--match", "v[0-9]*"]);
if (tag.status !== 0) {
  console.error(`no release tag reachable from HEAD: ${tag.stderr.trim()}`);
  process.exit(1);
}
const baseline = tag.stdout.trim();

const failures = [];
for (const crate of crates) {
  const result = run(
    "cargo",
    ["semver-checks", "-p", crate, "--baseline-rev", baseline, "--all-features", "--color", "never"],
    { stdio: ["ignore", "inherit", "inherit"] },
  );
  if (result.status !== 0) failures.push(crate);
}

if (failures.length > 0) {
  console.error(
    `semver verification failed against ${baseline}: ${failures.join(", ")}. ` +
      "Bump the version for the change, and add a Migration note for each breaking finding.",
  );
  process.exit(1);
}
console.log(`semver verification passed against ${baseline} (${crates.length} crates)`);
