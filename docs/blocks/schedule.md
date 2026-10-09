# Schedule

A week schedule: seven day columns with their events in time order,
previous, current, and next week buttons, and a New event dialog.

```bash
dxui add schedule
```

Components: Badge, Button, Dialog, Field, Input.

## Behavior

- `ScheduleBlock` takes `today`, a `Day` (built with `Day::from_ymd`), which
  picks the first week shown and the highlighted column, and `on_create`,
  called with each `ScheduleEvent` (`id`, `title`, `day`, `start`, `end`)
  the dialog adds.
- The dialog checks on Save that the title is not empty, the date is valid,
  and the end comes after the start; a failed check marks the field invalid
  and shows its error. A saved event moves the schedule to its week.
- The date and times use `Input` with `type="date"` and `type="time"`, which
  browsers and webviews render with their own pickers and keyboard support.
- `Day` counts days since 1970-01-01, so week navigation is addition; it
  converts to and from year, month, and day. Replace `sample_events` with
  your data.

## Accessibility Notes

Each day column is a region named by its full date, such as "Thursday,
October 15", and its events are a list. The week range is a polite live
region, so screen readers announce it after a week button. The dialog is
named "New event", and each input has a label and, after a failed check, a
described error.
