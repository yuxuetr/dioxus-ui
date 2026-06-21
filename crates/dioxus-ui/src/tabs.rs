use dioxus::prelude::*;
use dioxus_ui_core::classes;

pub const TABS_LIST_BASE_CLASS: &str = "inline-flex h-10 items-center justify-center rounded-md bg-zinc-100 p-1 text-zinc-600";
pub const TABS_TRIGGER_BASE_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";
pub const TABS_CONTENT_BASE_CLASS: &str = "mt-2 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600";

pub fn tabs_list_class(class: &str) -> String {
  classes([Some(TABS_LIST_BASE_CLASS), Some(class)])
}

pub fn tabs_trigger_class(active: bool, class: &str) -> String {
  let active_class = if active {
    "bg-white text-zinc-950 shadow-sm"
  } else {
    "text-zinc-600 hover:text-zinc-950"
  };

  classes([Some(TABS_TRIGGER_BASE_CLASS), Some(active_class), Some(class)])
}

pub fn tabs_content_class(class: &str) -> String {
  classes([Some(TABS_CONTENT_BASE_CLASS), Some(class)])
}

#[component]
pub fn TabsList(#[props(default)] class: String, children: Element) -> Element {
  let class = tabs_list_class(&class);

  rsx! {
    div {
      role: "tablist",
      class,
      {children}
    }
  }
}

#[component]
pub fn TabsTrigger(
  value: String,
  #[props(default)] active: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = tabs_trigger_class(active, &class);

  rsx! {
    button {
      r#type: "button",
      role: "tab",
      class,
      disabled,
      "aria-selected": active.to_string(),
      "data-value": value,
      {children}
    }
  }
}

#[component]
pub fn TabsContent(
  value: String,
  #[props(default)] active: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = tabs_content_class(&class);

  rsx! {
    div {
      role: "tabpanel",
      class,
      hidden: !active,
      "data-value": value,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tabs_trigger_class_reflects_active_state() {
    let actual = tabs_trigger_class(true, "min-w-24");

    assert!(actual.contains(TABS_TRIGGER_BASE_CLASS));
    assert!(actual.contains("bg-white text-zinc-950 shadow-sm"));
    assert!(actual.ends_with("min-w-24"));
  }

  #[test]
  fn tabs_content_class_appends_user_class() {
    let actual = tabs_content_class("p-4");

    assert!(actual.contains(TABS_CONTENT_BASE_CLASS));
    assert!(actual.ends_with("p-4"));
  }
}
