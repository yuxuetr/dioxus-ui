#!/usr/bin/env node
import { buildDocsCatalog } from "./docs-catalog-builder.mjs";

function escapeTableCell(value) {
  return String(value ?? "").replaceAll("|", "\\|");
}

export function renderDocsRouteManifestMarkdown(catalogData = buildDocsCatalog()) {
  const categoryRows = catalogData.catalogCategories.map((category) => [
    category.label,
    `/components#category-${category.id}`,
  ]);
  const componentRows = catalogData.catalog.map((item) => [
    `[${item.title}](${item.slug}.md)`,
    item.docs_route,
    item.markdown_path,
    item.category_route,
    item.source_route,
  ]);

  const lines = [
    "# Docs Route Manifest",
    "",
    "This page is generated from the docs catalog builder. Update it by running:",
    "",
    "```bash",
    "node scripts/docs-route-manifest-markdown.mjs",
    "```",
    "",
    "The manifest defines static route metadata for a future Dioxus docs runtime.",
    "It does not create rendered routes, router code, screenshots, or generated JSON.",
    "",
    `Component routes: ${catalogData.summary.docsRoutes}`,
    `Category routes: ${catalogData.summary.catalogCategories}`,
    "",
    "## Top-level Routes",
    "",
    "| Route | Purpose |",
    "| --- | --- |",
    "| `/components` | Component catalog index |",
    "| `/components/{slug}` | Component detail page |",
    "| `/components#category-{category}` | Grouped catalog anchor |",
    "| `/components/{slug}/source` | Future generated source preview |",
    "",
    "## Category Routes",
    "",
    "| Category | Route |",
    "| --- | --- |",
    ...categoryRows.map((row) => `| ${row.map(escapeTableCell).join(" | ")} |`),
    "",
    "## Component Routes",
    "",
    "| Component | Route | Markdown | Category Route | Source Route |",
    "| --- | --- | --- | --- | --- |",
    ...componentRows.map((row) => `| ${row.map(escapeTableCell).join(" | ")} |`),
    "",
  ];

  return lines.join("\n");
}

if (import.meta.url === `file://${process.argv[1]}`) {
  process.stdout.write(renderDocsRouteManifestMarkdown());
}
