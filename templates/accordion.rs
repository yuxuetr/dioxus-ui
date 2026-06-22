use dioxus::prelude::*;
use super::utils::classes;

pub const ACCORDION_ITEM_BASE_CLASS: &str = "border-b border-zinc-200";
pub const ACCORDION_TRIGGER_BASE_CLASS: &str = "flex w-full items-center justify-between py-4 text-left text-sm font-medium transition-colors hover:text-zinc-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";
pub const ACCORDION_CONTENT_BASE_CLASS: &str = "overflow-hidden pb-4 text-sm text-zinc-600";

pub fn accordion_item_class(class: &str) -> String {
  classes([Some(ACCORDION_ITEM_BASE_CLASS), Some(class)])
}

pub fn accordion_trigger_class(open: bool, class: &str) -> String {
  let open_class = if open {
    "text-zinc-950"
  } else {
    "text-zinc-900"
  };

  classes([Some(ACCORDION_TRIGGER_BASE_CLASS), Some(open_class), Some(class)])
}

pub fn accordion_content_class(class: &str) -> String {
  classes([Some(ACCORDION_CONTENT_BASE_CLASS), Some(class)])
}

#[component]
pub fn AccordionItem(#[props(default)] class: String, children: Element) -> Element {
  let class = accordion_item_class(&class);

  rsx! {
    div {
      class,
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
  let class = accordion_trigger_class(open, &class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-expanded": open.to_string(),
      {children}
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

  rsx! {
    div {
      class,
      hidden: !open,
      {children}
    }
  }
}
