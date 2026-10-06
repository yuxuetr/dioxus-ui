use dioxus::prelude::*;
use dioxus_shadcn::{HoverCard, HoverCardContent, HoverCardDescription, HoverCardHeader, HoverCardTitle, HoverCardTrigger};

#[component]
pub fn HoverCardProfileDemo() -> Element {
  rsx! {
    HoverCard {
      HoverCardTrigger {
        href: "https://dioxuslabs.com",
        class: "text-sm font-medium underline underline-offset-4",
        "@dioxus"
      }
      HoverCardContent {
        HoverCardHeader {
          HoverCardTitle { "Dioxus" }
          HoverCardDescription { "Fullstack app framework for Rust." }
        }
        p { class: "mt-2 text-xs text-muted-foreground", "Opens on hover or keyboard focus." }
      }
    }
  }
}
