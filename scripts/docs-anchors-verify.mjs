#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { dirname, extname, isAbsolute, join, normalize, relative } from "node:path";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const markdownFiles = execFileSync("git", ["ls-files", "*.md"], {
  cwd: repoRoot,
  encoding: "utf8",
})
  .split("\n")
  .filter(Boolean);

const markdownFileSet = new Set(markdownFiles.map((file) => {
  return normalize(join(repoRoot, file));
}));

const inlineLinkPattern = /!?\[[^\]]*\]\(([^)\s]+(?:\s+"[^"]*")?)\)/g;
const referenceDefinitionPattern = /^\s*\[[^\]]+\]:\s+(\S+)/gm;
const headingPattern = /^(#{1,6})\s+(.+?)\s*#*\s*$/gm;
const explicitAnchorPattern = /<a\s+[^>]*(?:id|name)=["']([^"']+)["'][^>]*>/gi;

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

  return destination;
};

const splitDestination = (rawDestination) => {
  const destination = stripDestination(rawDestination);
  const hashIndex = destination.indexOf("#");

  if (hashIndex < 0) {
    return { path: destination, fragment: "" };
  }

  const path = destination.slice(0, hashIndex);
  let fragment = destination.slice(hashIndex + 1);
  const queryIndex = fragment.indexOf("?");
  if (queryIndex >= 0) {
    fragment = fragment.slice(0, queryIndex);
  }

  return {
    path,
    fragment: decodeURIComponent(fragment),
  };
};

const isExternalDestination = (destination) => {
  if (destination.startsWith("//")) {
    return true;
  }

  return externalSchemes.some((scheme) => {
    return destination.toLowerCase().startsWith(scheme);
  });
};

const stripInlineMarkdown = (heading) => {
  return heading
    .replace(/`([^`]+)`/g, "$1")
    .replace(/!\[[^\]]*\]\([^)]*\)/g, "")
    .replace(/\[([^\]]+)\]\([^)]*\)/g, "$1")
    .replace(/[*_~]/g, "")
    .replace(/<[^>]+>/g, "")
    .trim();
};

const slugifyHeading = (heading) => {
  return stripInlineMarkdown(heading)
    .toLowerCase()
    .replace(/[^\p{Letter}\p{Number}\p{Mark}\p{Script=Han}\s-]/gu, "")
    .trim()
    .replace(/\s+/g, "-");
};

const collectAnchors = (markdownFile) => {
  const content = readFileSync(markdownFile, "utf8");
  const anchors = new Set();
  const slugCounts = new Map();

  headingPattern.lastIndex = 0;
  for (const match of content.matchAll(headingPattern)) {
    const headingText = match[2] ?? "";
    const baseSlug = slugifyHeading(headingText);
    if (baseSlug.length === 0) {
      continue;
    }

    const count = slugCounts.get(baseSlug) ?? 0;
    slugCounts.set(baseSlug, count + 1);
    anchors.add(count === 0 ? baseSlug : `${baseSlug}-${count}`);
  }

  explicitAnchorPattern.lastIndex = 0;
  for (const match of content.matchAll(explicitAnchorPattern)) {
    const anchor = match[1] ?? "";
    if (anchor.length > 0) {
      anchors.add(anchor);
    }
  }

  return anchors;
};

const anchorCache = new Map();

const getAnchors = (markdownFile) => {
  if (!anchorCache.has(markdownFile)) {
    anchorCache.set(markdownFile, collectAnchors(markdownFile));
  }

  return anchorCache.get(markdownFile);
};

const missingAnchors = [];

for (const markdownFile of markdownFiles) {
  const absoluteMarkdownFile = normalize(join(repoRoot, markdownFile));
  const content = readFileSync(absoluteMarkdownFile, "utf8");
  const sourceDir = dirname(absoluteMarkdownFile);
  const patterns = [inlineLinkPattern, referenceDefinitionPattern];

  for (const pattern of patterns) {
    pattern.lastIndex = 0;
    for (const match of content.matchAll(pattern)) {
      const rawDestination = match[1] ?? "";
      const destination = stripDestination(rawDestination);

      if (isExternalDestination(destination)) {
        continue;
      }

      const { path, fragment } = splitDestination(rawDestination);
      if (fragment.length === 0) {
        continue;
      }

      const pathWithoutQuery = path.split("?")[0] ?? "";
      const targetFile =
        pathWithoutQuery.length === 0
          ? absoluteMarkdownFile
          : normalize(join(sourceDir, pathWithoutQuery));

      if (
        isAbsolute(pathWithoutQuery) ||
        !targetFile.startsWith(repoRoot) ||
        extname(targetFile) !== ".md" ||
        !markdownFileSet.has(targetFile) ||
        !existsSync(targetFile)
      ) {
        continue;
      }

      const anchors = getAnchors(targetFile);
      if (!anchors.has(fragment)) {
        missingAnchors.push({
          file: markdownFile,
          link: rawDestination,
          target: relative(repoRoot, targetFile),
          fragment,
        });
      }
    }
  }
}

if (missingAnchors.length > 0) {
  console.error("Markdown anchor verification failed");
  for (const { file, link, target, fragment } of missingAnchors) {
    console.error(`- ${file} has missing local anchor: ${link} -> ${target}#${fragment}`);
  }
  process.exitCode = 1;
} else {
  console.log(`Markdown anchor verification passed (${markdownFiles.length} files)`);
}
