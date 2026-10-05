# TODOs

## Progress

- Overall: 100%
- Current milestone: none (M183 complete)
- Current task: none

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180), `docs/archive/TODOs.completed-20261005-m181.md` (M181), and `docs/archive/TODOs.completed-20261005-m182.md` (M182)

## Goals

- Every overlay, menu, and popup the runtime preview can open passes the axe-core audit in both themes, and a regression fails the runtime check.

## Evidence

- A one-off probe opened each of the preview's 24 popup triggers, the Context Menu, the Tooltip, and the Hover Card, and ran the RFC 0054 audit in both themes. The audit points that M182 uses missed these findings:
  - Open `Select`: the listbox has no accessible name, and the `role="combobox"` trigger has no `aria-controls` (critical).
  - Disabled `SelectItem` and `DropdownItem` set `data-disabled` but not `aria-disabled`, so assistive technology does not hear that they are disabled, and axe holds their faded text to the contrast minimum.
  - Open `DatePickerContent` is a `role="dialog"` with no accessible name.
  - Open `HoverCardContent` is a `role="dialog"` with no accessible name; the Radix Hover Card that shadcn/ui wraps renders no role.

## M183 Open State Accessibility Audit

- DONE M183.1 Design the open state audit
  - Decide each fix from the evidence (how the Select listbox and the DatePicker dialog get a name, the trigger to content link, and the Hover Card role), and where the runtime check audits open states.
  - Record the decisions, the WebKit audit status, and reevaluation conditions in an RFC.
  - Done: RFC 0055. The Select listbox and the DatePicker dialog take their trigger's name through ids derived from `anchor_id`; WebKit audits stay out until a WebKit-only defect is reported.

- DONE M183.2 Fix the Select and menu item findings
  - Link the Select trigger and listbox and name the listbox; set `aria-disabled` on disabled `SelectItem` and `DropdownItem`; in the crate and the templates, with tests.
  - Done: `SelectContent` renders `id="{anchor_id}-content"` and `aria-labelledby="{anchor_id}"`, and `SelectTrigger` points `aria-controls` at it; a Select without ids renders neither.

- DONE M183.3 Fix the DatePicker and Hover Card findings
  - Name the DatePicker dialog and drop the Hover Card dialog role; in the crate and the templates, with tests.
  - Done: `DatePickerContent` renders `aria-labelledby="{anchor_id}"`; the runtime check finds the Hover Card content by its `data-dxui-hover-content` marker.

- DONE M183.4 Audit open states in the runtime check
  - Run the audit while each overlay, menu, and popup fixture is open, and reverse-verify that a reintroduced finding fails the check.
  - Done: 14 open states are audited. The audit also caught an unnamed, unlinked Combobox list that the probe missed, fixed the same way as Select. Reverse checks: the Hover Card dialog role and a `SelectItem` without `aria-disabled` each fail the check.

- DONE M183.5 Complete the open state audit milestone
  - Update CHANGELOG, the affected component docs, and the docs index.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release`, `npm run verify:runtime-interactions`, `npm run verify:site`, and the Desktop self-test.
  - Push local commits to `origin/main`.
  - Done: release gate, runtime check (35 fixtures, 14 open states audited), site check (67 routes, 65 examples), and the Desktop self-test (10 scenarios) pass.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
