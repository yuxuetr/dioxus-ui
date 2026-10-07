//! Dropdown: a menu opened from a trigger, with group, label, item, checkbox,
//! radio, submenu, separator, and shortcut parts.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
pub use dioxus_shadcn_primitives::{
  DismissBehavior, DropdownPrimitiveConfig, OverlayAlign, OverlaySide,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::density::{density_control_class, use_density, with_density};
use crate::listbox::{ListboxMode, use_listbox};
use crate::menu_marks::{
  MENU_CHECKBOX_MARK_CLASS, MENU_RADIO_MARK_CLASS, MENU_SUB_TRIGGER_CLASS, menu_mark_state_class,
};
use crate::menu_radio::{use_menu_radio_group, use_menu_radio_item};
use crate::menu_sub::{use_menu_sub, use_menu_sub_content, use_menu_sub_part};
use crate::overlay_root::{OverlayRoot, overlay_trigger, use_overlay_root};
use crate::root_state::use_root_context;

const DROPDOWN_CONTENT_BASE_CLASS: &str = "z-50 min-w-32 overflow-hidden rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-md";
const DROPDOWN_GROUP_BASE_CLASS: &str = "p-1";
const DROPDOWN_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-muted-foreground";
const DROPDOWN_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
const DROPDOWN_ITEM_INSET_CLASS: &str = "pl-8";
const DROPDOWN_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-border";
const DROPDOWN_SHORTCUT_BASE_CLASS: &str = "ml-auto text-xs tracking-normal text-muted-foreground";

/// Classes for the menu panel: base classes with `class` merged over them.
pub fn dropdown_content_class(class: &str) -> String {
  merge_classes(classes([Some(DROPDOWN_CONTENT_BASE_CLASS)]), class)
}

fn dropdown_group_class(class: &str) -> String {
  merge_classes(classes([Some(DROPDOWN_GROUP_BASE_CLASS)]), class)
}

fn dropdown_label_class(class: &str) -> String {
  merge_classes(classes([Some(DROPDOWN_LABEL_BASE_CLASS)]), class)
}

/// Classes for an item: base classes, the destructive colors when `destructive` or
/// the plain ones otherwise, then `class` merged over them.
pub fn dropdown_item_class(destructive: bool, class: &str) -> String {
  let variant_class = if destructive {
    "text-destructive focus:bg-destructive/10 focus:text-destructive"
  } else {
    "text-foreground focus:bg-accent"
  };

  merge_classes(classes([Some(DROPDOWN_ITEM_BASE_CLASS), Some(variant_class)]), class)
}

/// An item whose label lines up with checkbox and radio items.
pub fn dropdown_inset_item_class(destructive: bool, class: &str) -> String {
  dropdown_item_class(
    destructive,
    &merge_classes(classes([Some(DROPDOWN_ITEM_INSET_CLASS)]), class),
  )
}

/// An inset item with a check mark shown while `checked`.
pub fn dropdown_checkbox_item_class(checked: bool, class: &str) -> String {
  let mark = merge_classes(
    classes([Some(MENU_CHECKBOX_MARK_CLASS), Some(menu_mark_state_class(checked))]),
    class,
  );
  dropdown_inset_item_class(false, &mark)
}

/// An inset item with a dot shown while `checked`.
pub fn dropdown_radio_item_class(checked: bool, class: &str) -> String {
  let mark = merge_classes(
    classes([Some(MENU_RADIO_MARK_CLASS), Some(menu_mark_state_class(checked))]),
    class,
  );
  dropdown_inset_item_class(false, &mark)
}

fn dropdown_separator_class(class: &str) -> String {
  merge_classes(classes([Some(DROPDOWN_SEPARATOR_BASE_CLASS)]), class)
}

/// A sub trigger: an item with a chevron at its end.
pub fn dropdown_sub_trigger_class(inset: bool, class: &str) -> String {
  let class = merge_classes(classes([Some(MENU_SUB_TRIGGER_CLASS)]), class);
  if inset { dropdown_inset_item_class(false, &class) } else { dropdown_item_class(false, &class) }
}

/// Classes for the shortcut hint at an item's end: base classes with `class` merged
/// over them.
pub fn dropdown_shortcut_class(class: &str) -> String {
  merge_classes(classes([Some(DROPDOWN_SHORTCUT_BASE_CLASS)]), class)
}

/// What a `Dropdown` shares with its parts.
#[derive(Clone, Copy)]
struct DropdownContext(OverlayRoot);

/// The root of a dropdown menu: it owns whether the menu is open. Pass `open`
/// to control it, or `default_open` to start it; `on_open_change` hears every
/// change the user makes either way.
#[component]
pub fn Dropdown(
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let root = use_overlay_root("dropdown", open, default_open, on_open_change);
  use_context_provider(|| DropdownContext(root));

  rsx! { {children} }
}

/// A button that toggles the menu and anchors it. Style it with `class`,
/// such as `button_class(..)`.
#[component]
pub fn DropdownTrigger(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_root_context::<DropdownContext>("DropdownTrigger", "Dropdown").0;
  overlay_trigger(root, "menu", class, disabled, attributes, children)
}

/// Opening focuses the first enabled item. Arrows move focus with wrapping,
/// Home, End, and typeahead jump, and activating an item closes the menu and
/// returns focus to the trigger. The menu is placed next to the trigger.
/// Escape and outside interactions close it per `dismiss`.
#[component]
pub fn DropdownContent(
  #[props(default)] class: String,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::End)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let root = use_root_context::<DropdownContext>("DropdownContent", "Dropdown").0;
  let class = dropdown_content_class(&class);
  let open = root.is_open();
  let anchor_id = Some(root.trigger_id());
  let menu = use_listbox(open, anchor_id.clone(), ListboxMode::Menu, None, Some(root.set_open));
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id, anchor_point: None, side, align, side_offset },
    dismiss,
    Some(root.set_open),
  );

  rsx! {
    div {
      role: "menu",
      id: root.content_id(),
      class,
      hidden: !open,
      "aria-labelledby": root.trigger_id(),
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
  let class = with_density(density_control_class(use_density()), &class);
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
  let class = dropdown_checkbox_item_class(
    checked,
    &with_density(density_control_class(use_density()), &class),
  );

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

/// Groups radio items and owns the checked item's value. Pass `value` to
/// control it, or `default_value` to start it; `on_value_change` hears every
/// change the user makes either way.
#[component]
pub fn DropdownRadioGroup(
  #[props(default)] value: ReadSignal<Option<String>>,
  #[props(default)] default_value: Option<String>,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let group = use_menu_radio_group(value, default_value, on_value_change);

  rsx! {
    div {
      role: "group",
      class,
      "data-value": group.value(),
      {children}
    }
  }
}

/// A radio item: activation checks it in its group and calls `onclick`.
/// Shows a dot while checked.
#[component]
pub fn DropdownRadioItem(
  value: String,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let group = use_menu_radio_item("DropdownRadioItem", "DropdownRadioGroup");
  let checked = group.value().as_ref() == Some(&value);
  let class =
    dropdown_radio_item_class(checked, &with_density(density_control_class(use_density()), &class));

  rsx! {
    div {
      role: "menuitemradio",
      class,
      "aria-checked": checked.to_string(),
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      "data-state": if checked { "checked" } else { "unchecked" },
      onclick: move |event| {
        if !disabled {
          group.choose(value.clone());
          if let Some(handler) = onclick {
            handler.call(event);
          }
        }
      },
      {children}
    }
  }
}

/// A nested menu (RFC 0067) that owns whether it is open. Put
/// `DropdownSubTrigger` and `DropdownSubContent` inside. Pass `open` to
/// control it, or `default_open` to start it; `on_open_change` hears every
/// change either way.
#[component]
pub fn DropdownSub(
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  use_menu_sub(open, default_open, on_open_change);

  rsx! {
    div {
      role: "group",
      class,
      {children}
    }
  }
}

/// Opens its submenu on click, Enter, Space, ArrowRight (ArrowLeft in
/// right-to-left), or hover.
#[component]
pub fn DropdownSubTrigger(
  #[props(default)] inset: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let sub = use_menu_sub_part("DropdownSubTrigger", "DropdownSub");
  let open = sub.is_open();
  let class =
    dropdown_sub_trigger_class(inset, &with_density(density_control_class(use_density()), &class));

  rsx! {
    div {
      role: "menuitem",
      id: sub.trigger_id(),
      class,
      "aria-haspopup": "menu",
      "aria-expanded": open.to_string(),
      "aria-controls": sub.content_id(),
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      "data-state": if open { "open" } else { "closed" },
      onclick: move |_| {
        if !disabled {
          sub.set_open.call(true);
        }
      },
      {children}
    }
  }
}

/// The submenu, placed at its trigger's inline end. ArrowLeft (ArrowRight in
/// right-to-left) and Escape close it and return focus to the trigger;
/// choosing an item closes every level.
#[component]
pub fn DropdownSubContent(#[props(default)] class: String, children: Element) -> Element {
  let (sub, listbox, anchored) = use_menu_sub_content("DropdownSubContent", "DropdownSub");
  let open = sub.is_open();
  let class = dropdown_content_class(&class);

  rsx! {
    div {
      role: "menu",
      id: sub.content_id(),
      class,
      hidden: !open,
      "aria-labelledby": sub.trigger_id(),
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-submenu": "",
      "data-dxui-anchored": anchored,
      "data-dxui-listbox": listbox,
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
        DropdownRadioGroup { default_value: "top",
          DropdownRadioItem { value: "top", "Top" }
          DropdownRadioItem { value: "bottom", disabled: true, "Bottom" }
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
  fn ssr_submenu_links_its_trigger_and_content() {
    fn app() -> Element {
      rsx! {
        DropdownSub { default_open: true,
          DropdownSubTrigger { "Share" }
          DropdownSubContent { DropdownItem { "Copy link" } }
        }
        DropdownSub {
          DropdownSubTrigger { "Export" }
          DropdownSubContent { DropdownItem { "PDF" } }
        }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert_eq!(html.matches(r#"role="group""#).count(), 2);
    assert_eq!(html.matches(r#"aria-haspopup="menu""#).count(), 2);
    assert_eq!(html.matches(r#"aria-expanded="true""#).count(), 1);
    assert_eq!(html.matches(r#"aria-expanded="false""#).count(), 1);
    assert_eq!(html.matches("data-dxui-submenu").count(), 2);
    assert_eq!(html.matches(" hidden").count(), 1, "{html}");
    assert!(html.contains(MENU_SUB_TRIGGER_CLASS));
    for sub in ["dxui-menu-sub-", "-trigger", "-content"] {
      assert!(html.contains(sub), "{sub}");
    }
    // Each trigger controls the content that names itself after it.
    let ids = html
      .split(r#"aria-controls=""#)
      .skip(1)
      .filter_map(|rest| rest.split('"').next())
      .collect::<Vec<_>>();
    assert_eq!(ids.len(), 2);
    for id in ids {
      assert!(html.contains(&format!(r#"id="{id}""#)), "{id}");
      let trigger = id.replace("-content", "-trigger");
      assert!(html.contains(&format!(r#"aria-labelledby="{trigger}""#)), "{trigger}");
    }
  }

  #[test]
  fn ssr_dropdown_links_the_trigger_to_the_menu() {
    fn app() -> Element {
      rsx! {
        Dropdown { default_open: true,
          DropdownTrigger { "Options" }
          DropdownContent { DropdownItem { "Archive" } }
        }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains(r#"id="dxui-dropdown-0-trigger""#), "{html}");
    assert!(html.contains(
      r#"aria-haspopup="menu" aria-expanded="true" aria-controls="dxui-dropdown-0-content""#
    ));
    assert!(html.contains(r#"role="menu" id="dxui-dropdown-0-content""#), "{html}");
    assert!(html.contains(r#"aria-labelledby="dxui-dropdown-0-trigger""#));
    assert!(!html.contains(" hidden"), "{html}");
  }

  #[test]
  fn ssr_controlled_radio_group_checks_its_value() {
    fn app() -> Element {
      rsx! {
        DropdownRadioGroup { value: "bottom", default_value: "top",
          DropdownRadioItem { value: "top", "Top" }
          DropdownRadioItem { value: "bottom", "Bottom" }
        }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert_eq!(html.matches(r#"aria-checked="true""#).count(), 1);
    assert!(html.contains(r#"aria-checked="true" aria-disabled="false" data-disabled="false" data-state="checked">Bottom"#), "{html}");
  }

  #[test]
  fn ssr_dropdown_parts_outside_their_root_render_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        DropdownContent { "Menu" }
        DropdownRadioItem { value: "top", "Top" }
        DropdownSubTrigger { "Share" }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains("before"));
    assert!(!html.contains("Menu") && !html.contains("Top") && !html.contains("Share"), "{html}");
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
