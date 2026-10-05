# RFC 0046: Listbox Width Follows Trigger

- Status: Accepted
- Created: 2026-10-05

## Summary

Open Select and Combobox lists at least as wide as their trigger. The
anchoring script exposes the anchor's width as `--dxui-anchor-width` on the
content, and the Select and Combobox content classes use it as a minimum
width.

## Current State

As of M170:

- `SelectContent` and `ComboboxContent` are `min-w-32` and otherwise size to
  their options. A full-width Select trigger, 255px wide in the preview,
  opens a 64px list under its left edge; a Combobox input does the same.
  Browser checks with compiled Tailwind showed this in the open-state
  screenshots.
- The anchoring script measures the anchor on every placement but shares
  only the position with the content.

## Decision

- The anchoring script sets `--dxui-anchor-width` on the content to the
  anchor's width in pixels before it measures the content, and removes it
  with the other placement styles when the content closes. A viewport point
  anchor, as in a context menu, has a width of 0.
- `SELECT_CONTENT_BASE_CLASS` and `COMBOBOX_CONTENT_BASE_CLASS` replace
  `min-w-32` with `min-w-[max(8rem,var(--dxui-anchor-width,0px))]`, so a list
  is never narrower than 8rem or than its trigger, and long options can still
  make it wider. Before the script runs, as in server-rendered HTML, the
  fallback keeps the 8rem minimum.
- Other anchored content keeps sizing to its content; menus, popovers, and
  the calendar are not meant to match their trigger.

## Scope

In scope:

- the custom property in the anchoring script of the crate and the template
- the Select and Combobox content classes
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| A maximum width tied to the trigger | Long options should stay readable | A consumer needs lists clipped to the trigger |
| Matching width for Dropdown, Popover, Date Picker, and Hover Card | They are not lists of values for the trigger | A consumer asks for a trigger-wide menu |
| Desktop and Mobile scenarios | The anchoring script already runs in the WebViews | A WebView reports a different width |

## Verification

- The CLI parity test keeps the template anchoring script identical to the
  crate script.
- `npm run verify:tailwind-conflicts` passes with the new minimum width.
- `npm run verify:runtime-interactions` asserts that the open Select list is
  at least as wide as its trigger and the open Combobox list at least as wide
  as its input.
- Reverse checks: not setting the custom property, or keeping `min-w-32`,
  each make the verifier fail.
