import { readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const defaultRepoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

export const catalogCategories = [
  { id: "actions", label: "Actions" },
  { id: "forms", label: "Forms" },
  { id: "overlays", label: "Overlays" },
  { id: "navigation", label: "Navigation" },
  { id: "layout", label: "Layout" },
  { id: "data-display", label: "Data Display" },
  { id: "feedback", label: "Feedback" },
  { id: "messaging", label: "Messaging" },
];

const componentCategories = {
  accordion: "layout",
  alert: "feedback",
  "alert-dialog": "overlays",
  "aspect-ratio": "layout",
  attachment: "messaging",
  avatar: "data-display",
  badge: "data-display",
  breadcrumb: "navigation",
  bubble: "messaging",
  button: "actions",
  "button-group": "actions",
  calendar: "forms",
  card: "layout",
  carousel: "layout",
  chart: "data-display",
  checkbox: "forms",
  collapsible: "layout",
  combobox: "overlays",
  command: "actions",
  "context-menu": "overlays",
  "data-table": "data-display",
  "date-picker": "forms",
  dialog: "overlays",
  direction: "layout",
  drawer: "overlays",
  dropdown: "overlays",
  empty: "data-display",
  field: "forms",
  "hover-card": "overlays",
  input: "forms",
  "input-group": "forms",
  "input-otp": "forms",
  item: "layout",
  kbd: "actions",
  label: "forms",
  marker: "messaging",
  menubar: "overlays",
  message: "messaging",
  "message-scroller": "messaging",
  "native-select": "forms",
  "navigation-menu": "navigation",
  pagination: "navigation",
  popover: "overlays",
  progress: "data-display",
  "radio-group": "forms",
  resizable: "layout",
  "scroll-area": "layout",
  select: "forms",
  separator: "layout",
  sheet: "overlays",
  sidebar: "navigation",
  skeleton: "feedback",
  slider: "forms",
  sonner: "feedback",
  spinner: "feedback",
  switch: "forms",
  table: "data-display",
  tabs: "navigation",
  textarea: "forms",
  toast: "feedback",
  toggle: "actions",
  "toggle-group": "actions",
  tooltip: "overlays",
  typography: "data-display",
  fab: "actions",
  dock: "navigation",
  swap: "actions",
  "file-input": "forms",
  "tags-input": "forms",
  "number-input": "forms",
  rating: "forms",
  diff: "data-display",
  countdown: "data-display",
  "radial-progress": "data-display",
  status: "feedback",
  indicator: "layout",
  steps: "navigation",
  timeline: "data-display",
  stat: "data-display",
};

const categoryLabels = new Map(catalogCategories.map((category) => [category.id, category.label]));

// Registry JSON and templates ship inside the CLI crate; registry `source`
// paths are relative to it.
const cliRoot = "crates/dioxus-shadcn-cli";

// Crate source files that are not public components.
const internalCrateModules = new Set(["lib", "modal_focus", "anchored_overlay", "dismiss_timer", "listbox", "roving_group", "hover_open", "default_attribute", "dialog_labels", "menu_marks", "menu_sub", "media_query"]);

function namesFromFiles(repoRoot, dir, extension) {
  return readdirSync(join(repoRoot, dir))
    .filter((file) => file.endsWith(extension))
    .map((file) => file.slice(0, -extension.length))
    .sort();
}

function normalize(name) {
  return name.replaceAll("_", "-");
}

// Acronyms in slugs keep their capitals, as in the component docs' headings.
const acronyms = new Set(["otp"]);

function titleCase(name) {
  return name
    .split("-")
    .map((part) => (acronyms.has(part) ? part.toUpperCase() : `${part.slice(0, 1).toUpperCase()}${part.slice(1)}`))
    .join(" ");
}

function categoryAnchor(category) {
  return `category-${category}`;
}

function parseCargoFeatures(repoRoot) {
  const cargo = readFileSync(join(repoRoot, "crates/dioxus-shadcn/Cargo.toml"), "utf8");
  const featureBlock = cargo.split(/\n\[features\]\n/)[1]?.split(/\n\[/)[0] ?? "";

  return [...featureBlock.matchAll(/^([a-zA-Z0-9_-]+)\s*=/gm)]
    .map((match) => match[1])
    .filter((name) => name !== "default")
    .sort();
}

function parseLibModules(repoRoot) {
  const lib = readFileSync(join(repoRoot, "crates/dioxus-shadcn/src/lib.rs"), "utf8");
  return [...lib.matchAll(/^pub mod ([a-zA-Z0-9_]+);/gm)]
    .map((match) => normalize(match[1]))
    .sort();
}

function readRegistryEntry(repoRoot, name) {
  const registryPath = join(repoRoot, cliRoot, "registry", `${name}.json`);
  return JSON.parse(readFileSync(registryPath, "utf8"));
}

function sourceStats(repoRoot, sourcePath) {
  if (!sourcePath) {
    return {
      bytes: undefined,
      lines: undefined,
    };
  }

  const source = readFileSync(join(repoRoot, sourcePath), "utf8");

  return {
    bytes: Buffer.byteLength(source, "utf8"),
    lines: source.length === 0 ? 0 : source.split("\n").length,
  };
}

function buildCatalogItems(repoRoot, publicComponentNames) {
  return publicComponentNames.map((name) => {
    const entry = readRegistryEntry(repoRoot, name);
    const primaryFile = entry.files?.[0];
    const crateImport = name.replaceAll("-", "_");
    const category = componentCategories[name];
    const templatePath = primaryFile ? `${cliRoot}/${primaryFile.source}` : undefined;
    const stats = sourceStats(repoRoot, templatePath);

    return {
      name,
      category,
      category_label: categoryLabels.get(category),
      description: entry.description,
      registry_path: `${cliRoot}/registry/${name}.json`,
      template_path: templatePath,
      docs_path: `docs/components/${name}.md`,
      markdown_path: `docs/components/${name}.md`,
      docs_route: `/components/${name}`,
      category_anchor: category ? categoryAnchor(category) : undefined,
      category_route: category ? `/components#${categoryAnchor(category)}` : undefined,
      source_route: `/components/${name}/source`,
      source_preview_route: `/components/${name}/source`,
      source_preview_path: templatePath,
      source_preview_target: primaryFile?.target,
      source_preview_language: "rust",
      source_preview_lines: stats.lines,
      source_preview_bytes: stats.bytes,
      crate_feature: name,
      crate_module: `crates/dioxus-shadcn/src/${crateImport}.rs`,
      source_copy_target: primaryFile?.target,
      slug: name,
      title: titleCase(name),
      crate_import: crateImport,
      source_copy_command: `dxui add ${name}`,
      crate_feature_toml: `dioxus-shadcn = { features = ["${name}"] }`,
    };
  });
}

export function buildDocsCatalog(options = {}) {
  const repoRoot = options.repoRoot ?? defaultRepoRoot;
  const sourceCopyHelpers = new Set(options.sourceCopyHelpers ?? ["utils"]);
  const registryNames = namesFromFiles(repoRoot, `${cliRoot}/registry`, ".json")
    .filter((name) => name !== "schema");
  const templateNames = namesFromFiles(repoRoot, `${cliRoot}/templates`, ".rs").map(normalize);
  const crateModuleNames = namesFromFiles(repoRoot, "crates/dioxus-shadcn/src", ".rs")
    .filter((name) => !internalCrateModules.has(name))
    .map(normalize);
  const docsNames = namesFromFiles(repoRoot, "docs/components", ".md");
  const featureNames = parseCargoFeatures(repoRoot);
  const libModuleNames = parseLibModules(repoRoot);
  const publicComponentNames = registryNames.filter((name) => !sourceCopyHelpers.has(name));
  const catalog = buildCatalogItems(repoRoot, publicComponentNames);
  const categoryIds = catalogCategories.map((category) => category.id);
  const missingCategoryNames = publicComponentNames.filter((name) => !componentCategories[name]);
  const unknownCategoryNames = catalog
    .filter((item) => item.category && !categoryIds.includes(item.category))
    .map((item) => item.name);
  const docsRoutes = catalog.map((item) => item.docs_route);
  const duplicateDocsRoutes = docsRoutes.filter((route, index) => docsRoutes.indexOf(route) !== index);
  const sourceRoutes = catalog.map((item) => item.source_route);
  const duplicateSourceRoutes = sourceRoutes.filter((route, index) => sourceRoutes.indexOf(route) !== index);
  const sourcePreviewRoutes = catalog.map((item) => item.source_preview_route);
  const duplicateSourcePreviewRoutes = sourcePreviewRoutes
    .filter((route, index) => sourcePreviewRoutes.indexOf(route) !== index);

  return {
    repoRoot,
    catalog,
    catalogCategories,
    registryNames,
    templateNames,
    crateModuleNames,
    docsNames,
    featureNames,
    libModuleNames,
    publicComponentNames,
    sourceCopyHelpers,
    missingCategoryNames,
    unknownCategoryNames,
    duplicateDocsRoutes: [...new Set(duplicateDocsRoutes)].sort(),
    duplicateSourceRoutes: [...new Set(duplicateSourceRoutes)].sort(),
    duplicateSourcePreviewRoutes: [...new Set(duplicateSourcePreviewRoutes)].sort(),
    summary: {
      publicComponents: catalog.length,
      registryEntries: registryNames.length,
      sourceCopyHelpers: [...sourceCopyHelpers],
      templates: templateNames.length,
      crateModules: crateModuleNames.length,
      crateFeatures: featureNames.length,
      componentDocs: catalog.filter((item) => docsNames.includes(item.name)).length,
      catalogCategories: catalogCategories.length,
      docsRoutes: docsRoutes.length,
      sourcePreviewRoutes: sourcePreviewRoutes.length,
    },
  };
}
