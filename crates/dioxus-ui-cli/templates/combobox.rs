use dioxus::prelude::*;
use super::utils::{AnchoredPlacement, ListboxMode, classes, use_anchored_overlay, use_listbox};
pub use super::utils::{DismissBehavior, OverlayAlign, OverlaySide, PopoverPrimitiveConfig};

pub const COMBOBOX_TRIGGER_BASE_CLASS: &str = "flex h-10 w-full items-center justify-between rounded-md border border-zinc-200 bg-white px-3 py-2 text-sm text-zinc-950 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:cursor-not-allowed disabled:opacity-50";
pub const COMBOBOX_INPUT_BASE_CLASS: &str = "flex h-10 w-full rounded-md bg-transparent px-3 py-2 text-sm outline-none placeholder:text-zinc-500 disabled:cursor-not-allowed disabled:opacity-50";
pub const COMBOBOX_CONTENT_BASE_CLASS: &str = "z-50 max-h-96 min-w-32 overflow-hidden rounded-md border border-zinc-200 bg-white p-1 text-zinc-950 shadow-md";
pub const COMBOBOX_LIST_BASE_CLASS: &str = "max-h-80 overflow-y-auto overflow-x-hidden";
pub const COMBOBOX_EMPTY_BASE_CLASS: &str = "py-6 text-center text-sm text-zinc-500";
pub const COMBOBOX_GROUP_BASE_CLASS: &str = "overflow-hidden p-1 text-zinc-950";
pub const COMBOBOX_VALUE_BASE_CLASS: &str = "truncate";
pub const COMBOBOX_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-active:bg-zinc-100 data-active:text-zinc-950 data-highlighted:bg-zinc-100 data-highlighted:text-zinc-950 data-disabled:pointer-events-none data-disabled:opacity-50 data-selected:bg-zinc-100";

pub fn combobox_trigger_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-red-500 focus-visible:ring-red-500"
  } else {
    "focus-visible:ring-blue-600"
  };

  classes([
    Some(COMBOBOX_TRIGGER_BASE_CLASS),
    Some(invalid_class),
    Some(class),
  ])
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

pub fn combobox_group_class(class: &str) -> String {
  classes([Some(COMBOBOX_GROUP_BASE_CLASS), Some(class)])
}

pub fn combobox_value_class(class: &str) -> String {
  classes([Some(COMBOBOX_VALUE_BASE_CLASS), Some(class)])
}

pub fn combobox_item_class(active: bool, selected: bool, class: &str) -> String {
  let active_class = if active { "bg-zinc-100 text-zinc-950" } else { "" };
  let selected_class = if selected { "bg-zinc-100" } else { "" };

  classes([
    Some(COMBOBOX_ITEM_BASE_CLASS),
    Some(active_class),
    Some(selected_class),
    Some(class),
  ])
}

#[component]
pub fn ComboboxTrigger(
  #[props(default)] open: bool,
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
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
) -> Element {
  let class = combobox_input_class(&class);
  let active_descendant = active_id.unwrap_or_default();

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

#[component]
pub fn ComboboxList(
  #[props(default)] active_id: Option<String>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = combobox_list_class(&class);
  let active_descendant = active_id.unwrap_or_default();

  rsx! {
    div {
      role: "listbox",
      class,
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
