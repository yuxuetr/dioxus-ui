#!/usr/bin/env node
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readText = (relativePath) => readFileSync(join(repoRoot, relativePath), "utf8");
const normalizeWhitespace = (text) => text.replace(/\s+/g, " ");

const getSection = (toml, sectionName) => {
  const sectionHeader = `[${sectionName}]`;
  const lines = toml.split("\n");
  const startIndex = lines.findIndex((line) => line.trim() === sectionHeader);
  if (startIndex < 0) {
    return null;
  }

  const sectionLines = [];
  for (const line of lines.slice(startIndex + 1)) {
    if (/^\s*\[/.test(line)) {
      break;
    }
    sectionLines.push(line);
  }
  return sectionLines.join("\n");
};

const getStringField = (section, field) => {
  const match = section.match(new RegExp(`^\\s*${field}\\s*=\\s*"([^"]+)"\\s*$`, "m"));
  return match?.[1] ?? null;
};

const hasWorkspaceInheritance = (section, field) => {
  return new RegExp(`^\\s*${field}\\.workspace\\s*=\\s*true\\s*$`, "m").test(section);
};

const rootCargo = readText("Cargo.toml");
const packageJson = JSON.parse(readText("package.json"));
const publishBlockers = readText("docs/publish-readiness-blockers.md");
const licenseDoc = readText("docs/license-readiness-metadata.md");
const cargoPublishDoc = normalizeWhitespace(readText("docs/cargo-publish-metadata.md"));
const releaseDoc = normalizeWhitespace(readText("docs/release.md"));
const qualityDoc = normalizeWhitespace(readText("docs/quality-gates.md"));
const siteDoc = normalizeWhitespace(readText("docs/site.md"));
const scripts = packageJson.scripts ?? {};
const failures = [];

const requireIncludes = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (!text.includes(fragment)) {
      failures.push(`${name} is missing: ${fragment}`);
    }
  }
};

if (scripts["verify:license-readiness"] !== "node scripts/license-readiness-verify.mjs") {
  failures.push("package.json verify:license-readiness script is missing or mismatched");
}

if (!scripts["verify:release"]?.includes("npm run verify:license-readiness")) {
  failures.push("package.json verify:release must include npm run verify:license-readiness");
}

const workspacePackage = getSection(rootCargo, "workspace.package");
if (workspacePackage === null) {
  failures.push("Cargo.toml is missing [workspace.package]");
} else if (getStringField(workspacePackage, "license") !== "MIT OR Apache-2.0") {
  failures.push('Cargo.toml [workspace.package] must keep license = "MIT OR Apache-2.0"');
}

for (const fileName of ["LICENSE-MIT", "LICENSE-APACHE"]) {
  if (existsSync(join(repoRoot, fileName))) {
    failures.push(`${fileName} is present; update license readiness docs before resolving this blocker`);
  }
}

for (const crateName of readdirSync(join(repoRoot, "crates"))) {
  const manifest = readText(`crates/${crateName}/Cargo.toml`);
  const packageSection = getSection(manifest, "package");
  if (packageSection === null) {
    failures.push(`crates/${crateName}/Cargo.toml is missing [package]`);
  } else if (!hasWorkspaceInheritance(packageSection, "license")) {
    failures.push(`crates/${crateName}/Cargo.toml must inherit license.workspace = true`);
  }
}

requireIncludes("docs/license-readiness-metadata.md", licenseDoc, [
  "License Readiness Metadata",
  "`MIT OR Apache-2.0`",
  "LICENSE-MIT",
  "LICENSE-APACHE",
  "without resolving it",
]);

requireIncludes("docs/publish-readiness-blockers.md", publishBlockers, [
  "Root license files not committed",
  "`MIT OR Apache-2.0` is declared in workspace metadata, but `LICENSE-MIT` and `LICENSE-APACHE` are not committed",
  "Maintainer commits reviewed root license files before publishing",
]);

requireIncludes("docs/cargo-publish-metadata.md", cargoPublishDoc, [
  "root license files are not yet committed",
]);

requireIncludes("docs/release.md", releaseDoc, [
  "License readiness checks are read-only",
  "workspace license metadata and missing root `LICENSE-MIT` and `LICENSE-APACHE` files",
  "they do not choose license terms, generate license text, change copyright holders, run `cargo package`, run `cargo publish`, or contact crates.io",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:license-readiness`",
  "workspace license metadata and missing root `LICENSE-MIT` and `LICENSE-APACHE` files",
  "does not choose license terms, generate license text, change copyright holders, run `cargo package`, run `cargo publish`, or contact crates.io",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M100 License Readiness Metadata Gate Usage",
  "npm run verify:license-readiness",
  "workspace license metadata and missing root `LICENSE-MIT` and `LICENSE-APACHE` files",
]);

if (failures.length > 0) {
  console.error("license readiness metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("license readiness metadata verification passed");
}
