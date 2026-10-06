use dioxus::prelude::*;
use dioxus_shadcn_core::classes;
pub use dioxus_shadcn_primitives::{
  DismissBehavior, DropdownPrimitiveConfig, OverlayAlign, OverlaySide,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::listbox::{ListboxMode, use_listbox};
use crate::menu_marks::{MENU_CHECKBOX_MARK_CLASS, MENU_RADIO_MARK_CLASS, menu_mark_state_class};

pub const DROPDOWN_CONTENT_BASE_CLASS: &str = "z-50 min-w-32 overflow-hidden rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-md";
pub const DROPDOWN_GROUP_BASE_CLASS: &str = "p-1";
pub const DROPDOWN_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-muted-foreground";
pub const DROPDOWN_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
pub const DROPDOWN_ITEM_INSET_CLASS: &str = "pl-8";
pub const DROPDOWN_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-border";
pub const DROPDOWN_SHORTCUT_BASE_CLASS: &str =
  "ml-auto text-xs tracking-normal text-muted-foreground";

pub fn dropdown_content_class(class: &str) -> String {
  classes([Some(DROPDOWN_CONTENT_BASE_CLASS), Some(class)])
}

pub fn dropdown_group_class(class: &str) -> String {
  classes([Some(DROPDOWN_GROUP_BASE_CLASS), Some(class)])
}

pub fn dropdown_label_class(class: &str) -> String {
  classes([Some(DROPDOWN_LABEL_BASE_CLASS), Some(class)])
}

pub fn dropdown_item_class(destructive: bool, class: &str) -> String {
  let variant_class = if destructive {
    "text-destructive focus:bg-destructive/10 focus:text-destructive"
  } else {
    "text-foreground focus:bg-accent"
  };

  classes([Some(DROPDOWN_ITEM_BASE_CLASS), Some(variant_class), Some(class)])
}

/// An item whose label lines up with checkbox and radio items.
pub fn dropdown_inset_item_class(destructive: bool, class: &str) -> String {
  dropdown_item_class(destructive, &classes([Some(DROPDOWN_ITEM_INSET_CLASS), Some(class)]))
}

/// An inset item with a check mark shown while `checked`.
pub fn dropdown_checkbox_item_class(checked: bool, class: &str) -> String {
  let mark =
    classes([Some(MENU_CHECKBOX_MARK_CLASS), Some(menu_mark_state_class(checked)), Some(class)]);
  dropdown_inset_item_class(false, &mark)
}

/// An inset item with a dot shown while `checked`.
pub fn dropdown_radio_item_class(checked: bool, class: &str) -> String {
  let mark =
    classes([Some(MENU_RADIO_MARK_CLASS), Some(menu_mark_state_class(checked)), Some(class)]);
  dropdown_inset_item_class(false, &mark)
}

pub fn dropdown_separator_class(class: &str) -> String {
  classes([Some(DROPDOWN_SEPARATOR_BASE_CLASS), Some(class)])
}

pub fn dropdown_shortcut_class(class: &str) -> String {
  classes([Some(DROPDOWN_SHORTCUT_BASE_CLASS), Some(class)])
}

/// Opening focuses the first enabled item. Arrows move focus with wrapping,
/// Home, End, and typeahead jump, and activating an item requests close and
/// returns focus to the `anchor_id` element, or without one to where it was
/// before opening. With `anchor_id` the menu is placed next to that element.
/// Escape and outside interactions request close per `dismiss`.
#[component]
pub fn DropdownContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] anchor_id: Option<String>,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::End)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let class = dropdown_content_class(&class);
  let menu = use_listbox(open, anchor_id.clone(), ListboxMode::Menu, None, on_open_change);
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id, anchor_point: None, side, align, side_offset },
    dismiss,
    on_open_change,
  );

  rsx! {
    div {
      role: "menu",
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-anchored": anchored,
      "data-dxui-listbox": menu,
      {children}
    }
  }
}

#[component]
pub fn DropdownGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = dropdown_group_class(&class);

  rsx! {
    div {
      role: "group",
      class,
      {children}
    }
  }
}

#[component]
pub fn DropdownLabel(#[props(default)] class: String, children: Element) -> Element {
  let class = dropdown_label_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// Enter, Space, or click on an enabled item calls `onclick`; the menu then
/// requests close. `inset` lines the label up with checkbox and radio items.
#[component]
pub fn DropdownItem(
  #[props(default)] inset: bool,
  #[props(default)] destructive: bool,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = if inset {
    dropdown_inset_item_class(destructive, &class)
  } else {
    dropdown_item_class(destructive, &class)
  };

  rsx! {
    div {
      role: "menuitem",
      class,
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      onclick: move |event| {
        if let Some(handler) = onclick.filter(|_| !disabled) {
          handler.call(event);
        }
      },
      {children}
    }
  }
}

/// A controlled checkbox item: `onclick` reports activation, and the app
/// flips `checked`. Shows a check mark while checked.
#[component]
pub fn DropdownCheckboxItem(
  #[props(default)] checked: bool,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = dropdown_checkbox_item_class(checked, &class);

  rsx! {
    div {
      role: "menuitemcheckbox",
      class,
      "aria-checked": checked.to_string(),
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      "data-state": if checked { "checked" } else { "unchecked" },
      onclick: move |event| {
        if let Some(handler) = onclick.filter(|_| !disabled) {
          handler.call(event);
        }
      },
      {children}
    }
  }
}

/// Groups radio items; `value` is the checked item's value, for styling.
#[component]
pub fn DropdownRadioGroup(
  #[props(default)] value: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  rsx! {
    div {
      role: "group",
      class,
      "data-value": value,
      {children}
    }
  }
}

/// A controlled radio item: `onclick` reports activation, and the app moves
/// `checked` to it. Shows a dot while checked.
#[component]
pub fn DropdownRadioItem(
  #[props(default)] checked: bool,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = dropdown_radio_item_class(checked, &class);

  rsx! {
    div {
      role: "menuitemradio",
      class,
      "aria-checked": checked.to_string(),
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      "data-state": if checked { "checked" } else { "unchecked" },
      onclick: move |event| {
        if let Some(handler) = onclick.filter(|_| !disabled) {
          handler.call(event);
        }
      },
      {children}
    }
  }
}

#[component]
pub fn DropdownSeparator(#[props(default)] class: String) -> Element {
  let class = dropdown_separator_class(&class);

  rsx! {
    div {
      role: "separator",
      class,
    }
  }
}

/// Text such as a key combination, at the end of an item.
#[component]
pub fn DropdownShortcut(#[props(default)] class: String, children: Element) -> Element {
  let class = dropdown_shortcut_class(&class);

  rsx! {
    span {
      class,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ssr_disabled_item_sets_aria_disabled() {
    fn app() -> Element {
      rsx! { DropdownItem { disabled: true, "Archive" } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains(r#"aria-disabled="true""#));
    assert!(html.contains(r#"data-disabled="true""#));
  }

  #[test]
  fn dropdown_item_class_reflects_destructive_state() {
    let actual = dropdown_item_class(true, "gap-2");

    assert!(actual.contains(DROPDOWN_ITEM_BASE_CLASS));
    assert!(actual.contains("text-destructive focus:bg-destructive/10 focus:text-destructive"));
    assert!(actual.ends_with("gap-2"));
  }

  #[test]
  fn ssr_checkable_items_show_their_mark_only_when_checked() {
    fn app() -> Element {
      rsx! {
        DropdownCheckboxItem { checked: true, "Status bar" }
        DropdownCheckboxItem { "Activity bar" }
        DropdownRadioGroup { value: "top",
          DropdownRadioItem { checked: true, "Top" }
          DropdownRadioItem { disabled: true, "Bottom" }
        }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert_eq!(html.matches(r#"role="menuitemcheckbox""#).count(), 2);
    assert_eq!(html.matches(r#"role="menuitemradio""#).count(), 2);
    assert_eq!(html.matches(r#"aria-checked="true""#).count(), 2);
    assert_eq!(html.matches("before:opacity-100").count(), 2);
    assert_eq!(html.matches("before:opacity-0").count(), 2);
    assert_eq!(html.matches(MENU_CHECKBOX_MARK_CLASS).count(), 2);
    assert_eq!(html.matches(MENU_RADIO_MARK_CLASS).count(), 2);
    assert!(html.contains(r#"data-value="top""#));
    assert!(html.contains(r#"aria-disabled="true""#));
  }

  #[test]
  fn inset_items_line_up_with_checkable_items() {
    let inset = dropdown_inset_item_class(true, "gap-2");

    assert!(inset.contains(DROPDOWN_ITEM_INSET_CLASS));
    assert!(inset.contains("text-destructive"));
    assert!(inset.ends_with("gap-2"));
    assert!(dropdown_checkbox_item_class(false, "").contains(DROPDOWN_ITEM_INSET_CLASS));
    assert!(dropdown_shortcut_class("font-mono").ends_with("font-mono"));
  }

  #[test]
  fn dropdown_primitive_config_is_reexported() {
    let config = DropdownPrimitiveConfig::controlled(true);

    assert!(config.open);
  }
}
