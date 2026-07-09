#!/usr/bin/env node
import { writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { buildDocsCatalog } from "./docs-catalog-builder.mjs";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const statusPath = join(repoRoot, "docs/components/status.md");

const sortByCategoryThenName = (left, right) => {
  if (left.category_label === right.category_label) {
    return left.name.localeCompare(right.name);
  }

  return left.category_label.localeCompare(right.category_label);
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
    "## Public Components",
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
