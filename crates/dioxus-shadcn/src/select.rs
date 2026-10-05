use dioxus::prelude::*;
use dioxus_shadcn_core::classes;
pub use dioxus_shadcn_primitives::{
  DismissBehavior, OverlayAlign, OverlaySide, SelectPrimitiveConfig,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::default_attribute::default_attribute;
use crate::listbox::{ListboxMode, use_listbox};

pub const SELECT_TRIGGER_BASE_CLASS: &str = "flex h-10 w-full items-center justify-between rounded-md border bg-background px-3 py-2 text-sm text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
pub const SELECT_VALUE_BASE_CLASS: &str = "truncate";
pub const SELECT_CONTENT_BASE_CLASS: &str = "z-50 max-h-96 min-w-[max(8rem,var(--dxui-anchor-width,0px))] overflow-hidden rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-md";
pub const SELECT_GROUP_BASE_CLASS: &str = "p-1";
pub const SELECT_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-muted-foreground";
pub const SELECT_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent data-highlighted:bg-accent data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
pub const SELECT_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-border";

pub fn select_trigger_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  classes([Some(SELECT_TRIGGER_BASE_CLASS), Some(invalid_class), Some(class)])
}

pub fn select_value_class(class: &str) -> String {
  classes([Some(SELECT_VALUE_BASE_CLASS), Some(class)])
}

pub fn select_content_class(class: &str) -> String {
  classes([Some(SELECT_CONTENT_BASE_CLASS), Some(class)])
}

pub fn select_group_class(class: &str) -> String {
  classes([Some(SELECT_GROUP_BASE_CLASS), Some(class)])
}

pub fn select_label_class(class: &str) -> String {
  classes([Some(SELECT_LABEL_BASE_CLASS), Some(class)])
}

pub fn select_item_class(selected: bool, class: &str) -> String {
  let selected_class =
    if selected { "bg-accent text-accent-foreground" } else { "text-foreground" };

  classes([Some(SELECT_ITEM_BASE_CLASS), Some(selected_class), Some(class)])
}

pub fn select_separator_class(class: &str) -> String {
  classes([Some(SELECT_SEPARATOR_BASE_CLASS), Some(class)])
}

/// Click requests `!open` through `on_open_change`, and ArrowDown or ArrowUp
/// on a closed trigger requests open. Pass `id` as the content's `anchor_id`.
#[component]
pub fn SelectTrigger(
  #[props(default)] id: Option<String>,
  #[props(default)] open: bool,
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = select_trigger_class(invalid, &class);
  let controls = id
    .as_ref()
    .and_then(|id| default_attribute(&attributes, "aria-controls", format!("{id}-content")));

  rsx! {
    button {
      r#type: "button",
      role: "combobox",
      id,
      class,
      disabled,
      "aria-controls": controls,
      "aria-expanded": open.to_string(),
      "aria-haspopup": "listbox",
      "aria-invalid": invalid.to_string(),
      onclick: move |_| {
        if let Some(handler) = on_open_change {
          handler.call(!open);
        }
      },
      onkeydown: move |event| {
        let opens = !open && matches!(event.key(), Key::ArrowDown | Key::ArrowUp);
        if let Some(handler) = on_open_change.filter(|_| opens) {
          event.prevent_default();
          handler.call(true);
        }
      },
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn SelectValue(#[props(default)] class: String, children: Element) -> Element {
  let class = select_value_class(&class);

  rsx! {
    span {
      class,
      {children}
    }
  }
}

/// With `anchor_id` (the trigger's `id`) the listbox is placed next to the
/// trigger while focus stays on it: arrows, Home, End, and typeahead move the
/// highlighted option, and Enter, Space, or click choose it through
/// `on_value_change` before requesting close. Escape and outside interactions
/// request close per `dismiss`. The listbox takes the trigger's name, and its
/// `id` is the trigger's `aria-controls` value, `{anchor_id}-content`.
#[component]
pub fn SelectContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] anchor_id: Option<String>,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::Start)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let class = select_content_class(&class);
  let id = anchor_id.as_ref().map(|anchor_id| format!("{anchor_id}-content"));
  let labelledby = anchor_id.clone();
  let listbox =
    use_listbox(open, anchor_id.clone(), ListboxMode::Select, on_value_change, on_open_change);
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id, anchor_point: None, side, align, side_offset },
    dismiss,
    on_open_change,
  );

  rsx! {
    div {
      role: "listbox",
      id,
      class,
      "aria-labelledby": labelledby,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-anchored": anchored,
      "data-dxui-listbox": listbox,
      {children}
    }
  }
}

#[component]
pub fn SelectGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = select_group_class(&class);

  rsx! {
    div {
      role: "group",
      class,
      {children}
    }
  }
}

#[component]
pub fn SelectLabel(#[props(default)] class: String, children: Element) -> Element {
  let class = select_label_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn SelectItem(
  value: String,
  #[props(default)] selected: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = select_item_class(selected, &class);

  rsx! {
    div {
      role: "option",
      class,
      "aria-selected": selected.to_string(),
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      "data-value": value,
      {children}
    }
  }
}

#[component]
pub fn SelectSeparator(#[props(default)] class: String) -> Element {
  let class = select_separator_class(&class);

  rsx! {
    div {
      role: "separator",
      class,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_trigger_controls_the_listbox_that_takes_its_name() {
    fn app() -> Element {
      rsx! {
        SelectTrigger { id: "fruit", "Pick" }
        SelectContent { open: true, anchor_id: "fruit",
          SelectItem { value: "apple", "Apple" }
          SelectItem { value: "apricot", disabled: true, "Apricot" }
        }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"aria-controls="fruit-content""#));
    assert!(html.contains(r#"id="fruit-content""#));
    assert!(html.contains(r#"aria-labelledby="fruit""#));
    assert!(html.contains(r#"aria-disabled="false""#));
    assert!(html.contains(r#"aria-disabled="true""#));
  }

  #[test]
  fn ssr_select_without_ids_renders_no_link() {
    fn app() -> Element {
      rsx! {
        SelectTrigger { "Pick" }
        SelectContent { open: true, SelectItem { value: "apple", "Apple" } }
      }
    }
    let html = render(app);

    assert!(!html.contains("aria-controls"));
    assert!(!html.contains("aria-labelledby"));
  }

  #[test]
  fn select_trigger_class_reflects_invalid_state() {
    let actual = select_trigger_class(true, "w-44");

    assert!(actual.contains(SELECT_TRIGGER_BASE_CLASS));
    assert!(actual.contains("border-destructive focus-visible:ring-destructive"));
    assert!(actual.ends_with("w-44"));
  }

  #[test]
  fn select_primitive_config_is_reexported() {
    let config = SelectPrimitiveConfig::controlled(true, Some("system".to_string()));

    assert!(config.open);
    assert_eq!(config.value, Some("system".to_string()));
  }
}
