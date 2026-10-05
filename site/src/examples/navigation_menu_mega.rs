use dioxus::prelude::*;
use dioxus_shadcn::{
  NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink,
  NavigationMenuList, NavigationMenuOrientation, NavigationMenuTrigger,
};

struct Area {
  value: &'static str,
  label: &'static str,
  links: [(&'static str, &'static str); 3],
}

const AREAS: [Area; 3] = [
  Area {
    value: "web",
    label: "Web",
    links: [("Dashboards", "Admin panels and analytics"), ("Stores", "Catalogs and checkout"), ("Docs", "Sites with search")],
  },
  Area {
    value: "mobile",
    label: "Mobile",
    links: [("iOS", "Native WKWebView apps"), ("Android", "WebView apps"), ("Offline", "Local-first data")],
  },
  Area {
    value: "desktop",
    label: "Desktop",
    links: [("macOS", "Menu bar and windows"), ("Windows", "WebView2 apps"), ("Linux", "WebKitGTK apps")],
  },
];

#[component]
pub fn Demo() -> Element {
  let mut active = use_signal(String::new);
  let mut area = use_signal(|| "web".to_string());

  rsx! {
    div { class: "min-h-72",
      NavigationMenu {
        "aria-label": "Solutions",
        on_value_change: move |value: String| {
          if !value.is_empty() {
            area.set("web".to_string());
          }
          active.set(value);
        },
        NavigationMenuList {
          NavigationMenuItem { value: "solutions",
            NavigationMenuTrigger { open: active() == "solutions", "Solutions" }
            NavigationMenuContent { open: active() == "solutions",
              NavigationMenu {
                orientation: NavigationMenuOrientation::Vertical,
                "aria-label": "Solution areas",
                on_value_change: move |value: String| {
                  if !value.is_empty() {
                    area.set(value);
                  }
                },
                NavigationMenuList { class: "w-32",
                  for Area { value, label, .. } in AREAS {
                    NavigationMenuItem { key: "{value}", value,
                      NavigationMenuTrigger { open: area() == value, "{label}" }
                    }
                  }
                }
                for Area { value, links, .. } in AREAS {
                  NavigationMenuContent { key: "{value}", value, open: area() == value,
                    div { class: "grid w-64 gap-1",
                      for (title, description) in links {
                        NavigationMenuLink { key: "{title}", href: "#",
                          p { class: "font-medium", "{title}" }
                          p { class: "mt-1 text-muted-foreground", "{description}" }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
          NavigationMenuItem { NavigationMenuLink { href: "#", "Pricing" } }
        }
      }
    }
  }
}
