#!/usr/bin/env node
// Compares the three library crates with the last release tag (RFC 0079).
// cargo-semver-checks takes the release type from the version bump, so a
// breaking change fails until the version says so: before 1.0, a minor bump.
// The baseline is built from the tag in this repository, so the check needs
// no network and runs against what was published from that tag.
//
// From 1.0 on, a pre-release on either side holds the bump to no breaking
// change: cargo-semver-checks reads `1.0.0-rc.1` to `1.0.0-rc.2` as a major
// change, while 1.x promises that neither an `rc` nor the release it leads
// to breaks the API. A breaking change the release owner accepts restarts the
// rc period; SEMVER_RC_BREAK=1 lets that one release through.
//
// Usage: node scripts/semver-verify.mjs
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
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

const parseVersion = (text) => {
  const match = /^v?(\d+)\.\d+\.\d+(-[0-9A-Za-z.-]+)?$/.exec(text);
  if (!match) {
    console.error(`not a version: ${text}`);
    process.exit(1);
  }
  return { major: Number(match[1]), prerelease: Boolean(match[2]) };
};
const workspaceVersion = /^\[workspace\.package\][^[]*?^version = "([^"]+)"/m.exec(
  readFileSync(join(repoRoot, "Cargo.toml"), "utf8"),
)?.[1];
if (!workspaceVersion) {
  console.error("Cargo.toml has no [workspace.package] version");
  process.exit(1);
}
const current = parseVersion(workspaceVersion);
const previous = parseVersion(baseline);
const releaseTypeArgs =
  process.env.SEMVER_RC_BREAK !== "1" &&
  current.major >= 1 &&
  current.major === previous.major &&
  (current.prerelease || previous.prerelease)
    ? ["--release-type", "minor"]
    : [];

const failures = [];
for (const crate of crates) {
  const result = run(
    "cargo",
    ["semver-checks", "-p", crate, "--baseline-rev", baseline, "--all-features", "--color", "never", ...releaseTypeArgs],
    { stdio: ["ignore", "inherit", "inherit"] },
  );
  if (result.status !== 0) failures.push(crate);
}

if (failures.length > 0) {
  const advice =
    releaseTypeArgs.length > 0
      ? `${workspaceVersion} must not break the API of ${baseline}. Undo the breaking change, or accept it and restart the rc period with SEMVER_RC_BREAK=1.`
      : "Bump the version for the change, and add a Migration note for each breaking finding.";
  console.error(`semver verification failed against ${baseline}: ${failures.join(", ")}. ${advice}`);
  process.exit(1);
}
const releaseType = releaseTypeArgs.length > 0 ? ", no breaking change" : "";
console.log(`semver verification passed against ${baseline} (${crates.length} crates${releaseType})`);
