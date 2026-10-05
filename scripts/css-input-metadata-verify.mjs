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
  "@theme {",
  "--color-background: var(--dxui-background);",
  "--color-foreground: var(--dxui-foreground);",
  ":root {",
  "--dxui-background: #ffffff;",
  "--dxui-foreground: #09090b;",
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

// The previews carry the CLI's opt-in dark theme block verbatim, so browser
// checks exercise the theme `dxui init` generates.
const darkThemeStart = cliSource.indexOf("/* Opt-in dark theme");
const darkThemeBlock =
  darkThemeStart < 0 ? "" : cliSource.slice(darkThemeStart, cliSource.indexOf("\n}\n", darkThemeStart) + 3);
if (!darkThemeBlock.includes(".dark {")) {
  failures.push("crates/dioxus-ui-cli/src/main.rs DEFAULT_CSS missing the opt-in dark theme block");
}

for (const { label, source } of previewCssInputs) {
  if (darkThemeBlock && !source.includes(darkThemeBlock)) {
    failures.push(`${label} dark theme block differs from the CLI DEFAULT_CSS block`);
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
