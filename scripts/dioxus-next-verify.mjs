#!/usr/bin/env node
// Runs the checks Stage 14 needs against the newest Dioxus 0.8 version on
// crates.io, so each pre-release shows what the move to 0.8 will take. The
// tree (HEAD plus uncommitted changes to tracked files) is copied into a
// scratch worktree with `dioxus` and `dioxus-ssr` pinned to that version; the
// main tree and its build directory are left alone.
//
// Usage: node scripts/dioxus-next-verify.mjs
// DIOXUS_NEXT_DX names a `dx` of the same version when the one on PATH is not.
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const run = (command, args, options = {}) => spawnSync(command, args, { encoding: "utf8", ...options });

function fail(message) {
  console.error(message);
  process.exit(1);
}

// Newest 0.8 version on crates.io, pre-releases included.
async function newestDioxus08() {
  const response = await fetch("https://crates.io/api/v1/crates/dioxus/versions", {
    headers: { "User-Agent": "dioxus-ui verify:dioxus-next (github.com/yuxuetr/dioxus-ui)" },
  });
  if (!response.ok) fail(`crates.io answered ${response.status} for the dioxus versions`);
  const { versions } = await response.json();
  // crates.io lists versions newest first.
  const version = versions.find((entry) => !entry.yanked && entry.num.startsWith("0.8."))?.num;
  if (!version) fail("crates.io lists no Dioxus 0.8 version");
  return version;
}

function replaceLine(path, pattern, line) {
  const text = readFileSync(path, "utf8");
  if (!pattern.test(text)) fail(`${path} has no line matching ${pattern}`);
  writeFileSync(path, text.replace(pattern, line));
}

const version = await newestDioxus08();

const env = { ...process.env };
if (process.env.DIOXUS_NEXT_DX) env.PATH = `${dirname(process.env.DIOXUS_NEXT_DX)}:${env.PATH}`;
const dxVersion = run("dx", ["--version"], { env });
if (dxVersion.status !== 0 || !dxVersion.stdout.includes(` ${version} `)) {
  fail(
    `verify:dioxus-next needs dx ${version}, found: ${(dxVersion.stdout || dxVersion.stderr || "no dx").trim()}.\n` +
      `Install it apart from your dx, then point DIOXUS_NEXT_DX at it:\n` +
      `  cargo binstall dioxus-cli@${version} --root /tmp/dx-next --no-confirm\n` +
      `  DIOXUS_NEXT_DX=/tmp/dx-next/bin/dx npm run verify:dioxus-next`,
  );
}

const metadata = run("cargo", ["metadata", "--format-version", "1", "--no-deps"], { cwd: repoRoot });
if (metadata.status !== 0) fail(metadata.stderr);
// Its own build directory, so dx never serves a bundle built against 0.7.
env.CARGO_TARGET_DIR = join(JSON.parse(metadata.stdout).target_directory, "dioxus-next");

const worktree = join(mkdtempSync(join(tmpdir(), "dioxus-next-")), "tree");
const added = run("git", ["worktree", "add", "--detach", worktree, "HEAD"], { cwd: repoRoot });
if (added.status !== 0) fail(added.stderr);

const steps = [
  ["workspace tests", "cargo", ["test", "--workspace", "--all-features"]],
  ["clippy", "cargo", ["clippy", "--workspace", "--all-targets", "--all-features", "--", "-D", "warnings"]],
  ["generated fixture smoke", "bash", ["scripts/generated-fixture-smoke.sh"]],
  ["fullstack hydration", "node", ["scripts/fullstack-hydration-verify.mjs"]],
  ["browser interactions", "node", ["scripts/runtime-interactions-verify.mjs"]],
];
let failed = null;
try {
  const diff = run("git", ["diff", "HEAD", "--binary"], { cwd: repoRoot, maxBuffer: 256 * 1024 * 1024 });
  if (diff.stdout) {
    const applied = run("git", ["apply", "--binary", "-"], { cwd: worktree, input: diff.stdout });
    if (applied.status !== 0) throw new Error(`could not copy uncommitted changes: ${applied.stderr}`);
  }
  replaceLine(join(worktree, "Cargo.toml"), /^dioxus = ".*"$/m, `dioxus = "=${version}"`);
  replaceLine(join(worktree, "crates/dioxus-shadcn/Cargo.toml"), /^dioxus-ssr = ".*"$/m, `dioxus-ssr = "=${version}"`);
  replaceLine(join(worktree, "scripts/generated-fixture-smoke.sh"), /^dioxus = ".*"$/m, `dioxus = "=${version}"`);
  symlinkSync(join(repoRoot, "node_modules"), join(worktree, "node_modules"));

  for (const [name, command, args] of steps) {
    console.log(`\n== ${name} on Dioxus ${version}`);
    const result = run(command, args, { cwd: worktree, env, stdio: "inherit" });
    if (result.status !== 0) {
      failed = name;
      break;
    }
  }
} finally {
  run("git", ["worktree", "remove", "--force", worktree], { cwd: repoRoot });
}

if (failed) fail(`\ndioxus-next verification failed on Dioxus ${version}: ${failed}`);
console.log(`\ndioxus-next verification passed on Dioxus ${version} (${steps.map(([name]) => name).join(", ")})`);
