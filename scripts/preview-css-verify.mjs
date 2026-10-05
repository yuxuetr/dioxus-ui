#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { relative } from "node:path";
import { compiledStylesheet, renderCompiledStylesheet } from "./preview-tailwind.mjs";

// A committed compiled stylesheet must match a fresh compile of the current
// classes, or manual previews, the Desktop and Mobile self-tests, and the
// site run stale styles (RFC 0049, RFC 0052).
const name = process.argv[2] ?? "preview";
const { output } = compiledStylesheet(name);
const label = relative(process.cwd(), output);
const expected = await renderCompiledStylesheet(name);
const actual = existsSync(output) ? readFileSync(output, "utf8") : null;

if (actual !== expected) {
  console.error(`${label} is ${actual === null ? "missing" : "out of date"}`);
  console.error(`Run \`npm run ${name === "preview" ? "css:preview" : "css:site"}\` and commit the result.`);
  process.exitCode = 1;
} else {
  console.log(`${name} stylesheet verification passed (${expected.length} bytes)`);
}
