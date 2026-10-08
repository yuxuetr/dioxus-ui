//! Pagination: styled navigation parts for paged result sets, and the page range
//! they show.
use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

use crate::default_attribute::default_attribute;
use crate::density::{density_control_class, use_density, with_density};
use crate::safe_url::safe_href;

const PAGINATION_BASE_CLASS: &str = "mx-auto flex w-full justify-center";
const PAGINATION_CONTENT_BASE_CLASS: &str =
  "flex flex-row flex-wrap items-center justify-center gap-1";
const PAGINATION_ITEM_BASE_CLASS: &str = "";
const PAGINATION_LINK_BASE_CLASS: &str = "inline-flex h-10 min-w-10 items-center justify-center rounded-md px-3 text-sm font-medium transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
const PAGINATION_LINK_ACTIVE_CLASS: &str = "border border-border bg-background";
const PAGINATION_LINK_DISABLED_CLASS: &str = "pointer-events-none opacity-50";
const PAGINATION_ELLIPSIS_BASE_CLASS: &str =
  "flex h-10 w-10 items-center justify-center text-sm text-muted-foreground";

/// One entry of a pagination row: a page number or an ellipsis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaginationRangeItem {
  /// A link to this page number, 1-based.
  Page(u32),
  /// Stands for the skipped pages in a gap.
  Ellipsis,
}

/// The entries for page `current` of `total` (both 1-based): every page when
/// they fit in `2 * siblings + 5` entries, otherwise the first and last page,
/// `siblings` pages on each side of `current`, and an ellipsis for each gap of
/// two or more pages. Long ranges always have `2 * siblings + 5` entries, so
/// the row does not change width as the page moves. `current` is clamped to
/// `1..=total`; a `total` of 0 gives no entries.
pub fn pagination_range(current: u32, total: u32, siblings: u32) -> Vec<PaginationRangeItem> {
  use PaginationRangeItem::{Ellipsis, Page};

  let slots = siblings.saturating_mul(2).saturating_add(5);
  if total <= slots {
    return (1..total.saturating_add(1)).map(Page).collect();
  }
  let current = current.clamp(1, total);
  let left = current.saturating_sub(siblings).max(1);
  let right = current.saturating_add(siblings).min(total);
  // An edge run covers the pages a missing ellipsis would have stood for.
  let edge = slots - 2;
  match (left > 3, right < total - 2) {
    (false, _) => (1..edge + 1).map(Page).chain([Ellipsis, Page(total)]).collect(),
    (true, false) => {
      [Page(1), Ellipsis].into_iter().chain((total - edge + 1..total + 1).map(Page)).collect()
    }
    (true, true) => [Page(1), Ellipsis]
      .into_iter()
      .chain((left..right + 1).map(Page))
      .chain([Ellipsis, Page(total)])
      .collect(),
  }
}

fn pagination_class(class: &str) -> String {
  merge_classes(classes([Some(PAGINATION_BASE_CLASS)]), class)
}

fn pagination_content_class(class: &str) -> String {
  merge_classes(classes([Some(PAGINATION_CONTENT_BASE_CLASS)]), class)
}

fn pagination_item_class(class: &str) -> String {
  merge_classes(classes([Some(PAGINATION_ITEM_BASE_CLASS)]), class)
}

/// Classes for a page link: base classes, a border when `active`, dimmed and inert when
/// `disabled`, then `class` merged over them.
pub fn pagination_link_class(active: bool, disabled: bool, class: &str) -> String {
  merge_classes(
    classes([
      Some(PAGINATION_LINK_BASE_CLASS),
      active.then_some(PAGINATION_LINK_ACTIVE_CLASS),
      disabled.then_some(PAGINATION_LINK_DISABLED_CLASS),
    ]),
    class,
  )
}

fn pagination_ellipsis_class(class: &str) -> String {
  merge_classes(classes([Some(PAGINATION_ELLIPSIS_BASE_CLASS)]), class)
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
  let class = pagination_link_class(
    active,
    disabled,
    &with_density(density_control_class(use_density()), &class),
  );

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
  let class = pagination_link_class(
    false,
    disabled,
    &with_density(density_control_class(use_density()), &class),
  );

  pagination_control(
    href,
    false,
    disabled,
    onclick,
    class,
    default_attribute(&attributes, "aria-label", "Go to previous page"),
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
  let class = pagination_link_class(
    false,
    disabled,
    &with_density(density_control_class(use_density()), &class),
  );

  pagination_control(
    href,
    false,
    disabled,
    onclick,
    class,
    default_attribute(&attributes, "aria-label", "Go to next page"),
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
        href: (!disabled).then_some(href).and_then(safe_href),
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

#[cfg(test)]
mod tests {
  use super::*;
  use PaginationRangeItem::{Ellipsis, Page};

  #[test]
  fn pagination_range_shows_every_page_when_they_fit() {
    assert_eq!(pagination_range(1, 0, 1), vec![]);
    assert_eq!(pagination_range(3, 5, 1), (1..=5).map(Page).collect::<Vec<_>>());
    assert_eq!(pagination_range(4, 7, 1), (1..=7).map(Page).collect::<Vec<_>>());
  }

  #[test]
  fn pagination_range_keeps_its_length_as_the_page_moves() {
    assert_eq!(
      pagination_range(1, 10, 1),
      vec![Page(1), Page(2), Page(3), Page(4), Page(5), Ellipsis, Page(10)]
    );
    assert_eq!(
      pagination_range(4, 10, 1),
      vec![Page(1), Page(2), Page(3), Page(4), Page(5), Ellipsis, Page(10)]
    );
    assert_eq!(
      pagination_range(5, 10, 1),
      vec![Page(1), Ellipsis, Page(4), Page(5), Page(6), Ellipsis, Page(10)]
    );
    assert_eq!(
      pagination_range(7, 10, 1),
      vec![Page(1), Ellipsis, Page(6), Page(7), Page(8), Page(9), Page(10)]
    );
    assert_eq!(
      pagination_range(10, 10, 1),
      vec![Page(1), Ellipsis, Page(6), Page(7), Page(8), Page(9), Page(10)]
    );
    for current in 1..=50 {
      assert_eq!(pagination_range(current, 50, 2).len(), 9, "{current}");
    }
  }

  #[test]
  fn pagination_range_clamps_the_current_page() {
    assert_eq!(pagination_range(0, 10, 1), pagination_range(1, 10, 1));
    assert_eq!(pagination_range(99, 10, 1), pagination_range(10, 10, 1));
    assert_eq!(pagination_range(2, 20, 0), vec![Page(1), Page(2), Page(3), Ellipsis, Page(20)]);
  }

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  fn aria_labels(html: &str) -> Vec<&str> {
    html.split("aria-label=\"").skip(1).filter_map(|rest| rest.split('"').next()).collect()
  }

  #[test]
  fn ssr_renders_only_a_passed_aria_label() {
    fn app() -> Element {
      rsx! {
        PaginationPrevious { "aria-label": "Vorherige Seite" }
        PaginationNext { href: "/2", "aria-label": "Nächste Seite" }
      }
    }

    assert_eq!(aria_labels(&render(app)), ["Vorherige Seite", "Nächste Seite"]);
  }

  #[test]
  fn ssr_renders_the_default_attribute() {
    fn app() -> Element {
      rsx! {
        PaginationPrevious {}
        PaginationNext { href: "/2" }
      }
    }

    assert_eq!(aria_labels(&render(app)), ["Go to previous page", "Go to next page"]);
  }

  #[test]
  fn pagination_link_class_reflects_active_and_disabled_states() {
    let actual = pagination_link_class(true, true, "rounded-full");

    assert_eq!(
      actual,
      "inline-flex h-10 min-w-10 items-center justify-center px-3 text-sm font-medium transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring border border-border bg-background pointer-events-none opacity-50 rounded-full"
    );
    assert!(actual.contains(PAGINATION_LINK_ACTIVE_CLASS));
    assert!(actual.contains(PAGINATION_LINK_DISABLED_CLASS));
    assert!(actual.ends_with("rounded-full"));
  }
}
