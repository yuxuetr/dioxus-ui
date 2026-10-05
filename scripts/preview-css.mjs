#!/usr/bin/env node
import { writeFileSync } from "node:fs";
import { generatedPreviewCssPath, renderGeneratedPreviewCss } from "./preview-tailwind.mjs";

// Regenerates the compiled stylesheet every preview target links (RFC 0049).
writeFileSync(generatedPreviewCssPath, await renderGeneratedPreviewCss());
console.log(`wrote ${generatedPreviewCssPath}`);
