#!/usr/bin/env node
import { readFileSync, readdirSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { utilityConflicts } from "./preview-tailwind.mjs";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const roots = ["crates/dioxus-shadcn/src", "crates/dioxus-shadcn-cli/templates"];

// A class function joins an always-applied `*_BASE_CLASS` with state classes.
// When a state class sets a property the base class already sets, the
// stylesheet order picks the winner, so the state may never show. Pair each
// base class with every other class source the function can reach: other
// constants, string literals, and the literals and constants of helper
// functions and methods in the same file that it calls.
const utilityList = /^[a-z0-9:[\]=/.%_ -]+$/;
const failures = [];
let functionCount = 0;

// The ground truth must see token colors and must not see conflicts between
// names Tailwind does not know; either mistake silently passes every check.
const calibration = await utilityConflicts(["bg-primary bg-accent", "app-card app-card-title"]);
if (calibration.join() !== "bg-primary / bg-accent") {
  failures.push(`ground truth miscalibrated: expected only bg-primary / bg-accent, got [${calibration.join(", ")}]`);
}

for (const root of roots) {
  for (const entry of readdirSync(join(repoRoot, root)).sort()) {
    if (!entry.endsWith(".rs")) {
      continue;
    }
    const path = join(repoRoot, root, entry);
    const source = readFileSync(path, "utf8").split("#[cfg(test)]")[0];
    const constants = new Map(
      [...source.matchAll(/const (\w+_CLASS): &str =\s*"([^"]*)"/g)].map((match) => [
        match[1],
        match[2],
      ]),
    );
    // Functions and the impl they belong to; methods are resolved through the
    // type of the parameter they are called on.
    const implRanges = [...source.matchAll(/\nimpl (\w+) \{[^]*?\n\}\n/g)].map((match) => ({
      type: match[1],
      start: match.index,
      end: match.index + match[0].length,
    }));
    const functions = [...source.matchAll(/fn (\w+)(?:<[^>]*>)?\([^]*?\n\s*\}\n/g)].map(
      (match) => ({
        name: match[1],
        body: match[0],
        impl: implRanges.find((range) => match.index > range.start && match.index < range.end)?.type,
      }),
    );
    const literals = (body) =>
      [...body.matchAll(/"([^"\n]*)"/g)]
        .map((match) => match[1])
        .filter((value) => utilityList.test(value));
    const referencedIn = (body) =>
      [...constants.keys()].filter((constant) => new RegExp(`\\b${constant}\\b`).test(body));
    const calledHelpers = (fn) => {
      const free = functions.filter(
        (other) => !other.impl && other !== fn && new RegExp(`(?<![.\\w])${other.name}\\(`).test(fn.body),
      );
      const methods = [...fn.body.matchAll(/(\w+)\.(\w+)\(/g)].flatMap(([, receiver, method]) => {
        const type = fn.body.match(new RegExp(`\\b${receiver}: (\\w+)`))?.[1];
        return functions.filter((other) => other.impl === type && other.name === method);
      });
      return [...free, ...methods];
    };

    for (const fn of functions) {
      const { name, body } = fn;
      // A constant that is an element's whole class, such as a decorative
      // child's, is never joined with the base class.
      const ownElement = (constant) =>
        new RegExp(`\\bclass: ${constant}\\b`).test(body) &&
        !new RegExp(`Some\\(${constant}\\)|then_some\\(${constant}\\)|\\{ ${constant} \\}`).test(body);
      const referenced = referencedIn(body).filter((constant) => !ownElement(constant));
      const bases = referenced.filter((constant) => constant.endsWith("_BASE_CLASS"));
      if (bases.length === 0) {
        continue;
      }
      functionCount += 1;
      const helpers = calledHelpers(fn);
      const states = [
        ...referenced.filter((constant) => !bases.includes(constant)).map((c) => constants.get(c)),
        ...literals(body),
        ...helpers
          .map((helper) => helper.body)
          .flatMap((helperBody) => [
            ...literals(helperBody),
            ...referencedIn(helperBody)
              .filter((constant) => !constant.endsWith("_BASE_CLASS"))
              .map((constant) => constants.get(constant)),
          ]),
      ];
      for (const base of bases) {
        for (const state of states) {
          for (const conflict of await utilityConflicts([`${constants.get(base)} ${state}`])) {
            const [first, second] = conflict.split(" / ");
            const baseUtilities = constants.get(base).split(/\s+/);
            if (baseUtilities.includes(first) !== baseUtilities.includes(second)) {
              failures.push(`${relative(repoRoot, path)} ${name}: ${base} ${conflict}`);
            }
          }
        }
      }
    }
  }
}

const unique = [...new Set(failures)];
if (unique.length > 0) {
  console.error("Tailwind conflict verification failed");
  for (const failure of unique) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log(`Tailwind conflict verification passed (${functionCount} class functions)`);
}
