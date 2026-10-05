# RFC 0064: Typed Date Input

- Status: Accepted
- Created: 2026-10-05

## Summary

Add `DatePickerInput`, a text field for typing a date, and the
`parse_date` and `format_date` functions it uses, so a Date Picker can take a
typed date as well as a calendar pick.

## Current State

The 0.1.0 release notes exclude typed date parsing. A date is chosen only by
clicking or keying through the Calendar; typing a known date, such as a
birthday decades back, means paging through months. shadcn/ui's Date Picker
examples include an input next to the calendar button.

## Decision

### Parsing

`DateOrder` is `YearMonthDay` (default), `MonthDayYear`, or `DayMonthYear`.
`parse_date(text, order)` splits the trimmed text on `-`, `/`, `.`, or
spaces into three numbers. A four-digit first number reads as ISO
year-month-day in any order; otherwise the order decides. The year must have
four digits, since two-digit years are ambiguous, and `CalendarDate::new`
rejects month 13 or February 30. `format_date(date, order)` writes the date
zero-padded with the order's separator, `-` for `YearMonthDay` and `/`
otherwise. Both live in the Date Picker module, so source-copy apps get them
with the template, beside the Calendar template's `CalendarDate`.

### `DatePickerInput`

A text input controlled by `value: Option<CalendarDate>`:

- It keeps the text being typed, so "2026-0" survives a render, and calls
  `on_value_change(Some(date))` once the text parses, or
  `on_value_change(None)` when it is cleared.
- Text that does not parse sets `aria-invalid="true"` and the destructive
  border, but stays, so the user can fix it.
- On blur, a date that parses is rewritten in the order's format.
- A new `value` from the app, such as a calendar pick, replaces the text
  unless the text already means that date.
- The placeholder defaults to the order's pattern, such as `YYYY-MM-DD`.

The app pairs it with the calendar: the input's value is the calendar's
selection, and the Date Picker trigger opens the calendar beside it.

## Alternatives

- **A native `type="date"` input.** Its format follows the browser locale,
  it differs across WebViews, and it cannot share the calendar popover.
- **Locale detection.** The app knows its users' convention; `DateOrder`
  keeps the choice explicit and testable.

## Verification

- Unit tests for parsing: ISO in every order, each order, the separators,
  invalid months and days, two-digit years, and formatting round trips.
- SSR tests for the input's placeholder and invalid state.
- A runtime check for typing a date, an invalid date that stays, clearing,
  and blur formatting.
- A site example combining the input with the calendar popover.
