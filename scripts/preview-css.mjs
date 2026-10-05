#!/usr/bin/env node
import { writeFileSync } from "node:fs";
import { compiledStylesheet, renderCompiledStylesheet } from "./preview-tailwind.mjs";

// Regenerates a committed compiled stylesheet: `preview` (the default), which
// every preview target links (RFC 0049), or `site` (RFC 0052).
const name = process.argv[2] ?? "preview";
const { output } = compiledStylesheet(name);
writeFileSync(output, await renderCompiledStylesheet(name));
console.log(`wrote ${output}`);
