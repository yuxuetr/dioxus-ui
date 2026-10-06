use dioxus::prelude::*;
use dioxus_shadcn::{Direction, TextDirection, Toggle};

#[component]
pub fn DirectionRtlDemo() -> Element {
  let mut rtl = use_signal(|| true);
  let dir = if rtl() { TextDirection::Rtl } else { TextDirection::Ltr };

  rsx! {
    Toggle { pressed: rtl(), on_pressed_change: move |pressed| rtl.set(pressed), "Right to left" }
    Direction { class: "mt-4 block", dir,
      div { class: "flex gap-2 rounded-md border border-border p-4 text-sm",
        span { class: "font-semibold", "1." }
        span { "Arrow keys in roving groups follow the reading direction." }
      }
    }
  }
}
