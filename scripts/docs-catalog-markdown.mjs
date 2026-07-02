#!/usr/bin/env node
import { buildDocsCatalog } from "./docs-catalog-builder.mjs";

function escapeTableCell(value) {
  return String(value ?? "").replaceAll("|", "\\|");
}

export function renderDocsCatalogMarkdown(catalogData = buildDocsCatalog()) {
  const rows = catalogData.catalog.map((item) => [
    `[${item.title}](${item.slug}.md)`,
    item.description,
    `\`${item.source_copy_command}\``,
    `\`${item.crate_feature}\``,
    `\`${item.template_path}\``,
    `\`${item.source_copy_target}\``,
  ]);

  const lines = [
    "# Component Catalog",
    "",
    "This page is generated from the docs catalog builder. Update it by running:",
    "",
    "```bash",
    "node scripts/docs-catalog-markdown.mjs",
    "```",
    "",
    "The catalog is derived from registry entries, templates, component docs,",
    "crate features, and crate modules. It intentionally does not include visual",
    "preview routes, screenshot artifacts, or generated JSON metadata.",
    "",
    `Public components: ${catalogData.summary.publicComponents}`,
    "",
    "| Component | Description | CLI | Feature | Template | Source Target |",
    "| --- | --- | --- | --- | --- | --- |",
    ...rows.map((row) => `| ${row.map(escapeTableCell).join(" | ")} |`),
    "",
  ];

  return lines.join("\n");
}

if (import.meta.url === `file://${process.argv[1]}`) {
  process.stdout.write(renderDocsCatalogMarkdown());
}
