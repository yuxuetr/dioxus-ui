use dioxus::prelude::*;
use dioxus_ui::{
  NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink, NavigationMenuList,
  NavigationMenuTrigger,
};

#[component]
pub fn Demo() -> Element {
  let mut active = use_signal(String::new);
  let open = move |value: &str| active() == value;

  rsx! {
    NavigationMenu {
      "aria-label": "Product",
      on_value_change: move |value: String| active.set(value),
      NavigationMenuList {
        NavigationMenuItem { value: "docs",
          NavigationMenuTrigger { open: open("docs"), "Docs" }
          NavigationMenuContent { open: open("docs"),
            NavigationMenuLink { href: "#install", "Installation" }
            NavigationMenuLink { href: "#theming", "Theming" }
            NavigationMenuLink { href: "#cli", disabled: true, "CLI (soon)" }
          }
        }
        NavigationMenuItem { value: "examples",
          NavigationMenuTrigger { open: open("examples"), "Examples" }
          NavigationMenuContent { open: open("examples"),
            NavigationMenuLink { href: "#dashboard", "Dashboard" }
            NavigationMenuLink { href: "#chat", "Chat" }
          }
        }
        NavigationMenuItem {
          NavigationMenuLink { href: "#blog", "Blog" }
        }
      }
    }
  }
}
