use dioxus::prelude::*;
use dioxus_ui::Skeleton;

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "flex items-center gap-4", role: "status", "aria-busy": "true", "aria-label": "Loading profile",
      Skeleton { class: "h-12 w-12 rounded-full" }
      div { class: "grid gap-2",
        Skeleton { class: "h-4 w-48" }
        Skeleton { class: "h-4 w-32" }
      }
    }
  }
}
