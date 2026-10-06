#!/usr/bin/env node
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const readText = (path) => {
  return readFileSync(path, "utf8");
};

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

const getStringArrayField = (section, field) => {
  const match = section.match(new RegExp(`^\\s*${field}\\s*=\\s*\\[([^\\]]*)\\]\\s*$`, "m"));
  if (match === null) {
    return null;
  }

  return match[1]
    .split(",")
    .map((part) => part.trim())
    .filter((part) => part.length > 0)
    .map((part) => part.replace(/^"|"$/g, ""));
};

const hasWorkspaceInheritance = (section, field) => {
  return new RegExp(`^\\s*${field}\\.workspace\\s*=\\s*true\\s*$`, "m").test(section);
};

const hasPublishFalse = (section) => {
  return /^\s*publish\s*=\s*false\s*$/m.test(section);
};

const packageJson = JSON.parse(readRepoFile("package.json"));
const releaseDocs = readRepoFile("docs/release.md");
const qualityGates = readRepoFile("docs/quality-gates.md");
const docsSite = readRepoFile("docs/site.md");
const metadataDoc = readRepoFile("docs/cargo-publish-metadata.md");
const normalizedMetadataDoc = metadataDoc.replace(/\s+/g, " ");
const normalizedReleaseDocs = releaseDocs.replace(/\s+/g, " ");
const normalizedQualityGates = qualityGates.replace(/\s+/g, " ");
const normalizedDocsSite = docsSite.replace(/\s+/g, " ");
const rootCargoPath = join(repoRoot, "Cargo.toml");
const cratesDir = join(repoRoot, "crates");
const examplesDir = join(repoRoot, "examples");
const failures = [];

const requireFragment = (label, source, fragment) => {
  if (!source.includes(fragment)) {
    failures.push(`${label} missing fragment: ${fragment}`);
  }
};

const expectedWorkspaceArrays = {
  keywords: ["dioxus", "ui", "tailwind", "components"],
  categories: ["gui", "web-programming"],
};
const expectedDescriptions = {
  "dioxus-shadcn-core": "Core utilities and shared conventions for dioxus-shadcn.",
  "dioxus-shadcn-primitives": "Unstyled primitive state and accessibility helpers for dioxus-shadcn.",
  "dioxus-shadcn": "Tailwind-styled Dioxus UI components with source-copy friendly APIs.",
  "dioxus-shadcn-cli": "Command-line tool for adding dioxus-shadcn components to Dioxus projects.",
};
const inheritedPublishFields = ["version", "edition", "license", "repository", "readme", "keywords", "categories"];

const scripts = packageJson.scripts ?? {};
if (scripts["verify:cargo-publish-metadata"] !== "node scripts/cargo-publish-metadata-verify.mjs") {
  failures.push("package.json missing verify:cargo-publish-metadata script");
}

if (!scripts["verify:release"]?.includes("npm run verify:cargo-publish-metadata")) {
  failures.push("package.json verify:release missing Cargo publish metadata gate");
}

if (!existsSync(rootCargoPath)) {
  failures.push("root Cargo.toml is missing");
} else {
  const rootCargo = readText(rootCargoPath);
  const workspacePackage = getSection(rootCargo, "workspace.package");
  if (workspacePackage === null) {
    failures.push("root Cargo.toml is missing [workspace.package]");
  } else {
    if (getStringField(workspacePackage, "readme") !== "crates/README.md") {
      failures.push('root [workspace.package] must set readme = "crates/README.md"');
    }

    // Published crates resolve internal dependencies from crates.io by this
    // version, so it must track the workspace version.
    const workspaceVersion = getStringField(workspacePackage, "version");
    const workspaceDependencies = getSection(rootCargo, "workspace.dependencies") ?? "";
    for (const crateName of ["dioxus-shadcn-core", "dioxus-shadcn-primitives", "dioxus-shadcn"]) {
      const entry = workspaceDependencies.match(new RegExp(`^\\s*${crateName}\\s*=\\s*\\{([^}]*)\\}`, "m"))?.[1];
      const dependencyVersion = entry?.match(/\bversion\s*=\s*"([^"]+)"/)?.[1] ?? null;
      if (entry === undefined || !entry.includes(`path = "crates/${crateName}"`)) {
        failures.push(`root [workspace.dependencies] must declare ${crateName} with path = "crates/${crateName}"`);
      } else if (dependencyVersion !== workspaceVersion) {
        failures.push(
          `root [workspace.dependencies] ${crateName} version "${dependencyVersion}" must match workspace version "${workspaceVersion}"`,
        );
      }
    }

    for (const [field, expectedValues] of Object.entries(expectedWorkspaceArrays)) {
      const actualValues = getStringArrayField(workspacePackage, field);
      if (actualValues === null || actualValues.join("\0") !== expectedValues.join("\0")) {
        failures.push(`root [workspace.package] must set ${field} = [${expectedValues.join(", ")}]`);
      }
    }
  }
}

for (const [crateName, expectedDescription] of Object.entries(expectedDescriptions)) {
  const manifestPath = join(cratesDir, crateName, "Cargo.toml");
  if (!existsSync(manifestPath)) {
    failures.push(`missing publishable crate manifest: ${relative(repoRoot, manifestPath)}`);
    continue;
  }

  const manifest = readText(manifestPath);
  const packageSection = getSection(manifest, "package");
  if (packageSection === null) {
    failures.push(`${relative(repoRoot, manifestPath)} is missing [package]`);
    continue;
  }

  if (getStringField(packageSection, "description") !== expectedDescription) {
    failures.push(`${relative(repoRoot, manifestPath)} has missing or stale package description`);
  }

  for (const field of inheritedPublishFields) {
    if (!hasWorkspaceInheritance(packageSection, field)) {
      failures.push(`${relative(repoRoot, manifestPath)} must inherit package.${field} with ${field}.workspace = true`);
    }
  }
}

if (existsSync(examplesDir)) {
  const exampleManifestPaths = readdirSync(examplesDir, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => join(examplesDir, entry.name, "Cargo.toml"))
    .filter((path) => existsSync(path))
    .sort();

  for (const manifestPath of exampleManifestPaths) {
    const manifest = readText(manifestPath);
    const packageSection = getSection(manifest, "package");
    if (packageSection === null || !hasPublishFalse(packageSection)) {
      failures.push(`${relative(repoRoot, manifestPath)} must keep publish = false`);
    }
  }
}

const metadataFragments = [
  "dioxus-shadcn-core",
  "dioxus-shadcn-primitives",
  "dioxus-shadcn-cli",
  "shared workspace `readme`, `keywords`, and `categories` metadata",
  "Example and verification crates under `examples/` remain application fixtures",
  "does not claim the crates are ready to publish",
  "current `0.2.x` API surface is accepted",
  "CLI template delivery now uses embedded registry/template assets",
];

for (const fragment of metadataFragments) {
  requireFragment("docs/cargo-publish-metadata.md", normalizedMetadataDoc, fragment);
}

const releaseFragments = [
  "npm run verify:cargo-publish-metadata",
  "Cargo publish metadata checks are read-only",
  "crate descriptions, shared README/keywords/categories metadata",
  "they do not run `cargo publish`, run `cargo package`, contact crates.io, replace repository URLs, or create package archives",
];

for (const fragment of releaseFragments) {
  requireFragment("docs/release.md", normalizedReleaseDocs, fragment);
}

const qualityFragments = [
  "`npm run verify:cargo-publish-metadata`",
  "crate descriptions, shared README/keywords/categories metadata",
  "does not run `cargo publish`, run `cargo package`, contact crates.io, replace repository URLs, or create package archives",
];

for (const fragment of qualityFragments) {
  requireFragment("docs/quality-gates.md", normalizedQualityGates, fragment);
}

const siteFragments = [
  "M96 Cargo Publish Metadata Gate Usage",
  "npm run verify:cargo-publish-metadata",
  "shared README/keywords/categories metadata",
];

for (const fragment of siteFragments) {
  requireFragment("docs/site.md", normalizedDocsSite, fragment);
}

if (failures.length > 0) {
  console.error("Cargo publish metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("Cargo publish metadata verification passed");
}
