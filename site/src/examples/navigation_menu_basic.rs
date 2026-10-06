use dioxus::prelude::*;
use dioxus_shadcn::{
  NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink, NavigationMenuList,
  NavigationMenuTrigger,
};

#[component]
pub fn NavigationMenuBasicDemo() -> Element {
  rsx! {
    NavigationMenu { "aria-label": "Product",
      NavigationMenuList {
        NavigationMenuItem { value: "docs",
          NavigationMenuTrigger { "Docs" }
          NavigationMenuContent {
            NavigationMenuLink { href: "#install", "Installation" }
            NavigationMenuLink { href: "#theming", "Theming" }
            NavigationMenuLink { href: "#cli", disabled: true, "CLI (soon)" }
          }
        }
        NavigationMenuItem { value: "examples",
          NavigationMenuTrigger { "Examples" }
          NavigationMenuContent {
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
