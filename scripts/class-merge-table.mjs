#!/usr/bin/env node
// Generates the RFC 0076 merge table from Tailwind: for every utility root,
// which value forms Tailwind accepts and which longhand slots each form sets.
// A value form Tailwind does not accept, or one whose slots this table cannot
// tell, is left out, so the merge treats it as unknown and removes nothing.
//
// It writes the crate's copy and the template copy.
//
// Usage: node scripts/class-merge-table.mjs [--check]
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { __unstable__loadDesignSystem, compile } from "@tailwindcss/node";
import { Scanner } from "@tailwindcss/oxide";
import { launchBrowser } from "./browser-check-support.mjs";
import { declarationsByToken, expandLonghands, isSlot, physical } from "./class-merge-truth.mjs";
import { themeInput } from "./preview-tailwind.mjs";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const check = process.argv.includes("--check");
const outputs = ["crates/dioxus-shadcn-core/src/class_merge_table.rs", "crates/dioxus-shadcn-cli/templates/class_merge_table.rs"];

// The types the table is written in, so the table imports nothing and the
// crate and template copies are identical.
const ENUMS = `/// The type of an arbitrary value, as Tailwind infers it or a hint names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
  Color,
  Length,
  Percentage,
  Number,
  Url,
  Image,
  Shadow,
}

/// A value form a utility root accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Value {
  /// The root alone.
  Bare,
  /// A whole number after the root.
  Integer,
  /// A number in quarters after the root.
  Decimal,
  /// A fraction after the root.
  Fraction,
  /// A named value from one of the keyword groups, such as a theme color.
  Keyword(u16),
  /// An arbitrary value in brackets, its kind inferred from the value.
  Literal(Kind),
  /// An arbitrary value with a kind hint, in brackets or parentheses.
  Hinted(Kind),
  /// A CSS variable, in brackets or as the parenthesis shorthand.
  Var,
  /// Any arbitrary value, on a root whose every form sets the same slots.
  Arbitrary,
}`;
const tailwindVersion = createRequire(import.meta.url)("tailwindcss/package.json").version;

const design = await __unstable__loadDesignSystem(themeInput, { base: repoRoot });
const classList = design.getClassList().map(([name]) => name);

// Value forms probed on every functional root. Literal kinds are the ones
// the merge can infer from an arbitrary value; `Other` stands for any other
// arbitrary value: a root that accepts it along with every other form, all
// setting the same slots, takes any arbitrary value.
// Several samples per kind: Tailwind may treat two values the merge infers
// alike differently on one root, such as `0_0_0_1px_#000` and `13px_2px` on
// `bg`, and then the form is left out.
const kinds = {
  Color: ["#123456", "rgb(1_2_3)", "oklch(0.5_0.1_200)"],
  Length: ["13px", "2rem", "-3px"],
  Percentage: ["37%", "100%"],
  Number: ["1.5", "2", "0"],
  Url: ["url(x)"],
  Image: ["linear-gradient(red,blue)", "radial-gradient(red,blue)"],
  Shadow: ["0_0_0_1px_#000", "13px_2px", "0_1px_2px_red"],
};
const probes = (root) => [
  ["Bare", [root]],
  ["Integer", [`${root}-13`, `${root}-2`]],
  ["Decimal", [`${root}-2.25`, `${root}-0.5`]],
  ["Fraction", [`${root}-1/3`]],
  ...Object.entries(kinds).map(([kind, values]) => [`Literal(Kind::${kind})`, values.map((value) => `${root}-[${value}]`)]),
  ...Object.keys(kinds).map((kind) => [`Hinted(Kind::${kind})`, [`${root}-[${kind.toLowerCase()}:var(--x)]`, `${root}-(${kind.toLowerCase()}:--x)`]]),
  ["Var", [`${root}-[var(--x)]`, `${root}-(--x)`]],
  ["Other", [`${root}-[foo(1)]`]],
];

const functional = new Map();
const statics = [];
for (const name of classList) {
  const [parsed] = design.parseCandidate(name);
  if (!parsed) continue;
  if (parsed.kind === "static") {
    statics.push(name);
    continue;
  }
  if (!functional.has(parsed.root)) functional.set(parsed.root, new Set());
  if (parsed.value?.kind === "named") functional.get(parsed.root).add(parsed.value.fraction ?? parsed.value.value);
}
for (const root of design.utilities.keys("functional")) {
  if (!functional.has(root)) functional.set(root, new Set());
}

const probeTokens = [...functional.keys()].flatMap((root) => probes(root).flatMap(([, tokens]) => tokens));
const compiler = await compile(themeInput, { base: repoRoot, onDependency: () => {} });
const declarations = declarationsByToken(compiler.build([...classList, ...probeTokens]));
const properties = [...new Set([...declarations.values()].flat().filter(isSlot).map((d) => d.property))];
// Every property Chrome knows, so an arbitrary property such as `[mask:...]`
// can be classified, not only the ones Tailwind's own utilities set.
const browser = await launchBrowser("scripts/class-merge-table.mjs");
const page = await browser.newPage();
// The longhands come from a computed style; the shorthands are the prefixes
// of their names that Chrome accepts, such as `mask` from `mask-image`.
const cssProperties = await page.evaluate(() => {
  const longhandNames = [...getComputedStyle(document.body)].filter((name) => !name.startsWith("-"));
  const prefixes = longhandNames.flatMap((name) => name.split("-").map((_, end, parts) => parts.slice(0, end + 1).join("-")));
  return [...new Set([...longhandNames, ...prefixes])].filter((name) => CSS.supports(name, "initial"));
});
if (cssProperties.length < 300) throw new Error(`Chrome listed only ${cssProperties.length} CSS properties`);
const longhands = await expandLonghands(page, [...new Set([...properties, ...cssProperties])]);
await browser.close();

// A utility's slots in both directions, as `l:` and `r:` keys, so one
// subset test covers both. A utility that styles other elements, such as
// `divide-x` styling the children, keeps that selector in its slots;
// variants are matched by the merge itself.
const slotKey = (token) => {
  const list = (declarations.get(token) ?? []).filter(isSlot);
  if (list.length === 0) return null;
  const keys = new Set();
  for (const d of list) {
    const state = d.state === "&" ? "" : `${d.state} `;
    for (const longhand of longhands[d.property]) {
      keys.add(`${state}l:${physical(longhand, false)}`);
      keys.add(`${state}r:${physical(longhand, true)}`);
    }
  }
  return [...keys].sort().join("\n");
};

const slotNames = new Map();
const sets = new Map();
const setIndex = (key) => {
  if (!sets.has(key)) {
    for (const slot of key.split("\n")) if (!slotNames.has(slot)) slotNames.set(slot, slotNames.size);
    sets.set(key, sets.size);
  }
  return sets.get(key);
};

const staticRows = statics
  .map((name) => [name, slotKey(name)])
  .filter(([, key]) => key)
  .map(([name, key]) => [name, setIndex(key)])
  .sort(([a], [b]) => (a < b ? -1 : 1));

const numeric = /^(\d+|\d+\.\d+|\d+\/\d+)$/;
const groups = new Map();
const groupIndex = (values) => {
  const key = values.join(" ");
  if (!groups.has(key)) groups.set(key, groups.size);
  return groups.get(key);
};
const rootRows = [];
let skippedRoots = 0;
for (const [root, named] of [...functional].sort(([a], [b]) => (a < b ? -1 : 1))) {
  // A form counts when every sample compiles to the same slots.
  const forms = probes(root)
    .map(([form, tokens]) => {
      const keys = new Set(tokens.map(slotKey));
      return [form, keys.size === 1 ? [...keys][0] : null];
    })
    .filter(([, key]) => key);
  const keywordSets = new Map();
  for (const value of named) {
    if (numeric.test(value)) continue;
    const key = slotKey(`${root}-${value}`);
    if (!key) continue;
    if (!keywordSets.has(key)) keywordSets.set(key, []);
    keywordSets.get(key).push(value);
  }
  const distinct = new Set([...forms.map(([, key]) => key), ...keywordSets.keys()]);
  const arbitrary = (form) => /^(Literal|Hinted|Var|Other)/.test(form);
  // A root whose every form sets the same slots, and which accepts every
  // arbitrary form, takes any arbitrary value: one rule instead of fourteen.
  const anyArbitrary = distinct.size === 1 && forms.filter(([form]) => arbitrary(form)).length === 2 * Object.keys(kinds).length + 2;
  // On a root whose forms differ, Tailwind infers a literal's type and falls
  // back to a default when it cannot; a literal form is kept only where its
  // slots match the same kind named by a hint, so Tailwind inferred that kind.
  const hinted = new Map(forms.filter(([form]) => form.startsWith("Hinted")).map(([form, key]) => [form.replace("Hinted", "Literal"), key]));
  const rules = forms
    .filter(([form]) => !(anyArbitrary && arbitrary(form)) && form !== "Other")
    .filter(([form, key]) => anyArbitrary || !form.startsWith("Literal") || hinted.get(form) === key)
    .map(([form, key]) => [form, setIndex(key)]);
  if (anyArbitrary) rules.push(["Arbitrary", setIndex(forms[0][1])]);
  for (const [key, values] of keywordSets) rules.push([`Keyword(${groupIndex(values.sort())})`, setIndex(key)]);
  if (rules.length === 0) {
    skippedRoots += 1;
    continue;
  }
  rootRows.push([root, rules]);
}

const propertyRows = Object.keys(longhands)
  .filter((name) => !name.startsWith("--"))
  .map((name) => {
    const keys = new Set();
    for (const longhand of longhands[name]) {
      keys.add(`l:${physical(longhand, false)}`);
      keys.add(`r:${physical(longhand, true)}`);
    }
    return [name, setIndex([...keys].sort().join("\n"))];
  })
  .sort(([a], [b]) => (a < b ? -1 : 1));

// Names are stored reversed, so Tailwind's source scan of an app that copies
// the table finds no class names in it and generates no CSS for them.
const reverse = (text) => [...text].reverse().join("");
const byReversed = ([a], [b]) => (reverse(a) < reverse(b) ? -1 : reverse(a) > reverse(b) ? 1 : 0);
const slotOf = (slot) => slotNames.get(slot);
const quote = (text) => JSON.stringify(reverse(text));
const lines = [
  `// Generated by \`node scripts/class-merge-table.mjs\` from Tailwind CSS ${tailwindVersion}`,
  "// and the token stylesheet `dxui init` writes. Do not edit (RFC 0076).",
  "",
  ...ENUMS.split("\n"),
  "",
  "/// Hint names, reversed, and the kind each names.",
  "pub(super) const HINTS: &[(&str, Kind)] = &[",
  ...Object.keys(kinds).map((kind) => `  (${quote(kind.toLowerCase())}, Kind::${kind}),`),
  "];",
  "",
  `/// Slot sets, sorted. A slot is a longhand in one writing direction; ${slotNames.size} in all.`,
  "pub(super) const SETS: &[&[u16]] = &[",
  ...[...sets.keys()].map((key) => `  &[${key.split("\n").map(slotOf).sort((a, b) => a - b).join(", ")}],`),
  "];",
  "",
  "/// Static utilities, by reversed name.",
  "pub(super) const STATIC: &[(&str, u16)] = &[",
  ...[...staticRows].sort(byReversed).map(([name, set]) => `  (${quote(name)}, ${set}),`),
  "];",
  "",
  "/// Properties an arbitrary property can name, by reversed name.",
  "pub(super) const PROPERTIES: &[(&str, u16)] = &[",
  ...[...propertyRows].sort(byReversed).map(([name, set]) => `  (${quote(name)}, ${set}),`),
  "];",
  "",
  "/// Keyword values, reversed, each group in order.",
  "pub(super) const GROUPS: &[&[&str]] = &[",
  ...[...groups.keys()].map((key) => `  &[${key.split(" ").map((value) => [value]).sort(byReversed).map(([value]) => quote(value)).join(", ")}],`),
  "];",
  "",
  "/// Functional roots, by reversed name, with the value forms Tailwind accepts for each.",
  "pub(super) const ROOTS: &[(&str, &[(Value, u16)])] = &[",
  ...[...rootRows].sort(byReversed).map(([root, rules]) => `  (${quote(root)}, &[${rules.map(([form, set]) => `(Value::${form}, ${set})`).join(", ")}]),`),
  "];",
  "",
];
const stale = [];
const source = lines.join("\n");

// Neither copied helper may give Tailwind a class to generate.
const helperSources = ["crates/dioxus-shadcn-cli/templates/class_merge.rs"];
const leaked = [];
for (const [label, content] of [["class_merge_table.rs", source], ...helperSources.map((path) => [path, readFileSync(`${repoRoot}/${path}`, "utf8")])]) {
  const scanner = new Scanner({});
  const candidates = scanner.getCandidatesWithPositions({ content, extension: "rs" }).map(({ candidate }) => candidate);
  // A fresh compiler: build() keeps every candidate it was given before.
  const fresh = await compile(themeInput, { base: repoRoot, onDependency: () => {} });
  const generated = declarationsByToken(fresh.build(candidates));
  // `static` is the Rust keyword the debug report's set of reported messages
  // needs; it costs an app one rule.
  const names = [...generated.keys()].filter((name) => name !== "static");
  if (names.length > 0) leaked.push(`${label}: ${names.slice(0, 20).join(", ")}`);
}
if (leaked.length > 0) {
  console.error(`Tailwind would generate CSS for class names in the class merge helpers:\n${leaked.join("\n")}`);
  process.exit(1);
}

for (const relativePath of outputs) {
  const path = `${repoRoot}/${relativePath}`;
  if (check) {
    let current = "";
    try {
      current = readFileSync(path, "utf8");
    } catch {}
    if (current !== source) stale.push(relativePath);
  } else {
    writeFileSync(path, source);
  }
}
if (check && stale.length > 0) {
  console.error(`${stale.join(" and ")} out of date; run \`node scripts/class-merge-table.mjs\``);
  process.exit(1);
}
console.log(
  `${check ? "class merge tables are current" : "wrote the class merge tables"}: ${staticRows.length} static utilities, ` +
    `${rootRows.length} roots (${skippedRoots} with no form), ${groups.size} keyword groups, ${sets.size} slot sets, ${slotNames.size} slots`,
);
