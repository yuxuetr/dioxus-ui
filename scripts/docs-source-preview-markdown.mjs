#!/usr/bin/env node
import { buildDocsCatalog } from "./docs-catalog-builder.mjs";

function escapeTableCell(value) {
  return String(value ?? "").replaceAll("|", "\\|");
}

export function renderDocsSourcePreviewMarkdown(catalogData = buildDocsCatalog()) {
  const rows = catalogData.catalog.map((item) => [
    `[${item.title}](${item.slug}.md)`,
    item.source_preview_route,
    item.source_preview_path,
    item.source_preview_target,
    item.source_preview_language,
    item.source_preview_lines,
    item.source_preview_bytes,
  ]);

  const lines = [
    "# Source Preview Manifest",
    "",
    "This page is generated from the docs catalog builder. Update it by running:",
    "",
    "```bash",
    "node scripts/docs-source-preview-markdown.mjs",
    "```",
    "",
    "The manifest defines template metadata for future source preview routes.",
    "It does not embed full template source, syntax highlighting, rendered routes,",
    "screenshots, or generated JSON.",
    "",
    `Source preview routes: ${catalogData.summary.sourcePreviewRoutes}`,
    "",
    "## Source Preview Routes",
    "",
    "| Component | Route | Template | Target | Language | Lines | Bytes |",
    "| --- | --- | --- | --- | --- | --- | --- |",
    ...rows.map((row) => `| ${row.map(escapeTableCell).join(" | ")} |`),
    "",
  ];

  return lines.join("\n");
}

if (import.meta.url === `file://${process.argv[1]}`) {
  process.stdout.write(renderDocsSourcePreviewMarkdown());
}
