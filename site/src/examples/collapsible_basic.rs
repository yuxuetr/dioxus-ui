use dioxus::prelude::*;
use dioxus_shadcn::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[component]
pub fn Demo() -> Element {
  let mut open = use_signal(|| false);

  rsx! {
    Collapsible { class: "max-w-sm", open: open(),
      div { class: "flex items-center justify-between",
        span { class: "text-sm font-semibold", "@ada starred 3 repositories" }
        CollapsibleTrigger {
          open: open(),
          controls: "collapsible-basic-content",
          on_open_change: move |next| open.set(next),
          if open() { "Hide" } else { "Show" }
        }
      }
      div { class: "rounded-md border border-border px-4 py-2 font-mono text-sm", "dioxus-shadcn" }
      CollapsibleContent { open: open(), id: "collapsible-basic-content",
        div { class: "grid gap-2",
          div { class: "rounded-md border border-border px-4 py-2 font-mono text-sm", "dioxus" }
          div { class: "rounded-md border border-border px-4 py-2 font-mono text-sm", "tailwindcss" }
        }
      }
    }
  }
}
