use super::element_id::next_element_id;
use super::listbox::{ListboxMode, use_listbox};
use super::root_state::use_root_context;
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

pub const COMMAND_BASE_CLASS: &str =
  "flex h-full w-full flex-col overflow-hidden rounded-md bg-background text-foreground";
pub const COMMAND_INPUT_BASE_CLASS: &str = "flex h-11 w-full rounded-md bg-transparent px-3 py-2 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50";
pub const COMMAND_LIST_BASE_CLASS: &str = "max-h-80 overflow-y-auto overflow-x-hidden";
pub const COMMAND_EMPTY_BASE_CLASS: &str = "py-6 text-center text-sm text-muted-foreground";
pub const COMMAND_STATUS_BASE_CLASS: &str = "sr-only";
pub const COMMAND_GROUP_BASE_CLASS: &str = "overflow-hidden p-1 text-foreground";
pub const COMMAND_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-muted-foreground";
pub const COMMAND_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-highlighted:bg-accent data-highlighted:text-accent-foreground data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
pub const COMMAND_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-border";
pub const COMMAND_SHORTCUT_BASE_CLASS: &str =
  "ml-auto text-xs tracking-normal text-muted-foreground";

pub fn command_class(class: &str) -> String {
  merge_classes(classes([Some(COMMAND_BASE_CLASS)]), class)
}

pub fn command_input_class(class: &str) -> String {
  merge_classes(classes([Some(COMMAND_INPUT_BASE_CLASS)]), class)
}

pub fn command_list_class(class: &str) -> String {
  merge_classes(classes([Some(COMMAND_LIST_BASE_CLASS)]), class)
}

pub fn command_empty_class(class: &str) -> String {
  merge_classes(classes([Some(COMMAND_EMPTY_BASE_CLASS)]), class)
}

pub fn command_status_class(class: &str) -> String {
  merge_classes(classes([Some(COMMAND_STATUS_BASE_CLASS)]), class)
}

pub fn command_group_class(class: &str) -> String {
  merge_classes(classes([Some(COMMAND_GROUP_BASE_CLASS)]), class)
}

pub fn command_label_class(class: &str) -> String {
  merge_classes(classes([Some(COMMAND_LABEL_BASE_CLASS)]), class)
}

pub fn command_item_class(class: &str) -> String {
  merge_classes(classes([Some(COMMAND_ITEM_BASE_CLASS)]), class)
}

pub fn command_separator_class(class: &str) -> String {
  merge_classes(classes([Some(COMMAND_SEPARATOR_BASE_CLASS)]), class)
}

pub fn command_shortcut_class(class: &str) -> String {
  merge_classes(classes([Some(COMMAND_SHORTCUT_BASE_CLASS)]), class)
}

/// Returns true when the trimmed `query` is empty or `label` contains it,
/// ignoring case.
pub fn command_matches(label: &str, query: &str) -> bool {
  let query = query.trim();

  query.is_empty() || label.to_lowercase().contains(&query.to_lowercase())
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

/// The root of a command list: it owns which option is highlighted (RFC 0077).
/// Keeps focus in `CommandInput` and highlights options from there, pointing
/// the input's `aria-activedescendant` at the highlighted one: the first
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
  let base_id = use_hook(|| format!("dxui-command-{}", next_element_id()));
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
  #[props(default)] disabled: bool,
  #[props(default)] oninput: Option<EventHandler<FormEvent>>,
  #[props(default)] class: String,
) -> Element {
  let context = use_root_context::<CommandContext>("CommandInput", "Command");
  let class = command_input_class(&class);

  rsx! {
    input {
      role: "combobox",
      id: context.input_id(),
      class,
      value,
      placeholder,
      disabled,
      autocomplete: "off",
      "aria-autocomplete": "list",
      "aria-controls": context.list_id(),
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
pub fn CommandList(#[props(default)] class: String, children: Element) -> Element {
  let context = use_root_context::<CommandContext>("CommandList", "Command");
  let class = command_list_class(&class);

  rsx! {
    div {
      role: "listbox",
      id: context.list_id(),
      class,
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

/// Reports `value`, or its `id` without one, when chosen; `data-highlighted`
/// marks it while highlighted.
#[component]
pub fn CommandItem(
  id: String,
  #[props(default)] value: Option<String>,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = command_item_class(&class);
  let value = command_item_value(&id, value);

  rsx! {
    div {
      id,
      role: "option",
      class,
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
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
