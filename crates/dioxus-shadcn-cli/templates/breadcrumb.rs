//! Breadcrumb: semantic parts for a navigation trail. It owns structure and
//! styling; routing, links, and which page is current stay with the app.
use super::utils::{classes, merge_classes};
use super::safe_url::safe_href;
use dioxus::prelude::*;

const BREADCRUMB_BASE_CLASS: &str = "";
const BREADCRUMB_LIST_BASE_CLASS: &str =
  "flex flex-wrap items-center gap-1.5 text-sm text-muted-foreground";
const BREADCRUMB_ITEM_BASE_CLASS: &str = "inline-flex items-center gap-1.5";
const BREADCRUMB_LINK_BASE_CLASS: &str = "transition-colors hover:text-foreground";
const BREADCRUMB_LINK_CURRENT_CLASS: &str = "font-normal text-foreground";
const BREADCRUMB_PAGE_BASE_CLASS: &str = "font-normal text-foreground";
const BREADCRUMB_SEPARATOR_BASE_CLASS: &str = "text-muted-foreground";
const BREADCRUMB_ELLIPSIS_BASE_CLASS: &str =
  "flex h-9 w-9 items-center justify-center text-muted-foreground";

/// Classes for the `nav` root; it has no base classes, so this is `class`.
pub fn breadcrumb_class(class: &str) -> String {
  merge_classes(classes([Some(BREADCRUMB_BASE_CLASS)]), class)
}

/// Classes for the list: a wrapping row of muted text, with `class` merged over it.
pub fn breadcrumb_list_class(class: &str) -> String {
  merge_classes(classes([Some(BREADCRUMB_LIST_BASE_CLASS)]), class)
}

/// Classes for one item in the trail, with `class` merged over them.
pub fn breadcrumb_item_class(class: &str) -> String {
  merge_classes(classes([Some(BREADCRUMB_ITEM_BASE_CLASS)]), class)
}

/// Classes for a link: base classes, foreground text when it is the `current`
/// page, then `class` merged over them.
pub fn breadcrumb_link_class(current: bool, class: &str) -> String {
  merge_classes(classes([Some(BREADCRUMB_LINK_BASE_CLASS), current.then_some(BREADCRUMB_LINK_CURRENT_CLASS)]), class)
}

/// Classes for the current page's text, with `class` merged over them.
pub fn breadcrumb_page_class(class: &str) -> String {
  merge_classes(classes([Some(BREADCRUMB_PAGE_BASE_CLASS)]), class)
}

/// Classes for the separator between items, with `class` merged over them.
pub fn breadcrumb_separator_class(class: &str) -> String {
  merge_classes(classes([Some(BREADCRUMB_SEPARATOR_BASE_CLASS)]), class)
}

/// Classes for the ellipsis that stands in for collapsed items, with `class`
/// merged over them.
pub fn breadcrumb_ellipsis_class(class: &str) -> String {
  merge_classes(classes([Some(BREADCRUMB_ELLIPSIS_BASE_CLASS)]), class)
}

#[component]
pub fn Breadcrumb(#[props(default)] class: String, children: Element) -> Element {
  let class = breadcrumb_class(&class);

  rsx! {
    nav {
      class,
      "aria-label": "breadcrumb",
      {children}
    }
  }
}

#[component]
pub fn BreadcrumbList(#[props(default)] class: String, children: Element) -> Element {
  let class = breadcrumb_list_class(&class);

  rsx! {
    ol {
      class,
      {children}
    }
  }
}

#[component]
pub fn BreadcrumbItem(#[props(default)] class: String, children: Element) -> Element {
  let class = breadcrumb_item_class(&class);

  rsx! {
    li {
      class,
      {children}
    }
  }
}

#[component]
pub fn BreadcrumbLink(
  #[props(default)] href: String,
  #[props(default)] current: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = breadcrumb_link_class(current, &class);

  rsx! {
    a {
      class,
      href: safe_href(href),
      "aria-current": if current { "page" } else { "false" },
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn BreadcrumbPage(#[props(default)] class: String, children: Element) -> Element {
  let class = breadcrumb_page_class(&class);

  rsx! {
    span {
      class,
      "aria-current": "page",
      {children}
    }
  }
}

#[component]
pub fn BreadcrumbSeparator(#[props(default)] class: String, children: Element) -> Element {
  let class = breadcrumb_separator_class(&class);

  rsx! {
    li {
      class,
      role: "presentation",
      "aria-hidden": "true",
      {children}
    }
  }
}

#[component]
pub fn BreadcrumbEllipsis(#[props(default)] class: String) -> Element {
  let class = breadcrumb_ellipsis_class(&class);

  rsx! {
    span {
      class,
      role: "presentation",
      "aria-hidden": "true",
      "..."
    }
  }
}
