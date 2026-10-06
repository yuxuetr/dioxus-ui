use dioxus::prelude::*;
use dioxus_shadcn::{Input, Label};

#[component]
pub fn InputStatesDemo() -> Element {
  let mut email = use_signal(String::new);

  rsx! {
    div { class: "grid max-w-sm gap-4",
      div { class: "grid gap-2",
        Label { r#for: "input-states-email", "Email" }
        Input {
          id: "input-states-email",
          r#type: "email",
          placeholder: "you@example.com",
          value: email(),
          on_value_change: move |value| email.set(value),
        }
      }
      div { class: "grid gap-2",
        Label { r#for: "input-states-invalid", "Invalid" }
        Input { id: "input-states-invalid", invalid: true, value: "not an email" }
      }
      div { class: "grid gap-2",
        Label { r#for: "input-states-disabled", "Disabled" }
        Input { id: "input-states-disabled", disabled: true, placeholder: "Unavailable" }
      }
    }
  }
}
