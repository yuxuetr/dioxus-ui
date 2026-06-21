use dioxus::prelude::*;
use dioxus_ui_core::classes;
use dioxus_ui_primitives::SelectPrimitiveConfig;

pub const SELECT_TRIGGER_BASE_CLASS: &str = "flex h-10 w-full items-center justify-between rounded-md border border-zinc-200 bg-white px-3 py-2 text-sm text-zinc-950 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:cursor-not-allowed disabled:opacity-50";
pub const SELECT_VALUE_BASE_CLASS: &str = "truncate";
pub const SELECT_CONTENT_BASE_CLASS: &str = "z-50 max-h-96 min-w-32 overflow-hidden rounded-md border border-zinc-200 bg-white p-1 text-zinc-950 shadow-md";
pub const SELECT_GROUP_BASE_CLASS: &str = "p-1";
pub const SELECT_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-zinc-500";
pub const SELECT_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-zinc-100 data-disabled:pointer-events-none data-disabled:opacity-50";
pub const SELECT_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-zinc-200";

pub fn select_trigger_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-red-500 focus-visible:ring-red-500"
  } else {
    "focus-visible:ring-blue-600"
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
  let selected_class = if selected {
    "bg-zinc-100 text-zinc-950"
  } else {
    "text-zinc-900"
  };

  classes([Some(SELECT_ITEM_BASE_CLASS), Some(selected_class), Some(class)])
}

pub fn select_separator_class(class: &str) -> String {
  classes([Some(SELECT_SEPARATOR_BASE_CLASS), Some(class)])
}

#[component]
pub fn SelectTrigger(
  #[props(default)] open: bool,
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = select_trigger_class(invalid, &class);

  rsx! {
    button {
      r#type: "button",
      role: "combobox",
      class,
      disabled,
      "aria-expanded": open.to_string(),
      "aria-invalid": invalid.to_string(),
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

#[component]
pub fn SelectContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = select_content_class(&class);

  rsx! {
    div {
      role: "listbox",
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
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
