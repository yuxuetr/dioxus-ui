use dioxus::prelude::*;
use dioxus_ui_core::classes;

pub const PAGINATION_BASE_CLASS: &str = "mx-auto flex w-full justify-center";
pub const PAGINATION_CONTENT_BASE_CLASS: &str = "flex flex-row items-center gap-1";
pub const PAGINATION_ITEM_BASE_CLASS: &str = "";
pub const PAGINATION_LINK_BASE_CLASS: &str = "inline-flex h-10 min-w-10 items-center justify-center rounded-md px-3 text-sm font-medium transition-colors hover:bg-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600";
pub const PAGINATION_LINK_ACTIVE_CLASS: &str = "border border-zinc-200 bg-white";
pub const PAGINATION_LINK_DISABLED_CLASS: &str = "pointer-events-none opacity-50";
pub const PAGINATION_ELLIPSIS_BASE_CLASS: &str =
  "flex h-10 w-10 items-center justify-center text-sm text-zinc-600";

pub fn pagination_class(class: &str) -> String {
  classes([Some(PAGINATION_BASE_CLASS), Some(class)])
}

pub fn pagination_content_class(class: &str) -> String {
  classes([Some(PAGINATION_CONTENT_BASE_CLASS), Some(class)])
}

pub fn pagination_item_class(class: &str) -> String {
  classes([Some(PAGINATION_ITEM_BASE_CLASS), Some(class)])
}

pub fn pagination_link_class(active: bool, disabled: bool, class: &str) -> String {
  classes([
    Some(PAGINATION_LINK_BASE_CLASS),
    active.then_some(PAGINATION_LINK_ACTIVE_CLASS),
    disabled.then_some(PAGINATION_LINK_DISABLED_CLASS),
    Some(class),
  ])
}

pub fn pagination_ellipsis_class(class: &str) -> String {
  classes([Some(PAGINATION_ELLIPSIS_BASE_CLASS), Some(class)])
}

#[component]
pub fn Pagination(#[props(default)] class: String, children: Element) -> Element {
  let class = pagination_class(&class);

  rsx! {
    nav {
      class,
      role: "navigation",
      "aria-label": "pagination",
      {children}
    }
  }
}

#[component]
pub fn PaginationContent(#[props(default)] class: String, children: Element) -> Element {
  let class = pagination_content_class(&class);

  rsx! {
    ul {
      class,
      {children}
    }
  }
}

#[component]
pub fn PaginationItem(#[props(default)] class: String, children: Element) -> Element {
  let class = pagination_item_class(&class);

  rsx! {
    li {
      class,
      {children}
    }
  }
}

#[component]
pub fn PaginationLink(
  #[props(default)] href: String,
  #[props(default)] active: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = pagination_link_class(active, disabled, &class);

  rsx! {
    a {
      class,
      href,
      "aria-current": if active { "page" } else { "false" },
      "aria-disabled": disabled.to_string(),
      {children}
    }
  }
}

#[component]
pub fn PaginationPrevious(
  #[props(default)] href: String,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
) -> Element {
  let class = pagination_link_class(false, disabled, &class);

  rsx! {
    a {
      class,
      href,
      "aria-label": "Go to previous page",
      "aria-disabled": disabled.to_string(),
      "Previous"
    }
  }
}

#[component]
pub fn PaginationNext(
  #[props(default)] href: String,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
) -> Element {
  let class = pagination_link_class(false, disabled, &class);

  rsx! {
    a {
      class,
      href,
      "aria-label": "Go to next page",
      "aria-disabled": disabled.to_string(),
      "Next"
    }
  }
}

#[component]
pub fn PaginationEllipsis(#[props(default)] class: String) -> Element {
  let class = pagination_ellipsis_class(&class);

  rsx! {
    span {
      class,
      "aria-hidden": "true",
      "..."
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pagination_link_class_reflects_active_and_disabled_states() {
    let actual = pagination_link_class(true, true, "rounded-full");

    assert!(actual.contains(PAGINATION_LINK_BASE_CLASS));
    assert!(actual.contains(PAGINATION_LINK_ACTIVE_CLASS));
    assert!(actual.contains(PAGINATION_LINK_DISABLED_CLASS));
    assert!(actual.ends_with("rounded-full"));
  }
}
