use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::ActiveDescendantState;

use crate::listbox::{ListboxMode, use_listbox};

static NEXT_COMMAND_ID: AtomicUsize = AtomicUsize::new(0);

pub const COMMAND_BASE_CLASS: &str =
  "flex h-full w-full flex-col overflow-hidden rounded-md bg-white text-zinc-950";
pub const COMMAND_INPUT_BASE_CLASS: &str = "flex h-11 w-full rounded-md bg-transparent px-3 py-2 text-sm outline-none placeholder:text-zinc-500 disabled:cursor-not-allowed disabled:opacity-50";
pub const COMMAND_LIST_BASE_CLASS: &str = "max-h-80 overflow-y-auto overflow-x-hidden";
pub const COMMAND_EMPTY_BASE_CLASS: &str = "py-6 text-center text-sm text-zinc-500";
pub const COMMAND_STATUS_BASE_CLASS: &str = "sr-only";
pub const COMMAND_GROUP_BASE_CLASS: &str = "overflow-hidden p-1 text-zinc-950";
pub const COMMAND_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-zinc-500";
pub const COMMAND_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-[active=true]:bg-zinc-100 data-[active=true]:text-zinc-950 data-highlighted:bg-zinc-100 data-highlighted:text-zinc-950 data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 data-[selected=true]:bg-zinc-100";
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

pub fn command_status_class(class: &str) -> String {
  classes([Some(COMMAND_STATUS_BASE_CLASS), Some(class)])
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

/// Returns true when the trimmed `query` is empty or `label` contains it,
/// ignoring case.
pub fn command_matches(label: &str, query: &str) -> bool {
  let query = query.trim();

  query.is_empty() || label.to_lowercase().contains(&query.to_lowercase())
}

pub fn command_active_descendant_state(active_id: Option<String>) -> ActiveDescendantState {
  ActiveDescendantState::new(active_id)
}

#[derive(Clone, PartialEq)]
struct CommandContext {
  base_id: String,
}

impl CommandContext {
  fn input_id(&self) -> String {
    format!("{}-input", self.base_id)
  }

  fn list_id(&self) -> String {
    format!("{}-list", self.base_id)
  }
}

/// Keeps focus in `CommandInput` and highlights options from there: the first
/// option starts highlighted, Up, Down, Home, and End move the highlight, and
/// a query change moves it back to the first option. Enter or a click on an
/// option calls `on_select` with its value.
#[component]
pub fn Command(
  #[props(default)] on_select: Option<EventHandler<String>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = command_class(&class);
  let base_id =
    use_hook(|| format!("dxui-command-{}", NEXT_COMMAND_ID.fetch_add(1, Ordering::Relaxed)));
  let context = use_context_provider(|| CommandContext { base_id });
  let listbox = use_listbox(true, Some(context.input_id()), ListboxMode::Command, on_select, None);

  rsx! {
    div {
      class,
      "data-dxui-listbox": listbox,
      {children}
    }
  }
}

/// Typed text reaches the app through `oninput`; the app filters the items it
/// renders, for example with `command_matches`.
#[component]
pub fn CommandInput(
  #[props(default)] value: String,
  #[props(default)] placeholder: String,
  #[props(default)] active_id: Option<String>,
  #[props(default)] disabled: bool,
  #[props(default)] oninput: Option<EventHandler<FormEvent>>,
  #[props(default)] class: String,
) -> Element {
  let class = command_input_class(&class);
  let active_descendant = active_id.unwrap_or_default();
  let context = try_use_context::<CommandContext>();
  let id = context.as_ref().map(CommandContext::input_id);
  let controls = context.as_ref().map(CommandContext::list_id);

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
      "aria-expanded": "true",
      oninput: move |event| {
        if let Some(handler) = oninput {
          handler.call(event);
        }
      },
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
  let id = try_use_context::<CommandContext>().map(|context| context.list_id());

  rsx! {
    div {
      role: "listbox",
      id,
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

/// A visually hidden polite status region for result announcements. Keep it
/// mounted and change only its children; an empty text says nothing.
#[component]
pub fn CommandStatus(#[props(default)] class: String, children: Element) -> Element {
  let class = command_status_class(&class);

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

/// Returns the value an item reports when chosen: `value`, or else its `id`.
fn command_item_value(id: &str, value: Option<String>) -> String {
  value.unwrap_or_else(|| id.to_string())
}

/// Reports `value`, or its `id` without one, when chosen inside `Command`.
#[component]
pub fn CommandItem(
  id: String,
  #[props(default)] value: Option<String>,
  #[props(default)] active: bool,
  #[props(default)] selected: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = command_item_class(active, selected, &class);
  let value = command_item_value(&id, value);

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
      "data-value": value,
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
  fn command_matches_ignores_case_and_surrounding_space() {
    assert!(command_matches("Settings", ""));
    assert!(command_matches("Settings", "  "));
    assert!(command_matches("Settings", " SET "));
    assert!(command_matches("Search Emoji", "emoji"));
    assert!(!command_matches("Calendar", "set"));
  }

  #[test]
  fn command_item_value_falls_back_to_id() {
    assert_eq!(command_item_value("command-calendar", None), "command-calendar");
    assert_eq!(command_item_value("command-calendar", Some("calendar".to_string())), "calendar");
  }

  #[test]
  fn command_active_descendant_helper_reuses_primitive_state() {
    let state = command_active_descendant_state(Some("item-1".to_string()));

    assert_eq!(state.container_attributes().aria_activedescendant.as_deref(), Some("item-1"));
  }
}
