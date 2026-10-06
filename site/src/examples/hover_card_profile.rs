use dioxus::prelude::*;
use dioxus_shadcn::{HoverCard, HoverCardContent, HoverCardDescription, HoverCardHeader, HoverCardTitle, HoverCardTrigger};

#[component]
pub fn HoverCardProfileDemo() -> Element {
  let mut open = use_signal(|| false);

  rsx! {
    HoverCard { on_open_change: move |next| open.set(next),
      HoverCardTrigger {
        href: "https://dioxuslabs.com",
        class: "text-sm font-medium underline underline-offset-4",
        "@dioxus"
      }
      HoverCardContent { open: open(),
        HoverCardHeader {
          HoverCardTitle { "Dioxus" }
          HoverCardDescription { "Fullstack app framework for Rust." }
        }
        p { class: "mt-2 text-xs text-muted-foreground", "Opens on hover or keyboard focus." }
      }
    }
  }
}
