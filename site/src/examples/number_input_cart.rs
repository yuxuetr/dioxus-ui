use dioxus::prelude::*;
use dioxus_shadcn::{Label, NumberInput};

#[component]
pub fn Demo() -> Element {
  let mut quantity = use_signal(|| 1.0);
  let mut weight = use_signal(|| 2.5);

  rsx! {
    div { class: "grid max-w-xs gap-4",
      div { class: "grid gap-2",
        Label { r#for: "number-input-quantity", "Quantity" }
        NumberInput {
          id: "number-input-quantity",
          min: 1.0,
          max: 10.0,
          value: quantity(),
          on_value_change: move |value| quantity.set(value),
        }
      }
      div { class: "grid gap-2",
        Label { r#for: "number-input-weight", "Weight (kg)" }
        NumberInput {
          id: "number-input-weight",
          min: 0.0,
          step: 0.1,
          value: weight(),
          on_value_change: move |value| weight.set(value),
        }
      }
      p { class: "text-sm text-muted-foreground", "{quantity} × {weight} kg" }
    }
  }
}
