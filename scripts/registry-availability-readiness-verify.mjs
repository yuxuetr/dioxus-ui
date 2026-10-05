#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readText = (relativePath) => readFileSync(join(repoRoot, relativePath), "utf8");
const normalizeWhitespace = (text) => text.replace(/\s+/g, " ");

const packageJson = JSON.parse(readText("package.json"));
const rootCargo = readText("Cargo.toml");
const readinessDoc = normalizeWhitespace(readText("docs/registry-availability-readiness-metadata.md"));
const publishBlockers = normalizeWhitespace(readText("docs/publish-readiness-blockers.md"));
const registryHandoff = readText("docs/registry-availability-blocker-handoff.md");
const cargoPublishDoc = normalizeWhitespace(readText("docs/cargo-publish-metadata.md"));
const releaseDoc = normalizeWhitespace(readText("docs/release.md"));
const qualityDoc = normalizeWhitespace(readText("docs/quality-gates.md"));
const siteDoc = normalizeWhitespace(readText("docs/site.md"));
const scripts = packageJson.scripts ?? {};
const failures = [];

const plannedCrates = [
  "dioxus-shadcn-core",
  "dioxus-shadcn-primitives",
  "dioxus-shadcn",
  "dioxus-shadcn-cli",
];

const requireIncludes = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (!text.includes(fragment)) {
      failures.push(`${name} is missing: ${fragment}`);
    }
  }
};

if (
  scripts["verify:registry-availability-readiness"] !==
  "node scripts/registry-availability-readiness-verify.mjs"
) {
  failures.push("package.json verify:registry-availability-readiness script is missing or mismatched");
}

if (!scripts["verify:release"]?.includes("npm run verify:registry-availability-readiness")) {
  failures.push("package.json verify:release must include npm run verify:registry-availability-readiness");
}

for (const crateName of plannedCrates) {
  const manifestPath =
    crateName === "dioxus-shadcn"
      ? "crates/dioxus-shadcn/Cargo.toml"
      : `crates/${crateName}/Cargo.toml`;
  const manifest = readText(manifestPath);
  requireIncludes(manifestPath, manifest, [`name = "${crateName}"`]);
  requireIncludes("docs/registry-availability-readiness-metadata.md", readinessDoc, [
    crateName,
    `| \`${crateName}\` | Available, or already owned by the release owner |`,
  ]);
}

requireIncludes("Cargo.toml", rootCargo, [
  "members = [",
  "crates/dioxus-shadcn-core",
  "crates/dioxus-shadcn-primitives",
  "crates/dioxus-shadcn",
  "crates/dioxus-shadcn-cli",
]);

requireIncludes("docs/registry-availability-readiness-metadata.md", readinessDoc, [
  "Registry Availability Readiness Metadata",
  "does not prove the names are available on crates.io",
  "## Deferral",
  "Deferral blocks crates.io publishing; it does not block local release readiness or internal trial.",
  "| Crate | Name Evidence | Owners | Publish Position |",
  "credential readiness confirmed by the release owner",
  "named release owner responsible for the actual publish",
  "a rename milestone is required before publish",
  "must not contact crates.io, check crate name availability, check ownership, inspect credentials, run `cargo package`, run `cargo publish`, or create package archives",
]);

requireIncludes("docs/publish-readiness-blockers.md", publishBlockers, [
  "Registry availability not checked",
  "Cargo publish metadata gate intentionally avoids crates.io lookups",
  "Release owner checks names and ownership during publish preparation",
  "checking crates.io name availability",
  "Deferral blocks crates.io publishing; it does not block local release readiness or internal trial.",
]);

requireIncludes("docs/registry-availability-blocker-handoff.md", registryHandoff, [
  "| Publish state | Deferred: blocks crates.io publishing, not local release readiness |",
]);

requireIncludes("docs/cargo-publish-metadata.md", cargoPublishDoc, [
  "crates.io name and ownership review remains unresolved",
]);

requireIncludes("docs/release.md", releaseDoc, [
  "Registry availability readiness checks are read-only",
  "Deferral blocks crates.io publishing but not local release readiness",
  "crates.io name and ownership review blocker",
  "they do not contact crates.io, check crate name availability, check ownership, inspect credentials, run `cargo package`, run `cargo publish`, or create package archives",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:registry-availability-readiness`",
  "Deferral blocks crates.io publishing but not local release readiness",
  "crates.io name and ownership review blocker",
  "does not contact crates.io, check crate name availability, check ownership, inspect credentials, run `cargo package`, run `cargo publish`, or create package archives",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M104 Registry Availability Readiness Metadata Gate Usage",
  "npm run verify:registry-availability-readiness",
  "crates.io name and ownership review blocker",
]);

if (failures.length > 0) {
  console.error("registry availability readiness metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("registry availability readiness metadata verification passed");
}
