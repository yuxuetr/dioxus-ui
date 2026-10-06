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
   `px-2` over `p-4`, keeps both, and the gate below must show that the user
   value applies. Where Tailwind's stylesheet order makes it lose instead, no
   merge can help (see [Validation](#validation)); the docs name the
   important modifier for those.
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

### Mechanism: a merge table generated from Tailwind

M209.1 decided between two candidates by the gate below; the generated
table passed it and `tw_merge` did not (see [Validation](#validation)):

| Candidate | For | Against |
| --- | --- | --- |
| `tw_merge` | Maintained, covers the whole Tailwind vocabulary | A new dependency in every copy-mode app; v4 support unverified |
| A merge table generated from Tailwind | Exact for every component utility, no dependency | Classifying arbitrary user tokens needs a parser this project would own |

The table maps every Tailwind utility root to the value forms Tailwind
accepts for it (numbers, fractions, keywords such as theme colors, arbitrary
values by type, `(--x)` variables) and the longhand slots each form sets. A
form it does not list, such as `px-foo`, an app's own theme name, or a
class that is not a utility, is unknown: it removes nothing and is never
removed. `scripts/class-merge-table.mjs` generates it from Tailwind and the
`dxui init` token stylesheet, and fails `--check` when it is stale.

### The ground-truth gate

`scripts/class-merge-gate.mjs` asks Tailwind and Chrome, not a hand-written
list:

- **Corpus.** Every utility in the crate's class constants and class
  literals (814), and as user utilities those same utilities, each one
  important and under `hover:`, the wider and narrower members of its family
  (`p`, `px`, `pt`, ... with the same value), arbitrary values, and tokens
  Tailwind does not know (3,171).
- **Slots.** Tailwind compiles every utility under the token stylesheet; a
  declaration's state is its enclosing at-rules other than `@supports` and
  the selector around the class. Chrome expands each property into
  longhands, and a logical longhand names its physical side in each writing
  direction. Declarations of standard properties that read a `--tw-*`
  variable compose by design and are not slots, as in RFC 0044.
- **Expected removals.** A component utility may be removed exactly when a
  user utility sets every one of its (state, longhand, importance) slots in
  both directions. A removal outside that set fails the gate.
- **Cascade.** Where the merge keeps both utilities and they share a slot,
  equal states mean equal specificity, so importance and then stylesheet
  order decide. A kept pair whose component utility still wins a slot, and
  which rule 1 allowed the merge to remove, fails the gate: an override that
  does not apply. A kept pair rule 1 does not allow removing is a partial
  overlap no merge can fix, and is reported.
- **Browser.** Chrome checks the cascade model: for every slot a pair shares
  in the plain state, both utilities on one element must compute what the
  user utility forced with `!important` computes exactly when the model says
  the user wins it.
- **Reverse checks.** A merge that removes on a shared prefix only (`p-4` by
  `px-2`), one that ignores variants, and one that drops unknown tokens must
  each fail the gate, or the gate fails; so must a cascade model with the
  order reversed, checked once by hand.

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
- No new dependency. The table is about 2,400 generated lines, which
  copy-mode apps receive as a helper (RFC 0074) with the components that
  take a `class`.
- An app's own theme names, such as `bg-brand` from `--color-brand`, are not
  in the table, so they do not replace a component utility; debug builds
  report them, and the important modifier still works. Re-evaluate when an
  issue asks for app theme names to override by position.

## Validation

M209.1 ran the gate on 2026-10-06 with Tailwind CSS 4.3.3 and Chrome from
Playwright, before any component changes:

| | Generated table | `tw_merge` 0.1.22 |
| --- | --- | --- |
| False removals | 0 | 196 |
| Overrides that do not apply although removal was allowed | 0 | 484 |
| User tokens dropped | 0 | 0 |
| User tokens Tailwind knows but the merge cannot classify | 0 | 231 |
| Cost per call, no user class | 0.74 µs | 1.63 µs |
| Cost per call, `mt-4` | 0.92 µs | 1.85 µs |
| Cost per call, `px-2 bg-accent text-sm` | 1.42 µs | 2.53 µs |

Costs are release builds on the development machine, over the crate's
class constants. The busiest site component page, Date Picker, has 442
elements with a class, so even a user class on each would cost under 1 ms
per render with the table; elements without a user class need no merge.

`tw_merge`'s false removals drop styles the user did not replace:

- `group/item` removes the component's `group`, which turns off every
  `group-hover:` style of the component.
- `border-b-primary`, a bottom border color, removes `border-b`, the bottom
  border width, so the border disappears.
- `text-sm` removes `leading-6`, although in Tailwind 4 `leading-*` wins over
  a font size's line height.
- Tokens Tailwind does not know, such as `bg-primry` or `border-b-dashed`,
  remove the component's background or border as if they were colors.

Its lost overrides are utilities it does not know (`ps-*`, `pe-*`,
`bg-(--x)`, `decoration-*`) or whose wider groups it does not know (`sr-only`
over `w-*`, `inset-x-*` over `start-*`, `flex-1` over `shrink-*`).

The gate also found 412 partial overlaps no merge can fix, where Tailwind's
order makes the user utility lose a longhand it shares with a component
utility it does not replace: almost all a logical utility over a physical
one (`ms-1` over `-ml-4`, `rounded-s-none` over `rounded-l-none`), which
overlap in one writing direction only, and a few corner groups
(`rounded-t-none` over `rounded-l-none`). These need the important modifier.

Chrome agreed with the cascade model on all 15,432 shared plain-state
longhands; with the order comparison reversed it disagreed on all of them.
Each reverse merge was rejected, and the table with importance ignored
failed with 5,337 false removals.

Building the corpus showed that `verify:tailwind-conflicts` compiled without
the token stylesheet, so it could not see conflicts between token colors.
Fixing it found the active Menu item losing `text-accent-foreground`.
