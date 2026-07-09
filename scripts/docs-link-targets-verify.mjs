#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { dirname, isAbsolute, join, normalize } from "node:path";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const markdownFiles = execFileSync("git", ["ls-files", "*.md"], {
  cwd: repoRoot,
  encoding: "utf8",
})
  .split("\n")
  .filter(Boolean);

const inlineLinkPattern = /!?\[[^\]]*\]\(([^)\s]+(?:\s+"[^"]*")?)\)/g;
const referenceDefinitionPattern = /^\s*\[[^\]]+\]:\s+(\S+)/gm;

const externalSchemes = [
  "http:",
  "https:",
  "mailto:",
  "tel:",
  "file:",
  "data:",
  "javascript:",
];

const stripDestination = (rawDestination) => {
  let destination = rawDestination.trim();

  if (destination.startsWith("<") && destination.includes(">")) {
    destination = destination.slice(1, destination.indexOf(">"));
  } else {
    destination = destination.split(/\s+/)[0] ?? "";
  }

  const hashIndex = destination.indexOf("#");
  if (hashIndex >= 0) {
    destination = destination.slice(0, hashIndex);
  }

  const queryIndex = destination.indexOf("?");
  if (queryIndex >= 0) {
    destination = destination.slice(0, queryIndex);
  }

  return decodeURIComponent(destination);
};

const shouldCheckDestination = (destination) => {
  if (destination.length === 0) {
    return false;
  }

  if (destination.startsWith("#") || destination.startsWith("//")) {
    return false;
  }

  if (isAbsolute(destination)) {
    return false;
  }

  return !externalSchemes.some((scheme) => {
    return destination.toLowerCase().startsWith(scheme);
  });
};

const missingTargets = [];

for (const markdownFile of markdownFiles) {
  const absoluteMarkdownFile = join(repoRoot, markdownFile);
  const content = readFileSync(absoluteMarkdownFile, "utf8");
  const sourceDir = dirname(absoluteMarkdownFile);
  const patterns = [inlineLinkPattern, referenceDefinitionPattern];

  for (const pattern of patterns) {
    pattern.lastIndex = 0;
    for (const match of content.matchAll(pattern)) {
      const rawDestination = match[1] ?? "";
      const destination = stripDestination(rawDestination);

      if (!shouldCheckDestination(destination)) {
        continue;
      }

      const absoluteTarget = normalize(join(sourceDir, destination));
      if (!absoluteTarget.startsWith(repoRoot) || !existsSync(absoluteTarget)) {
        missingTargets.push({
          file: markdownFile,
          link: rawDestination,
          target: destination,
        });
      }
    }
  }
}

if (missingTargets.length > 0) {
  console.error("Markdown link target verification failed");
  for (const { file, link, target } of missingTargets) {
    console.error(`- ${file} has missing local link target: ${link} -> ${target}`);
  }
  process.exitCode = 1;
} else {
  console.log(`Markdown link target verification passed (${markdownFiles.length} files)`);
}
