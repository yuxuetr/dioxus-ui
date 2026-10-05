use dioxus::prelude::*;
use dioxus_shadcn::{Badge, BadgeVariant};

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "flex flex-wrap items-center gap-2",
      Badge { variant: BadgeVariant::Success, "Paid" }
      Badge { variant: BadgeVariant::Warning, "Pending" }
      Badge { variant: BadgeVariant::Info, "New" }
    }
  }
}
