#!/usr/bin/env node
// The RFC 0076 ground-truth gate for merging a user class into a component's
// classes. Tailwind compiles every utility, Chrome expands each property into
// longhands, and a component utility must be removed exactly when a user
// utility sets every (state, longhand, importance) slot it sets, in both
// writing directions. A merge candidate runs in `examples/class-merge-gate`.
//
// Usage: node scripts/class-merge-gate.mjs [candidate...]   (default: table)
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { compile } from "@tailwindcss/node";
import { launchBrowser } from "./browser-check-support.mjs";
import { themeInput } from "./preview-tailwind.mjs";
import { declarationsByToken, expandLonghands, isSlot, physical, splitVariants } from "./class-merge-truth.mjs";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const candidates = process.argv.slice(2).length > 0 ? process.argv.slice(2) : ["table"];
const failures = [];

// ---------------------------------------------------------------- corpus

const utilityList = /^[a-z0-9:[\]=/.%_ -]+$/;
const classLists = [];
const crateRoot = join(repoRoot, "crates/dioxus-shadcn/src");
for (const entry of readdirSync(crateRoot).sort()) {
  if (!entry.endsWith(".rs")) continue;
  const source = readFileSync(join(crateRoot, entry), "utf8").split("#[cfg(test)]")[0];
  for (const [, value] of source.matchAll(/const \w+_CLASS: &str =\s*"([^"]*)"/g)) classLists.push(value);
  for (const [, value] of source.matchAll(/"([^"\n]*)"/g)) {
    if (value.includes("-") && utilityList.test(value)) classLists.push(value);
  }
}
const componentTokens = [...new Set(classLists.flatMap((list) => list.split(/\s+/)).filter(Boolean))].sort();

// User utilities: the component vocabulary itself (same group, other values,
// other variants), each utility important and under another state, the
// wider and narrower members of its family, and tokens no merge can know.
const families = [
  ["p", "px", "py", "pt", "pr", "pb", "pl", "ps", "pe"],
  ["m", "mx", "my", "mt", "mr", "mb", "ml", "ms", "me"],
  ["inset", "inset-x", "inset-y", "top", "right", "bottom", "left", "start", "end"],
  ["size", "w", "h"],
  ["gap", "gap-x", "gap-y"],
  ["rounded", "rounded-t", "rounded-r", "rounded-b", "rounded-l", "rounded-s", "rounded-e", "rounded-tl", "rounded-tr", "rounded-br", "rounded-bl", "rounded-ss", "rounded-se", "rounded-ee", "rounded-es"],
  ["border", "border-t", "border-r", "border-b", "border-l", "border-x", "border-y", "border-s", "border-e"],
  ["overflow", "overflow-x", "overflow-y"],
];
const familyPrefixes = families.flat().sort((a, b) => b.length - a.length);
const synthetic = [
  "px-[13px]", "bg-[#123456]", "text-[13px]", "text-[color:var(--app-ink)]", "w-[calc(100%-2rem)]",
  "[mask-type:alpha]", "grid-cols-[1fr_auto]", "bg-(--app-surface)", "app-card", "pxx-4", "bg-primry",
  "text-md", "hover:app-card", "border-b-b",
  // The calibration sample below.
  "data-[state=open]:px-2!", "bg-primary/50",
];
const userSet = new Set([...componentTokens, ...synthetic]);
for (const token of componentTokens) {
  const { variants, utility } = splitVariants(token);
  const prefix = variants.map((variant) => `${variant}:`).join("");
  if (!utility.endsWith("!")) userSet.add(`${token}!`);
  if (!variants.includes("hover")) userSet.add(`hover:${utility}`);
  const negative = utility.startsWith("-") ? "-" : "";
  const bare = utility.slice(negative.length).replace(/!$/, "");
  // `border-b` is a family member with no value, not `border` with value `b`.
  const familyPrefix = familyPrefixes.includes(bare)
    ? undefined
    : familyPrefixes.find((candidate) => bare.startsWith(`${candidate}-`));
  if (familyPrefix) {
    const value = bare.slice(familyPrefix.length + 1);
    for (const sibling of families.find((family) => family.includes(familyPrefix))) {
      userSet.add(`${prefix}${negative}${sibling}-${value}`);
    }
  }
}
const userTokens = [...userSet].sort();
const allTokens = [...new Set([...componentTokens, ...userTokens])];

// ---------------------------------------------------------- ground truth

const compiler = await compile(themeInput, { base: repoRoot, onDependency: () => {} });
const stylesheet = compiler.build(allTokens);
const declarations = declarationsByToken(stylesheet);

// Calibrate the bulk parse against one compile per token for a sample, so a
// parsing mistake cannot quietly shift every expectation.
for (const token of ["px-4", "hover:bg-accent", "data-[state=open]:px-2!", "bg-primary/50", "-translate-x-1/2", "app-card"]) {
  const single = await compile(themeInput, { base: repoRoot, onDependency: () => {} });
  const own = declarationsByToken(single.build([token])).get(token) ?? [];
  const comparable = (list) => JSON.stringify(list.map(({ order, ...rest }) => rest));
  if (comparable(own) !== comparable(declarations.get(token) ?? [])) {
    failures.push(`ground truth: ${token} parses differently alone than in the corpus: ${JSON.stringify(own)} vs ${JSON.stringify(declarations.get(token))}`);
  }
}

const browser = await launchBrowser("scripts/class-merge-gate.mjs");
const page = await browser.newPage();
const properties = [...new Set([...declarations.values()].flat().filter(isSlot).map((d) => d.property))];
const longhands = await expandLonghands(page, properties);
const slots = (token, rtl) =>
  new Set(
    (declarations.get(token) ?? [])
      .filter(isSlot)
      .flatMap((d) => longhands[d.property].map((l) => `${d.state}|${physical(l, rtl)}|${d.important ? "!" : ""}`)),
  );
const ltr = new Map(allTokens.map((token) => [token, slots(token, false)]));
const rtl = new Map(allTokens.map((token) => [token, slots(token, true)]));
const unknownUsers = userTokens.filter((token) => ltr.get(token).size === 0);

const subset = (inner, outer) => inner.size > 0 && [...inner].every((slot) => outer.has(slot));

// Expected removals, through an index from slot to the users that set it.
function removalsBy(slotsOf) {
  const index = new Map();
  userTokens.forEach((user, userIndex) => {
    for (const slot of slotsOf(user)) {
      if (!index.has(slot)) index.set(slot, new Set());
      index.get(slot).add(userIndex);
    }
  });
  const removed = new Set();
  componentTokens.forEach((component, componentIndex) => {
    const own = [...slotsOf(component)];
    if (own.length === 0) return;
    let users = index.get(own[0]) ?? new Set();
    for (const slot of own.slice(1)) users = new Set([...users].filter((user) => index.get(slot)?.has(user)));
    for (const userIndex of users) {
      if (userTokens[userIndex] !== component) removed.add(`${componentIndex},${userIndex}`);
    }
  });
  return removed;
}
const expected = removalsBy((token) => ltr.get(token));
for (const pair of [...expected]) {
  const [componentIndex, userIndex] = pair.split(",").map(Number);
  if (!subset(rtl.get(componentTokens[componentIndex]), rtl.get(userTokens[userIndex]))) expected.delete(pair);
}


// Per token and direction: for each (state, longhand) it sets, the
// declaration that wins inside the token, important first, then the latest.
const cascade = (token, rtl) => {
  const winners = new Map();
  for (const d of (declarations.get(token) ?? []).filter(isSlot)) {
    for (const longhand of longhands[d.property]) {
      const key = `${d.state}|${physical(longhand, rtl)}`;
      const previous = winners.get(key);
      if (!previous || d.important > previous.important || (d.important === previous.important && d.order > previous.order)) {
        winners.set(key, { important: d.important, order: d.order });
      }
    }
  }
  return winners;
};
const cascades = [false, true].map((direction) => new Map(allTokens.map((token) => [token, cascade(token, direction)])));

// Pairs whose utilities set a longhand in the same state, and whether the
// user utility wins every such longhand when both stay in the class list.
// With equal states the selectors have equal specificity, so importance and
// then stylesheet order decide.
const overlaps = new Map();
cascades.forEach((byToken) => {
  const index = new Map();
  userTokens.forEach((user, userIndex) => {
    for (const key of byToken.get(user).keys()) {
      if (!index.has(key)) index.set(key, []);
      index.get(key).push(userIndex);
    }
  });
  componentTokens.forEach((component, componentIndex) => {
    for (const [key, own] of byToken.get(component)) {
      for (const userIndex of index.get(key) ?? []) {
        if (userTokens[userIndex] === component) continue;
        const theirs = byToken.get(userTokens[userIndex]).get(key);
        const wins = theirs.important > own.important || (theirs.important === own.important && theirs.order > own.order);
        const pair = `${componentIndex},${userIndex}`;
        const entry = overlaps.get(pair) ?? { userWins: true, lost: [], shared: [] };
        if (byToken === cascades[0]) entry.shared.push(key);
        if (!wins) {
          entry.userWins = false;
          entry.lost.push(key);
        }
        overlaps.set(pair, entry);
      }
    }
  });
});

// --------------------------------------------------------------- checking

const describe = (pair) => {
  const [componentIndex, userIndex] = pair.split(",").map(Number);
  return `${componentTokens[componentIndex]} by ${userTokens[userIndex]}`;
};

// A removal outside rule 1 drops a style the user did not replace. A kept
// pair is fine unless the component utility still wins a longhand the user
// utility sets: then the override does not apply.
function judge(name, removed, droppedUsers) {
  const falseRemovals = [...removed].filter((pair) => !expected.has(pair));
  const missedRemovals = [...expected].filter((pair) => !removed.has(pair));
  const overridesLost = [...overlaps].filter(([pair, { userWins }]) => !userWins && !removed.has(pair)).map(([pair]) => pair);
  const unfixable = overridesLost.filter((pair) => !expected.has(pair));
  const fixable = overridesLost.filter((pair) => expected.has(pair));
  return {
    name, falseRemovals, missedRemovals, fixable, unfixable, droppedUsers,
    passed: falseRemovals.length + fixable.length + droppedUsers === 0,
  };
}

// The three merges RFC 0076 says the gate must reject.
const root = (token) => {
  const { utility } = splitVariants(token);
  return utility.replace(/^-/, "").replace(/!$/, "").replace(/-[^-]*$/, "");
};
const variantKey = (token) => splitVariants(token).variants.join(":");
const prefixOnly = new Set();
componentTokens.forEach((component, componentIndex) => {
  const componentRoot = root(component);
  if (!componentRoot) return;
  userTokens.forEach((user, userIndex) => {
    if (user !== component && variantKey(user) === variantKey(component) && root(user).startsWith(componentRoot)) {
      prefixOnly.add(`${componentIndex},${userIndex}`);
    }
  });
});
const stateless = (token) => new Set([...ltr.get(token)].map((slot) => slot.replace(/^[^|]*\|/, "|")));
const statelessSlots = new Map(allTokens.map((token) => [token, stateless(token)]));
const reverse = [
  judge("reverse: removes on a shared prefix (p-4 by px-2)", prefixOnly, 0),
  judge("reverse: ignores variants", removalsBy((token) => statelessSlots.get(token)), 0),
  judge("reverse: drops unknown user tokens", expected, unknownUsers.length * componentTokens.length),
];
for (const result of reverse) {
  if (result.passed) failures.push(`${result.name} passed the gate, so the gate cannot tell it from a correct merge`);
}

const corpusDir = mkdtempSync(join(tmpdir(), "class-merge-gate-"));
const corpusPath = join(corpusDir, "corpus.json");
writeFileSync(corpusPath, JSON.stringify({ components: componentTokens, users: userTokens, classLists }));
const results = [];
for (const candidate of candidates) {
  const output = execFileSync(
    "cargo",
    ["run", "--quiet", "--release", "-p", "dioxus-ui-class-merge-gate", "--", candidate, corpusPath],
    { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 },
  );
  const report = JSON.parse(output);
  const result = judge(candidate, new Set(report.removed.map(([c, u]) => `${c},${u}`)), report.droppedUsers.length);
  result.unknown = report.unknown.map((index) => userTokens[index]);
  result.cost = report.cost;
  results.push(result);
}

// Browser: Chrome must agree with the cascade model above. For each longhand
// a pair shares in the plain state (or ::before, ::after), both utilities on
// one element must compute what the user utility forced with `!important`
// computes exactly when the model says the user utility wins it, in the
// left-to-right page. Longhands whose values cannot be told apart are skipped.
const browserPairs = [...overlaps]
  .map(([pair, entry]) => {
    const [componentIndex, userIndex] = pair.split(",").map(Number);
    return { component: componentTokens[componentIndex], user: userTokens[userIndex], ...entry };
  })
  .map((entry) => ({ ...entry, shared: entry.shared.filter((key) => /^&(::before|::after)?\|/.test(key)) }))
  .filter(({ user, shared }) => shared.length > 0 && !user.endsWith("!"));
const forced = await compile(themeInput, { base: repoRoot, onDependency: () => {} });
const browserCss = forced.build([...allTokens, ...browserPairs.map(({ user }) => `${user}!`)]);
await page.setContent(`<!doctype html><html dir="ltr"><head><style>${browserCss}</style></head><body></body></html>`);
const browserReport = await page.evaluate((pairs) => {
  const disagreements = [];
  let decided = 0;
  const value = (className, pseudo, longhand) => {
    const element = document.createElement("div");
    element.className = className;
    document.body.append(element);
    const computed = getComputedStyle(element, pseudo).getPropertyValue(longhand);
    element.remove();
    return computed;
  };
  for (const { component, user, shared, lost } of pairs) {
    for (const key of shared) {
      const [state, longhand] = key.split("|");
      const pseudo = state === "&" ? null : state.slice(1);
      const alone = value(component, pseudo, longhand);
      const wanted = value(`${component} ${user}!`, pseudo, longhand);
      if (alone === wanted) continue;
      decided += 1;
      const applied = value(`${component} ${user}`, pseudo, longhand) === wanted;
      if (applied === lost.includes(key)) {
        disagreements.push(`${component} ${user} ${key}: model ${!lost.includes(key)}, Chrome ${applied}`);
      }
    }
  }
  return { decided, disagreements };
}, browserPairs.map(({ component, user, shared, lost }) => ({ component, user, shared, lost })));
await browser.close();
if (browserReport.disagreements.length > 0) failures.push("Chrome disagrees with the cascade model");

// ----------------------------------------------------------------- report

const sample = (pairs) => pairs.slice(0, 12).map(describe).join("; ");
console.log(
  `corpus: ${componentTokens.length} component utilities, ${userTokens.length} user utilities, ` +
    `${expected.size} expected removals, ${overlaps.size} overlapping pairs, ` +
    `${unknownUsers.length} user tokens Tailwind does not know`,
);
console.log(
  `browser: Chrome decided ${browserReport.decided} shared longhands in the plain state, ${browserReport.disagreements.length} against the cascade model` +
    (browserReport.disagreements.length ? ` (${browserReport.disagreements.slice(0, 12).join("; ")})` : ""),
);
for (const result of reverse) {
  console.log(`${result.name}: rejected (${result.falseRemovals.length} false removals, ${result.fixable.length} lost overrides, ${result.droppedUsers} dropped)`);
}
const unfixable = judge("expected", expected, 0).unfixable;
console.log(
  `\npartial overlaps no merge can fix, where the user utility loses a longhand it shares: ${unfixable.length}` +
    (unfixable.length ? ` (${sample(unfixable)})` : ""),
);
for (const result of results) {
  console.log(`\n${result.name}: ${result.passed ? "PASS" : "FAIL"}`);
  console.log(`  false removals: ${result.falseRemovals.length}${result.falseRemovals.length ? ` (${sample(result.falseRemovals)})` : ""}`);
  console.log(`  lost overrides it could have removed: ${result.fixable.length}${result.fixable.length ? ` (${sample(result.fixable)})` : ""}`);
  console.log(`  kept where removal was allowed, harmless: ${result.missedRemovals.length - result.fixable.length}`);
  console.log(`  dropped user tokens: ${result.droppedUsers}`);
  const knownToTailwind = result.unknown.filter((token) => !unknownUsers.includes(token));
  console.log(`  unclassified user tokens: ${result.unknown.length}, of which Tailwind knows ${knownToTailwind.length}${knownToTailwind.length ? ` (${knownToTailwind.slice(0, 12).join(", ")})` : ""}`);
  for (const [user, { nanosPerCall }] of Object.entries(result.cost)) {
    console.log(`  cost with user class "${user}": ${(nanosPerCall / 1000).toFixed(2)} µs per call`);
  }
  if (!result.passed) failures.push(`${result.name} failed the ground truth`);
}

// CLASS_MERGE_GATE_REPORT=<path> writes every finding, for recording in RFC 0076.
if (process.env.CLASS_MERGE_GATE_REPORT) {
  const pairs = (list) => list.map(describe);
  writeFileSync(
    process.env.CLASS_MERGE_GATE_REPORT,
    JSON.stringify(
      {
        unfixable: pairs(unfixable),
        candidates: results.map((result) => ({
          name: result.name,
          falseRemovals: pairs(result.falseRemovals),
          fixable: pairs(result.fixable),
          unknown: result.unknown,
        })),
      },
      null,
      2,
    ),
  );
}

if (failures.length > 0) {
  console.error("\nclass merge gate failed");
  for (const failure of failures) console.error(`- ${failure}`);
  process.exitCode = 1;
} else {
  console.log("\nclass merge gate passed");
}
