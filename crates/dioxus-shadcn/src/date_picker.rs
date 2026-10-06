use dioxus::prelude::*;
use dioxus_shadcn_core::classes;
pub use dioxus_shadcn_primitives::{
  DismissBehavior, OverlayAlign, OverlaySide, PopoverPrimitiveConfig,
};

use dioxus_shadcn_primitives::CalendarDate;

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::modal_focus::use_modal_focus_scope;

pub const DATE_PICKER_TRIGGER_BASE_CLASS: &str = "flex h-10 w-full items-center justify-between rounded-md border bg-background px-3 py-2 text-sm text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
pub const DATE_PICKER_VALUE_BASE_CLASS: &str =
  "truncate text-left data-[placeholder=true]:text-muted-foreground";
pub const DATE_PICKER_CONTENT_BASE_CLASS: &str = "z-50 w-auto rounded-md border border-border bg-popover p-0 text-popover-foreground shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";

pub fn date_picker_trigger_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  classes([Some(DATE_PICKER_TRIGGER_BASE_CLASS), Some(invalid_class), Some(class)])
}

pub fn date_picker_value_class(class: &str) -> String {
  classes([Some(DATE_PICKER_VALUE_BASE_CLASS), Some(class)])
}

pub fn date_picker_content_class(class: &str) -> String {
  classes([Some(DATE_PICKER_CONTENT_BASE_CLASS), Some(class)])
}

pub fn date_picker_side_attribute(side: OverlaySide) -> &'static str {
  match side {
    OverlaySide::Top => "top",
    OverlaySide::Right => "right",
    OverlaySide::Bottom => "bottom",
    OverlaySide::Left => "left",
    OverlaySide::Inline => "inline",
  }
}

pub fn date_picker_align_attribute(align: OverlayAlign) -> &'static str {
  match align {
    OverlayAlign::Start => "start",
    OverlayAlign::Center => "center",
    OverlayAlign::End => "end",
  }
}

/// Click requests `!open` through `on_open_change`. Pass `id` as the content's
/// `anchor_id`. Other attributes, such as `aria-label` for an icon-only
/// trigger, go to the button.
#[component]
pub fn DatePickerTrigger(
  #[props(default)] id: Option<String>,
  #[props(default)] open: bool,
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = date_picker_trigger_class(invalid, &class);

  rsx! {
    button {
      r#type: "button",
      id,
      class,
      disabled,
      onclick: move |_| {
        if let Some(handler) = on_open_change {
          handler.call(!open);
        }
      },
      "aria-expanded": open.to_string(),
      "aria-haspopup": "dialog",
      "aria-invalid": invalid.to_string(),
      "data-state": if open { "open" } else { "closed" },
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn DatePickerValue(
  #[props(default)] placeholder: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = date_picker_value_class(&class);
  let has_placeholder = !placeholder.is_empty();

  rsx! {
    span {
      class,
      "data-placeholder": has_placeholder.to_string(),
      "aria-label": placeholder,
      {children}
    }
  }
}

/// With `anchor_id` (the trigger's `id`) the content is placed next to the
/// trigger, flipping and shifting to stay in the viewport. Opening moves focus
/// to the element marked `data-dxui-autofocus` (a keyboard-managed Calendar's
/// focused day) or the first focusable element, Tab wraps inside, and closing
/// returns focus to the trigger. Escape and outside interactions request close
/// per `dismiss`. The dialog takes the `anchor_id` element's name.
#[component]
pub fn DatePickerContent(
  #[props(default)] open: bool,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::Center)] align: OverlayAlign,
  #[props(default)] anchor_id: Option<String>,
  #[props(default = 4)] side_offset: i32,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = date_picker_content_class(&class);
  let labelledby = anchor_id.clone();
  let focus_scope = use_modal_focus_scope(open, false);
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id, anchor_point: None, side, align, side_offset },
    dismiss,
    on_open_change,
  );

  rsx! {
    div {
      role: "dialog",
      class,
      tabindex: "-1",
      "aria-labelledby": labelledby,
      hidden: !open,
      "data-dxui-anchored": anchored,
      "data-dxui-focus-scope": focus_scope,
      "data-align": date_picker_align_attribute(align),
      "data-side": date_picker_side_attribute(side),
      "data-state": if open { "open" } else { "closed" },
      {children}
    }
  }
}

/// The order of the day, month, and year in a typed date.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DateOrder {
  /// ISO `2026-10-05`.
  #[default]
  YearMonthDay,
  /// `10/05/2026`.
  MonthDayYear,
  /// `05/10/2026`.
  DayMonthYear,
}

impl DateOrder {
  pub const fn placeholder(self) -> &'static str {
    match self {
      Self::YearMonthDay => "YYYY-MM-DD",
      Self::MonthDayYear => "MM/DD/YYYY",
      Self::DayMonthYear => "DD/MM/YYYY",
    }
  }

  pub const fn separator(self) -> char {
    match self {
      Self::YearMonthDay => '-',
      Self::MonthDayYear | Self::DayMonthYear => '/',
    }
  }
}

/// A date typed as three numbers separated by `-`, `/`, `.`, or spaces. A
/// four-digit first number reads as ISO year-month-day in any order;
/// otherwise `order` decides. Years need four digits, and impossible dates,
/// such as February 30, are rejected.
pub fn parse_date(text: &str, order: DateOrder) -> Option<CalendarDate> {
  let parts =
    text.trim().split(['-', '/', '.', ' ']).filter(|part| !part.is_empty()).collect::<Vec<_>>();
  let [first, second, third] = parts.as_slice() else {
    return None;
  };
  if ![first, second, third].iter().all(|part| part.bytes().all(|byte| byte.is_ascii_digit())) {
    return None;
  }
  let (year, month, day) = if first.len() == 4 {
    (*first, *second, *third)
  } else {
    match order {
      DateOrder::YearMonthDay => return None,
      DateOrder::MonthDayYear => (*third, *first, *second),
      DateOrder::DayMonthYear => (*third, *second, *first),
    }
  };
  if year.len() != 4 {
    return None;
  }
  CalendarDate::new(year.parse().ok()?, month.parse().ok()?, day.parse().ok()?)
}

/// The date zero-padded in `order` with its separator.
pub fn format_date(date: CalendarDate, order: DateOrder) -> String {
  let separator = order.separator();
  let (year, month, day) = (date.year, date.month, date.day);
  match order {
    DateOrder::YearMonthDay => format!("{year:04}{separator}{month:02}{separator}{day:02}"),
    DateOrder::MonthDayYear => format!("{month:02}{separator}{day:02}{separator}{year:04}"),
    DateOrder::DayMonthYear => format!("{day:02}{separator}{month:02}{separator}{year:04}"),
  }
}

pub const DATE_PICKER_INPUT_BASE_CLASS: &str = "flex h-10 w-full rounded-md border bg-background px-3 py-2 text-sm tabular-nums transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";

pub fn date_picker_input_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  classes([Some(DATE_PICKER_INPUT_BASE_CLASS), Some(invalid_class), Some(class)])
}

/// A text field for typing a date. It keeps the text being typed, calls
/// `on_value_change(Some(date))` once the text parses and
/// `on_value_change(None)` when it is cleared, marks text that does not
/// parse as invalid without clearing it, and rewrites a valid date in
/// `order`'s format on blur. A new `value`, such as a calendar pick, replaces
/// the text. Other attributes, such as `id` and `aria-label`, go to the input.
#[component]
pub fn DatePickerInput(
  #[props(default)] value: Option<CalendarDate>,
  #[props(default)] order: DateOrder,
  #[props(default)] placeholder: String,
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] on_value_change: Option<EventHandler<Option<CalendarDate>>>,
  #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
  let mut text = use_signal(|| value.map(|date| format_date(date, order)).unwrap_or_default());

  // Follow a new value unless the text already means it.
  use_effect(use_reactive((&value, &order), move |(value, order)| {
    let current = parse_date(&text.peek(), order);
    if current != value {
      text.set(value.map(|date| format_date(date, order)).unwrap_or_default());
    }
  }));

  let typed = text();
  let unparsed = !typed.trim().is_empty() && parse_date(&typed, order).is_none();
  let class = date_picker_input_class(invalid || unparsed, &class);
  let placeholder =
    if placeholder.is_empty() { order.placeholder().to_string() } else { placeholder };

  rsx! {
    input {
      class,
      r#type: "text",
      inputmode: "numeric",
      autocomplete: "off",
      placeholder,
      disabled,
      value: typed,
      "aria-invalid": (invalid || unparsed).then_some("true"),
      oninput: move |event| {
        let next = event.value();
        let parsed = parse_date(&next, order);
        let cleared = next.trim().is_empty();
        text.set(next);
        if let Some(handler) = on_value_change.filter(|_| parsed.is_some() || cleared) {
          handler.call(parsed);
        }
      },
      onblur: move |_| {
        let parsed = parse_date(&text.peek(), order);
        if let Some(date) = parsed {
          text.set(format_date(date, order));
        }
      },
      ..attributes,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_date_reads_iso_and_each_order() {
    let date = CalendarDate::new(2026, 3, 15);

    assert_eq!(parse_date("2026-03-15", DateOrder::DayMonthYear), date);
    assert_eq!(parse_date(" 2026/3/15 ", DateOrder::YearMonthDay), date);
    assert_eq!(parse_date("03/15/2026", DateOrder::MonthDayYear), date);
    assert_eq!(parse_date("15.03.2026", DateOrder::DayMonthYear), date);
    assert_eq!(parse_date("15 3 2026", DateOrder::DayMonthYear), date);
  }

  #[test]
  fn parse_date_rejects_ambiguous_and_impossible_dates() {
    assert_eq!(parse_date("03/15/26", DateOrder::MonthDayYear), None);
    assert_eq!(parse_date("03/15/2026", DateOrder::YearMonthDay), None);
    assert_eq!(parse_date("2026-13-01", DateOrder::YearMonthDay), None);
    assert_eq!(parse_date("2026-02-30", DateOrder::YearMonthDay), None);
    assert_eq!(parse_date("2026-0", DateOrder::YearMonthDay), None);
    assert_eq!(parse_date("2026-+3-01", DateOrder::YearMonthDay), None);
    assert_eq!(parse_date("", DateOrder::YearMonthDay), None);
  }

  #[test]
  fn format_date_round_trips() {
    let date = CalendarDate::unchecked(2026, 3, 5);

    for order in [DateOrder::YearMonthDay, DateOrder::MonthDayYear, DateOrder::DayMonthYear] {
      assert_eq!(parse_date(&format_date(date, order), order), Some(date));
    }
    assert_eq!(format_date(date, DateOrder::DayMonthYear), "05/03/2026");
  }

  #[test]
  fn ssr_date_input_shows_the_value_and_order_placeholder() {
    fn app() -> Element {
      rsx! {
        DatePickerInput { value: CalendarDate::new(2026, 10, 5), order: DateOrder::MonthDayYear }
        DatePickerInput { order: DateOrder::DayMonthYear, invalid: true }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains("value=\"10/05/2026\""));
    assert!(html.contains("placeholder=\"DD/MM/YYYY\""));
    assert_eq!(html.matches("aria-invalid=\"true\"").count(), 1);
  }

  #[test]
  fn ssr_content_takes_the_trigger_name() {
    fn app() -> Element {
      rsx! { DatePickerContent { open: true, anchor_id: "due-date", "Calendar" } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains(r#"role="dialog""#));
    assert!(html.contains(r#"aria-labelledby="due-date""#));
  }

  #[test]
  fn date_picker_trigger_class_reflects_invalid_state() {
    let actual = date_picker_trigger_class(true, "w-56");

    assert!(actual.contains(DATE_PICKER_TRIGGER_BASE_CLASS));
    assert!(actual.contains("border-destructive focus-visible:ring-destructive"));
    assert!(actual.ends_with("w-56"));
  }

  #[test]
  fn date_picker_side_and_align_attributes_map_values() {
    assert_eq!(date_picker_side_attribute(OverlaySide::Left), "left");
    assert_eq!(date_picker_align_attribute(OverlayAlign::End), "end");
  }

  #[test]
  fn date_picker_primitive_config_is_reexported() {
    let config = PopoverPrimitiveConfig::controlled(true);

    assert!(config.open);
  }
}
