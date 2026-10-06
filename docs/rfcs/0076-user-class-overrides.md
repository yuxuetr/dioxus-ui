# RFC 0076: User Class Overrides

- Status: Accepted
- Created: 2026-10-06

## Summary

A class passed through a component's `class` prop overrides the component's
own utilities for the same CSS property, as the last class wins in shadcn/ui's
`cn()`. The merge must never drop a component utility that the user class
does not fully replace, and every merge it does not understand must leave the
classes as they are, so an override can fail visibly but never remove a style
silently. The merge mechanism is chosen by a measured experiment against
Tailwind's own output before any component changes.

## Current State

[RFC 0044](0044-tailwind-utility-conflicts.md) keeps component class
functions free of conflicting utilities and appends the user class last, but
Tailwind orders utilities in the stylesheet by property and name, not by
their position in the class list. `Button { class: "px-2" }` renders
`... h-10 px-4 ... px-2`, and which padding applies depends on the
stylesheet, not on the order the user wrote. The component API docs tell
users to add Tailwind's important modifier (`px-2!`), and the fixtures use it
twice.

Measured on 2026-10-06 at `v0.4.2`:

- 81 of the 82 crate modules append a user class, at 352 call sites of
  `classes([... Some(class)])`; the templates repeat them.
- RFC 0044 deferred merging "until consumers report that the important
  modifier is not enough". The maintainer now decides that the last class
  must win: shadcn/ui users expect it, and the important modifier also
  beats the component's own state variants, such as `hover:` and
  `data-[state=open]:`, which an override of the base value should keep.
- Rust crates implementing tailwind-merge: `tw_merge` 0.1.22 (updated
  2026-09-24, about 29,500 recent downloads) and `tailwind_fuse` 0.3.2 (last
  updated 2025-01-19). Neither has been checked against the Tailwind v4
  classes these components use: token colors such as `bg-primary`, data
  variants such as `data-[state=open]:`, and `!` important suffixes.

## Requirements

1. **Last wins.** When a user utility sets every property a component
   utility sets, under the same variants and importance, the component
   utility is removed.
2. **No silent loss.** A component utility is removed only under rule 1. A
   user utility that sets some of a component utility's properties, such as
   `px-2` over `p-4`, keeps both, and the browser check below must show that
   the user value applies.
3. **Unknown means unchanged.** A user token the merge cannot classify
   (arbitrary values it does not parse, plugin or app utilities, typos) is
   appended without removing anything, which is today's behavior. In debug
   builds the merge reports which tokens it removed and which it could not
   classify, so an override that did not apply can be traced without reading
   the stylesheet.
4. **One place.** Every class function merges through one helper, in the
   crate and in the templates, so the behavior cannot drift per component.
5. **Same output everywhere.** The merge is a pure function of the component
   and user class strings, so SSR, hydration, and every renderer produce the
   same class attribute.

## Decision

### Behavior

The user class wins over component utilities by rule 1, starting with the
0.5.0 release, which lists it as a breaking change: an app that relied on a
component utility surviving its own class gets the user's value instead. The
important modifier keeps working and stops being needed for plain overrides.

### Mechanism: chosen by experiment

Two candidates, decided in M209.1 by the gate below, not by preference:

| Candidate | For | Against |
| --- | --- | --- |
| `tw_merge` | Maintained, covers the whole Tailwind vocabulary | A new dependency in every copy-mode app; v4 support unverified |
| A merge table generated from Tailwind | Exact for every component utility, no dependency | Classifying arbitrary user tokens needs a parser this project would own |

### The ground-truth gate

Tailwind decides which utilities conflict, so the gate asks Tailwind:

- **Corpus.** Every utility a class function can emit, which
  `npm run verify:tailwind-conflicts` already enumerates, paired with user
  utilities of the same group (`px-4` with `px-2`, `bg-primary` with
  `bg-secondary`, `data-[state=open]:bg-accent` with
  `data-[state=open]:bg-muted`), of wider and narrower groups (`p-2` with
  `px-4`, `px-2` with `p-4`), under other variants (`hover:px-2` with
  `px-4`), and with the important modifier.
- **Expected result.** `utilityConflicts` in `scripts/preview-tailwind.mjs`
  compiles each utility with the Tailwind Node API and returns the
  properties it sets under each variant. A component utility must be removed
  exactly when its property set is a subset of the user utility's under the
  same variants, and kept otherwise.
- **Check.** The merge's output must match the expected result for every
  pair. One false removal fails the gate; a missed removal also fails it,
  since it is an override that does not apply.
- **Browser.** For a sample covering each group, a rendered component with a
  user override must compute the user's value in Chrome with compiled
  Tailwind, including the narrower-group case of rule 2.
- **Reverse checks.** A merge that removes on a shared prefix only (`p-4` by
  `px-2`), one that ignores variants, and one that drops unknown tokens must
  each fail the gate.

A candidate that fails the gate on the corpus is rejected. If both pass, the
cheaper one per render wins, measured over the site's component pages.

## Alternatives

| Option | Why not |
| --- | --- |
| Keep the important modifier (RFC 0044) | Works, but makes the last class lose by default, and beats the component's state variants |
| Expose the component classes and let apps build their own | Every app repeats the merge; copy-mode users can already edit the source |
| Remove the base value from components so user classes never conflict | Components would render unstyled without app classes |

## Impact

- Breaking for apps that pass a class conflicting with a component utility
  and expect the component's value; noted in the 0.5.0 Migration section.
- `docs/component-api.md`, the RFC 0044 class rules, and the known
  limitations change when M209.2 lands, not before.
- If `tw_merge` is chosen, copy-mode apps add it to their `Cargo.toml`, and
  `dxui add` must say so, since it does not edit manifests.

## Validation

M209.1 records the gate results for both candidates here before M209.2
changes any component.
