use dioxus::prelude::*;
use dioxus_shadcn::{
  Calendar, CalendarBody, CalendarCaption, CalendarDate, CalendarDay, CalendarGrid, CalendarHeader,
  CalendarMonth, CalendarNav, CalendarNavButton, CalendarNavDirection, CalendarRow,
  CalendarWeekday, DateOrder, DatePicker, DatePickerContent, DatePickerInput, DatePickerTrigger, Label,
  calendar_month_grid, calendar_move_date,
};

#[component]
pub fn DatePickerInputDemo() -> Element {
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

  rsx! {
    DatePicker {
      id: "date-picker-input-trigger",
      open: open(),
      on_open_change: move |next| {
        if next {
          move_focus(selected().unwrap_or(focused()));
        }
        open.set(next);
      },
      div { class: "grid max-w-xs gap-2",
        Label { r#for: "date-picker-input-field", "Start date" }
        div { class: "flex gap-2",
          // Typing a date and picking one share the selection.
          DatePickerInput {
            id: "date-picker-input-field",
            order: DateOrder::MonthDayYear,
            value: selected(),
            on_value_change: move |date: Option<CalendarDate>| {
              selected.set(date);
              if let Some(date) = date {
                move_focus(date);
              }
            },
          }
          div { class: "w-10 shrink-0",
            DatePickerTrigger {
              "aria-label": "Open the calendar",
              svg {
                class: "size-4",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                "aria-hidden": "true",
                path { d: "M4 6h16v14H4zM4 10h16M8 3v4M16 3v4" }
              }
            }
          }
        }
      }
      DatePickerContent {
        Calendar {
          CalendarHeader {
            CalendarCaption { id: "date-picker-input-caption", "{month().year}-{month().month:02}" }
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
          CalendarGrid { "aria-labelledby": "date-picker-input-caption",
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
}
