use dioxus::prelude::*;
use dioxus_shadcn::{Label, RadioGroup, RadioGroupItem};

const PLANS: [(&str, &str, bool); 3] =
  [("free", "Free", false), ("pro", "Pro", false), ("team", "Team (sold out)", true)];

#[component]
pub fn Demo() -> Element {
  let mut plan = use_signal(|| Some("pro".to_string()));

  rsx! {
    RadioGroup {
      class: "grid gap-3",
      "aria-label": "Plan",
      value: plan(),
      on_value_change: move |value: String| plan.set(Some(value)),
      for (value, label, disabled) in PLANS {
        div { key: "{value}", class: "flex items-center gap-2",
          RadioGroupItem {
            id: "radio-group-basic-{value}",
            value,
            checked: plan().as_deref() == Some(value),
            disabled,
          }
          Label { r#for: "radio-group-basic-{value}", "{label}" }
        }
      }
    }
  }
}
