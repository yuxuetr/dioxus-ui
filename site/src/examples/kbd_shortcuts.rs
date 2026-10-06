use dioxus::prelude::*;
use dioxus_shadcn::{Kbd, KbdSize};

#[component]
pub fn KbdShortcutsDemo() -> Element {
  rsx! {
    div { class: "grid gap-3 text-sm",
      p { "Open the command menu with " Kbd { "Ctrl" } " + " Kbd { "K" } "." }
      div { class: "flex items-center gap-2",
        Kbd { size: KbdSize::Sm, "Esc" }
        Kbd { size: KbdSize::Md, "Enter" }
        Kbd { size: KbdSize::Lg, "Space" }
      }
    }
  }
}
