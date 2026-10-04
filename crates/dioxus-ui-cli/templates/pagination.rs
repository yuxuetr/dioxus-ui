use dioxus::prelude::*;
use super::utils::classes;

pub const PAGINATION_BASE_CLASS: &str = "mx-auto flex w-full justify-center";
pub const PAGINATION_CONTENT_BASE_CLASS: &str = "flex flex-row items-center gap-1";
pub const PAGINATION_ITEM_BASE_CLASS: &str = "";
pub const PAGINATION_LINK_BASE_CLASS: &str = "inline-flex h-10 min-w-10 items-center justify-center rounded-md px-3 text-sm font-medium transition-colors hover:bg-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600";
pub const PAGINATION_LINK_ACTIVE_CLASS: &str = "border border-zinc-200 bg-white";
pub const PAGINATION_LINK_DISABLED_CLASS: &str = "pointer-events-none opacity-50";
pub const PAGINATION_ELLIPSIS_BASE_CLASS: &str = "flex h-10 w-10 items-center justify-center text-sm text-zinc-600";

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
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = pagination_link_class(active, disabled, &class);

  pagination_control(href, active, disabled, onclick, class, None, attributes, children)
}

#[component]
pub fn PaginationPrevious(
  #[props(default)] href: String,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
) -> Element {
  let class = pagination_link_class(false, disabled, &class);

  pagination_control(
    href,
    false,
    disabled,
    onclick,
    class,
    Some("Go to previous page"),
    attributes,
    rsx! { "Previous" },
  )
}

#[component]
pub fn PaginationNext(
  #[props(default)] href: String,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
) -> Element {
  let class = pagination_link_class(false, disabled, &class);

  pagination_control(
    href,
    false,
    disabled,
    onclick,
    class,
    Some("Go to next page"),
    attributes,
    rsx! { "Next" },
  )
}

/// Renders a button when `href` is empty, so in-app pages work on every
/// renderer (Desktop opens anchors in the system browser), and an anchor
/// otherwise. A disabled anchor drops `href`, which takes it out of the Tab
/// order and stops Enter from following it.
#[allow(clippy::too_many_arguments)]
fn pagination_control(
  href: String,
  active: bool,
  disabled: bool,
  onclick: Option<EventHandler<MouseEvent>>,
  class: String,
  aria_label: Option<&'static str>,
  attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let aria_current = if active { "page" } else { "false" };
  let onclick = move |event: MouseEvent| {
    if !disabled {
      if let Some(handler) = onclick {
        handler.call(event);
      }
    }
  };

  if href.is_empty() {
    rsx! {
      button {
        r#type: "button",
        class,
        disabled,
        "aria-label": aria_label,
        "aria-current": aria_current,
        onclick,
        ..attributes,
        {children}
      }
    }
  } else {
    rsx! {
      a {
        class,
        href: (!disabled).then_some(href),
        role: disabled.then_some("link"),
        "aria-label": aria_label,
        "aria-current": aria_current,
        "aria-disabled": disabled.to_string(),
        onclick,
        ..attributes,
        {children}
      }
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
