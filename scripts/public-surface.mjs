#!/usr/bin/env node
// Lists the public items of the three library crates from rustdoc JSON (RFC
// 0079): each item's kind, whether it has a doc comment, and where outside its
// own file it is named. Names are matched as whole words, so a common name can
// over-count; the counts decide rules per kind, not per item.
//
// Usage: node scripts/public-surface.mjs [--json path]
import { execFileSync } from "node:child_process";
import { readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const crates = ["dioxus-shadcn-core", "dioxus-shadcn-primitives", "dioxus-shadcn"];
const jsonIndex = process.argv.indexOf("--json");
const jsonOut = jsonIndex === -1 ? null : process.argv[jsonIndex + 1];

// Where a name counts as used, in the order a use is reported: the first
// place that names an item is where it is used.
const PLACES = [
  ["docs", (file) => /^(README\.md|crates\/README\.md|docs\/component-api\.md|docs\/blocks\/.*\.md|docs\/components\/(?!catalog\.md|source-preview\.md).*\.md)$/.test(file)],
  ["site", (file) => file.startsWith("site/src/")],
  ["blocks", (file) => file.startsWith("crates/dioxus-shadcn-cli/blocks/")],
  ["styled", (file) => file.startsWith("crates/dioxus-shadcn/src/")],
  ["cli", (file) => file.startsWith("crates/dioxus-shadcn-cli/src/")],
  ["examples", (file) => /^examples\/[^/]+\/src\//.test(file)],
  ["scripts", (file) => file.startsWith("scripts/")],
];

const targetDir = JSON.parse(
  execFileSync("cargo", ["metadata", "--no-deps", "--format-version", "1"], { cwd: repoRoot, encoding: "utf8" }),
).target_directory;

const filesUnder = (dir) =>
  readdirSync(join(repoRoot, dir)).flatMap((name) => {
    const path = join(dir, name);
    if (name === "node_modules" || name === "target" || name === "dist") return [];
    return statSync(join(repoRoot, path)).isDirectory() ? filesUnder(path) : [path];
  });

// word -> files that name it, and the names component pages list under
// "API Surface", which is a promise whether or not the page uses the name.
const wordFiles = new Map();
const listed = new Set();
const API_SURFACE = /^## API Surface\n([\s\S]*?)(?=^## |(?![\s\S]))/m;
for (const file of ["README.md", "crates/README.md", ...["docs", "site/src", "examples", "scripts", "crates"].flatMap(filesUnder)]) {
  // A crate's lib.rs only re-exports its modules' items.
  if (!/\.(rs|md|mjs|js)$/.test(file) || file.startsWith("docs/archive/") || file.includes("/templates/") || /^crates\/[^/]+\/src\/lib\.rs$/.test(file)) continue;
  let text = readFileSync(join(repoRoot, file), "utf8");
  if (file.startsWith("docs/components/")) {
    for (const [, name] of text.match(API_SURFACE)?.[1].matchAll(/^- `([A-Za-z_][A-Za-z0-9_]*)`/gm) ?? []) listed.add(name);
    text = text.replace(API_SURFACE, "");
  }
  for (const word of new Set(text.match(/[A-Za-z_][A-Za-z0-9_]*/g) ?? [])) {
    if (!wordFiles.has(word)) wordFiles.set(word, new Set());
    wordFiles.get(word).add(file);
  }
}

const kindOf = (name, inner) => {
  if (inner === "function") {
    if (/^[A-Z]/.test(name)) return "component";
    return /_class(es)?$/.test(name) ? "class function" : "function";
  }
  if (inner === "constant") return /_CLASS(ES)?$/.test(name) ? "class constant" : "constant";
  if (inner === "struct" && name.endsWith("Props")) return "props";
  return inner.replace("_", " ");
};

const isHidden = (item) => JSON.stringify(item.attrs ?? []).includes("hidden");

const surface = [];
for (const crate of crates) {
  execFileSync("cargo", ["rustdoc", "-q", "-p", crate, "--all-features", "--", "-Z", "unstable-options", "--output-format", "json"], {
    cwd: repoRoot,
    env: { ...process.env, RUSTC_BOOTSTRAP: "1" },
    stdio: ["ignore", "ignore", "inherit"],
  });
  const doc = JSON.parse(readFileSync(join(targetDir, "doc", `${crate.replaceAll("-", "_")}.json`), "utf8"));
  const externalCrates = new Set(Object.values(doc.external_crates).map((external) => external.name));
  const local = (id) => doc.index[id] !== undefined && (doc.index[id].crate_id ?? 0) === 0;
  const crateSrc = `crates/${crate}/src/`;
  const seen = new Set();

  const record = (item, kind, path) => {
    const file = item.span?.filename ?? "";
    const named = new Set([...(wordFiles.get(item.name) ?? [])].filter((other) => other !== file));
    // The styled crate naming its own item is a use inside the crate.
    const outside = PLACES.filter(([place]) => !(place === "styled" && crate === "dioxus-shadcn"));
    const namedIn = outside.filter(([, test]) => [...named].some(test)).map(([place]) => place);
    const usedIn = namedIn[0];
    const inCrate = [...named].some((other) => other.startsWith(crateSrc));
    // Fields, variants, and inherent methods that lack docs, as missing_docs counts them.
    const parts = [];
    const inner = Object.values(item.inner)[0];
    for (const id of [...(inner?.kind?.plain?.fields ?? []), ...(inner?.variants ?? [])]) parts.push(doc.index[id]);
    // Trait items are public with the trait.
    for (const id of item.inner.trait?.items ?? []) parts.push({ ...doc.index[id], visibility: "public" });
    for (const implId of inner?.impls ?? []) {
      const impl = doc.index[implId]?.inner.impl;
      if (impl && impl.trait === null && !impl.is_synthetic && !impl.blanket_impl) parts.push(...impl.items.map((id) => doc.index[id]));
    }
    surface.push({
      crate,
      path,
      kind,
      file,
      documented: Boolean(item.docs),
      listed: listed.has(item.name),
      undocumentedParts: parts.filter((part) => part && (part.visibility === "public" || "variant" in part.inner) && !part.docs && !isHidden(part)).length,
      usedIn: usedIn ?? (inCrate ? "crate" : "nowhere"),
      namedIn,
    });
  };

  const walk = (moduleId, path) => {
    for (const id of doc.index[moduleId].inner.module.items) {
      const item = doc.index[id];
      if (!item || item.visibility !== "public" || isHidden(item)) continue;
      const [inner, body] = Object.entries(item.inner)[0];
      if (inner === "use") {
        // A `#[component]` re-exports its function from a hidden module, by
        // a path inside the crate; only a path into another crate is a
        // re-export of someone else's item.
        const fromCrate = body.source.split("::")[0];
        if (body.id !== null && local(body.id)) {
          // The first path that reaches an item names it; an item of a
          // private module is public only through a use like this one.
          const target = doc.index[body.id];
          if (seen.has(body.id) || isHidden(target)) continue;
          seen.add(body.id);
          if (body.is_glob) walk(body.id, path);
          else record({ ...target, name: body.name }, kindOf(body.name, Object.keys(target.inner)[0]), `${path}::${body.name}`);
          continue;
        }
        if (body.is_glob || !externalCrates.has(fromCrate)) continue;
        record({ ...item, name: body.name }, "re-export", `${path}::${body.name}`);
        continue;
      }
      if (seen.has(id) || inner === "impl") continue;
      seen.add(id);
      record(item, kindOf(item.name, inner), `${path}::${item.name}`);
      if (inner === "module") walk(id, `${path}::${item.name}`);
    }
  };
  walk(doc.root, crate.replaceAll("-", "_"));
}

const places = ["docs", "site", "blocks", "styled", "cli", "examples", "scripts", "crate", "nowhere"];
console.log("| Crate | Kind | Items | Undocumented | Undocumented parts | Listed | " + places.join(" | ") + " |");
console.log("| --- | --- | ---: | ---: | ---: | ---: | " + places.map(() => "---:").join(" | ") + " |");
const groups = new Map();
for (const item of surface) {
  const key = `${item.crate}\u0000${item.kind}`;
  if (!groups.has(key)) groups.set(key, []);
  groups.get(key).push(item);
}
for (const [key, items] of [...groups].sort()) {
  const [crate, kind] = key.split("\u0000");
  const counts = places.map((place) => items.filter((item) => item.usedIn === place).length);
  const undocumented = items.filter((item) => !item.documented).length;
  const parts = items.reduce((sum, item) => sum + item.undocumentedParts, 0);
  const listedCount = items.filter((item) => item.listed).length;
  console.log(`| ${crate} | ${kind} | ${items.length} | ${undocumented} | ${parts} | ${listedCount} | ${counts.join(" | ")} |`);
}

if (jsonOut) writeFileSync(jsonOut, `${JSON.stringify(surface, null, 2)}\n`);
