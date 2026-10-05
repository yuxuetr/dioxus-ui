# RFC 0054: Automated Accessibility Audit

- Status: Accepted
- Created: 2026-10-05

## Summary

Run axe-core in the site and runtime browser checks, in the light and dark
themes, and fix what a first run found: an unsupported `aria-orientation` on
two group roots, viewports that only a mouse can scroll, a heading level in
Alert, and documents with no language.

## Current State

As of M181, the browser checks assert behavior, text contrast, and layout
that the project wrote by hand. Nothing checks ARIA validity, names, or
landmarks across the whole page. A one-off axe-core 4 run with the
`wcag2a`, `wcag2aa`, `wcag21a`, `wcag21aa`, and `best-practice` tags over the
64 site component pages and the runtime preview reported:

| Rule | Where | Cause |
| --- | --- | --- |
| `aria-allowed-attr` (critical) | `ButtonGroup`, `ToggleGroup` | `aria-orientation` on `role="group"`, which does not support it |
| `scrollable-region-focusable` (serious) | `ScrollAreaViewport`, `MessageScrollerViewport`, a preview Message Scroller | an overflowing element with no focusable content is not in the Tab order, so keyboard users cannot scroll it; WebKit, which runs the Desktop and iOS previews, never makes scrollers focusable |
| `heading-order` (moderate) | `AlertTitle` | it renders an `h5`, which skips levels under most page headings |
| `html-has-lang` (serious) | site, previews | the document has no `lang` |
| `aria-prohibited-attr` (serious) | Skeleton site example | `aria-label` on a `div` with no role |
| `aria-input-field-name`, `button-name` (serious, critical) | preview fixtures | a listbox and a Select trigger with no accessible name |
| `landmark-unique` (moderate) | preview | two Paginations and the Toast and Sonner viewports share a landmark name |

## Decision

### Component fixes

- `ButtonGroup` and `ToggleGroup` drop `aria-orientation`. `ButtonGroup`
  keeps its `data-orientation`, and `ToggleGroup` renders the same value as
  `data-orientation`. The roving focus script reads
  `data-dxui-roving-orientation`, so arrow key behavior does not change. `Tabs`, `RadioGroup`, `Slider`,
  `Separator`, and `Resizable` keep `aria-orientation`, since their roles
  support it.
- `ScrollAreaViewport` and `MessageScrollerViewport` render `tabindex="0"`
  and pass through global and `div` attributes after it. An app names a
  viewport with `role="region"` and `aria-label`, or passes `tabindex: "-1"`
  when its content already has a focusable element.
- `AlertTitle` renders a `div`, as in shadcn/ui v4. An app that wants a
  heading wraps the text in its own heading at the right level.

### Documents and fixtures

- The site and the previews set `lang="en"` on the document.
- The Skeleton example gives its busy wrapper `role="status"`, the preview
  listbox and Select trigger get labels, and the fixtures that repeat a
  landmark get distinct names where the component accepts one.

### Audit

- `scripts/browser-check-support.mjs` gains `accessibilityViolations(page)`,
  which injects axe-core from `node_modules` and returns the violations for
  the five tags above.
- `npm run verify:site` audits each component page and the guides in both
  themes, after the examples render.
- `npm run verify:runtime-interactions` audits the preview at the points
  where it checks text contrast: the first render, an open Dialog, and after
  the interactions, in both themes.
- No rule is disabled on the site. The preview disables `landmark-unique`
  only: it shows several instances of the same landmark component on one
  page, such as the Toast and Sonner viewports, which an app renders one of.
- `axe-core` is a dev dependency (MPL-2.0); nothing ships it.

## Scope

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| A configurable label on the Toast and Sonner viewports | An app renders one notification region, and the viewports take no attributes | An app renders both or ships a non-English region label |
| Auditing every preview state, such as each open menu | The audit points match the contrast points, which cover an open overlay | A component fails an audit in a state the points miss |
| Desktop and Mobile audits | axe-core needs injected script access; the self-tests have no result channel for it | A WebKit-only accessibility defect is reported |
| Screen reader testing | An automated audit cannot judge announcements | A consumer reports an announcement defect |

## Verification

- SSR tests: the group roots render no `aria-orientation`, the viewports
  render `tabindex="0"` and passed attributes, and `AlertTitle` renders a
  `div`.
- The runtime check keeps passing its ToggleGroup arrow key assertions.
- Both browser checks fail with the rule id and the first offending element
  when axe-core reports a violation.
- Reverse checks: restoring `aria-orientation` on `ToggleGroup`, dropping the
  viewport `tabindex`, or restoring the `h5` each make a check fail.
