use dioxus::prelude::*;
use dioxus_ui::{Button, ButtonVariant};

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "flex flex-wrap items-center gap-2",
      Button { "Primary" }
      Button { variant: ButtonVariant::Secondary, "Secondary" }
      Button { variant: ButtonVariant::Outline, "Outline" }
      Button { variant: ButtonVariant::Ghost, "Ghost" }
      Button { variant: ButtonVariant::Destructive, "Destructive" }
      Button { variant: ButtonVariant::Link, "Link" }
    }
  }
}
