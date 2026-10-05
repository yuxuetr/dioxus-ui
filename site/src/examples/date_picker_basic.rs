use dioxus::prelude::*;
use dioxus_shadcn::{
  Calendar, CalendarBody, CalendarCaption, CalendarDate, CalendarDay, CalendarGrid, CalendarHeader,
  CalendarMonth, CalendarNav, CalendarNavButton, CalendarNavDirection, CalendarRow,
  CalendarWeekday, DatePickerContent, DatePickerTrigger, DatePickerValue, Label,
  calendar_month_grid, calendar_move_date,
};

#[component]
pub fn Demo() -> Element {
  let mut open = use_signal(|| false);
  let mut month = use_signal(|| CalendarMonth::unchecked(2026, 10));
  let mut focused = use_signal(|| CalendarDate::unchecked(2026, 10, 15));
  let mut selected = use_signal(|| None::<CalendarDate>);
  let mut move_focus = move |date: CalendarDate| {
    focused.set(date);
    month.set(CalendarMonth::unchecked(date.year, date.month));
  };
  let grid =
    calendar_month_grid(month(), CalendarWeekday::Sunday, None, selected(), None, None, &[]);
  let label = selected().map(|date| format!("{}-{:02}-{:02}", date.year, date.month, date.day));

  rsx! {
    div { class: "grid max-w-xs gap-2",
      Label { r#for: "date-picker-basic-trigger", "Due date" }
      DatePickerTrigger {
        id: "date-picker-basic-trigger",
        open: open(),
        on_open_change: move |next| {
          if next {
            move_focus(selected().unwrap_or(focused()));
          }
          open.set(next);
        },
        DatePickerValue { placeholder: "Pick a date", {label.unwrap_or_default()} }
      }
    }
    DatePickerContent {
      open: open(),
      anchor_id: "date-picker-basic-trigger",
      on_open_change: move |next| open.set(next),
      Calendar {
        CalendarHeader {
          CalendarCaption { id: "date-picker-basic-caption", "{month().year}-{month().month:02}" }
          CalendarNav {
            CalendarNavButton {
              direction: CalendarNavDirection::Previous,
              onclick: move |_| month.set(month().add_months(-1)),
              "<"
            }
            CalendarNavButton {
              direction: CalendarNavDirection::Next,
              onclick: move |_| month.set(month().add_months(1)),
              ">"
            }
          }
        }
        CalendarGrid { "aria-labelledby": "date-picker-basic-caption",
          CalendarBody {
            for week in grid.weeks {
              CalendarRow {
                for day in week {
                  CalendarDay {
                    key: "{day.date.year}-{day.date.month}-{day.date.day}",
                    date: day.date,
                    selected: day.selected,
                    outside_month: day.outside_month,
                    focused: day.date == focused(),
                    on_key_move: move |key_move| {
                      move_focus(calendar_move_date(focused(), key_move, CalendarWeekday::Sunday))
                    },
                    on_select: move |date| {
                      selected.set(Some(date));
                      open.set(false);
                    },
                    "{day.date.day}"
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
