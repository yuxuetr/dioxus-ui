# Calendar and Date Picker API Plan

This document defines the M15 Calendar and Date Picker APIs. The goal is to
provide predictable date-grid primitives and controlled styled parts without
taking ownership of locale formatting, time zones, or application-specific
parsing too early.

Status: Implemented in M15.

## Scope

M15 covers:

- Calendar
- Date Picker

Both components should ship in crate mode and source-copy mode. Crate mode can
reuse `dioxus-shadcn-core` and `dioxus-shadcn-primitives`; generated templates must
remain self-contained and must not import internal crates.

## Date Strategy

The first implementation uses a small first-party date primitive for UI state:

```rust
CalendarDate { year, month, day }
CalendarMonth { year, month }
CalendarDay { date, outside_month, today, selected, disabled, range_state }
CalendarRangeState::{Outside, Single, Start, Middle, End}
CalendarWeekday::{Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday}
```

Rules:

- dates are calendar dates only, not instants
- no timezone conversion in primitives
- no locale formatting in primitives
- month grid generation is deterministic and pure
- apps pass labels and formatted values when locale-specific text matters

This is a focused design pass instead of pulling in a broad date/time dependency
immediately. A proven date/time crate can still be introduced later if Date
Picker parsing, localization, or calendar systems become part of the library
contract.

## Shared Rules

Calendar and Date Picker use controlled state first:

- explicit selected date or range props
- explicit visible month props
- `disabled`, `selected`, `outside_month`, and `today` state on day cells
- `class: String` on every styled part
- `children: Element` for composition slots

All Tailwind classes must be complete static tokens in source. Runtime selection
between predefined strings is allowed; runtime construction of class names is
not allowed.

Primitive reuse:

- month grid generation for visible days
- date comparison and range classification
- keyboard movement over dates
- popover placement for Date Picker content

Deferred runtime work:

- parsing typed input into dates
- locale month and weekday labels
- time zone conversion
- non-Gregorian calendars
- DOM focus commands
- portal mounting

## Calendar

Calendar is a controlled date grid. It can be used as a standalone picker or as
the calendar body inside Date Picker.

Implemented crate API:

```rust
Calendar { class, children }
CalendarHeader { class, children }
CalendarCaption { class, children }
CalendarNav { class, children }
CalendarNavButton { direction, disabled, class, children }
CalendarGrid { class, children }
CalendarHead { class, children }
CalendarHeadCell { class, children }
CalendarBody { class, children }
CalendarRow { class, children }
CalendarDay { date, selected, today, outside_month, disabled, range_state, class, children }
```

Behavior defaults:

- grid uses grid, row, columnheader, rowgroup, and gridcell semantics
- day cells expose selected, disabled, outside-month, and range data attributes
- keyboard movement helpers are pure and app-invoked
- selection is controlled by the consuming app

Calendar should not parse strings, store current system date, or mutate visible
month internally in the first implementation.

## Date Picker

Date Picker composes a trigger/input-like surface with popover content and a
Calendar body.

Implemented crate API:

```rust
DatePickerTrigger { open, invalid, disabled, class, children }
DatePickerValue { placeholder, class, children }
DatePickerContent { open, side, align, class, children }
```

Behavior defaults:

- trigger uses button semantics
- content is popover-backed and controlled by `open`
- value display is provided by the app, not formatted internally
- invalid state maps to `aria-invalid`

Use Date Picker when users need a visual date grid. Use native date inputs or
Native Select-style forms when platform-native mobile behavior is more
important than a custom grid.

## Platform Defaults

| Target | Date components |
| --- | --- |
| Web | Calendar can render inline or in Popover; typed parsing remains app-owned. |
| Desktop | Same controlled model as Web; avoid assuming direct DOM focus commands. |
| Mobile | Prefer native date inputs or Sheet-style presentation for long date workflows. |

## Implementation Order

1. Calendar date primitives
2. Calendar styled parts
3. Date Picker composition parts
4. documentation, examples, and parity updates

This order validated deterministic date math before exposing styled grids and
popover composition.

## Quality Gates

Each M15 component should include:

- primitive tests for month boundaries, leap years, disabled dates, and ranges
- crate-mode class composition tests
- registry entry and self-contained template
- component docs page
- web and desktop demo usage
- generated fixture smoke coverage
- per-feature compile coverage

Before marking each task done, run:

```bash
cargo test --workspace --all-features
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```
