use dioxus::prelude::*;
use dioxus_shadcn::{Button, Empty, EmptyActions, EmptyDescription, EmptyHeader, EmptyTitle};

#[component]
pub fn EmptyProjectsDemo() -> Element {
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
