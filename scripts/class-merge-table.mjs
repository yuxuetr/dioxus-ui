#!/usr/bin/env node
// Generates the RFC 0076 merge table from Tailwind: for every utility root,
// which value forms Tailwind accepts and which longhand slots each form sets.
// A value form Tailwind does not accept, or one whose slots this table cannot
// tell, is left out, so the merge treats it as unknown and removes nothing.
//
// Usage: node scripts/class-merge-table.mjs [--check] <output.rs>
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { __unstable__loadDesignSystem, compile } from "@tailwindcss/node";
import { launchBrowser } from "./browser-check-support.mjs";
import { declarationsByToken, expandLonghands, isSlot, physical } from "./class-merge-truth.mjs";
import { themeInput } from "./preview-tailwind.mjs";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const args = process.argv.slice(2);
const check = args.includes("--check");
const output = args.find((arg) => !arg.startsWith("--"));
if (!output) {
  console.error("usage: node scripts/class-merge-table.mjs [--check] <output.rs>");
  process.exit(2);
}
const tailwindVersion = createRequire(import.meta.url)("tailwindcss/package.json").version;

const design = await __unstable__loadDesignSystem(themeInput, { base: repoRoot });
const classList = design.getClassList().map(([name]) => name);

// Value forms probed on every functional root. Literal kinds are the ones
// the merge can infer from an arbitrary value; `Other` stands for any other
// arbitrary value: a root that accepts it along with every other form, all
// setting the same slots, takes any arbitrary value.
const kinds = {
  Color: "#123456",
  Length: "13px",
  Percentage: "37%",
  Number: "1.5",
  Url: "url(x)",
  Image: "linear-gradient(red,blue)",
  Shadow: "0_0_0_1px_#000",
};
const probes = (root) => [
  ["Bare", root],
  ["Integer", `${root}-13`],
  ["Decimal", `${root}-2.25`],
  ["Fraction", `${root}-1/3`],
  ...Object.entries(kinds).map(([kind, value]) => [`Literal(Kind::${kind})`, `${root}-[${value}]`]),
  ...Object.keys(kinds).map((kind) => [`Hinted(Kind::${kind})`, `${root}-[${kind.toLowerCase()}:var(--x)]`]),
  ["Var", `${root}-[var(--x)]`],
  ["Other", `${root}-[foo(1)]`],
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

const probeTokens = [...functional.keys()].flatMap((root) => probes(root).map(([, token]) => token));
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
  const forms = probes(root)
    .map(([form, token]) => [form, slotKey(token)])
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
  const rules = forms
    .filter(([form]) => !(anyArbitrary && arbitrary(form)) && form !== "Other")
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

const slotOf = (slot) => slotNames.get(slot);
const quote = (text) => JSON.stringify(text);
const lines = [
  `// Generated by \`node scripts/class-merge-table.mjs\` from Tailwind CSS ${tailwindVersion}`,
  "// and the token stylesheet `dxui init` writes. Do not edit (RFC 0076).",
  "",
  "use super::{Kind, Value};",
  "",
  `/// Slot sets, sorted. A slot is a longhand in one writing direction; ${slotNames.size} in all.`,
  "pub(super) const SETS: &[&[u16]] = &[",
  ...[...sets.keys()].map((key) => `  &[${key.split("\n").map(slotOf).sort((a, b) => a - b).join(", ")}],`),
  "];",
  "",
  "/// Static utilities, sorted by name.",
  "pub(super) const STATIC: &[(&str, u16)] = &[",
  ...staticRows.map(([name, set]) => `  (${quote(name)}, ${set}),`),
  "];",
  "",
  "/// Properties an arbitrary property such as `[mask-type:alpha]` can name, sorted.",
  "pub(super) const PROPERTIES: &[(&str, u16)] = &[",
  ...propertyRows.map(([name, set]) => `  (${quote(name)}, ${set}),`),
  "];",
  "",
  "/// Keyword values, each group sorted.",
  "pub(super) const GROUPS: &[&[&str]] = &[",
  ...[...groups.keys()].map((key) => `  &[${key.split(" ").map(quote).join(", ")}],`),
  "];",
  "",
  "/// Functional roots, sorted, with the value forms Tailwind accepts for each.",
  "pub(super) const ROOTS: &[(&str, &[(Value, u16)])] = &[",
  ...rootRows.map(([root, rules]) => `  (${quote(root)}, &[${rules.map(([form, set]) => `(Value::${form}, ${set})`).join(", ")}]),`),
  "];",
  "",
];
const source = lines.join("\n");

if (check) {
  let current = "";
  try {
    current = readFileSync(output, "utf8");
  } catch {}
  if (current !== source) {
    console.error(`${output} is out of date; run \`node scripts/class-merge-table.mjs ${output}\``);
    process.exit(1);
  }
  console.log(`${output} is current`);
} else {
  writeFileSync(output, source);
  console.log(
    `wrote ${output}: ${staticRows.length} static utilities, ${rootRows.length} roots (${skippedRoots} with no form), ` +
      `${groups.size} keyword groups, ${sets.size} slot sets, ${slotNames.size} slots`,
  );
}
