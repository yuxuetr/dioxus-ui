use dioxus::prelude::*;
use dioxus_ui::{Field, FieldDescription, FieldError, FieldGroup, FieldLabel, Input};

#[component]
pub fn Demo() -> Element {
  let mut username = use_signal(|| "ab".to_string());
  let too_short = username().chars().count() < 3;

  rsx! {
    FieldGroup { class: "max-w-sm",
      Field {
        FieldLabel { r#for: "field-basic-email", "Email" }
        Input { id: "field-basic-email", r#type: "email", placeholder: "you@example.com" }
        FieldDescription { "We never share your email." }
      }
      Field { invalid: too_short,
        FieldLabel { r#for: "field-basic-username", "Username" }
        Input {
          id: "field-basic-username",
          invalid: too_short,
          value: username(),
          on_value_change: move |value| username.set(value),
        }
        if too_short {
          FieldError { "Use at least 3 characters." }
        }
      }
      Field { disabled: true,
        FieldLabel { r#for: "field-basic-workspace", "Workspace" }
        Input { id: "field-basic-workspace", disabled: true, value: "acme" }
      }
    }
  }
}
