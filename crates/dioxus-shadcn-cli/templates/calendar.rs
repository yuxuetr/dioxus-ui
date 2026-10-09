//! Calendar: styled parts for a month grid of dates, with month navigation and single
//! or range selection that the app controls.
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::utils::{classes, merge_classes};
use super::density::{density_control_class, use_density, with_density};
use dioxus::prelude::*;
pub use super::calendar_grid::{
  CalendarDate, CalendarDay as CalendarPrimitiveDay, CalendarKeyMove, CalendarMonth,
  CalendarMonthGrid, CalendarRangeState, CalendarWeekday, calendar_month_grid, calendar_move_date,
  calendar_range_state, days_in_month, is_leap_year,
};

/// Which way a `CalendarNavButton` moves the month.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CalendarNavDirection {
  /// Back to the previous month.
  #[default]
  Previous,
  /// On to the next month.
  Next,
}

const CALENDAR_BASE_CLASS: &str =
  "w-fit rounded-md border border-border bg-background p-3 text-foreground";
const CALENDAR_HEADER_BASE_CLASS: &str = "mb-3 flex items-center justify-between gap-2";
const CALENDAR_CAPTION_BASE_CLASS: &str = "text-sm font-medium";
const CALENDAR_NAV_BASE_CLASS: &str = "flex items-center gap-1";
const CALENDAR_NAV_BUTTON_BASE_CLASS: &str = "inline-flex h-8 w-8 items-center justify-center rounded-md text-sm transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
const CALENDAR_GRID_BASE_CLASS: &str = "grid gap-1";
const CALENDAR_HEAD_BASE_CLASS: &str = "grid grid-cols-7 gap-1";
const CALENDAR_HEAD_CELL_BASE_CLASS: &str =
  "flex h-8 w-8 items-center justify-center text-xs font-medium text-muted-foreground";
const CALENDAR_BODY_BASE_CLASS: &str = "grid gap-1";
const CALENDAR_ROW_BASE_CLASS: &str = "grid grid-cols-7 gap-1";
const CALENDAR_DAY_BASE_CLASS: &str = "inline-flex h-8 w-8 items-center justify-center rounded-md text-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
const CALENDAR_DAY_SELECTED_CLASS: &str =
  "bg-primary text-primary-foreground hover:bg-primary/90";
const CALENDAR_DAY_TODAY_CLASS: &str = "border border-input";
const CALENDAR_DAY_OUTSIDE_CLASS: &str = "text-muted-foreground";
const CALENDAR_DAY_RANGE_CLASS: &str = "bg-accent text-accent-foreground";

fn calendar_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_BASE_CLASS)]), class)
}

fn calendar_header_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_HEADER_BASE_CLASS)]), class)
}

fn calendar_caption_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_CAPTION_BASE_CLASS)]), class)
}

fn calendar_nav_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_NAV_BASE_CLASS)]), class)
}

fn calendar_nav_button_class(disabled: bool, class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_NAV_BUTTON_BASE_CLASS), disabled.then_some("pointer-events-none opacity-50")]), class)
}

fn calendar_grid_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_GRID_BASE_CLASS)]), class)
}

fn calendar_head_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_HEAD_BASE_CLASS)]), class)
}

fn calendar_head_cell_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_HEAD_CELL_BASE_CLASS)]), class)
}

fn calendar_body_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_BODY_BASE_CLASS)]), class)
}

fn calendar_row_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_ROW_BASE_CLASS)]), class)
}

fn calendar_day_class(
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

  merge_classes(classes([Some(CALENDAR_DAY_BASE_CLASS), (!in_range && !selected).then_some("hover:bg-accent"), (in_range && !selected).then_some(CALENDAR_DAY_RANGE_CLASS), selected.then_some(CALENDAR_DAY_SELECTED_CLASS), today.then_some(CALENDAR_DAY_TODAY_CLASS), outside_month.then_some(CALENDAR_DAY_OUTSIDE_CLASS), disabled.then_some("pointer-events-none opacity-50")]), class)
}

// Set when a day handles a navigation key, and taken by the day that becomes
// focused next, whether Dioxus reuses its element or mounts a new one.
static CALENDAR_FOCUS_PENDING: AtomicBool = AtomicBool::new(false);

/// Maps a key on a focused day to a calendar move: arrows move by a day or a
/// week, Page Up and Page Down by a month (a year with Shift), and Home and End
/// to the start and end of the week.
pub fn calendar_key_move(key: &Key, shift: bool) -> Option<CalendarKeyMove> {
  match key {
    Key::ArrowLeft => Some(CalendarKeyMove::PreviousDay),
    Key::ArrowRight => Some(CalendarKeyMove::NextDay),
    Key::ArrowUp => Some(CalendarKeyMove::PreviousWeek),
    Key::ArrowDown => Some(CalendarKeyMove::NextWeek),
    Key::PageUp if shift => Some(CalendarKeyMove::PreviousYear),
    Key::PageUp => Some(CalendarKeyMove::PreviousMonth),
    Key::PageDown if shift => Some(CalendarKeyMove::NextYear),
    Key::PageDown => Some(CalendarKeyMove::NextMonth),
    Key::Home => Some(CalendarKeyMove::StartOfWeek),
    Key::End => Some(CalendarKeyMove::EndOfWeek),
    _ => None,
  }
}

fn calendar_range_attribute(range_state: CalendarRangeState) -> &'static str {
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
pub fn CalendarCaption(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = calendar_caption_class(&class);

  rsx! {
    div {
      class,
      ..attributes,
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
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = calendar_nav_button_class(disabled, &with_density(density_control_class(use_density()), &class));
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
      onclick: move |event| {
        if let Some(handler) = onclick {
          handler.call(event);
        }
      },
      {children}
    }
  }
}

#[component]
pub fn CalendarGrid(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = calendar_grid_class(&class);

  rsx! {
    div {
      role: "grid",
      class,
      ..attributes,
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

/// With `on_key_move` the grid is one Tab stop: only the `focused` day has
/// `tabindex="0"`. Navigation keys report a `CalendarKeyMove`; the app applies
/// it with `calendar_move_date` and moves `focused`, and the newly focused day
/// takes DOM focus. Click calls `on_select`.
#[component]
pub fn CalendarDay(
  date: CalendarDate,
  #[props(default)] selected: bool,
  #[props(default)] today: bool,
  #[props(default)] outside_month: bool,
  #[props(default)] disabled: bool,
  #[props(default)] range_state: CalendarRangeState,
  #[props(default)] focused: bool,
  #[props(default)] on_key_move: Option<EventHandler<CalendarKeyMove>>,
  #[props(default)] on_select: Option<EventHandler<CalendarDate>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = calendar_day_class(selected, today, outside_month, disabled, range_state, &with_density(density_control_class(use_density()), &class));
  let keyboard_managed = on_key_move.is_some();
  let mut mounted = use_signal(|| None::<Rc<MountedData>>);

  // Runs on mount (after `onmounted`) and whenever this element shows another
  // focused date, so it covers days that Dioxus reuses and days keyed by date
  // that mount when the month changes. Only a pending key move moves focus,
  // so rendering or app-driven changes to `focused` never steal it.
  use_effect(use_reactive((&focused, &date), move |(focused, _)| {
    if !focused || !CALENDAR_FOCUS_PENDING.swap(false, Ordering::Relaxed) {
      return;
    }
    if let Some(element) = mounted.peek().clone() {
      spawn(async move {
        // A focus error means the day is hidden or gone; nothing to focus.
        let _ = element.set_focus(true).await;
      });
    }
  }));

  rsx! {
    button {
      r#type: "button",
      role: "gridcell",
      class,
      disabled,
      tabindex: keyboard_managed.then_some(if focused { "0" } else { "-1" }),
      "data-dxui-autofocus": (keyboard_managed && focused).then_some("true"),
      onmounted: move |event| mounted.set(Some(event.data())),
      onkeydown: move |event| {
        let key_move = calendar_key_move(&event.key(), event.modifiers().shift());
        if let (Some(handler), Some(key_move)) = (on_key_move, key_move) {
          event.prevent_default();
          CALENDAR_FOCUS_PENDING.store(true, Ordering::Relaxed);
          handler.call(key_move);
        }
      },
      onclick: move |_| {
        if let Some(handler) = on_select {
          handler.call(date);
        }
      },
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
