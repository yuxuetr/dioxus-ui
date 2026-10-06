use dioxus::prelude::*;
use dioxus_shadcn::{Label, RangeSlider};

#[component]
pub fn Demo() -> Element {
  let mut price = use_signal(|| (20.0, 80.0));
  let (low, high) = price();

  rsx! {
    div { class: "grid max-w-sm gap-4",
      div { class: "flex items-center justify-between",
        Label { id: "price-range-label", "Price" }
        span { class: "text-sm text-muted-foreground", "${low} – ${high}" }
      }
      RangeSlider {
        "aria-labelledby": "price-range-label",
        value: price(),
        max: 200.0,
        step: 5.0,
        min_steps_between: 2,
        start_label: "Minimum price",
        end_label: "Maximum price",
        on_value_change: move |next| price.set(next),
      }
    }
  }
}
