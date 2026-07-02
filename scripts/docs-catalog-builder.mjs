import { readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const defaultRepoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

function namesFromFiles(repoRoot, dir, extension) {
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

function parseCargoFeatures(repoRoot) {
  const cargo = readFileSync(join(repoRoot, "crates/dioxus-ui/Cargo.toml"), "utf8");
  const featureBlock = cargo.split(/\n\[features\]\n/)[1]?.split(/\n\[/)[0] ?? "";

  return [...featureBlock.matchAll(/^([a-zA-Z0-9_-]+)\s*=/gm)]
    .map((match) => match[1])
    .filter((name) => name !== "default")
    .sort();
}

function parseLibModules(repoRoot) {
  const lib = readFileSync(join(repoRoot, "crates/dioxus-ui/src/lib.rs"), "utf8");
  return [...lib.matchAll(/^pub mod ([a-zA-Z0-9_]+);/gm)]
    .map((match) => normalize(match[1]))
    .sort();
}

function readRegistryEntry(repoRoot, name) {
  const registryPath = join(repoRoot, "registry", `${name}.json`);
  return JSON.parse(readFileSync(registryPath, "utf8"));
}

function buildCatalogItems(repoRoot, publicComponentNames) {
  return publicComponentNames.map((name) => {
    const entry = readRegistryEntry(repoRoot, name);
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
}

export function buildDocsCatalog(options = {}) {
  const repoRoot = options.repoRoot ?? defaultRepoRoot;
  const sourceCopyHelpers = new Set(options.sourceCopyHelpers ?? ["utils"]);
  const registryNames = namesFromFiles(repoRoot, "registry", ".json")
    .filter((name) => name !== "schema");
  const templateNames = namesFromFiles(repoRoot, "templates", ".rs").map(normalize);
  const crateModuleNames = namesFromFiles(repoRoot, "crates/dioxus-ui/src", ".rs")
    .filter((name) => name !== "lib")
    .map(normalize);
  const docsNames = namesFromFiles(repoRoot, "docs/components", ".md");
  const featureNames = parseCargoFeatures(repoRoot);
  const libModuleNames = parseLibModules(repoRoot);
  const publicComponentNames = registryNames.filter((name) => !sourceCopyHelpers.has(name));
  const catalog = buildCatalogItems(repoRoot, publicComponentNames);

  return {
    repoRoot,
    catalog,
    registryNames,
    templateNames,
    crateModuleNames,
    docsNames,
    featureNames,
    libModuleNames,
    publicComponentNames,
    sourceCopyHelpers,
    summary: {
      publicComponents: catalog.length,
      registryEntries: registryNames.length,
      sourceCopyHelpers: [...sourceCopyHelpers],
      templates: templateNames.length,
      crateModules: crateModuleNames.length,
      crateFeatures: featureNames.length,
      componentDocs: catalog.filter((item) => docsNames.includes(item.name)).length,
    },
  };
}
