use dioxus::prelude::*;
use dioxus_shadcn::{Dock, DockItem, DockLabel};

#[component]
fn Icon(path: &'static str) -> Element {
  rsx! {
    svg {
      view_box: "0 0 24 24",
      fill: "none",
      stroke: "currentColor",
      stroke_width: "2",
      stroke_linecap: "round",
      stroke_linejoin: "round",
      "aria-hidden": "true",
      path { d: path }
    }
  }
}

const TABS: [(&str, &str); 4] = [
  ("Home", "M3 11l9-8 9 8M5 10v10h14V10"),
  ("Search", "M11 18a7 7 0 1 1 0-14 7 7 0 0 1 0 14zM21 21l-5-5"),
  ("Inbox", "M4 4h16v16H4zM4 13h5l1 3h4l1-3h5"),
  ("Profile", "M12 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8zM4 21a8 8 0 0 1 16 0"),
];

#[component]
pub fn DockPhoneDemo() -> Element {
  let mut current = use_signal(|| "Home");

  rsx! {
    // A phone-sized frame; in an app the Dock is fixed to the viewport.
    div { class: "mx-auto flex h-72 w-72 flex-col overflow-hidden rounded-xl border border-border",
      div { class: "grid flex-1 place-items-center text-sm text-muted-foreground", "{current} screen" }
      Dock { fixed: false, "aria-label": "Main",
        for (label, path) in TABS {
          DockItem {
            key: "{label}",
            active: current() == label,
            onclick: move |_| current.set(label),
            Icon { path }
            DockLabel { "{label}" }
          }
        }
      }
    }
  }
}
