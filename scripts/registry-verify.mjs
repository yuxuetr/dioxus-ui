#!/usr/bin/env node
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const cliRoot = join(repoRoot, "crates/dioxus-shadcn-cli");
const slugPattern = /^[a-z][a-z0-9-]*$/;
// Components in `registry/`, and the helpers they share in `helpers/`
// (RFC 0074), which `dxui list` does not show.
const files = ["registry", "helpers"].flatMap((dir) =>
  readdirSync(join(cliRoot, dir))
    .filter((file) => file.endsWith(".json") && file !== "schema.json")
    .sort()
    .map((file) => `${dir}/${file}`),
);
const entryName = (file) => file.slice(file.indexOf("/") + 1, -".json".length);
const registryNames = new Set(files.map(entryName));
const seenNames = new Set();
const seenTargets = new Map();
const failures = [];

const isNonEmptyString = (value) => {
  return typeof value === "string" && value.trim().length > 0;
};

const readEntry = (file) => {
  const path = join(cliRoot, file);
  try {
    return JSON.parse(readFileSync(path, "utf8"));
  } catch (error) {
    failures.push(`${file}: invalid JSON: ${error.message}`);
    return undefined;
  }
};

const validateMappings = (file, entryName, label, mappings, options = {}) => {
  const { requireEntries = false } = options;

  if (!Array.isArray(mappings)) {
    failures.push(`${file}: ${label} must be an array`);
    return;
  }

  if (requireEntries && mappings.length === 0) {
    failures.push(`${file}: ${label} must include at least one mapping`);
  }

  for (const [index, mapping] of mappings.entries()) {
    const prefix = `${file}: ${label}[${index}]`;

    if (mapping === null || typeof mapping !== "object" || Array.isArray(mapping)) {
      failures.push(`${prefix} must be an object`);
      continue;
    }

    const keys = Object.keys(mapping).sort();
    if (keys.join(",") !== "source,target") {
      failures.push(`${prefix} must contain only source and target fields`);
    }

    if (!isNonEmptyString(mapping.source)) {
      failures.push(`${prefix}.source must be a non-empty string`);
    } else if (!existsSync(join(repoRoot, "crates/dioxus-shadcn-cli", mapping.source))) {
      failures.push(`${prefix}.source does not exist: ${mapping.source}`);
    }

    if (!isNonEmptyString(mapping.target)) {
      failures.push(`${prefix}.target must be a non-empty string`);
    } else if (label === "files" && !mapping.target.startsWith("src/components/ui/")) {
      failures.push(`${prefix}.target must stay under src/components/ui/: ${mapping.target}`);
    }

    if (label === "files" && isNonEmptyString(mapping.target)) {
      const owner = seenTargets.get(mapping.target);
      if (owner && owner !== entryName) {
        failures.push(`${prefix}.target duplicates ${owner}: ${mapping.target}`);
      } else {
        seenTargets.set(mapping.target, entryName);
      }
    }
  }
};

for (const file of files) {
  const expectedName = entryName(file);
  const entry = readEntry(file);
  if (!entry) {
    continue;
  }

  const keys = Object.keys(entry).sort();
  const allowedKeys = ["assets", "dependencies", "description", "files", "name"];
  for (const key of keys) {
    if (!allowedKeys.includes(key)) {
      failures.push(`${file}: unsupported field "${key}"`);
    }
  }

  if (entry.name !== expectedName) {
    failures.push(`${file}: name must match filename "${expectedName}"`);
  }

  if (!isNonEmptyString(entry.name) || !slugPattern.test(entry.name)) {
    failures.push(`${file}: name must use component slug format`);
  } else if (seenNames.has(entry.name)) {
    failures.push(`${file}: duplicate registry name "${entry.name}"`);
  } else {
    seenNames.add(entry.name);
  }

  if (!isNonEmptyString(entry.description)) {
    failures.push(`${file}: description must be a non-empty string`);
  }

  validateMappings(file, expectedName, "files", entry.files, { requireEntries: true });

  if (entry.dependencies !== undefined) {
    if (!Array.isArray(entry.dependencies)) {
      failures.push(`${file}: dependencies must be an array`);
    } else {
      const seenDependencies = new Set();
      for (const dependency of entry.dependencies) {
        if (!isNonEmptyString(dependency) || !slugPattern.test(dependency)) {
          failures.push(`${file}: dependency must use component slug format`);
        } else if (!registryNames.has(dependency)) {
          failures.push(`${file}: unknown dependency "${dependency}"`);
        } else if (seenDependencies.has(dependency)) {
          failures.push(`${file}: duplicate dependency "${dependency}"`);
        } else {
          seenDependencies.add(dependency);
        }
      }
    }
  }

  if (entry.assets !== undefined) {
    validateMappings(file, expectedName, "assets", entry.assets);
  }
}

if (failures.length > 0) {
  console.error("registry metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log(`registry metadata verification passed (${files.length} entries)`);
}
