//! Safe URL: keeps a link's `href` only when following it cannot run script.

/// The schemes a link may use. Others, such as `javascript:` and `data:`,
/// can run script when the link is followed.
const SAFE_SCHEMES: [&str; 4] = ["http", "https", "mailto", "tel"];

/// `href` when it is relative, a fragment, or uses a scheme in
/// `SAFE_SCHEMES`; otherwise `None`, which renders no `href`. The scheme is
/// read as a browser reads it: without surrounding spaces and control
/// characters, and without tabs and line breaks anywhere.
pub(crate) fn safe_href(href: String) -> Option<String> {
  let cleaned = href
    .trim_matches(|character: char| character <= ' ')
    .chars()
    .filter(|character| !matches!(character, '\t' | '\n' | '\r'))
    .collect::<String>();
  match cleaned.split_once(':') {
    Some((scheme, _)) if is_scheme(scheme) => {
      SAFE_SCHEMES.iter().any(|safe| scheme.eq_ignore_ascii_case(safe)).then_some(href)
    }
    _ => Some(href),
  }
}

/// Whether `text` is a URL scheme: a letter, then letters, digits, `+`, `-`,
/// or `.`. Text before a colon that is not one, such as `/a` in `/a:b`, makes
/// the URL relative.
fn is_scheme(text: &str) -> bool {
  let mut characters = text.chars();
  characters.next().is_some_and(|character| character.is_ascii_alphabetic())
    && characters
      .all(|character| character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.'))
}

#[cfg(test)]
mod tests {
  use super::*;

  fn kept(href: &str) -> bool {
    safe_href(href.to_string()).as_deref() == Some(href)
  }

  #[test]
  fn keeps_relative_urls_fragments_and_safe_schemes() {
    for href in [
      "",
      "#",
      "#section",
      "/docs",
      "docs/button",
      "?page=2",
      "/a:b",
      "https://dioxuslabs.com",
      "HTTP://example.com",
      "mailto:team@example.com",
      "tel:+15551234",
    ] {
      assert!(kept(href), "{href:?} should be kept");
    }
  }

  #[test]
  fn drops_schemes_that_can_run_script() {
    for href in [
      "javascript:alert(1)",
      "JavaScript:alert(1)",
      "  javascript:alert(1)",
      "\u{1}javascript:alert(1)",
      "java\tscript:alert(1)",
      "java\nscript:alert(1)",
      "data:text/html,<script>alert(1)</script>",
      "vbscript:msgbox(1)",
    ] {
      assert_eq!(safe_href(href.to_string()), None, "{href:?} should be dropped");
    }
  }

  /// Every component that takes a link's URL renders a `javascript:` one
  /// without its `href`, and keeps a safe one.
  #[test]
  #[cfg(all(
    feature = "breadcrumb",
    feature = "dock",
    feature = "hover-card",
    feature = "menu",
    feature = "navigation-menu",
    feature = "pagination",
    feature = "sidebar"
  ))]
  fn components_render_no_script_href() {
    use crate::{
      BreadcrumbLink, DockItem, HoverCard, HoverCardTrigger, Menu, MenuItem, NavigationMenu,
      NavigationMenuContent, NavigationMenuItem, NavigationMenuLink, NavigationMenuList,
      NavigationMenuTrigger, PaginationLink, PaginationNext, PaginationPrevious, Sidebar,
      SidebarContent, SidebarItem, SidebarProvider,
    };
    use dioxus::prelude::*;

    fn app() -> Element {
      let script = "javascript:alert(1)";
      rsx! {
        BreadcrumbLink { href: script, "Breadcrumb" }
        DockItem { href: script.to_string(), "Dock" }
        HoverCard { HoverCardTrigger { href: script, "Hover card" } }
        Menu { MenuItem { href: script, "Menu" } }
        NavigationMenu { default_value: "web",
          NavigationMenuList {
            NavigationMenuItem { value: "web", NavigationMenuTrigger { "Web" } }
          }
          NavigationMenuContent { value: "web",
            NavigationMenuLink { href: script, "Navigation" }
            NavigationMenuLink { href: "/retired", disabled: true, "Retired" }
          }
        }
        PaginationPrevious { href: script }
        PaginationLink { href: script, "1" }
        PaginationNext { href: script }
        SidebarProvider {
          Sidebar { SidebarContent { SidebarItem { href: script, "Sidebar" } } }
        }
        a { href: "/kept", "Control" }
        BreadcrumbLink { href: "https://dioxuslabs.com", "Safe" }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(!html.contains("javascript:"), "{html}");
    for label in ["Breadcrumb", "Dock", "Hover card", "Menu", "Navigation", "Sidebar"] {
      assert!(html.contains(label), "{label} did not render: {html}");
    }
    assert!(html.contains(r#"href="https://dioxuslabs.com""#), "{html}");
    // A disabled Navigation Menu link keeps its text but cannot be followed.
    assert!(html.contains("Retired") && !html.contains("/retired"), "{html}");
  }
}
