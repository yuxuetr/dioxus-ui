use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  ActiveDescendantState, DismissBehavior, OverlayAlign, OverlaySide, PopoverPrimitiveConfig,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::default_attribute::default_attribute;
use crate::listbox::{ListboxMode, use_listbox};

pub const COMBOBOX_TRIGGER_BASE_CLASS: &str = "flex h-10 w-full items-center justify-between rounded-md border bg-background px-3 py-2 text-sm text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
pub const COMBOBOX_INPUT_BASE_CLASS: &str = "flex h-10 w-full rounded-md bg-transparent px-3 py-2 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50";
pub const COMBOBOX_CONTENT_BASE_CLASS: &str = "z-50 max-h-96 min-w-[max(8rem,var(--dxui-anchor-width,0px))] overflow-hidden rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-md";
pub const COMBOBOX_LIST_BASE_CLASS: &str = "max-h-80 overflow-y-auto overflow-x-hidden";
pub const COMBOBOX_EMPTY_BASE_CLASS: &str = "py-6 text-center text-sm text-muted-foreground";
pub const COMBOBOX_STATUS_BASE_CLASS: &str = "sr-only";
pub const COMBOBOX_GROUP_BASE_CLASS: &str = "overflow-hidden p-1 text-foreground";
pub const COMBOBOX_VALUE_BASE_CLASS: &str = "truncate";
pub const COMBOBOX_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-[active=true]:bg-accent data-[active=true]:text-accent-foreground data-highlighted:bg-accent data-highlighted:text-accent-foreground data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 data-[selected=true]:bg-accent";

pub fn combobox_trigger_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  classes([Some(COMBOBOX_TRIGGER_BASE_CLASS), Some(invalid_class), Some(class)])
}

pub fn combobox_input_class(class: &str) -> String {
  classes([Some(COMBOBOX_INPUT_BASE_CLASS), Some(class)])
}

pub fn combobox_content_class(class: &str) -> String {
  classes([Some(COMBOBOX_CONTENT_BASE_CLASS), Some(class)])
}

pub fn combobox_list_class(class: &str) -> String {
  classes([Some(COMBOBOX_LIST_BASE_CLASS), Some(class)])
}

pub fn combobox_empty_class(class: &str) -> String {
  classes([Some(COMBOBOX_EMPTY_BASE_CLASS), Some(class)])
}

pub fn combobox_status_class(class: &str) -> String {
  classes([Some(COMBOBOX_STATUS_BASE_CLASS), Some(class)])
}

pub fn combobox_group_class(class: &str) -> String {
  classes([Some(COMBOBOX_GROUP_BASE_CLASS), Some(class)])
}

pub fn combobox_value_class(class: &str) -> String {
  classes([Some(COMBOBOX_VALUE_BASE_CLASS), Some(class)])
}

pub fn combobox_item_class(active: bool, selected: bool, class: &str) -> String {
  let active_class = if active { "bg-accent text-accent-foreground" } else { "" };
  let selected_class = if selected { "bg-accent" } else { "" };

  classes([Some(COMBOBOX_ITEM_BASE_CLASS), Some(active_class), Some(selected_class), Some(class)])
}

pub fn combobox_active_descendant_state(active_id: Option<String>) -> ActiveDescendantState {
  ActiveDescendantState::new(active_id)
}

#[component]
pub fn ComboboxTrigger(
  #[props(default)] open: bool,
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = combobox_trigger_class(invalid, &class);

  rsx! {
    button {
      r#type: "button",
      role: "combobox",
      class,
      disabled,
      "aria-expanded": open.to_string(),
      "aria-invalid": invalid.to_string(),
      "data-state": if open { "open" } else { "closed" },
      onclick: move |event| {
        if let Some(handler) = onclick {
          handler.call(event);
        }
      },
      ..attributes,
      {children}
    }
  }
}

/// Typed text reaches the app through `oninput`; the app filters the items it
/// renders. ArrowDown on a closed input requests open through
/// `on_open_change`. Pass `id` as the content's `anchor_id`.
#[component]
pub fn ComboboxInput(
  #[props(default)] id: Option<String>,
  #[props(default)] value: String,
  #[props(default)] placeholder: String,
  #[props(default = true)] open: bool,
  #[props(default)] active_id: Option<String>,
  #[props(default)] disabled: bool,
  #[props(default)] oninput: Option<EventHandler<FormEvent>>,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
  let class = combobox_input_class(&class);
  let active_descendant = active_id.unwrap_or_default();
  let controls = id
    .as_ref()
    .and_then(|id| default_attribute(&attributes, "aria-controls", format!("{id}-list")));

  rsx! {
    input {
      role: "combobox",
      id,
      class,
      value,
      placeholder,
      disabled,
      autocomplete: "off",
      "aria-activedescendant": active_descendant,
      "aria-autocomplete": "list",
      "aria-controls": controls,
      "aria-expanded": open.to_string(),
      oninput: move |event| {
        if let Some(handler) = oninput {
          handler.call(event);
        }
      },
      onkeydown: move |event| {
        let opens = !open && event.key() == Key::ArrowDown;
        if let Some(handler) = on_open_change.filter(|_| opens) {
          event.prevent_default();
          handler.call(true);
        }
      },
      ..attributes,
    }
  }
}

/// With `anchor_id` (the input's `id`) the content is placed next to the input
/// while focus stays in it: ArrowDown and ArrowUp move the highlighted option,
/// and Enter or click choose it through `on_value_change` before requesting
/// close. Escape and outside interactions request close per `dismiss`.
#[component]
pub fn ComboboxContent(
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
  let class = combobox_content_class(&class);
  use_context_provider(|| ComboboxAnchor(anchor_id.clone()));
  let listbox =
    use_listbox(open, anchor_id.clone(), ListboxMode::Combobox, on_value_change, on_open_change);
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id, anchor_point: None, side, align, side_offset },
    dismiss,
    on_open_change,
  );

  rsx! {
    div {
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-anchored": anchored,
      "data-dxui-listbox": listbox,
      {children}
    }
  }
}

/// The content's `anchor_id`, which names its list and derives the list's `id`.
#[derive(Clone, PartialEq)]
struct ComboboxAnchor(Option<String>);

/// Inside `ComboboxContent` with an `anchor_id`, the listbox takes the input's
/// name, and its `id` is the input's `aria-controls` value, `{anchor_id}-list`.
#[component]
pub fn ComboboxList(
  #[props(default)] active_id: Option<String>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = combobox_list_class(&class);
  let active_descendant = active_id.unwrap_or_default();
  let anchor_id = try_use_context::<ComboboxAnchor>().and_then(|anchor| anchor.0);
  let id = anchor_id.as_ref().map(|anchor_id| format!("{anchor_id}-list"));

  rsx! {
    div {
      role: "listbox",
      id,
      class,
      "aria-labelledby": anchor_id,
      "aria-activedescendant": active_descendant,
      {children}
    }
  }
}

#[component]
pub fn ComboboxEmpty(#[props(default)] class: String, children: Element) -> Element {
  let class = combobox_empty_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// A visually hidden polite status region for result announcements. Keep it
/// mounted and change only its children; an empty text says nothing.
/// Place it outside `ComboboxContent`, which is hidden while closed.
#[component]
pub fn ComboboxStatus(#[props(default)] class: String, children: Element) -> Element {
  let class = combobox_status_class(&class);

  rsx! {
    div {
      role: "status",
      class,
      "aria-live": "polite",
      "aria-atomic": "true",
      {children}
    }
  }
}

#[component]
pub fn ComboboxGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = combobox_group_class(&class);

  rsx! {
    div {
      role: "group",
      class,
      {children}
    }
  }
}

#[component]
pub fn ComboboxValue(#[props(default)] class: String, children: Element) -> Element {
  let class = combobox_value_class(&class);

  rsx! {
    span {
      class,
      {children}
    }
  }
}

#[component]
pub fn ComboboxItem(
  value: String,
  #[props(default)] active: bool,
  #[props(default)] selected: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = combobox_item_class(active, selected, &class);

  rsx! {
    div {
      role: "option",
      class,
      "aria-disabled": disabled.to_string(),
      "aria-selected": selected.to_string(),
      "data-active": active.to_string(),
      "data-disabled": disabled.to_string(),
      "data-selected": selected.to_string(),
      "data-value": value,
      {children}
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
  fn ssr_trigger_renders_passed_attributes_and_keeps_its_state() {
    fn app() -> Element {
      rsx! { ComboboxTrigger { id: "fruit", "aria-controls": "fruit-list", "Pick" } }
    }
    let html = render(app);

    assert!(html.contains(r#"id="fruit""#));
    assert!(html.contains(r#"aria-controls="fruit-list""#));
    assert!(html.contains(r#"aria-expanded="false""#));
    assert!(html.contains(r#"role="combobox""#));
  }

  #[test]
  fn ssr_input_controls_the_list_that_takes_its_name() {
    fn app() -> Element {
      rsx! {
        ComboboxInput { id: "fruit", open: true }
        ComboboxContent { open: true, anchor_id: "fruit",
          ComboboxList { ComboboxItem { value: "apple", "Apple" } }
        }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"aria-controls="fruit-list""#));
    assert!(html.contains(r#"id="fruit-list""#));
    assert!(html.contains(r#"aria-labelledby="fruit""#));
  }

  #[test]
  fn ssr_passed_aria_controls_replaces_the_derived_one() {
    fn app() -> Element {
      rsx! { ComboboxInput { id: "fruit", "aria-controls": "custom-list" } }
    }
    let html = render(app);

    assert!(html.contains(r#"aria-controls="custom-list""#));
    assert!(!html.contains("fruit-list"));
  }

  #[test]
  fn combobox_trigger_class_reflects_invalid_state() {
    let actual = combobox_trigger_class(true, "w-60");

    assert!(actual.contains(COMBOBOX_TRIGGER_BASE_CLASS));
    assert!(actual.contains("border-destructive focus-visible:ring-destructive"));
    assert!(actual.ends_with("w-60"));
  }

  #[test]
  fn combobox_item_class_reflects_active_and_selected_state() {
    let actual = combobox_item_class(true, true, "gap-2");

    assert!(actual.contains(COMBOBOX_ITEM_BASE_CLASS));
    assert!(actual.contains("bg-accent text-accent-foreground"));
    assert!(actual.ends_with("gap-2"));
  }

  #[test]
  fn combobox_primitive_config_is_reexported() {
    let config = PopoverPrimitiveConfig::controlled(true);

    assert!(config.open);
  }
}
