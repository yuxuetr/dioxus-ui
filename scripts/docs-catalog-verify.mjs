#!/usr/bin/env node
import { existsSync } from "node:fs";
import { join } from "node:path";
import { buildDocsCatalog } from "./docs-catalog-builder.mjs";

const {
  repoRoot,
  catalog,
  registryNames,
  templateNames,
  crateModuleNames,
  featureNames,
  libModuleNames,
  publicComponentNames,
  sourceCopyHelpers,
  missingCategoryNames,
  unknownCategoryNames,
  duplicateDocsRoutes,
  duplicateSourceRoutes,
  duplicateSourcePreviewRoutes,
  summary,
} = buildDocsCatalog();

const failures = [];

for (const item of catalog) {
  if (!item.description) failures.push(`${item.name}: missing registry description`);
  if (!item.category) failures.push(`${item.name}: missing catalog category`);
  if (!item.category_label) failures.push(`${item.name}: missing catalog category label`);
  if (!item.docs_route) failures.push(`${item.name}: missing docs route`);
  if (!item.markdown_path) failures.push(`${item.name}: missing markdown path`);
  if (!item.category_anchor) failures.push(`${item.name}: missing category anchor`);
  if (!item.category_route) failures.push(`${item.name}: missing category route`);
  if (!item.source_route) failures.push(`${item.name}: missing source route`);
  if (!item.source_preview_route) failures.push(`${item.name}: missing source preview route`);
  if (!item.source_preview_path) failures.push(`${item.name}: missing source preview path`);
  if (!item.source_preview_target) failures.push(`${item.name}: missing source preview target`);
  if (item.source_preview_language !== "rust") failures.push(`${item.name}: invalid source preview language`);
  if (!Number.isInteger(item.source_preview_lines) || item.source_preview_lines <= 0) {
    failures.push(`${item.name}: invalid source preview line count`);
  }
  if (!Number.isInteger(item.source_preview_bytes) || item.source_preview_bytes <= 0) {
    failures.push(`${item.name}: invalid source preview byte count`);
  }
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

for (const name of missingCategoryNames) {
  failures.push(`${name}: missing catalog grouping metadata`);
}

for (const name of unknownCategoryNames) {
  failures.push(`${name}: unknown catalog grouping category`);
}

for (const route of duplicateDocsRoutes) {
  failures.push(`${route}: duplicate docs route`);
}

for (const route of duplicateSourceRoutes) {
  failures.push(`${route}: duplicate source route`);
}

for (const route of duplicateSourcePreviewRoutes) {
  failures.push(`${route}: duplicate source preview route`);
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
        ...summary,
        sample: catalog.slice(0, 3),
      },
      null,
      2,
    ),
  );
}
