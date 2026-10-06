use dioxus::prelude::*;
use dioxus_shadcn::{Tooltip, TooltipContent, TooltipTrigger};

#[component]
pub fn TooltipBasicDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut copies = use_signal(|| 0);

  rsx! {
    Tooltip { on_open_change: move |next| open.set(next),
      TooltipTrigger {
        class: "rounded-md border border-input px-3 py-2 text-sm hover:bg-accent",
        onclick: move |_| copies += 1,
        "Copy link"
      }
      TooltipContent { open: open(), "Copies the page address" }
    }
    p { class: "mt-3 text-sm text-muted-foreground", "Copied {copies} times" }
  }
}
