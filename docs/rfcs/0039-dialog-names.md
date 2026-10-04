# RFC 0039: Dialog Names

- Status: Accepted
- Created: 2026-10-05

## Summary

Name dialogs by their titles. Dialog, Alert Dialog, Sheet, Drawer, and
Popover content generate ids for their title and description parts and point
`aria-labelledby` and `aria-describedby` at the parts that are mounted. Their
content parts pass through attributes, so a dialog without a title can take
`aria-label`.

## Current State

As of M163:

- `DialogContent`, `AlertDialogContent`, `SheetContent`, `DrawerContent`, and
  `PopoverContent` render `role="dialog"` or `role="alertdialog"` with no
  `aria-labelledby` or `aria-describedby`. Their title and description parts
  render a heading and a paragraph without ids. Every dialog has an empty
  accessible name, which the APG dialog pattern does not allow, and a screen
  reader announces only "dialog" on open.
- None of the content parts accepts attributes, so an app cannot add the
  name itself.
- The browser verifier finds dialogs by role without a name.

## Decision

- A shared `use_dialog_labels` hook, called by each content part, allocates
  an id such as `dxui-dialog-3` and provides it through context with two
  flags, one per part.
- Each title and description part reads the context, renders
  `dxui-dialog-3-title` or `dxui-dialog-3-description` as its `id`, and sets
  its flag while it is mounted. Outside a content part it renders no `id`.
- The content renders `aria-labelledby` only while a title is mounted and
  `aria-describedby` only while a description is mounted, so it never
  points at a missing id.
- Content parts extend `GlobalAttributes` and `div`. A passed
  `aria-labelledby` or `aria-label` replaces the generated `aria-labelledby`,
  and a passed `aria-describedby` replaces the generated one, so SSR writes a
  single value.
- In the crate the hook lives in a private module compiled for the five
  features; in source-copy templates it lives in `utils.rs`.

## Scope

In scope:

- the shared hook, the title and description ids, the content attributes, and
  attribute spreading on the content parts in the crate and the template
- the docs pages of the five components
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Hover Card and Date Picker content names | Hover Card content is supplementary and opens on hover; Date Picker content holds a labelled calendar grid | A consumer reports an unnamed Hover Card or Date Picker |
| Attribute spreading on titles, descriptions, headers, and footers | A passed `id` on a title would break the generated link | A consumer needs to style or identify one |
| A warning for a dialog without a title | Dioxus has no development-only warning channel the components use | A consumer ships an unnamed dialog |
| Names in server-rendered HTML before hydration | Title and description parts register in an effect, which SSR does not run, so the first HTML has no `aria-labelledby`; dialogs normally open on the client | A consumer server-renders an open dialog |
| Browser fixtures for Sheet and Drawer | They call the same hook and render the same parts as Dialog; the generated fixture smoke compiles their templates | Their content diverges from Dialog |

## Verification

- The CLI generated fixture smoke keeps the templates compiling.
- `npm run verify:runtime-interactions` asserts that the Dialog, Alert
  Dialog, and Popover fixtures are found by role with their title as the name
  and their description as the accessible description, and that a Popover
  given `aria-label` uses it over its title.
- A closed Popover with neither part renders neither attribute, and a
  page-wide check fails on any `aria-labelledby`, `aria-describedby`, or
  `aria-controls` id that matches no element.
- Reverse checks: not rendering the title id, rendering `aria-labelledby` or
  `aria-describedby` without the part mounted, ignoring a passed
  `aria-label`, or not spreading the content attributes each make the
  verifier fail.
