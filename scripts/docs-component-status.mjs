#!/usr/bin/env node
import { writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { buildDocsCatalog } from "./docs-catalog-builder.mjs";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const statusPath = join(repoRoot, "docs/components/component-status.md");

const sortByCategoryThenName = (left, right) => {
  if (left.category_label === right.category_label) {
    return left.name.localeCompare(right.name);
  }

  return left.category_label.localeCompare(right.category_label);
};

const mark = (value) => {
  return value ? "yes" : "no";
};

const hasText = (value) => {
  return typeof value === "string" && value.length > 0;
};

const coverageFor = (item) => {
  const coverage = {
    docs: hasText(item.markdown_path),
    template: hasText(item.template_path),
    target: hasText(item.source_copy_target),
    feature: hasText(item.crate_feature),
    module: hasText(item.crate_module),
    sourcePreview: hasText(item.source_preview_route) && hasText(item.source_preview_path),
  };

  return {
    ...coverage,
    complete: Object.values(coverage).every(Boolean),
  };
};

export function renderComponentStatusMarkdown(options = {}) {
  const {
    catalog,
    catalogCategories,
    sourceCopyHelpers,
    summary,
  } = buildDocsCatalog(options);
  const sortedCatalog = [...catalog].sort(sortByCategoryThenName);
  const sortedSourceCopyHelpers = [...sourceCopyHelpers].sort();
  const coverageRecords = sortedCatalog.map((item) => {
    return { item, coverage: coverageFor(item) };
  });
  const completeCoverageCount = coverageRecords.filter(({ coverage }) => {
    return coverage.complete;
  }).length;
  const incompleteCoverageCount = coverageRecords.length - completeCoverageCount;
  const categoryCounts = new Map();

  for (const item of sortedCatalog) {
    categoryCounts.set(item.category, (categoryCounts.get(item.category) ?? 0) + 1);
  }

  const lines = [
    "# Component Status",
    "",
    "This page is generated from the docs catalog builder. Update it by running:",
    "",
    "```bash",
    "node scripts/docs-component-status.mjs",
    "```",
    "",
    "It reflects the local implementation surface: registry entries, source-copy",
    "templates, crate features, styled crate modules, and component docs pages.",
    "It does not refresh live upstream shadcn/ui parity and does not claim visual",
    "parity.",
    "",
    "## Summary",
    "",
    `- Public components: ${summary.publicComponents}`,
    `- Registry entries: ${summary.registryEntries}`,
    `- Source-copy helpers: ${sortedSourceCopyHelpers.length === 0 ? "none" : sortedSourceCopyHelpers.join(", ")}`,
    `- Templates: ${summary.templates}`,
    `- Crate modules: ${summary.crateModules}`,
    `- Crate features: ${summary.crateFeatures}`,
    `- Component docs pages: ${summary.componentDocs}`,
    `- Complete local wiring: ${completeCoverageCount}`,
    `- Incomplete local wiring: ${incompleteCoverageCount}`,
    "",
    "## Category Counts",
    "",
    "| Category | Components |",
    "| --- | ---: |",
  ];

  for (const category of catalogCategories) {
    lines.push(`| ${category.label} | ${categoryCounts.get(category.id) ?? 0} |`);
  }

  lines.push(
    "",
    "## Coverage Matrix",
    "",
    "| Component | Category | Docs | Template | Target | Feature | Module | Source Preview | Complete |",
    "| --- | --- | --- | --- | --- | --- | --- | --- | --- |",
  );

  for (const { item, coverage } of coverageRecords) {
    lines.push(
      `| ${item.title} | ${item.category_label} | ${mark(coverage.docs)} | ${mark(coverage.template)} | ${mark(coverage.target)} | ${mark(coverage.feature)} | ${mark(coverage.module)} | ${mark(coverage.sourcePreview)} | ${mark(coverage.complete)} |`,
    );
  }

  lines.push(
    "",
    "## Public Component Details",
    "",
    "| Component | Category | Docs | CLI | Feature | Template | Target |",
    "| --- | --- | --- | --- | --- | --- | --- |",
  );

  for (const item of sortedCatalog) {
    lines.push(
      `| ${item.title} | ${item.category_label} | [docs](${item.markdown_path.replace("docs/components/", "")}) | \`${item.source_copy_command}\` | \`${item.crate_feature}\` | \`${item.template_path}\` | \`${item.source_copy_target}\` |`,
    );
  }

  lines.push("");

  return lines.join("\n");
}

if (import.meta.url === `file://${process.argv[1]}`) {
  writeFileSync(statusPath, renderComponentStatusMarkdown(), "utf8");
}
