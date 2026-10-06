use dioxus::prelude::*;
use dioxus_shadcn::{Menu, MenuGroup, MenuItem, MenuTitle};

#[component]
pub fn Demo() -> Element {
  let mut page = use_signal(|| "start");
  let mut components_open = use_signal(|| true);

  rsx! {
    nav { class: "w-60 rounded-md border border-border p-2", "aria-label": "Documentation",
      Menu {
        MenuTitle { "Guides" }
        MenuItem { active: page() == "start", onclick: move |_| page.set("start"), "Getting started" }
        MenuItem { active: page() == "theming", onclick: move |_| page.set("theming"), "Theming" }
        MenuItem { disabled: true, "Migration (soon)" }
        MenuGroup {
          label: rsx! { "Components" },
          open: components_open(),
          on_open_change: move |open| components_open.set(open),
          for (value, label) in [("button", "Button"), ("dialog", "Dialog"), ("menu", "Menu")] {
            MenuItem { key: "{value}", active: page() == value, onclick: move |_| page.set(value), "{label}" }
          }
        }
      }
    }
  }
}
