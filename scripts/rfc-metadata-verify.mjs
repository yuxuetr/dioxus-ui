#!/usr/bin/env node
import { readdirSync, readFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const readFirstLine = (relativePath) => {
  return readRepoFile(relativePath).split("\n")[0]?.trim() ?? "";
};

const readme = readRepoFile("README.md");
const docsReadme = readRepoFile("docs/README.md");
const rfcDir = join(repoRoot, "docs/rfcs");
const rfcFiles = readdirSync(rfcDir)
  .filter((fileName) => fileName.endsWith(".md"))
  .sort();
const failures = [];
const seenNumbers = new Set();
const rfcEntries = [];

for (const fileName of rfcFiles) {
  const fileMatch = fileName.match(/^(\d{4})-([a-z0-9]+(?:-[a-z0-9]+)*)\.md$/);
  if (fileMatch === null) {
    failures.push(`RFC filename must be NNNN-kebab-title.md: ${fileName}`);
    continue;
  }

  const [, number, slug] = fileMatch;
  const relativePath = `docs/rfcs/${fileName}`;
  const docsRelativePath = `rfcs/${fileName}`;
  const firstLine = readFirstLine(relativePath);
  const headingMatch = firstLine.match(/^# RFC (\d{4}): (.+)$/);
  if (headingMatch === null) {
    failures.push(`${relativePath} first heading must be '# RFC ${number}: Title'`);
    continue;
  }

  const [, headingNumber, title] = headingMatch;
  if (headingNumber !== number) {
    failures.push(`${relativePath} heading number ${headingNumber} does not match filename ${number}`);
  }

  if (seenNumbers.has(number)) {
    failures.push(`duplicate RFC number: ${number}`);
  }
  seenNumbers.add(number);

  rfcEntries.push({
    number,
    slug,
    title,
    rootPath: relativePath,
    docsPath: docsRelativePath,
    markdownLabel: `RFC ${number}: ${title}`,
  });
}

const sortedNumbers = [...seenNumbers].sort();
for (let index = 0; index < sortedNumbers.length; index += 1) {
  const expectedNumber = String(index + 1).padStart(4, "0");
  if (sortedNumbers[index] !== expectedNumber) {
    failures.push(`RFC numbers must be contiguous from 0001; expected ${expectedNumber}, got ${sortedNumbers[index]}`);
  }
}

for (const entry of rfcEntries) {
  const rootLink = `[${entry.markdownLabel}](${entry.rootPath})`;
  const docsLink = `[${entry.markdownLabel}](${entry.docsPath})`;

  if (!readme.includes(rootLink)) {
    failures.push(`README.md missing RFC link: ${rootLink}`);
  }

  if (!docsReadme.includes(docsLink)) {
    failures.push(`docs/README.md missing RFC link: ${docsLink}`);
  }
}

if (failures.length > 0) {
  console.error("RFC metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log(`RFC metadata verification passed (${rfcEntries.length} RFCs)`);
}
