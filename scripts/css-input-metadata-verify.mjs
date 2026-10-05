#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const failures = [];
const forbiddenTailwindV3Directives = [
  "@tailwind base",
  "@tailwind components",
  "@tailwind utilities",
];

const assertIncludes = ({ label, source, fragment }) => {
  if (!source.includes(fragment)) {
    failures.push(`${label} missing fragment: ${fragment}`);
  }
};

const assertExcludes = ({ label, source, fragment }) => {
  if (source.includes(fragment)) {
    failures.push(`${label} must not include Tailwind CSS v3 directive: ${fragment}`);
  }
};

const cliSource = readRepoFile("crates/dioxus-ui-cli/src/main.rs");
const previewCssInputs = [
  {
    label: "examples/web-demo/assets/preview.css",
    source: readRepoFile("examples/web-demo/assets/preview.css"),
  },
  {
    label: "examples/desktop-demo/assets/preview.css",
    source: readRepoFile("examples/desktop-demo/assets/preview.css"),
  },
];

const requiredCliFragments = [
  'const DEFAULT_CSS: &str = r#"@import "tailwindcss";',
  "@custom-variant dark (&:is(.dark *));",
  ":root {",
  "--primary: oklch(0.21 0.006 285.885);",
  ".dark {\n  color-scheme: dark;",
  "@theme inline {",
  "--color-primary: var(--primary);",
  "--color-background: var(--background);",
];

for (const fragment of requiredCliFragments) {
  assertIncludes({
    label: "crates/dioxus-ui-cli/src/main.rs DEFAULT_CSS",
    source: cliSource,
    fragment,
  });
}

for (const directive of forbiddenTailwindV3Directives) {
  assertExcludes({
    label: "crates/dioxus-ui-cli/src/main.rs DEFAULT_CSS",
    source: cliSource,
    fragment: directive,
  });
}

const requiredPreviewFragments = [
  '@import "tailwindcss";',
  '@source "../../../crates";',
  '@source "../src";',
  '@source "../../preview-states/src";',
];

// The previews carry everything the CLI stylesheet holds after its import
// verbatim (the RFC 0051 tokens and the opt-in dark theme), so browser checks
// exercise the stylesheet `dxui init` generates.
const cliImport = 'const DEFAULT_CSS: &str = r#"@import "tailwindcss";\n';
const cliBodyStart = cliSource.indexOf(cliImport);
const cliBody =
  cliBodyStart < 0 ? "" : cliSource.slice(cliBodyStart + cliImport.length, cliSource.indexOf('"#;', cliBodyStart));
if (!cliBody.includes("@theme inline {") || !cliBody.includes("/* Opt-in dark theme")) {
  failures.push("crates/dioxus-ui-cli/src/main.rs DEFAULT_CSS missing the token or opt-in dark theme blocks");
}

for (const { label, source } of previewCssInputs) {
  if (cliBody && !source.includes(cliBody)) {
    failures.push(`${label} token and dark theme blocks differ from the CLI DEFAULT_CSS`);
  }
  for (const fragment of requiredPreviewFragments) {
    assertIncludes({ label, source, fragment });
  }

  for (const directive of forbiddenTailwindV3Directives) {
    assertExcludes({ label, source, fragment: directive });
  }
}

if (failures.length > 0) {
  console.error("CSS input metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log(`CSS input metadata verification passed (${previewCssInputs.length} preview inputs)`);
}
