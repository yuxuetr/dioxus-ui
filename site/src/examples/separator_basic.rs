use dioxus::prelude::*;
use dioxus_shadcn::{Separator, SeparatorOrientation};

#[component]
pub fn SeparatorBasicDemo() -> Element {
  rsx! {
    div { class: "max-w-sm",
      h4 { class: "text-sm font-medium", "dioxus-shadcn" }
      p { class: "text-sm text-muted-foreground", "Components for Dioxus." }
      Separator { class: "my-4" }
      div { class: "flex h-5 items-center gap-4 text-sm",
        span { "Blog" }
        Separator { orientation: SeparatorOrientation::Vertical }
        span { "Docs" }
        Separator { orientation: SeparatorOrientation::Vertical }
        span { "Source" }
      }
    }
  }
}
