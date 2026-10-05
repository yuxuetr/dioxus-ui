use dioxus::prelude::*;
use dioxus_ui::{Field, FieldDescription, FieldError, FieldGroup, FieldLabel, Input};

#[component]
pub fn Demo() -> Element {
  let mut username = use_signal(|| "ab".to_string());
  let too_short = username().chars().count() < 3;

  rsx! {
    FieldGroup { class: "max-w-sm",
      Field {
        FieldLabel { "Email" }
        Input { r#type: "email", "aria-label": "Email", placeholder: "you@example.com" }
        FieldDescription { "We never share your email." }
      }
      Field { invalid: too_short,
        FieldLabel { "Username" }
        Input {
          "aria-label": "Username",
          invalid: too_short,
          value: username(),
          on_value_change: move |value| username.set(value),
        }
        if too_short {
          FieldError { "Use at least 3 characters." }
        }
      }
      Field { disabled: true,
        FieldLabel { "Workspace" }
        Input { "aria-label": "Workspace", disabled: true, value: "acme" }
      }
    }
  }
}
