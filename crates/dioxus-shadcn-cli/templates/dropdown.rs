use super::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use super::listbox::{ListboxMode, use_listbox};
use super::menu_marks::{
  MENU_CHECKBOX_MARK_CLASS, MENU_RADIO_MARK_CLASS, MENU_SUB_TRIGGER_CLASS, menu_mark_state_class,
};
use super::menu_radio::{use_menu_radio_group, use_menu_radio_item};
use super::menu_sub::{use_menu_sub, use_menu_sub_content, use_menu_sub_part};
pub use super::overlay::{DismissBehavior, DropdownPrimitiveConfig, OverlayAlign, OverlaySide};
use super::overlay_root::{OverlayRoot, overlay_trigger, use_overlay_root};
use super::root_state::use_root_context;
use super::utils::{classes, merge_classes};
use super::density::{density_control_class, use_density, with_density};
use dioxus::prelude::*;

pub const DROPDOWN_CONTENT_BASE_CLASS: &str = "z-50 min-w-32 overflow-hidden rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-md";
pub const DROPDOWN_GROUP_BASE_CLASS: &str = "p-1";
pub const DROPDOWN_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-muted-foreground";
pub const DROPDOWN_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
pub const DROPDOWN_ITEM_INSET_CLASS: &str = "pl-8";
pub const DROPDOWN_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-border";
pub const DROPDOWN_SHORTCUT_BASE_CLASS: &str =
  "ml-auto text-xs tracking-normal text-muted-foreground";

pub fn dropdown_content_class(class: &str) -> String {
  merge_classes(classes([Some(DROPDOWN_CONTENT_BASE_CLASS)]), class)
}

pub fn dropdown_group_class(class: &str) -> String {
  merge_classes(classes([Some(DROPDOWN_GROUP_BASE_CLASS)]), class)
}

pub fn dropdown_label_class(class: &str) -> String {
  merge_classes(classes([Some(DROPDOWN_LABEL_BASE_CLASS)]), class)
}

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

pub fn dropdown_separator_class(class: &str) -> String {
  merge_classes(classes([Some(DROPDOWN_SEPARATOR_BASE_CLASS)]), class)
}

/// A sub trigger: an item with a chevron at its end.
pub fn dropdown_sub_trigger_class(inset: bool, class: &str) -> String {
  let class = merge_classes(classes([Some(MENU_SUB_TRIGGER_CLASS)]), class);
  if inset { dropdown_inset_item_class(false, &class) } else { dropdown_item_class(false, &class) }
}

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
  let class = dropdown_checkbox_item_class(checked, &with_density(density_control_class(use_density()), &class));

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
  let class = dropdown_radio_item_class(checked, &with_density(density_control_class(use_density()), &class));

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
  let class = dropdown_sub_trigger_class(inset, &with_density(density_control_class(use_density()), &class));

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
