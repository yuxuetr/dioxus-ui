#!/usr/bin/env node
import { copyFileSync, existsSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { buildDocsCatalog } from "./docs-catalog-builder.mjs";

// Prepares a `dx build` of the component site for GitHub Pages, which serves
// files only: each site route gets a copy of index.html so a deep link loads
// with status 200, and 404.html lets the router render its not found page.
const publicDir = process.argv[2];
if (!publicDir || !existsSync(join(publicDir, "index.html"))) {
  console.error("usage: node scripts/site-pages.mjs <dx web public dir containing index.html>");
  process.exit(1);
}

const index = join(publicDir, "index.html");
const routes = [
  "docs/getting-started",
  "docs/theming",
  ...buildDocsCatalog().catalog.map((item) => `components/${item.slug}`),
];
for (const route of routes) {
  mkdirSync(join(publicDir, route), { recursive: true });
  copyFileSync(index, join(publicDir, route, "index.html"));
}
copyFileSync(index, join(publicDir, "404.html"));
console.log(`site pages: ${routes.length} route pages and 404.html in ${publicDir}`);
