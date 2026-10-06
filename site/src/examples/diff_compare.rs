use dioxus::prelude::*;
use dioxus_shadcn::{Diff, DiffAfter, DiffBefore};

#[component]
pub fn DiffCompareDemo() -> Element {
  let mut position = use_signal(|| 50.0);

  rsx! {
    div { class: "grid max-w-lg gap-2",
      Diff {
        class: "aspect-video border border-border",
        position: position(),
        label: "Compare the wireframe and the final design",
        on_position_change: move |value| position.set(value),
        DiffBefore { class: "grid place-items-center bg-muted",
          div { class: "grid w-3/4 gap-2",
            div { class: "h-4 w-1/2 rounded-sm border-2 border-dashed border-muted-foreground" }
            div { class: "h-16 rounded-sm border-2 border-dashed border-muted-foreground" }
            p { class: "text-sm text-muted-foreground", "Wireframe" }
          }
        }
        DiffAfter { class: "grid place-items-center bg-primary text-primary-foreground",
          div { class: "grid w-3/4 gap-2",
            div { class: "h-4 w-1/2 rounded-sm bg-primary-foreground" }
            div { class: "h-16 rounded-sm bg-primary-foreground/20" }
            p { class: "text-sm", "Final design" }
          }
        }
      }
      p { class: "text-sm text-muted-foreground", "Divider at {position}%" }
    }
  }
}
