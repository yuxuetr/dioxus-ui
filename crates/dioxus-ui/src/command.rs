use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::ActiveDescendantState;

pub const COMMAND_BASE_CLASS: &str =
  "flex h-full w-full flex-col overflow-hidden rounded-md bg-white text-zinc-950";
pub const COMMAND_INPUT_BASE_CLASS: &str = "flex h-11 w-full rounded-md bg-transparent px-3 py-2 text-sm outline-none placeholder:text-zinc-500 disabled:cursor-not-allowed disabled:opacity-50";
pub const COMMAND_LIST_BASE_CLASS: &str = "max-h-80 overflow-y-auto overflow-x-hidden";
pub const COMMAND_EMPTY_BASE_CLASS: &str = "py-6 text-center text-sm text-zinc-500";
pub const COMMAND_GROUP_BASE_CLASS: &str = "overflow-hidden p-1 text-zinc-950";
pub const COMMAND_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-zinc-500";
pub const COMMAND_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-active:bg-zinc-100 data-active:text-zinc-950 data-disabled:pointer-events-none data-disabled:opacity-50 data-selected:bg-zinc-100";
pub const COMMAND_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-zinc-200";
pub const COMMAND_SHORTCUT_BASE_CLASS: &str = "ml-auto text-xs tracking-normal text-zinc-500";

pub fn command_class(class: &str) -> String {
  classes([Some(COMMAND_BASE_CLASS), Some(class)])
}

pub fn command_input_class(class: &str) -> String {
  classes([Some(COMMAND_INPUT_BASE_CLASS), Some(class)])
}

pub fn command_list_class(class: &str) -> String {
  classes([Some(COMMAND_LIST_BASE_CLASS), Some(class)])
}

pub fn command_empty_class(class: &str) -> String {
  classes([Some(COMMAND_EMPTY_BASE_CLASS), Some(class)])
}

pub fn command_group_class(class: &str) -> String {
  classes([Some(COMMAND_GROUP_BASE_CLASS), Some(class)])
}

pub fn command_label_class(class: &str) -> String {
  classes([Some(COMMAND_LABEL_BASE_CLASS), Some(class)])
}

pub fn command_item_class(active: bool, selected: bool, class: &str) -> String {
  let active_class = if active { "bg-zinc-100 text-zinc-950" } else { "" };
  let selected_class = if selected { "bg-zinc-100" } else { "" };

  classes([Some(COMMAND_ITEM_BASE_CLASS), Some(active_class), Some(selected_class), Some(class)])
}

pub fn command_separator_class(class: &str) -> String {
  classes([Some(COMMAND_SEPARATOR_BASE_CLASS), Some(class)])
}

pub fn command_shortcut_class(class: &str) -> String {
  classes([Some(COMMAND_SHORTCUT_BASE_CLASS), Some(class)])
}

pub fn command_active_descendant_state(active_id: Option<String>) -> ActiveDescendantState {
  ActiveDescendantState::new(active_id)
}

#[component]
pub fn Command(#[props(default)] class: String, children: Element) -> Element {
  let class = command_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CommandInput(
  #[props(default)] value: String,
  #[props(default)] active_id: Option<String>,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
) -> Element {
  let class = command_input_class(&class);
  let active_descendant = active_id.unwrap_or_default();

  rsx! {
    input {
      role: "combobox",
      class,
      value,
      disabled,
      "aria-activedescendant": active_descendant,
      "aria-autocomplete": "list",
      "aria-expanded": "true",
    }
  }
}

#[component]
pub fn CommandList(
  #[props(default)] active_id: Option<String>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = command_list_class(&class);
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
pub fn CommandEmpty(#[props(default)] class: String, children: Element) -> Element {
  let class = command_empty_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CommandGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = command_group_class(&class);

  rsx! {
    div {
      role: "group",
      class,
      {children}
    }
  }
}

#[component]
pub fn CommandLabel(#[props(default)] class: String, children: Element) -> Element {
  let class = command_label_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CommandItem(
  id: String,
  #[props(default)] active: bool,
  #[props(default)] selected: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = command_item_class(active, selected, &class);

  rsx! {
    div {
      id,
      role: "option",
      class,
      "aria-disabled": disabled.to_string(),
      "aria-selected": selected.to_string(),
      "data-active": active.to_string(),
      "data-disabled": disabled.to_string(),
      "data-selected": selected.to_string(),
      {children}
    }
  }
}

#[component]
pub fn CommandSeparator(#[props(default)] class: String) -> Element {
  let class = command_separator_class(&class);

  rsx! {
    div {
      role: "separator",
      class,
    }
  }
}

#[component]
pub fn CommandShortcut(#[props(default)] class: String, children: Element) -> Element {
  let class = command_shortcut_class(&class);

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
  fn command_item_class_reflects_active_and_selected_state() {
    let actual = command_item_class(true, true, "gap-2");

    assert!(actual.contains(COMMAND_ITEM_BASE_CLASS));
    assert!(actual.contains("bg-zinc-100 text-zinc-950"));
    assert!(actual.ends_with("gap-2"));
  }

  #[test]
  fn command_active_descendant_helper_reuses_primitive_state() {
    let state = command_active_descendant_state(Some("item-1".to_string()));

    assert_eq!(state.container_attributes().aria_activedescendant.as_deref(), Some("item-1"));
  }
}
