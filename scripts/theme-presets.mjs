#!/usr/bin/env node
import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { basename, join } from "node:path";
import { AA_TEXT_RATIO, contrastRatio, formatOklch, parseOklch, tintedContrastRatio } from "./oklch-contrast.mjs";

// Regenerates crates/dioxus-shadcn-cli/themes/ from daisyUI's theme sources
// (packages/daisyui/src/themes), as RFC 0057 maps them.
const sourceDir = process.argv[2];
if (!sourceDir) {
  console.error("usage: node scripts/theme-presets.mjs <daisyui themes dir>");
  process.exit(1);
}
const outputDir = new URL("../crates/dioxus-shadcn-cli/themes/", import.meta.url);
const skipped = new Set(["light", "dark"]);

// A background is a color, or { over, alpha } for the foreground's own tint at
// `alpha` over a color, as `text-destructive` on `bg-destructive/10`.
const passes = (foreground, backgrounds) => {
  return backgrounds.every((background) => {
    const ratio = Array.isArray(background)
      ? contrastRatio(foreground, background)
      : tintedContrastRatio(foreground, background.over, foreground, background.alpha);
    return ratio >= AA_TEXT_RATIO;
  });
};

// Values as the preset file writes them, so the check sees the shipped color.
const rounded = (color) => parseOklch(formatOklch(color));

// Mixes the foreground toward black or white, whichever contrasts more with
// the first background, until every pair passes; black or white always
// reaches 4.58:1, so this ends.
const adjust = (foreground, backgrounds) => {
  if (passes(foreground, backgrounds)) {
    return foreground;
  }
  const black = [0, 0, foreground[2]];
  const white = [1, 0, foreground[2]];
  const target = contrastRatio(black, backgrounds[0]) > contrastRatio(white, backgrounds[0]) ? black : white;
  for (let step = 0; step <= 50; step += 1) {
    const candidate = rounded(mix(foreground, target, step / 50));
    if (passes(candidate, backgrounds)) {
      return candidate;
    }
  }
  throw new Error(`no mix passes for ${formatOklch(foreground)}`);
};

const mix = (from, to, amount) => {
  const hue = from[1] < 0.02 ? to[2] : to[1] < 0.02 ? from[2] : from[2] + (((to[2] - from[2] + 540) % 360) - 180) * amount;
  return [from[0] + (to[0] - from[0]) * amount, from[1] + (to[1] - from[1]) * amount, (hue + 360) % 360];
};

const preset = (name, source) => {
  const colors = Object.fromEntries(
    [...source.matchAll(/--color-([a-z0-9-]+):\s*(oklch\([^)]*\))/g)].map((match) => [match[1], parseOklch(match[2])]),
  );
  const scheme = source.match(/color-scheme:\s*(light|dark)/)?.[1];
  const radius = source.match(/--radius-box:\s*([^;]+);/)?.[1];
  if (!scheme || !radius) {
    throw new Error(`${name}: missing color-scheme or --radius-box`);
  }
  const adjusted = [];
  const fixed = (token, foreground, backgrounds) => {
    const result = adjust(foreground, backgrounds);
    if (result !== foreground) {
      adjusted.push(token);
    }
    return result;
  };

  const surfaces = [colors["base-100"], colors["base-200"]];
  const content = fixed("foreground", colors["base-content"], surfaces);
  let muted = content;
  for (let step = 40; step >= 0; step -= 1) {
    const candidate = rounded(mix(content, colors["base-200"], step / 100));
    if (passes(candidate, surfaces)) {
      muted = candidate;
      break;
    }
  }
  const primaryForeground = fixed("primary-foreground", colors["primary-content"], [colors.primary]);
  const secondaryForeground = fixed("secondary-foreground", colors["secondary-content"], [colors.secondary]);
  const destructive = fixed("destructive", colors.error, [...surfaces, { over: colors["base-100"], alpha: 0.1 }]);
  const destructiveForeground = fixed("destructive-foreground", colors["error-content"], [destructive]);
  const statusForeground = (status) => fixed(`${status}-foreground`, colors[`${status}-content`], [colors[status]]);

  const tokens = [
    ["radius", radius.trim()],
    ["background", colors["base-100"]],
    ["foreground", content],
    ["card", colors["base-100"]],
    ["card-foreground", content],
    ["popover", colors["base-100"]],
    ["popover-foreground", content],
    ["primary", colors.primary],
    ["primary-foreground", primaryForeground],
    ["secondary", colors.secondary],
    ["secondary-foreground", secondaryForeground],
    ["muted", colors["base-200"]],
    ["muted-foreground", muted],
    ["accent", colors["base-200"]],
    ["accent-foreground", content],
    ["destructive", destructive],
    ["destructive-foreground", destructiveForeground],
    ["success", colors.success],
    ["success-foreground", statusForeground("success")],
    ["warning", colors.warning],
    ["warning-foreground", statusForeground("warning")],
    ["info", colors.info],
    ["info-foreground", statusForeground("info")],
    ["border", colors["base-300"]],
    ["input", colors["base-300"]],
    ["ring", colors.primary],
    ["chart-1", colors.primary],
    ["chart-2", colors.secondary],
    ["chart-3", colors.accent],
    ["chart-4", colors.info],
    ["chart-5", colors.success],
    ["sidebar", colors["base-200"]],
    ["sidebar-foreground", content],
    ["sidebar-primary", colors.primary],
    ["sidebar-primary-foreground", primaryForeground],
    ["sidebar-accent", colors["base-200"]],
    ["sidebar-accent-foreground", content],
    ["sidebar-border", colors["base-300"]],
    ["sidebar-ring", colors.primary],
  ];
  const note = adjusted.length > 0 ? `\n   Adjusted for WCAG AA: ${adjusted.join(", ")}.` : "";
  const body = tokens.map(([token, value]) => `  --${token}: ${typeof value === "string" ? value : formatOklch(value)};`).join("\n");
  return `/* dxui theme: ${name} */
/* Ported from the daisyUI ${name} theme (https://daisyui.com), MIT License,
   Copyright (c) 2020 Pouya Saadeghi. Mapped to the dioxus-shadcn tokens by
   RFC 0057.${note} */
[data-theme="${name}"] {
  color-scheme: ${scheme};
${body}
}
`;
};

mkdirSync(outputDir, { recursive: true });
const names = [];
for (const file of readdirSync(sourceDir).filter((file) => file.endsWith(".css")).sort()) {
  const name = basename(file, ".css");
  if (skipped.has(name)) {
    continue;
  }
  writeFileSync(join(outputDir.pathname, file), preset(name, readFileSync(join(sourceDir, file), "utf8")));
  names.push(name);
}
console.log(`theme presets: wrote ${names.length} presets`);
