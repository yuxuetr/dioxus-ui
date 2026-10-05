use dioxus::prelude::*;
use dioxus_shadcn::{Status, StatusVariant};

#[component]
pub fn Demo() -> Element {
  rsx! {
    ul { class: "grid gap-3 text-sm",
      for (variant, text) in [
        (StatusVariant::Success, "All systems operational"),
        (StatusVariant::Warning, "Degraded performance"),
        (StatusVariant::Destructive, "Major outage"),
        (StatusVariant::Info, "Scheduled maintenance"),
        (StatusVariant::Neutral, "Not monitored"),
      ] {
        // The text says the state, so the dot stays hidden.
        li { key: "{text}", class: "flex items-center gap-2", Status { variant }, "{text}" }
      }
    }
  }
}
