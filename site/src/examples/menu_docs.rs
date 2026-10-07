use dioxus::prelude::*;
use dioxus_shadcn::{Menu, MenuGroup, MenuItem, MenuTitle};

#[component]
pub fn MenuDocsDemo() -> Element {
  let mut page = use_signal(|| "start");

  rsx! {
    nav { class: "w-60 rounded-md border border-border p-2", "aria-label": "Documentation",
      Menu {
        MenuTitle { "Guides" }
        MenuItem { active: page() == "start", onclick: move |_| page.set("start"), "Getting started" }
        MenuItem { active: page() == "theming", onclick: move |_| page.set("theming"), "Theming" }
        MenuItem { disabled: true, "Migration (soon)" }
        MenuGroup {
          label: rsx! { "Components" },
          default_open: true,
          for (value, label) in [("button", "Button"), ("dialog", "Dialog"), ("menu", "Menu")] {
            MenuItem { key: "{value}", active: page() == value, onclick: move |_| page.set(value), "{label}" }
          }
        }
      }
    }
  }
}
