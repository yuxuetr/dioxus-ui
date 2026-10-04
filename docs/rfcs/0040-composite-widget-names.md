# RFC 0040: Composite Widget Names

- Status: Accepted
- Created: 2026-10-05

## Summary

Let apps name the composite widgets. `TabsList`, `ToggleGroup`, `Menubar`,
`NavigationMenu`, `CalendarGrid`, and `CalendarCaption` pass through global
and element attributes, so a tab list, toggle group, or menu bar can take
`aria-label`, a navigation landmark can be told apart from other `nav`
elements, and a calendar grid can point `aria-labelledby` at its caption.

## Current State

As of M164:

- `TabsList` renders `role="tablist"`, `ToggleGroup` renders `role="group"`,
  `Menubar` renders `role="menubar"`, and `CalendarGrid` renders
  `role="grid"`, all without a name and without a way to pass one. The APG
  patterns ask for a name on each when more than one can appear, and a
  calendar grid is announced as an unnamed table.
- `NavigationMenu` renders a `nav` landmark with no name, so a page with
  several navigation landmarks lists them all as "navigation".
- `CalendarCaption` cannot take an `id`, so even a grid that accepted
  attributes could not point at it.

## Decision

- `TabsList`, `ToggleGroup`, `Menubar`, `CalendarGrid`, and
  `CalendarCaption` extend `GlobalAttributes` and `div`. `NavigationMenu`
  extends `GlobalAttributes` and `nav`.
- The spread comes after the explicit attributes, so the roles, orientation,
  and the roving, menubar, and navigation script hooks stay in place.

## Scope

In scope:

- attribute spreading on these six parts in the crate and the template
- the Tabs, Toggle Group, Menubar, Navigation Menu, and Calendar docs pages
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Linking the calendar grid to its caption automatically | The caption and grid are siblings the app composes; Date Picker can pass the ids | A consumer reports repeating the same ids |
| Spreading on the remaining parts of these components | Items are named by their content, and roots that only group them have no role | A consumer needs to name or identify one |
| Desktop and Mobile self-test scenarios | The change adds attributes and no behavior | The parts gain script behavior |

## Verification

- The CLI generated fixture smoke keeps the templates compiling.
- The Web preview names the Tabs, Toggle Group, Menubar, Navigation Menu, and
  Calendar fixtures, and `npm run verify:runtime-interactions` finds each by
  role and name, and checks that their keyboard behavior still works.
- Reverse checks: not spreading the attributes of any of the six parts makes
  the verifier fail.
