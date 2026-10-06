use dioxus::prelude::*;
use dioxus_shadcn::{Label, Textarea};

#[component]
pub fn TextareaBasicDemo() -> Element {
  let mut message = use_signal(String::new);
  let count = message().chars().count();

  rsx! {
    div { class: "grid max-w-sm gap-2",
      Label { r#for: "textarea-basic-message", "Message" }
      Textarea {
        id: "textarea-basic-message",
        placeholder: "Tell us what you think",
        invalid: count > 140,
        value: message(),
        on_value_change: move |value| message.set(value),
      }
      p { class: "text-sm text-muted-foreground", "{count} / 140" }
      Label { r#for: "textarea-basic-disabled", "Disabled" }
      Textarea { id: "textarea-basic-disabled", disabled: true, placeholder: "Read only" }
    }
  }
}
