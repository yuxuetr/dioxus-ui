use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonSize, ButtonVariant, RadialProgress, RadialProgressSize};

#[component]
pub fn Demo() -> Element {
  let mut value = use_signal(|| 40.0_f64);

  rsx! {
    div { class: "flex flex-wrap items-center gap-6",
      RadialProgress { value: value(), size: RadialProgressSize::Sm, "aria-label": "Small progress" }
      RadialProgress { value: value(), "aria-label": "Upload progress" }
      RadialProgress {
        value: value(),
        size: RadialProgressSize::Lg,
        class: "[&_circle:last-of-type]:stroke-success",
        "aria-label": "Tasks done",
        span { class: "text-center leading-tight",
          "{(value() / 20.0).round()}/5"
          br {}
          span { class: "text-xs text-muted-foreground", "tasks" }
        }
      }
      div { class: "flex gap-2",
        Button {
          variant: ButtonVariant::Outline,
          size: ButtonSize::Sm,
          onclick: move |_| value.set((value() - 20.0).max(0.0)),
          "-20"
        }
        Button {
          variant: ButtonVariant::Outline,
          size: ButtonSize::Sm,
          onclick: move |_| value.set((value() + 20.0).min(100.0)),
          "+20"
        }
      }
    }
  }
}
