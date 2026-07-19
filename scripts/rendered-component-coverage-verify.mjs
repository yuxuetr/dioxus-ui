#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { buildDocsCatalog } from "./docs-catalog-builder.mjs";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => readFileSync(join(repoRoot, relativePath), "utf8");
const readJson = (relativePath) => JSON.parse(readRepoFile(relativePath));
const normalizeWhitespace = (text) => text.replace(/\s+/g, " ");

const packageJson = readJson("package.json");
const coverage = readJson("docs/components/rendered-coverage.json");
const renderedDoc = normalizeWhitespace(readRepoFile("docs/components/rendered-component-verification.md"));
const releaseDoc = normalizeWhitespace(readRepoFile("docs/release.md"));
const qualityDoc = normalizeWhitespace(readRepoFile("docs/quality-gates.md"));
const siteDoc = normalizeWhitespace(readRepoFile("docs/site.md"));
const sharedPreview = readRepoFile("examples/preview-states/src/lib.rs");
const scripts = packageJson.scripts ?? {};
const { catalog, catalogCategories } = buildDocsCatalog({ repoRoot });
const failures = [];

const knownLevels = new Set(["static", "controlled", "runtime-planned", "app-owned"]);
const knownPanels = new Set(catalogCategories.map((category) => category.id));
const runtimeSensitiveComponents = new Set([
  "alert-dialog",
  "carousel",
  "combobox",
  "context-menu",
  "date-picker",
  "dialog",
  "drawer",
  "dropdown",
  "hover-card",
  "menubar",
  "navigation-menu",
  "popover",
  "resizable",
  "scroll-area",
  "select",
  "sheet",
  "sidebar",
  "sonner",
  "toast",
  "tooltip",
]);

const requireIncludes = (name, text, fragments) => {
  for (const fragment of fragments) {
    if (!text.includes(fragment)) {
      failures.push(`${name} is missing: ${fragment}`);
    }
  }
};

if (coverage.schema_version !== 1) {
  failures.push("docs/components/rendered-coverage.json must set schema_version = 1");
}

if (!Array.isArray(coverage.records)) {
  failures.push("docs/components/rendered-coverage.json must contain records array");
}

const records = Array.isArray(coverage.records) ? coverage.records : [];
const catalogByName = new Map(catalog.map((item) => [item.name, item]));
const recordNames = records.map((record) => record.component);
const duplicateNames = recordNames.filter((name, index) => recordNames.indexOf(name) !== index);
const duplicateTestIds = records
  .map((record) => record.test_id)
  .filter((testId, index, all) => all.indexOf(testId) !== index);

for (const item of catalog) {
  if (!recordNames.includes(item.name)) {
    failures.push(`rendered coverage manifest missing component: ${item.name}`);
  }
}

for (const record of records) {
  const item = catalogByName.get(record.component);
  if (item === undefined) {
    failures.push(`rendered coverage manifest has unknown component: ${record.component}`);
    continue;
  }

  if (record.label !== item.title) {
    failures.push(`${record.component} label must be ${item.title}`);
  }

  if (record.category !== item.category) {
    failures.push(`${record.component} category must be ${item.category}`);
  }

  if (!knownPanels.has(record.panel)) {
    failures.push(`${record.component} panel is unknown: ${record.panel}`);
  }

  if (!knownLevels.has(record.coverage_level)) {
    failures.push(`${record.component} coverage_level is unknown: ${record.coverage_level}`);
  }

  if (record.test_id !== `component-preview-${record.component}`) {
    failures.push(`${record.component} test_id must be component-preview-${record.component}`);
  }

  if (typeof record.notes !== "string" || record.notes.trim().length === 0) {
    failures.push(`${record.component} must include non-empty coverage notes`);
  }

  if (runtimeSensitiveComponents.has(record.component) && record.coverage_level === "static") {
    failures.push(`${record.component} is runtime-sensitive and must not be marked static`);
  }
}

for (const duplicateName of [...new Set(duplicateNames)].sort()) {
  failures.push(`rendered coverage manifest has duplicate component: ${duplicateName}`);
}

for (const duplicateTestId of [...new Set(duplicateTestIds)].sort()) {
  failures.push(`rendered coverage manifest has duplicate test_id: ${duplicateTestId}`);
}

if (scripts["verify:rendered-component-coverage"] !== "node scripts/rendered-component-coverage-verify.mjs") {
  failures.push("package.json verify:rendered-component-coverage script is missing or mismatched");
}

if (!scripts["verify:release"]?.includes("npm run verify:rendered-component-coverage")) {
  failures.push("package.json verify:release must include npm run verify:rendered-component-coverage");
}

requireIncludes("docs/components/rendered-component-verification.md", renderedDoc, [
  "Rendered Component Verification",
  "data-component-preview",
  "npm run verify:rendered-component-coverage",
  "must not start a server, launch a browser, write screenshots, update generated docs, change component APIs, edit templates, or claim visual parity",
]);

requireIncludes("docs/release.md", releaseDoc, [
  "npm run verify:rendered-component-coverage",
  "Rendered component coverage checks are read-only",
  "stable rendered preview target metadata",
]);

requireIncludes("docs/quality-gates.md", qualityDoc, [
  "`npm run verify:rendered-component-coverage`",
  "stable rendered preview target metadata",
  "does not start a server, launch a browser, write screenshots, update generated docs, change component APIs, edit templates, or claim visual parity",
]);

requireIncludes("docs/site.md", siteDoc, [
  "M109 Rendered Component Verification Usage",
  "npm run verify:rendered-component-coverage",
  "stable rendered preview target metadata",
]);

requireIncludes("examples/preview-states/src/lib.rs", sharedPreview, [
  "pub struct ComponentPreviewTarget",
  "pub const COMPONENT_PREVIEW_TARGETS",
  '"data-component-preview": "{target.test_id}"',
]);

for (const record of records) {
  requireIncludes("examples/preview-states/src/lib.rs", sharedPreview, [
    `component: "${record.component}"`,
    `test_id: "${record.test_id}"`,
    `panel: "${record.panel}"`,
    `coverage_level: "${record.coverage_level}"`,
  ]);
}

if (failures.length > 0) {
  console.error("rendered component coverage verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log(`rendered component coverage verification passed (${records.length} components)`);
}
