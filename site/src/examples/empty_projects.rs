use dioxus::prelude::*;
use dioxus_ui::{Button, Empty, EmptyActions, EmptyDescription, EmptyHeader, EmptyTitle};

#[component]
pub fn Demo() -> Element {
  rsx! {
    Empty {
      EmptyHeader {
        EmptyTitle { "No projects yet" }
        EmptyDescription { "Create your first project to get started." }
      }
      EmptyActions { Button { "Create project" } }
    }
  }
}
