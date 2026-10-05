# TODOs

## Progress

- Overall: 60%
- Current milestone: M181 Interactive Part Callbacks And Attributes
- Current task: M181.4

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), and `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180)

## Goals

- Every component part that renders a native button, link, or label an app needs to act on accepts the callback and attributes the native element would, so no site example has to wrap a part in a `div onclick` or repeat a label in `aria-label`.

## Evidence

- The M180 site examples work around parts that cannot be wired up: `ButtonGroupItem`, `AttachmentAction`, `AttachmentTrigger`, and `InputGroupAction` render buttons with no `onclick`; `MessageScrollerJumpButton` has no `onclick` (the example wraps it in a `div`) and no `type="button"`; `TooltipTrigger` cannot run an action; `FieldLabel` has no `for`, so the Field example labels each input with `aria-label` instead.
- `docs/component-api.md` defers attribute forwarding "before a concrete need appears"; these are the concrete needs.

## M181 Interactive Part Callbacks And Attributes

- DONE M181.1 Design part callbacks and attributes
  - List the parts that render a native interactive element without a way for the app to act on it, and decide per part: `onclick`, `r#for`, and attribute spreading (`GlobalAttributes` plus the element), spread after the explicit attributes as in RFC 0028 to RFC 0036.
  - Keep parts whose click already reports through a component callback (close buttons, triggers with `on_open_change`, calendar days) out of scope, with reevaluation conditions.
  - Record the decision in an RFC and update the Attribute Forwarding section of `docs/component-api.md`.
  - Done: RFC 0053. The audit also found `ComboboxTrigger` without a click handler, so M181.2 covers it too.

- DONE M181.2 Wire up the action parts
  - `ButtonGroupItem`, `InputGroupAction`, `AttachmentAction`, `AttachmentTrigger`, `MessageScrollerJumpButton`, and `TooltipTrigger`, in the crate and the templates, with SSR tests for the passed attributes.
  - Add preview fixtures and extend `npm run verify:runtime-interactions` to click each part and assert its callback ran, then reverse-verify.
  - Done: plus `ComboboxTrigger`; the `action-parts` fixture covers clicks, a disabled press, and the jump button inside a form. Reverse checks: a dropped callback and a dropped `type="button"` both fail.

- DONE M181.3 Wire up labels and links
  - `FieldLabel` gains `r#for` and attributes; `BreadcrumbLink` and `HoverCardTrigger` gain attributes, in the crate and the templates, with SSR tests.
  - Extend the runtime check to assert a `FieldLabel` click focuses its input, then reverse-verify.
  - Done: an empty `for` is omitted, since `for=""` unlinks a wrapped input; the same bug in `Label` is fixed in its own commit. Reverse check: dropping `for` fails the runtime check.

- TODO M181.4 Drop the site example workarounds
  - Use the new callbacks and `r#for` in the site examples so each action does something visible, remove the `div onclick` wrapper and the duplicate `aria-label`s, and keep `npm run verify:site` passing.

- TODO M181.5 Complete the interactive part milestone
  - Update CHANGELOG, the component docs pages, and the docs index.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release`, `npm run verify:runtime-interactions`, and `npm run verify:site`.
  - Push local commits to `origin/main`.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
