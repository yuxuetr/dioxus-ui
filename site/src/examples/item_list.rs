use dioxus::prelude::*;
use dioxus_ui::{Badge, BadgeVariant, Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemTitle};

const PEOPLE: [(&str, &str, &str); 3] = [
  ("AL", "Ada Lovelace", "ada@example.com"),
  ("GH", "Grace Hopper", "grace@example.com"),
  ("AT", "Alan Turing", "alan@example.com"),
];

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "grid max-w-md gap-1",
      for (initials, name, email) in PEOPLE {
        div { key: "{name}",
          Item { selected: name == "Grace Hopper",
            ItemMedia {
              span { class: "flex h-9 w-9 items-center justify-center rounded-full bg-muted text-xs font-medium", "{initials}" }
            }
            ItemContent {
              ItemTitle { "{name}" }
              ItemDescription { "{email}" }
            }
            ItemActions {
              if name == "Grace Hopper" {
                Badge { variant: BadgeVariant::Secondary, "Selected" }
              }
            }
          }
        }
      }
    }
  }
}
