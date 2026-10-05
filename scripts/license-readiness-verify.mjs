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
const preparationPlan = readText("docs/license-decision-preparation-plan.md");
const decisionTemplate = readText("docs/license-decision-record-template.md");
const followUpMap = readText("docs/license-local-follow-up-map.md");
const cargoPublishDoc = normalizeWhitespace(readText("docs/cargo-publish-metadata.md"));
const releaseDoc = normalizeWhitespace(readText("docs/release.md"));
const qualityDoc = normalizeWhitespace(readText("docs/quality-gates.md"));
const readme = normalizeWhitespace(readText("README.md"));
const docsIndex = readText("docs/README.md");
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
} else if (getStringField(workspacePackage, "license") !== "MIT") {
  failures.push('Cargo.toml [workspace.package] must keep license = "MIT"');
}

if (!existsSync(join(repoRoot, "LICENSE"))) {
  failures.push("LICENSE is missing");
} else {
  const licenseText = readText("LICENSE");
  requireIncludes("LICENSE", licenseText, [
    "MIT License",
    "Copyright (c) 2026 yuxuetr",
    "Permission is hereby granted, free of charge",
  ]);
}

const crateNames = readdirSync(join(repoRoot, "crates"), { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => entry.name);
for (const crateName of crateNames) {
  const manifest = readText(`crates/${crateName}/Cargo.toml`);
  const packageSection = getSection(manifest, "package");
  if (packageSection === null) {
    failures.push(`crates/${crateName}/Cargo.toml is missing [package]`);
  } else if (!hasWorkspaceInheritance(packageSection, "license")) {
    failures.push(`crates/${crateName}/Cargo.toml must inherit license.workspace = true`);
  }
  // Cargo packages only files inside the crate, so each crate links the root
  // LICENSE to ship the MIT notice.
  if (!existsSync(join(repoRoot, "crates", crateName, "LICENSE"))) {
    failures.push(`crates/${crateName}/LICENSE must link the root LICENSE so the package ships it`);
  }
}

requireIncludes("docs/license-readiness-metadata.md", licenseDoc, [
  "License Readiness Metadata",
  "`MIT`",
  "LICENSE",
  "License Decision Preparation Plan",
  "License Decision Record Template",
  "License Local Follow-up Map",
  "after resolution",
]);

requireIncludes("docs/license-decision-preparation-plan.md", preparationPlan, [
  "License Decision Preparation Plan",
  "`MIT`",
  "approved `LICENSE` text",
  "copyright holder text",
  "M130 must not",
  "generate replacement license text",
]);

requireIncludes("docs/license-decision-record-template.md", decisionTemplate, [
  "License Decision Record Template",
  "Workspace license expression accepted",
  "`LICENSE` text approved by",
  "Copyright holder text",
  "Root license file commit approved",
  "Decision state: `approved` / `blocked` / `deferred`",
]);

requireIncludes("docs/license-local-follow-up-map.md", followUpMap, [
  "License Local Follow-up Map",
  "Move license readiness out of current blockers only after approved files",
  "Keep root license files absent",
  "Workspace license metadata",
  "does not commit license files without maintainer approval",
]);

requireIncludes("docs/publish-readiness-blockers.md", publishBlockers, [
  "Root license files not committed",
  "`LICENSE` contains reviewed MIT license text",
  "npm run verify:license-readiness",
]);

requireIncludes("docs/cargo-publish-metadata.md", cargoPublishDoc, [
  "root MIT license text is committed",
]);

requireIncludes("docs/release.md", releaseDoc, [
  "License readiness checks are read-only",
  "workspace MIT license metadata and committed root `LICENSE` file",
  "they do not choose different license terms, generate replacement license text, change copyright holders, run `cargo package`, run `cargo publish`, or contact crates.io",
  "License Decision Preparation Plan",
  "License Decision Record Template",
  "License Local Follow-up Map",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:license-readiness`",
  "workspace MIT license metadata and committed root `LICENSE` file",
  "does not choose different license terms, generate replacement license text, change copyright holders, run `cargo package`, run `cargo publish`, or contact crates.io",
  "License Decision Preparation Plan",
  "License Decision Record Template",
  "License Local Follow-up Map",
]);

requireIncludes("README.md", readme, [
  "License Decision Preparation Plan",
  "License Decision Record Template",
  "License Local Follow-up Map",
]);

requireIncludes("docs/README.md", docsIndex, [
  "License Decision Preparation Plan",
  "License Decision Record Template",
  "License Local Follow-up Map",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M100 License Readiness Metadata Gate Usage",
  "M130 License Decision Preparation",
  "npm run verify:license-readiness",
  "workspace MIT license metadata and committed root `LICENSE` file",
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
