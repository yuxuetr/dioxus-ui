#!/usr/bin/env node
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join } from "node:path";

const repoRoot = new URL("..", import.meta.url).pathname;

function namesFromFiles(dir, extension) {
  return readdirSync(join(repoRoot, dir))
    .filter((file) => file.endsWith(extension))
    .map((file) => file.slice(0, -extension.length))
    .sort();
}

function normalize(name) {
  return name.replaceAll("_", "-");
}

function titleCase(name) {
  return name
    .split("-")
    .map((part) => `${part.slice(0, 1).toUpperCase()}${part.slice(1)}`)
    .join(" ");
}

function parseCargoFeatures() {
  const cargo = readFileSync(join(repoRoot, "crates/dioxus-ui/Cargo.toml"), "utf8");
  const featureBlock = cargo.split(/\n\[features\]\n/)[1]?.split(/\n\[/)[0] ?? "";

  return [...featureBlock.matchAll(/^([a-zA-Z0-9_-]+)\s*=/gm)]
    .map((match) => match[1])
    .filter((name) => name !== "default")
    .sort();
}

function parseLibModules() {
  const lib = readFileSync(join(repoRoot, "crates/dioxus-ui/src/lib.rs"), "utf8");
  return [...lib.matchAll(/^pub mod ([a-zA-Z0-9_]+);/gm)]
    .map((match) => normalize(match[1]))
    .sort();
}

function readRegistryEntry(name) {
  const registryPath = join(repoRoot, "registry", `${name}.json`);
  return JSON.parse(readFileSync(registryPath, "utf8"));
}

const registryNames = namesFromFiles("registry", ".json").filter((name) => name !== "schema");
const templateNames = namesFromFiles("templates", ".rs").map(normalize);
const crateModuleNames = namesFromFiles("crates/dioxus-ui/src", ".rs")
  .filter((name) => name !== "lib")
  .map(normalize);
const docsNames = namesFromFiles("docs/components", ".md");
const featureNames = parseCargoFeatures();
const libModuleNames = parseLibModules();

const sourceCopyHelpers = new Set(["utils"]);
const publicComponentNames = registryNames.filter((name) => !sourceCopyHelpers.has(name));

const catalog = publicComponentNames.map((name) => {
  const entry = readRegistryEntry(name);
  const primaryFile = entry.files?.[0];
  const crateImport = name.replaceAll("-", "_");

  return {
    name,
    description: entry.description,
    registry_path: `registry/${name}.json`,
    template_path: primaryFile?.source,
    docs_path: `docs/components/${name}.md`,
    crate_feature: name,
    crate_module: `crates/dioxus-ui/src/${crateImport}.rs`,
    source_copy_target: primaryFile?.target,
    slug: name,
    title: titleCase(name),
    crate_import: crateImport,
    source_copy_command: `dxui add ${name}`,
    crate_feature_toml: `dioxus-ui = { features = ["${name}"] }`,
  };
});

const failures = [];

for (const item of catalog) {
  if (!item.description) failures.push(`${item.name}: missing registry description`);
  if (!item.template_path) failures.push(`${item.name}: missing primary template path`);
  if (!item.source_copy_target) failures.push(`${item.name}: missing source-copy target`);

  if (!existsSync(join(repoRoot, item.registry_path))) {
    failures.push(`${item.name}: missing registry path ${item.registry_path}`);
  }

  if (!existsSync(join(repoRoot, item.template_path))) {
    failures.push(`${item.name}: missing template path ${item.template_path}`);
  }

  if (!existsSync(join(repoRoot, item.docs_path))) {
    failures.push(`${item.name}: missing docs path ${item.docs_path}`);
  }

  if (!existsSync(join(repoRoot, item.crate_module))) {
    failures.push(`${item.name}: missing crate module ${item.crate_module}`);
  }

  if (!featureNames.includes(item.crate_feature)) {
    failures.push(`${item.name}: missing crate feature ${item.crate_feature}`);
  }

  if (!libModuleNames.includes(item.name)) {
    failures.push(`${item.name}: missing lib.rs module export`);
  }
}

const missingRegistryTemplates = registryNames.filter((name) => !templateNames.includes(name));
const extraTemplates = templateNames.filter((name) => !registryNames.includes(name));
const extraCrateModules = crateModuleNames.filter((name) => !publicComponentNames.includes(name));
const extraFeatures = featureNames.filter((name) => !publicComponentNames.includes(name));

for (const name of missingRegistryTemplates) {
  failures.push(`${name}: registry entry missing template file`);
}

for (const name of extraTemplates) {
  failures.push(`${name}: template file missing registry entry`);
}

for (const name of extraCrateModules) {
  failures.push(`${name}: crate module missing public registry component`);
}

for (const name of extraFeatures) {
  failures.push(`${name}: crate feature missing public registry component`);
}

if (failures.length > 0) {
  console.error("docs catalog verification failed:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("docs catalog verification passed");
  console.log(
    JSON.stringify(
      {
        publicComponents: catalog.length,
        registryEntries: registryNames.length,
        sourceCopyHelpers: [...sourceCopyHelpers],
        templates: templateNames.length,
        crateModules: crateModuleNames.length,
        crateFeatures: featureNames.length,
        componentDocs: catalog.filter((item) => existsSync(join(repoRoot, item.docs_path))).length,
        sample: catalog.slice(0, 3),
      },
      null,
      2,
    ),
  );
}
