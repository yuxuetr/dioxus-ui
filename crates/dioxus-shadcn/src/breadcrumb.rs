use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

pub const BREADCRUMB_BASE_CLASS: &str = "";
pub const BREADCRUMB_LIST_BASE_CLASS: &str =
  "flex flex-wrap items-center gap-1.5 text-sm text-muted-foreground";
pub const BREADCRUMB_ITEM_BASE_CLASS: &str = "inline-flex items-center gap-1.5";
pub const BREADCRUMB_LINK_BASE_CLASS: &str = "transition-colors hover:text-foreground";
pub const BREADCRUMB_LINK_CURRENT_CLASS: &str = "font-normal text-foreground";
pub const BREADCRUMB_PAGE_BASE_CLASS: &str = "font-normal text-foreground";
pub const BREADCRUMB_SEPARATOR_BASE_CLASS: &str = "text-muted-foreground";
pub const BREADCRUMB_ELLIPSIS_BASE_CLASS: &str =
  "flex h-9 w-9 items-center justify-center text-muted-foreground";

pub fn breadcrumb_class(class: &str) -> String {
  merge_classes(classes([Some(BREADCRUMB_BASE_CLASS)]), class)
}

pub fn breadcrumb_list_class(class: &str) -> String {
  merge_classes(classes([Some(BREADCRUMB_LIST_BASE_CLASS)]), class)
}

pub fn breadcrumb_item_class(class: &str) -> String {
  merge_classes(classes([Some(BREADCRUMB_ITEM_BASE_CLASS)]), class)
}

pub fn breadcrumb_link_class(current: bool, class: &str) -> String {
  merge_classes(
    classes([Some(BREADCRUMB_LINK_BASE_CLASS), current.then_some(BREADCRUMB_LINK_CURRENT_CLASS)]),
    class,
  )
}

pub fn breadcrumb_page_class(class: &str) -> String {
  merge_classes(classes([Some(BREADCRUMB_PAGE_BASE_CLASS)]), class)
}

pub fn breadcrumb_separator_class(class: &str) -> String {
  merge_classes(classes([Some(BREADCRUMB_SEPARATOR_BASE_CLASS)]), class)
}

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
      href,
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

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_link_renders_passed_attributes_and_keeps_aria_current() {
    fn app() -> Element {
      rsx! { BreadcrumbLink { href: "/docs", current: true, title: "Documentation", "Docs" } }
    }
    let html = render(app);

    assert!(html.contains(r#"title="Documentation""#));
    assert!(html.contains(r#"aria-current="page""#));
  }

  #[test]
  fn breadcrumb_link_class_reflects_current_state() {
    let actual = breadcrumb_link_class(true, "underline");

    assert!(actual.contains(BREADCRUMB_LINK_BASE_CLASS));
    assert!(actual.contains(BREADCRUMB_LINK_CURRENT_CLASS));
    assert!(actual.ends_with("underline"));
  }

  #[test]
  fn breadcrumb_list_class_appends_user_class() {
    let actual = breadcrumb_list_class("gap-2");

    assert_eq!(actual, "flex flex-wrap items-center text-sm text-muted-foreground gap-2");
    assert!(actual.ends_with("gap-2"));
  }
}
