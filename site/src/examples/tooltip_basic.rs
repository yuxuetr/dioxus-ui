use dioxus::prelude::*;
use dioxus_ui::{Tooltip, TooltipContent, TooltipTrigger};

#[component]
pub fn Demo() -> Element {
  let mut open = use_signal(|| false);

  rsx! {
    Tooltip { on_open_change: move |next| open.set(next),
      TooltipTrigger {
        class: "rounded-md border border-input px-3 py-2 text-sm hover:bg-accent",
        "Hover or focus me"
      }
      TooltipContent { open: open(), "Saved 2 minutes ago" }
    }
  }
}
