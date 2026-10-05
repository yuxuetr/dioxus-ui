#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { relative } from "node:path";
import { generatedPreviewCssPath, renderGeneratedPreviewCss } from "./preview-tailwind.mjs";

// The committed preview stylesheet must match a fresh compile of the current
// classes, or manual previews and the Desktop and Mobile self-tests run stale
// styles (RFC 0049).
const label = relative(process.cwd(), generatedPreviewCssPath);
const expected = await renderGeneratedPreviewCss();
const actual = existsSync(generatedPreviewCssPath) ? readFileSync(generatedPreviewCssPath, "utf8") : null;

if (actual !== expected) {
  console.error(`${label} is ${actual === null ? "missing" : "out of date"}`);
  console.error("Run `npm run css:preview` and commit the result.");
  process.exitCode = 1;
} else {
  console.log(`preview stylesheet verification passed (${expected.length} bytes)`);
}
