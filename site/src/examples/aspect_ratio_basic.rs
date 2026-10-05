use dioxus::prelude::*;
use dioxus_ui::AspectRatio;

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "grid max-w-md gap-4 sm:grid-cols-2",
      AspectRatio { ratio: 16.0 / 9.0, class: "rounded-md bg-muted",
        div { class: "flex h-full items-center justify-center text-sm text-muted-foreground", "16:9" }
      }
      AspectRatio { ratio: 1.0, class: "rounded-md bg-muted",
        div { class: "flex h-full items-center justify-center text-sm text-muted-foreground", "1:1" }
      }
    }
  }
}
