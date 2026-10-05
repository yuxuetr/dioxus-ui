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

const cliSource = readRepoFile("crates/dioxus-shadcn-cli/src/main.rs");
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
    label: "crates/dioxus-shadcn-cli/src/main.rs DEFAULT_CSS",
    source: cliSource,
    fragment,
  });
}

for (const directive of forbiddenTailwindV3Directives) {
  assertExcludes({
    label: "crates/dioxus-shadcn-cli/src/main.rs DEFAULT_CSS",
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
// verbatim (the RFC 0051 tokens, light and dark), so browser checks
// exercise the stylesheet `dxui init` generates.
const cliImport = 'const DEFAULT_CSS: &str = r#"@import "tailwindcss";\n';
const cliBodyStart = cliSource.indexOf(cliImport);
const cliBody =
  cliBodyStart < 0 ? "" : cliSource.slice(cliBodyStart + cliImport.length, cliSource.indexOf('"#;', cliBodyStart));
if (!cliBody.includes("@theme inline {")) {
  failures.push("crates/dioxus-shadcn-cli/src/main.rs DEFAULT_CSS missing the token blocks");
}
// The dark theme redefines only the tokens (RFC 0051), so app palette classes
// keep their colors under `.dark`.
if (/--color-(?:white|black|zinc|blue|red|green|amber|emerald)-?\d*:/.test(cliBody)) {
  failures.push("crates/dioxus-shadcn-cli/src/main.rs DEFAULT_CSS must not redefine Tailwind palette variables");
}

// The component site (RFC 0052) carries the same body, so it renders the
// stylesheet `dxui init` generates.
const siteCssLabel = "site/assets/site.css";
const siteCss = readRepoFile(siteCssLabel);
if (cliBody && !siteCss.includes(cliBody)) {
  failures.push(`${siteCssLabel} token blocks differ from the CLI DEFAULT_CSS`);
}
for (const fragment of ['@import "tailwindcss";', '@source "../src";', '@source "../../crates/dioxus-shadcn/src";']) {
  assertIncludes({ label: siteCssLabel, source: siteCss, fragment });
}

for (const { label, source } of previewCssInputs) {
  if (cliBody && !source.includes(cliBody)) {
    failures.push(`${label} token blocks differ from the CLI DEFAULT_CSS`);
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
  console.log(`CSS input metadata verification passed (${previewCssInputs.length} preview inputs and the site input)`);
}
