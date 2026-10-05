use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn Demo() -> Element {
  let mut clicks = use_signal(|| 0);

  rsx! {
    div { class: "flex flex-wrap items-center gap-2",
      Button { size: ButtonSize::Sm, "Small" }
      Button { size: ButtonSize::Md, "Medium" }
      Button { size: ButtonSize::Lg, "Large" }
      Button {
        size: ButtonSize::Icon,
        variant: ButtonVariant::Outline,
        "aria-label": "Add item",
        onclick: move |_| clicks += 1,
        "+"
      }
      Button { disabled: true, "Disabled" }
    }
    p { class: "mt-3 text-sm text-muted-foreground", "Add item pressed {clicks} times." }
  }
}
