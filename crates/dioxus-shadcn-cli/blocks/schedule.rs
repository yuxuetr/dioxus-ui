use dioxus::prelude::*;

use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::dialog::{
  Dialog, DialogContent, DialogDescription, DialogOverlay, DialogTitle,
};
use crate::components::ui::field::{Field, FieldError, FieldLabel};
use crate::components::ui::input::Input;

/// A calendar day as days since 1970-01-01, so weeks are plain arithmetic.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Day(i64);

impl Day {
  /// The day for a Gregorian date, or `None` when it does not exist.
  pub fn from_ymd(year: i64, month: u32, day: u32) -> Option<Day> {
    let days_in_month = match month {
      1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
      4 | 6 | 9 | 11 => 30,
      2 if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 => 29,
      2 => 28,
      _ => return None,
    };
    if day == 0 || day > days_in_month {
      return None;
    }
    // Days from civil, after Howard Hinnant's algorithm.
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let month = i64::from(month);
    let day_of_year =
      (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + i64::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    Some(Day(era * 146_097 + day_of_era - 719_468))
  }

  /// Year, month, and day.
  pub fn ymd(self) -> (i64, u32, u32) {
    let z = self.0 + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z - era * 146_097;
    let year_of_era =
      (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * month_index + 2) / 5 + 1) as u32;
    let month = if month_index < 10 { month_index + 3 } else { month_index - 9 } as u32;
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    (year, month, day)
  }

  fn add(self, days: i64) -> Day {
    Day(self.0 + days)
  }

  /// 0 for Sunday through 6 for Saturday.
  fn weekday(self) -> usize {
    (self.0 + 4).rem_euclid(7) as usize
  }

  fn week_start(self) -> Day {
    self.add(-(self.weekday() as i64))
  }

  fn iso(self) -> String {
    let (year, month, day) = self.ymd();
    format!("{year:04}-{month:02}-{day:02}")
  }

  fn parse(text: &str) -> Option<Day> {
    let mut parts = text.splitn(3, '-');
    let year = parts.next()?.parse().ok()?;
    let month = parts.next()?.parse().ok()?;
    let day = parts.next()?.parse().ok()?;
    Day::from_ymd(year, month, day)
  }
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

fn month_name(month: u32) -> &'static str {
  MONTHS.get(month.saturating_sub(1) as usize).copied().unwrap_or("")
}

/// An event on one day; `start` and `end` are 24-hour `HH:MM`.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleEvent {
  pub id: u32,
  pub title: String,
  pub day: Day,
  pub start: String,
  pub end: String,
}

fn sample_events(today: Day) -> Vec<ScheduleEvent> {
  let event = |id, title: &str, offset, start: &str, end: &str| ScheduleEvent {
    id,
    title: title.to_string(),
    day: today.add(offset),
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

/// A week schedule: seven day columns with their events in time order,
/// previous and next week buttons, and a New event dialog with a title, a
/// date, and start and end times. `on_create` hears each event added; pass
/// `today` to start on another week, and replace `sample_events` with your
/// data.
#[component]
pub fn ScheduleBlock(
  #[props(default = Day::from_ymd(2026, 10, 15).unwrap_or(Day(0)))] today: Day,
  #[props(default)] on_create: Option<EventHandler<ScheduleEvent>>,
) -> Element {
  let mut events = use_signal(|| sample_events(today));
  let mut week = use_signal(|| today.week_start());
  let mut next_id = use_signal(|| 100_u32);
  let mut dialog_open = use_signal(|| false);
  let mut title = use_signal(String::new);
  let mut date = use_signal(|| today.iso());
  let mut start = use_signal(|| "10:00".to_string());
  let mut end = use_signal(|| "11:00".to_string());
  let mut submitted = use_signal(|| false);

  let title_error = (submitted() && title().trim().is_empty()).then_some("Enter a title.");
  let date_error = (submitted() && Day::parse(&date()).is_none()).then_some("Enter a date.");
  let time_error = (submitted() && end() <= start()).then_some("End after the start time.");
  let days: Vec<Day> = (0..7).map(|offset| week().add(offset)).collect();
  let (first_year, first_month, first_day) = week().ymd();
  let (last_year, last_month, last_day) = week().add(6).ymd();
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
            onclick: move |_| week.set(week().add(-7)),
            "Previous week"
          }
          Button {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Sm,
            onclick: move |_| week.set(today.week_start()),
            "This week"
          }
          Button {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Sm,
            onclick: move |_| week.set(week().add(7)),
            "Next week"
          }
          Button {
            size: ButtonSize::Sm,
            onclick: move |_| {
              title.set(String::new());
              date.set(today.iso());
              submitted.set(false);
              dialog_open.set(true);
            },
            "New event"
          }
        }
      }
      div { class: "grid gap-3 md:grid-cols-7",
        for day in days {
          {
            let (_, month, number) = day.ymd();
            let mut items: Vec<ScheduleEvent> = events().into_iter().filter(|event| event.day == day).collect();
            items.sort_by(|a, b| a.start.cmp(&b.start));
            let label = format!("{}, {} {number}", WEEKDAYS[day.weekday()], month_name(month));
            rsx! {
              section {
                key: "{day.0}",
                class: if day == today { "grid content-start gap-2 rounded-md border border-primary p-2" } else { "grid content-start gap-2 rounded-md border p-2" },
                "aria-label": "{label}",
                h2 { class: "flex items-baseline gap-1 text-sm font-medium",
                  span { "{&WEEKDAYS[day.weekday()][..3]}" }
                  span { class: "text-muted-foreground", "{number}" }
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
              let Some(day) = Day::parse(&date()) else {
                return;
              };
              if title().trim().is_empty() || end() <= start() {
                return;
              }
              let id = next_id();
              next_id.set(id + 1);
              let created = ScheduleEvent { id, title: title().trim().to_string(), day, start: start(), end: end() };
              events.with_mut(|all| all.push(created.clone()));
              week.set(day.week_start());
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
