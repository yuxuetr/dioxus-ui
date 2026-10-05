use dioxus::prelude::*;
use dioxus_ui::{
  Button, ButtonVariant, DialogClose, DialogContent, DialogDescription, DialogOverlay, DialogTitle,
  Input, Label,
};

#[component]
pub fn Demo() -> Element {
  let mut open = use_signal(|| false);
  let mut name = use_signal(|| "dioxus-ui".to_string());

  rsx! {
    Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(true), "Rename project" }
    p { class: "mt-3 text-sm text-muted-foreground", "Project: {name}" }
    DialogOverlay { open: open(), on_open_change: move |next| open.set(next) }
    DialogContent { open: open(), on_open_change: move |next| open.set(next),
      DialogTitle { "Rename project" }
      DialogDescription { "Focus stays inside the dialog until it closes." }
      div { class: "grid gap-2",
        Label { r#for: "dialog-form-name", "Name" }
        Input { id: "dialog-form-name", value: name(), on_value_change: move |value| name.set(value) }
      }
      div { class: "flex justify-end",
        Button { onclick: move |_| open.set(false), "Save" }
      }
      DialogClose { on_open_change: move |next| open.set(next), "×" }
    }
  }
}
