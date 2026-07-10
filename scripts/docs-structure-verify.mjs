#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { buildDocsCatalog } from "./docs-catalog-builder.mjs";

const { catalog, repoRoot } = buildDocsCatalog();

const normalizeHeading = (value) => {
  return value
    .replace(/\bOtp\b/g, "OTP")
    .replace(/\bKbd\b/g, "Kbd")
    .trim();
};

const sectionBody = (markdown, heading) => {
  const pattern = new RegExp(`^## ${heading}\\s*$`, "m");
  const match = markdown.match(pattern);
  if (!match || match.index === undefined) {
    return undefined;
  }

  const start = match.index + match[0].length;
  const rest = markdown.slice(start);
  const nextHeading = rest.search(/^## /m);

  return nextHeading === -1 ? rest : rest.slice(0, nextHeading);
};

const hasNonEmptyBody = (markdown, heading) => {
  const body = sectionBody(markdown, heading);
  return body !== undefined && body.trim().length > 0;
};

const hasBullet = (markdown, heading) => {
  const body = sectionBody(markdown, heading);
  return body !== undefined && /^- \S/m.test(body);
};

const featurePattern = (feature) => {
  return new RegExp(`features\\s*=\\s*\\[[^\\]]*"${feature}"[^\\]]*\\]`);
};

const failures = [];

for (const item of catalog) {
  const docsPath = join(repoRoot, item.markdown_path);
  const markdown = readFileSync(docsPath, "utf8");
  const firstLine = markdown.split("\n")[0]?.trim() ?? "";
  const expectedTitle = `# ${normalizeHeading(item.title)}`;

  if (firstLine !== expectedTitle) {
    failures.push(`${item.markdown_path}: expected first heading "${expectedTitle}", found "${firstLine}"`);
  }

  const sourceCopy = sectionBody(markdown, "Source Copy");
  if (sourceCopy === undefined) {
    failures.push(`${item.markdown_path}: missing "Source Copy" section`);
  } else if (!sourceCopy.includes(item.source_copy_command)) {
    failures.push(`${item.markdown_path}: Source Copy section must include "${item.source_copy_command}"`);
  }

  const crateFeature = sectionBody(markdown, "Crate Feature");
  if (crateFeature === undefined) {
    failures.push(`${item.markdown_path}: missing "Crate Feature" section`);
  } else if (!featurePattern(item.crate_feature).test(crateFeature)) {
    failures.push(`${item.markdown_path}: Crate Feature section must include feature "${item.crate_feature}"`);
  }

  if (!hasBullet(markdown, "API Surface")) {
    failures.push(`${item.markdown_path}: API Surface section must include at least one bullet`);
  }

  if (!hasNonEmptyBody(markdown, "Accessibility Notes")) {
    failures.push(`${item.markdown_path}: Accessibility Notes section must include non-empty prose`);
  }
}

if (failures.length > 0) {
  console.error("component docs structure verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log(`component docs structure verification passed (${catalog.length} component docs)`);
}
