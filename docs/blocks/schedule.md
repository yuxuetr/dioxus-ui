# Schedule

A week schedule: a month Calendar that picks the week, seven day columns
with their events in time order, previous, current, and next week buttons,
and a New event dialog.

```bash
dxui add schedule
```

Components: Badge, Button, Calendar, Dialog, Field, Input.

## Behavior

- `ScheduleBlock` takes `today`, a `CalendarDate`, which picks the first
  week shown and the highlighted column, and `on_create`, called with each
  `ScheduleEvent` (`id`, `title`, `day`, `start`, `end`) the dialog adds.
- The month Calendar marks the shown week as a range. Choosing a day, or
  moving to one with the arrow keys, Page Up, or Page Down, shows its week;
  the week buttons move the calendar along with it, and its own month
  buttons page months without changing the week.
- The dialog checks on Save that the title is not empty, the date is valid,
  and the end comes after the start; a failed check marks the field invalid
  and shows its error. A saved event moves the schedule to its week.
- The date and times use `Input` with `type="date"` and `type="time"`, which
  browsers and webviews render with their own pickers and keyboard support.
- Weeks start on Sunday (`WEEK_START`) and use the Calendar's date helpers,
  which `dxui add schedule` copies with the Calendar. Replace
  `sample_events` with your data.

## Accessibility Notes

The month grid is named by its caption, such as "October 2026", and keeps
one day in the tab order. Each day column is a region named by its full
date, such as "Thursday, October 15", and its events are a list. The week range is a polite live
region, so screen readers announce it after a week button. The dialog is
named "New event", and each input has a label and, after a failed check, a
described error.
