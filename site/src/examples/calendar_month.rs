use dioxus::prelude::*;
use dioxus_shadcn::{
  Calendar, CalendarBody, CalendarCaption, CalendarDate, CalendarDay, CalendarGrid, CalendarHeader,
  CalendarMonth, CalendarNav, CalendarNavButton, CalendarNavDirection, CalendarRow,
  CalendarWeekday, calendar_month_grid, calendar_move_date,
};

#[component]
pub fn CalendarMonthDemo() -> Element {
  let mut month = use_signal(|| CalendarMonth::unchecked(2026, 10));
  let mut focused = use_signal(|| CalendarDate::unchecked(2026, 10, 15));
  let mut selected = use_signal(|| CalendarDate::unchecked(2026, 10, 15));
  // Arrow keys and Page Up/Down move focus, following it into other months.
  let mut move_focus = move |date: CalendarDate| {
    focused.set(date);
    month.set(CalendarMonth::unchecked(date.year, date.month));
  };
  let grid =
    calendar_month_grid(month(), CalendarWeekday::Sunday, None, Some(selected()), None, None, &[]);

  rsx! {
    Calendar {
      CalendarHeader {
        CalendarCaption { id: "calendar-month-caption", "{month().year}-{month().month:02}" }
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
      CalendarGrid { "aria-labelledby": "calendar-month-caption",
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
                  on_select: move |date| selected.set(date),
                  "{day.date.day}"
                }
              }
            }
          }
        }
      }
    }
    p { class: "mt-3 text-sm text-muted-foreground",
      "Selected: {selected().year}-{selected().month:02}-{selected().day:02}"
    }
  }
}
