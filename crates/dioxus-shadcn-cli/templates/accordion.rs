use super::choice::{Choice, use_choice};
use super::element_id::next_element_id;
use super::root_state::use_root_context;
use super::roving_group::{group_part_id, use_roving_group};
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

pub const ACCORDION_ITEM_BASE_CLASS: &str = "border-b border-border";
pub const ACCORDION_TRIGGER_BASE_CLASS: &str = "flex w-full items-center justify-between py-4 text-left text-sm font-medium text-foreground transition-colors hover:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
pub const ACCORDION_CONTENT_BASE_CLASS: &str = "overflow-hidden pb-4 text-sm text-muted-foreground";

pub fn accordion_item_class(class: &str) -> String {
  merge_classes(classes([Some(ACCORDION_ITEM_BASE_CLASS)]), class)
}

pub fn accordion_trigger_class(class: &str) -> String {
  merge_classes(classes([Some(ACCORDION_TRIGGER_BASE_CLASS)]), class)
}

pub fn accordion_content_class(class: &str) -> String {
  merge_classes(classes([Some(ACCORDION_CONTENT_BASE_CLASS)]), class)
}

#[derive(Clone)]
struct AccordionContext {
  base_id: String,
  open: Choice,
}

#[derive(Clone, PartialEq)]
struct AccordionItemContext {
  value: String,
}

/// What a trigger or content shares with its item: whether the item is open,
/// and the trigger and content ids.
struct AccordionPart {
  open: bool,
  value: String,
  trigger_id: String,
  content_id: String,
}

fn use_accordion_part(part: &str) -> AccordionPart {
  let context = use_root_context::<AccordionContext>(part, "Accordion");
  let value = use_root_context::<AccordionItemContext>(part, "AccordionItem").value;

  AccordionPart {
    open: context.open.chosen().contains(&value),
    trigger_id: group_part_id(&context.base_id, "trigger", &value),
    content_id: group_part_id(&context.base_id, "content", &value),
    value,
  }
}

/// The root of an accordion: it owns which items are open and links each
/// trigger to its content (RFC 0077). One item is open at a time, named by
/// `value` (controlled) or `default_value`, the empty string while all are
/// closed; with `multiple`, any number, named by `values` or
/// `default_values`. A click on a trigger, including Enter and Space, opens
/// its item, or closes it when open. Up and Down move focus between enabled
/// triggers and wrap; Home and End jump to the first and last. Every enabled
/// trigger stays a Tab stop.
#[component]
pub fn Accordion(
  #[props(default)] multiple: bool,
  #[props(default)] value: ReadSignal<Option<String>>,
  #[props(default)] default_value: String,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] values: ReadSignal<Option<Vec<String>>>,
  #[props(default)] default_values: Vec<String>,
  #[props(default)] on_values_change: Option<EventHandler<Vec<String>>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let base_id = use_hook(|| format!("dxui-accordion-{}", next_element_id()));
  let open = use_choice(
    multiple,
    value,
    Some(default_value),
    on_value_change,
    values,
    default_values,
    on_values_change,
  );
  use_context_provider(|| AccordionContext { base_id, open });
  let toggle = use_callback(move |value: String| open.toggle(value));
  let scope_id = use_roving_group(Some(toggle));

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
  let context = use_root_context::<AccordionContext>("AccordionItem", "Accordion");
  let class = accordion_item_class(&class);
  let open = context.open.chosen().contains(&value);
  use_context_provider(|| AccordionItemContext { value: value.clone() });

  rsx! {
    div {
      class,
      "data-state": if open { "open" } else { "closed" },
      "data-value": value,
      {children}
    }
  }
}

#[component]
pub fn AccordionTrigger(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = accordion_trigger_class(&class);
  let part = use_accordion_part("AccordionTrigger");

  rsx! {
    h3 {
      class: "flex",
      button {
        r#type: "button",
        id: part.trigger_id,
        class,
        disabled,
        "aria-expanded": part.open.to_string(),
        "aria-controls": part.content_id,
        "data-state": if part.open { "open" } else { "closed" },
        "data-value": part.value,
        "data-dxui-roving-item": "",
        {children}
      }
    }
  }
}

#[component]
pub fn AccordionContent(#[props(default)] class: String, children: Element) -> Element {
  let class = accordion_content_class(&class);
  let part = use_accordion_part("AccordionContent");

  rsx! {
    div {
      role: "region",
      id: part.content_id,
      class,
      hidden: !part.open,
      "aria-labelledby": part.trigger_id,
      "data-state": if part.open { "open" } else { "closed" },
      {children}
    }
  }
}
