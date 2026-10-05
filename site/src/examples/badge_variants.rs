use dioxus::prelude::*;
use dioxus_ui::{Badge, BadgeVariant};

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "flex flex-wrap items-center gap-2",
      Badge { "Default" }
      Badge { variant: BadgeVariant::Secondary, "Secondary" }
      Badge { variant: BadgeVariant::Destructive, "Destructive" }
      Badge { variant: BadgeVariant::Outline, "Outline" }
    }
  }
}
