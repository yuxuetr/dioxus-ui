use dioxus::prelude::*;
use dioxus_shadcn::{Label, RadioGroup, RadioGroupItem};

const PLANS: [(&str, &str, bool); 3] =
  [("free", "Free", false), ("pro", "Pro", false), ("team", "Team (sold out)", true)];

#[component]
pub fn RadioGroupBasicDemo() -> Element {
  rsx! {
    RadioGroup {
      class: "grid gap-3",
      "aria-label": "Plan",
      default_value: "pro",
      for (value, label, disabled) in PLANS {
        div { key: "{value}", class: "flex items-center gap-2",
          RadioGroupItem {
            id: "radio-group-basic-{value}",
            value,
            disabled,
          }
          Label { r#for: "radio-group-basic-{value}", "{label}" }
        }
      }
    }
  }
}
