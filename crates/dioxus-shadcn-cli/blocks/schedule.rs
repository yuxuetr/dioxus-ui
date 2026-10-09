use dioxus::prelude::*;

use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::calendar::{
  Calendar, CalendarBody, CalendarCaption, CalendarDate, CalendarDay, CalendarGrid, CalendarHead,
  CalendarHeadCell, CalendarHeader, CalendarKeyMove, CalendarMonth, CalendarNav, CalendarNavButton,
  CalendarNavDirection, CalendarRow, CalendarWeekday, calendar_month_grid, calendar_move_date,
};
use crate::components::ui::dialog::{
  Dialog, DialogContent, DialogDescription, DialogOverlay, DialogTitle,
};
use crate::components::ui::field::{Field, FieldError, FieldLabel};
use crate::components::ui::input::Input;

const WEEK_START: CalendarWeekday = CalendarWeekday::Sunday;

fn week_of(date: CalendarDate) -> CalendarDate {
  calendar_move_date(date, CalendarKeyMove::StartOfWeek, WEEK_START)
}

fn iso(date: CalendarDate) -> String {
  format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
}

fn parse_iso(text: &str) -> Option<CalendarDate> {
  let mut parts = text.splitn(3, '-');
  let year = parts.next()?.parse().ok()?;
  let month = parts.next()?.parse().ok()?;
  let day = parts.next()?.parse().ok()?;
  CalendarDate::new(year, month, day)
}

fn weekday_name(date: CalendarDate) -> &'static str {
  WEEKDAYS[usize::from(date.weekday().number_from_sunday())]
}

const WEEKDAYS: [&str; 7] =
  ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
const MONTHS: [&str; 12] = [
  "January",
  "February",
  "March",
  "April",
  "May",
  "June",
  "July",
  "August",
  "September",
  "October",
  "November",
  "December",
];

fn month_name(month: u8) -> &'static str {
  MONTHS.get(month.saturating_sub(1) as usize).copied().unwrap_or("")
}

/// An event on one day; `start` and `end` are 24-hour `HH:MM`.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleEvent {
  pub id: u32,
  pub title: String,
  pub day: CalendarDate,
  pub start: String,
  pub end: String,
}

fn sample_events(today: CalendarDate) -> Vec<ScheduleEvent> {
  let event = |id, title: &str, offset, start: &str, end: &str| ScheduleEvent {
    id,
    title: title.to_string(),
    day: today.add_days(offset),
    start: start.to_string(),
    end: end.to_string(),
  };
  vec![
    event(1, "Standup", -3, "09:30", "09:45"),
    event(2, "Planning", -2, "13:00", "14:00"),
    event(3, "Standup", 0, "09:30", "09:45"),
    event(4, "Customer call", 0, "11:00", "11:30"),
    event(5, "Retro", 1, "16:00", "17:00"),
  ]
}

/// A week schedule: a month Calendar that picks the week and marks it, seven
/// day columns with their events in time order, previous and next week
/// buttons, and a New event dialog with a title, a date, and start and end
/// times. `on_create` hears each event added; pass `today` to start on
/// another week, and replace `sample_events` with your data.
#[component]
pub fn ScheduleBlock(
  #[props(default = CalendarDate::unchecked(2026, 10, 15))] today: CalendarDate,
  #[props(default)] on_create: Option<EventHandler<ScheduleEvent>>,
) -> Element {
  let mut events = use_signal(|| sample_events(today));
  let mut week = use_signal(|| week_of(today));
  let mut month = use_signal(|| CalendarMonth::unchecked(today.year, today.month));
  let mut focused = use_signal(|| today);
  let mut next_id = use_signal(|| 100_u32);
  let mut dialog_open = use_signal(|| false);
  let mut title = use_signal(String::new);
  let mut date = use_signal(|| iso(today));
  let mut start = use_signal(|| "10:00".to_string());
  let mut end = use_signal(|| "11:00".to_string());
  let mut submitted = use_signal(|| false);

  let title_error = (submitted() && title().trim().is_empty()).then_some("Enter a title.");
  let date_error = (submitted() && parse_iso(&date()).is_none()).then_some("Enter a date.");
  let time_error = (submitted() && end() <= start()).then_some("End after the start time.");
  // Shows the week of `day` and the month around it, with `day` focused.
  let mut show_week = move |day: CalendarDate| {
    week.set(week_of(day));
    month.set(CalendarMonth::unchecked(day.year, day.month));
    focused.set(day);
  };
  let days: Vec<CalendarDate> = (0..7).map(|offset| week().add_days(offset)).collect();
  let first = week();
  let last = week().add_days(6);
  let (first_year, first_month, first_day) = (first.year, first.month, first.day);
  let (last_year, last_month, last_day) = (last.year, last.month, last.day);
  let grid =
    calendar_month_grid(month(), WEEK_START, Some(today), None, Some(first), Some(last), &[]);
  let range = if first_month == last_month {
    format!("{} {first_day} \u{2013} {last_day}, {last_year}", month_name(first_month))
  } else if first_year == last_year {
    format!(
      "{} {first_day} \u{2013} {} {last_day}, {last_year}",
      month_name(first_month),
      month_name(last_month)
    )
  } else {
    format!(
      "{} {first_day}, {first_year} \u{2013} {} {last_day}, {last_year}",
      month_name(first_month),
      month_name(last_month)
    )
  };

  rsx! {
    div { class: "grid min-h-screen content-start gap-4 bg-background p-4 text-foreground",
      header { class: "flex flex-wrap items-center gap-3",
        h1 { class: "text-xl font-semibold", "Schedule" }
        p { class: "text-sm text-muted-foreground", "aria-live": "polite", "{range}" }
        div { class: "ms-auto flex items-center gap-2",
          Button {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Sm,
            onclick: move |_| show_week(week().add_days(-7)),
            "Previous week"
          }
          Button {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Sm,
            onclick: move |_| show_week(today),
            "This week"
          }
          Button {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Sm,
            onclick: move |_| show_week(week().add_days(7)),
            "Next week"
          }
          Button {
            size: ButtonSize::Sm,
            onclick: move |_| {
              title.set(String::new());
              date.set(iso(today));
              submitted.set(false);
              dialog_open.set(true);
            },
            "New event"
          }
        }
      }
      div { class: "grid items-start gap-4 lg:grid-cols-[auto_1fr]",
        Calendar { class: "justify-self-start rounded-md border",
          CalendarHeader {
            CalendarCaption { id: "schedule-month-caption",
              "{month_name(month().month)} {month().year}"
            }
            CalendarNav {
              CalendarNavButton {
                direction: CalendarNavDirection::Previous,
                onclick: move |_| month.set(month().add_months(-1)),
                "\u{2039}"
              }
              CalendarNavButton {
                direction: CalendarNavDirection::Next,
                onclick: move |_| month.set(month().add_months(1)),
                "\u{203a}"
              }
            }
          }
          CalendarGrid { "aria-labelledby": "schedule-month-caption",
            CalendarHead {
              for name in WEEKDAYS {
                CalendarHeadCell { key: "{name}", "{&name[..2]}" }
              }
            }
            CalendarBody {
              for row in grid.weeks {
                CalendarRow {
                  for cell in row {
                    CalendarDay {
                      key: "{iso(cell.date)}",
                      date: cell.date,
                      selected: cell.selected,
                      today: cell.today,
                      outside_month: cell.outside_month,
                      range_state: cell.range_state,
                      focused: cell.date == focused(),
                      on_key_move: move |key_move| show_week(calendar_move_date(focused(), key_move, WEEK_START)),
                      on_select: show_week,
                      "{cell.date.day}"
                    }
                  }
                }
              }
            }
          }
        }
        div { class: "grid gap-3 md:grid-cols-7",
          for day in days {
            {
              let mut items: Vec<ScheduleEvent> = events().into_iter().filter(|event| event.day == day).collect();
              items.sort_by(|a, b| a.start.cmp(&b.start));
              let label = format!("{}, {} {}", weekday_name(day), month_name(day.month), day.day);
              rsx! {
                section {
                  key: "{iso(day)}",
                  class: if day == today { "grid content-start gap-2 rounded-md border border-primary p-2" } else { "grid content-start gap-2 rounded-md border p-2" },
                  "aria-label": "{label}",
                  h2 { class: "flex items-baseline gap-1 text-sm font-medium",
                    span { "{&weekday_name(day)[..3]}" }
                    span { class: "text-muted-foreground", "{day.day}" }
                    if day == today {
                      Badge { class: "ms-auto", variant: BadgeVariant::Default, "Today" }
                    }
                  }
                  if items.is_empty() {
                    p { class: "text-xs text-muted-foreground", "No events" }
                  }
                  ul { class: "grid gap-2",
                    for event in items {
                      li { key: "{event.id}", class: "rounded-md bg-accent p-2 text-sm",
                        p { class: "font-medium", "{event.title}" }
                        p { class: "text-xs text-muted-foreground", "{event.start}\u{2013}{event.end}" }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      Dialog { open: dialog_open(), on_open_change: move |next| dialog_open.set(next),
        DialogOverlay {}
        DialogContent {
          DialogTitle { "New event" }
          DialogDescription { "Add an event to the schedule." }
          form {
            class: "grid gap-4",
            novalidate: true,
            onsubmit: move |event| {
              event.prevent_default();
              submitted.set(true);
              let Some(day) = parse_iso(&date()) else {
                return;
              };
              if title().trim().is_empty() || end() <= start() {
                return;
              }
              let id = next_id();
              next_id.set(id + 1);
              let created = ScheduleEvent { id, title: title().trim().to_string(), day, start: start(), end: end() };
              events.with_mut(|all| all.push(created.clone()));
              show_week(day);
              dialog_open.set(false);
              if let Some(handler) = on_create {
                handler.call(created);
              }
            },
            Field { invalid: title_error.is_some(),
              FieldLabel { r#for: "schedule-title", "Title" }
              Input {
                id: "schedule-title",
                value: title(),
                invalid: title_error.is_some(),
                "aria-describedby": title_error.map(|_| "schedule-title-error"),
                on_value_change: move |value| title.set(value),
              }
              if let Some(message) = title_error {
                FieldError { id: "schedule-title-error", "{message}" }
              }
            }
            Field { invalid: date_error.is_some(),
              FieldLabel { r#for: "schedule-date", "Date" }
              Input {
                id: "schedule-date",
                r#type: "date",
                value: date(),
                invalid: date_error.is_some(),
                "aria-describedby": date_error.map(|_| "schedule-date-error"),
                on_value_change: move |value| date.set(value),
              }
              if let Some(message) = date_error {
                FieldError { id: "schedule-date-error", "{message}" }
              }
            }
            div { class: "grid grid-cols-2 gap-3",
              Field { invalid: time_error.is_some(),
                FieldLabel { r#for: "schedule-start", "Start" }
                Input {
                  id: "schedule-start",
                  r#type: "time",
                  value: start(),
                  on_value_change: move |value| start.set(value),
                }
              }
              Field { invalid: time_error.is_some(),
                FieldLabel { r#for: "schedule-end", "End" }
                Input {
                  id: "schedule-end",
                  r#type: "time",
                  value: end(),
                  invalid: time_error.is_some(),
                  "aria-describedby": time_error.map(|_| "schedule-time-error"),
                  on_value_change: move |value| end.set(value),
                }
              }
            }
            if let Some(message) = time_error {
              FieldError { id: "schedule-time-error", "{message}" }
            }
            div { class: "flex justify-end gap-2",
              Button { r#type: "button", variant: ButtonVariant::Outline, onclick: move |_| dialog_open.set(false), "Cancel" }
              Button { r#type: "submit", "Save" }
            }
          }
        }
      }
    }
  }
}
