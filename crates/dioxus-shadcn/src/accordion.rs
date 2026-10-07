//! Accordion: collapsible content sections. The root owns which items are open
//! (RFC 0077); items, triggers, and content are styled parts.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

use crate::choice::{Choice, use_choice};
use crate::density::{density_control_class, use_density, with_density};
use crate::element_id::next_element_id;
use crate::root_state::use_root_context;
use crate::roving_group::{group_part_id, use_roving_group};

const ACCORDION_ITEM_BASE_CLASS: &str = "border-b border-border";
const ACCORDION_TRIGGER_BASE_CLASS: &str = "flex w-full items-center justify-between py-4 text-left text-sm font-medium text-foreground transition-colors hover:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
const ACCORDION_CONTENT_BASE_CLASS: &str = "overflow-hidden pb-4 text-sm text-muted-foreground";

/// Classes for an item: its bottom border, with `class` merged over it.
pub fn accordion_item_class(class: &str) -> String {
  merge_classes(classes([Some(ACCORDION_ITEM_BASE_CLASS)]), class)
}

/// Classes for the trigger button that opens and closes an item, with `class`
/// merged over them.
pub fn accordion_trigger_class(class: &str) -> String {
  merge_classes(classes([Some(ACCORDION_TRIGGER_BASE_CLASS)]), class)
}

/// Classes for an item's content panel, with `class` merged over them.
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
  let class = accordion_trigger_class(&with_density(density_control_class(use_density()), &class));
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

#[cfg(test)]
mod tests {
  use super::*;

  thread_local! {
    static SEEN: std::cell::RefCell<Vec<Vec<String>>> = const { std::cell::RefCell::new(Vec::new()) };
  }

  fn record(choice: Choice) {
    SEEN.with(|seen| seen.borrow_mut().push(choice.chosen()));
  }

  fn toggled(multiple: bool) -> Vec<Vec<String>> {
    SEEN.with(|seen| seen.borrow_mut().clear());
    let app = if multiple { multiple_app } else { single_app };
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    SEEN.with(|seen| seen.borrow().clone())
  }

  fn toggle_twice(choice: Choice) {
    use_hook(move || {
      for value in ["a", "b", "b"] {
        choice.toggle(value.to_string());
        record(choice);
      }
    });
  }

  fn single_app() -> Element {
    toggle_twice(use_choice(
      false,
      ReadSignal::default(),
      None,
      None,
      ReadSignal::default(),
      Vec::new(),
      None,
    ));
    rsx! {}
  }

  fn multiple_app() -> Element {
    toggle_twice(use_choice(
      true,
      ReadSignal::default(),
      None,
      None,
      ReadSignal::default(),
      Vec::new(),
      None,
    ));
    rsx! {}
  }

  #[test]
  fn toggling_the_chosen_single_value_clears_it() {
    let strings =
      |values: &[&str]| values.iter().map(|value| value.to_string()).collect::<Vec<_>>();

    assert_eq!(toggled(false), [strings(&["a"]), strings(&["b"]), strings(&[])]);
    assert_eq!(toggled(true), [strings(&["a"]), strings(&["a", "b"]), strings(&["a"])]);
  }

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  fn items() -> Element {
    rsx! {
      for value in ["shipping", "returns"] {
        AccordionItem { key: "{value}", value,
          AccordionTrigger { "{value}" }
          AccordionContent { "About {value}" }
        }
      }
    }
  }

  #[test]
  fn ssr_opens_the_default_item_and_links_its_parts() {
    fn app() -> Element {
      rsx! { Accordion { default_value: "returns", {items()} } }
    }
    let html = render(app);

    assert_eq!(html.matches(r#"aria-expanded="true""#).count(), 1, "{html}");
    assert!(html.contains(r#"id="dxui-accordion-0-trigger-returns""#));
    assert!(
      html.contains(r#"aria-expanded="true" aria-controls="dxui-accordion-0-content-returns""#)
    );
    assert!(html.contains(r#"aria-labelledby="dxui-accordion-0-trigger-shipping""#));
    assert_eq!(html.matches(r#"aria-expanded="false""#).count(), 1);
  }

  #[test]
  fn ssr_a_multiple_accordion_opens_every_default_item() {
    fn app() -> Element {
      rsx! {
        Accordion { multiple: true, default_values: vec!["shipping".to_string(), "returns".to_string()],
          {items()}
        }
      }
    }

    assert_eq!(render(app).matches(r#"aria-expanded="true""#).count(), 2);
  }

  #[test]
  fn ssr_a_controlled_empty_value_closes_every_item() {
    fn app() -> Element {
      rsx! { Accordion { value: String::new(), default_value: "returns", {items()} } }
    }
    let html = render(app);

    assert_eq!(html.matches(r#"aria-expanded="true""#).count(), 0);
    assert_eq!(html.matches(r#"data-state="closed""#).count(), 6);
  }

  #[test]
  fn a_part_outside_its_item_renders_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        Accordion { AccordionTrigger { "Shipping" } }
        AccordionItem { value: "returns", "Returns" }
        p { "after" }
      }
    }

    let html = render(app);

    assert!(html.starts_with("<p>before</p>") && html.ends_with("<p>after</p>"), "{html}");
    assert!(!html.contains("Shipping") && !html.contains("Returns"), "{html}");
  }

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
}
