use dioxus::prelude::*;
use dioxus_shadcn::{ScrollArea, ScrollAreaContent, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaViewport, Separator};

#[component]
pub fn ScrollAreaTagsDemo() -> Element {
  rsx! {
    ScrollArea { class: "h-56 w-48 rounded-md border border-border",
      ScrollAreaViewport {
        ScrollAreaContent { class: "p-4",
          h4 { class: "mb-4 text-sm font-medium", "Tags" }
          for version in (1..=30).rev() {
            div { key: "{version}",
              div { class: "text-sm", "v0.{version}.0" }
              Separator { class: "my-2" }
            }
          }
        }
      }
      ScrollAreaScrollbar { ScrollAreaThumb {} }
    }
  }
}
