use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  CalendarDate, CalendarDay as CalendarPrimitiveDay, CalendarKeyMove, CalendarMonth,
  CalendarMonthGrid, CalendarRangeState, CalendarWeekday, calendar_month_grid, calendar_move_date,
  calendar_range_state, days_in_month, is_leap_year,
};

pub const CALENDAR_BASE_CLASS: &str =
  "w-fit rounded-md border border-zinc-200 bg-white p-3 text-zinc-950";
pub const CALENDAR_HEADER_BASE_CLASS: &str = "mb-3 flex items-center justify-between gap-2";
pub const CALENDAR_CAPTION_BASE_CLASS: &str = "text-sm font-medium";
pub const CALENDAR_NAV_BASE_CLASS: &str = "flex items-center gap-1";
pub const CALENDAR_NAV_BUTTON_BASE_CLASS: &str = "inline-flex h-8 w-8 items-center justify-center rounded-md text-sm transition-colors hover:bg-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";
pub const CALENDAR_GRID_BASE_CLASS: &str = "grid gap-1";
pub const CALENDAR_HEAD_BASE_CLASS: &str = "grid grid-cols-7 gap-1";
pub const CALENDAR_HEAD_CELL_BASE_CLASS: &str =
  "flex h-8 w-8 items-center justify-center text-xs font-medium text-zinc-500";
pub const CALENDAR_BODY_BASE_CLASS: &str = "grid gap-1";
pub const CALENDAR_ROW_BASE_CLASS: &str = "grid grid-cols-7 gap-1";
pub const CALENDAR_DAY_BASE_CLASS: &str = "inline-flex h-8 w-8 items-center justify-center rounded-md text-sm transition-colors hover:bg-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";
pub const CALENDAR_DAY_SELECTED_CLASS: &str = "bg-blue-600 text-white hover:bg-blue-700";
pub const CALENDAR_DAY_TODAY_CLASS: &str = "border border-zinc-300";
pub const CALENDAR_DAY_OUTSIDE_CLASS: &str = "text-zinc-400";
pub const CALENDAR_DAY_RANGE_CLASS: &str = "bg-blue-100 text-blue-950 hover:bg-blue-200";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CalendarNavDirection {
  #[default]
  Previous,
  Next,
}

pub fn calendar_class(class: &str) -> String {
  classes([Some(CALENDAR_BASE_CLASS), Some(class)])
}

pub fn calendar_header_class(class: &str) -> String {
  classes([Some(CALENDAR_HEADER_BASE_CLASS), Some(class)])
}

pub fn calendar_caption_class(class: &str) -> String {
  classes([Some(CALENDAR_CAPTION_BASE_CLASS), Some(class)])
}

pub fn calendar_nav_class(class: &str) -> String {
  classes([Some(CALENDAR_NAV_BASE_CLASS), Some(class)])
}

pub fn calendar_nav_button_class(disabled: bool, class: &str) -> String {
  classes([
    Some(CALENDAR_NAV_BUTTON_BASE_CLASS),
    disabled.then_some("pointer-events-none opacity-50"),
    Some(class),
  ])
}

pub fn calendar_grid_class(class: &str) -> String {
  classes([Some(CALENDAR_GRID_BASE_CLASS), Some(class)])
}

pub fn calendar_head_class(class: &str) -> String {
  classes([Some(CALENDAR_HEAD_BASE_CLASS), Some(class)])
}

pub fn calendar_head_cell_class(class: &str) -> String {
  classes([Some(CALENDAR_HEAD_CELL_BASE_CLASS), Some(class)])
}

pub fn calendar_body_class(class: &str) -> String {
  classes([Some(CALENDAR_BODY_BASE_CLASS), Some(class)])
}

pub fn calendar_row_class(class: &str) -> String {
  classes([Some(CALENDAR_ROW_BASE_CLASS), Some(class)])
}

pub fn calendar_day_class(
  selected: bool,
  today: bool,
  outside_month: bool,
  disabled: bool,
  range_state: CalendarRangeState,
  class: &str,
) -> String {
  let in_range = matches!(
    range_state,
    CalendarRangeState::Start
      | CalendarRangeState::Middle
      | CalendarRangeState::End
      | CalendarRangeState::Single
  );

  classes([
    Some(CALENDAR_DAY_BASE_CLASS),
    (in_range && !selected).then_some(CALENDAR_DAY_RANGE_CLASS),
    selected.then_some(CALENDAR_DAY_SELECTED_CLASS),
    today.then_some(CALENDAR_DAY_TODAY_CLASS),
    outside_month.then_some(CALENDAR_DAY_OUTSIDE_CLASS),
    disabled.then_some("pointer-events-none opacity-50"),
    Some(class),
  ])
}

pub fn calendar_range_attribute(range_state: CalendarRangeState) -> &'static str {
  match range_state {
    CalendarRangeState::Outside => "outside",
    CalendarRangeState::Single => "single",
    CalendarRangeState::Start => "start",
    CalendarRangeState::Middle => "middle",
    CalendarRangeState::End => "end",
  }
}

#[component]
pub fn Calendar(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarHeader(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_header_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarCaption(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_caption_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarNav(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_nav_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarNavButton(
  #[props(default)] direction: CalendarNavDirection,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = calendar_nav_button_class(disabled, &class);
  let label = match direction {
    CalendarNavDirection::Previous => "Go to previous month",
    CalendarNavDirection::Next => "Go to next month",
  };

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-label": label,
      "data-direction": match direction {
        CalendarNavDirection::Previous => "previous",
        CalendarNavDirection::Next => "next",
      },
      {children}
    }
  }
}

#[component]
pub fn CalendarGrid(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_grid_class(&class);

  rsx! {
    div {
      role: "grid",
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarHead(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_head_class(&class);

  rsx! {
    div {
      role: "row",
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarHeadCell(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_head_cell_class(&class);

  rsx! {
    div {
      role: "columnheader",
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarBody(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_body_class(&class);

  rsx! {
    div {
      role: "rowgroup",
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarRow(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_row_class(&class);

  rsx! {
    div {
      role: "row",
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarDay(
  date: CalendarDate,
  #[props(default)] selected: bool,
  #[props(default)] today: bool,
  #[props(default)] outside_month: bool,
  #[props(default)] disabled: bool,
  #[props(default)] range_state: CalendarRangeState,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = calendar_day_class(selected, today, outside_month, disabled, range_state, &class);

  rsx! {
    button {
      r#type: "button",
      role: "gridcell",
      class,
      disabled,
      "aria-disabled": disabled.to_string(),
      "aria-selected": selected.to_string(),
      "data-date": format!("{:04}-{:02}-{:02}", date.year, date.month, date.day),
      "data-outside-month": outside_month.to_string(),
      "data-range": calendar_range_attribute(range_state),
      "data-selected": selected.to_string(),
      "data-today": today.to_string(),
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calendar_day_class_reflects_states() {
    let actual =
      calendar_day_class(true, true, true, false, CalendarRangeState::Single, "font-semibold");

    assert!(actual.contains(CALENDAR_DAY_BASE_CLASS));
    assert!(actual.contains(CALENDAR_DAY_SELECTED_CLASS));
    assert!(actual.contains(CALENDAR_DAY_TODAY_CLASS));
    assert!(actual.contains(CALENDAR_DAY_OUTSIDE_CLASS));
    assert!(actual.ends_with("font-semibold"));
  }

  #[test]
  fn calendar_range_attribute_maps_states() {
    assert_eq!(calendar_range_attribute(CalendarRangeState::Middle), "middle");
  }

  #[test]
  fn calendar_primitives_are_reexported() {
    assert_eq!(days_in_month(2024, 2), 29);
    assert_eq!(CalendarDate::unchecked(2024, 6, 1).weekday(), CalendarWeekday::Saturday);
  }
}
