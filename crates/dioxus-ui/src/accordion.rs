use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use dioxus_ui_core::classes;

use crate::roving_group::{group_part_id, use_roving_group};

static NEXT_ACCORDION_ID: AtomicUsize = AtomicUsize::new(0);

pub const ACCORDION_ITEM_BASE_CLASS: &str = "border-b border-border";
pub const ACCORDION_TRIGGER_BASE_CLASS: &str = "flex w-full items-center justify-between py-4 text-left text-sm font-medium text-foreground transition-colors hover:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
pub const ACCORDION_CONTENT_BASE_CLASS: &str = "overflow-hidden pb-4 text-sm text-muted-foreground";

pub fn accordion_item_class(class: &str) -> String {
  classes([Some(ACCORDION_ITEM_BASE_CLASS), Some(class)])
}

pub fn accordion_trigger_class(class: &str) -> String {
  classes([Some(ACCORDION_TRIGGER_BASE_CLASS), Some(class)])
}

pub fn accordion_content_class(class: &str) -> String {
  classes([Some(ACCORDION_CONTENT_BASE_CLASS), Some(class)])
}

/// Returns the open value of a single-open accordion after `toggled_value` is
/// toggled. Toggling the open item closes it.
pub fn accordion_single_open(current: Option<&str>, toggled_value: &str) -> Option<String> {
  if current == Some(toggled_value) { None } else { Some(toggled_value.to_string()) }
}

/// Returns the open values of a multiple-open accordion after `toggled_value`
/// is toggled.
pub fn accordion_multiple_open(current: &[String], toggled_value: &str) -> Vec<String> {
  let mut next = current.to_vec();

  if let Some(index) = next.iter().position(|value| value == toggled_value) {
    next.remove(index);
  } else {
    next.push(toggled_value.to_string());
  }

  next
}

#[derive(Clone, PartialEq)]
struct AccordionContext {
  base_id: String,
}

#[derive(Clone, PartialEq)]
struct AccordionItemContext {
  value: String,
}

/// Returns the trigger and content ids for the enclosing item, when the part is
/// inside both `Accordion` and `AccordionItem`.
fn accordion_part_ids() -> Option<(String, String, String)> {
  let base_id = try_use_context::<AccordionContext>()?.base_id;
  let value = try_use_context::<AccordionItemContext>()?.value;
  let trigger_id = group_part_id(&base_id, "trigger", &value);
  let content_id = group_part_id(&base_id, "content", &value);

  Some((value, trigger_id, content_id))
}

/// Links each trigger to its content by id and reports toggles: a click on a
/// trigger, including Enter and Space, calls `on_toggle` with its item's
/// `value`. Up and Down move focus between enabled triggers and wrap; Home and
/// End jump to the first and last. Every enabled trigger stays a Tab stop.
#[component]
pub fn Accordion(
  #[props(default)] on_toggle: Option<EventHandler<String>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let base_id =
    use_hook(|| format!("dxui-accordion-{}", NEXT_ACCORDION_ID.fetch_add(1, Ordering::Relaxed)));
  use_context_provider(|| AccordionContext { base_id });
  let scope_id = use_roving_group(on_toggle);

  rsx! {
    div {
      class,
      "data-dxui-roving-group": scope_id,
      "data-dxui-roving-orientation": "vertical",
      "data-dxui-roving-loop": "true",
      "data-dxui-roving-tab-stops": "all",
      {children}
    }
  }
}

#[component]
pub fn AccordionItem(value: String, #[props(default)] class: String, children: Element) -> Element {
  let class = accordion_item_class(&class);
  use_context_provider(|| AccordionItemContext { value: value.clone() });

  rsx! {
    div {
      class,
      "data-value": value,
      {children}
    }
  }
}

#[component]
pub fn AccordionTrigger(
  #[props(default)] open: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = accordion_trigger_class(&class);
  let ids = accordion_part_ids();
  let is_item = ids.is_some().then_some("");
  let (value, id, controls) = ids.map_or((None, None, None), |(value, trigger_id, content_id)| {
    (Some(value), Some(trigger_id), Some(content_id))
  });

  rsx! {
    h3 {
      class: "flex",
      button {
        r#type: "button",
        id,
        class,
        disabled,
        "aria-expanded": open.to_string(),
        "aria-controls": controls,
        "data-state": if open { "open" } else { "closed" },
        "data-value": value,
        "data-dxui-roving-item": is_item,
        {children}
      }
    }
  }
}

#[component]
pub fn AccordionContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = accordion_content_class(&class);
  let ids = accordion_part_ids();
  let (id, labelledby) =
    ids.map_or((None, None), |(_, trigger_id, content_id)| (Some(content_id), Some(trigger_id)));

  rsx! {
    div {
      role: "region",
      id,
      class,
      hidden: !open,
      "aria-labelledby": labelledby,
      "data-state": if open { "open" } else { "closed" },
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn accordion_trigger_class_appends_user_class() {
    let actual = accordion_trigger_class("gap-2");

    assert!(actual.contains(ACCORDION_TRIGGER_BASE_CLASS));
    assert!(actual.ends_with("gap-2"));
  }

  #[test]
  fn accordion_content_class_appends_user_class() {
    let actual = accordion_content_class("px-2");

    assert!(actual.contains(ACCORDION_CONTENT_BASE_CLASS));
    assert!(actual.ends_with("px-2"));
  }

  #[test]
  fn accordion_single_open_opens_and_closes() {
    assert_eq!(accordion_single_open(None, "shipping"), Some("shipping".to_string()));
    assert_eq!(accordion_single_open(Some("returns"), "shipping"), Some("shipping".to_string()));
    assert_eq!(accordion_single_open(Some("shipping"), "shipping"), None);
  }

  #[test]
  fn accordion_multiple_open_adds_and_removes() {
    let open = accordion_multiple_open(&["shipping".to_string()], "returns");
    assert_eq!(open, vec!["shipping".to_string(), "returns".to_string()]);
    assert_eq!(accordion_multiple_open(&open, "shipping"), vec!["returns".to_string()]);
  }
}
