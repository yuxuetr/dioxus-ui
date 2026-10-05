#!/usr/bin/env node
import { readFileSync, readdirSync } from "node:fs";
import { AA_TEXT_RATIO, TEXT_PAIRS, parseOklch, tintedContrastRatio } from "./oklch-contrast.mjs";

// Checks the theme presets (RFC 0057) and the default theme: every preset sets
// the default theme's tokens under its marker and data-theme selector, keeps
// daisyUI's notice, and every text pair meets WCAG AA.
const themesDir = new URL("../crates/dioxus-shadcn-cli/themes/", import.meta.url);
const cliSource = readFileSync(new URL("../crates/dioxus-shadcn-cli/src/main.rs", import.meta.url), "utf8");
const failures = [];

const declarations = (block) => {
  return new Map([...block.matchAll(/--([a-z0-9-]+):\s*([^;]+);/g)].map((match) => [match[1], match[2].trim()]));
};

const checkPairs = (label, tokens) => {
  for (const [foreground, background, [tint, alpha] = [background, 0]] of TEXT_PAIRS) {
    const color = (token) => parseOklch(tokens.get(token));
    const ratio = tintedContrastRatio(color(foreground), color(background), color(tint), alpha);
    if (ratio < AA_TEXT_RATIO) {
      const surface = alpha > 0 ? `${tint}/${alpha * 100} over ${background}` : background;
      failures.push(`${label}: ${foreground} on ${surface} is ${ratio.toFixed(2)}:1, below ${AA_TEXT_RATIO}:1`);
    }
  }
};

const rootBlock = cliSource.match(/\n:root \{([^}]*)\}/)?.[1];
const darkBlock = cliSource.match(/\n\.dark \{([^}]*)\}/)?.[1];
if (!rootBlock || !darkBlock) {
  throw new Error("could not find the default theme's :root and .dark blocks in the CLI stylesheet");
}
const defaultTokens = declarations(rootBlock);
checkPairs("default theme (light)", defaultTokens);
checkPairs("default theme (dark)", new Map([...defaultTokens, ...declarations(darkBlock)]));

const files = readdirSync(themesDir).filter((file) => file.endsWith(".css")).sort();
if (files.length === 0) {
  failures.push("crates/dioxus-shadcn-cli/themes/ has no presets");
}
for (const file of files) {
  const name = file.replace(/\.css$/, "");
  const label = `themes/${file}`;
  const source = readFileSync(new URL(file, themesDir), "utf8");
  if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(name)) {
    failures.push(`${label}: preset names must be kebab-case`);
  }
  if (!source.startsWith(`/* dxui theme: ${name} */\n`)) {
    failures.push(`${label}: must start with /* dxui theme: ${name} */`);
  }
  if (!source.includes("MIT License") || !source.includes("Copyright (c) 2020 Pouya Saadeghi")) {
    failures.push(`${label}: missing the daisyUI MIT notice`);
  }
  const block = source.match(new RegExp(`\\[data-theme="${name}"\\] \\{([^}]*)\\}`))?.[1];
  if (!block) {
    failures.push(`${label}: missing the [data-theme="${name}"] rule`);
    continue;
  }
  if (!/color-scheme: (light|dark);/.test(block)) {
    failures.push(`${label}: missing color-scheme`);
  }
  const tokens = declarations(block);
  const missing = [...defaultTokens.keys()].filter((token) => !tokens.has(token));
  const extra = [...tokens.keys()].filter((token) => !defaultTokens.has(token));
  if (missing.length > 0 || extra.length > 0) {
    failures.push(`${label}: tokens differ from the default theme (missing: ${missing.join(", ") || "none"}; extra: ${extra.join(", ") || "none"})`);
    continue;
  }
  checkPairs(label, tokens);
}

if (failures.length > 0) {
  console.error("theme preset verification failed:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}
console.log(`theme preset verification passed (${files.length} presets and the default theme)`);
