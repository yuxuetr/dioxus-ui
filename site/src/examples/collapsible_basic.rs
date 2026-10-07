use dioxus::prelude::*;
use dioxus_shadcn::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[component]
pub fn CollapsibleBasicDemo() -> Element {
  let mut open = use_signal(|| false);

  rsx! {
    // The app reads `open` for the trigger label, so it controls the root.
    Collapsible { class: "max-w-sm", open: open(), on_open_change: move |next| open.set(next),
      div { class: "flex items-center justify-between",
        span { class: "text-sm font-semibold", "@ada starred 3 repositories" }
        CollapsibleTrigger {
          if open() { "Hide" } else { "Show" }
        }
      }
      div { class: "rounded-md border border-border px-4 py-2 font-mono text-sm", "dioxus-shadcn" }
      CollapsibleContent {
        div { class: "grid gap-2",
          div { class: "rounded-md border border-border px-4 py-2 font-mono text-sm", "dioxus" }
          div { class: "rounded-md border border-border px-4 py-2 font-mono text-sm", "tailwindcss" }
        }
      }
    }
  }
}
