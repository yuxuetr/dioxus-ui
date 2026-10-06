use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, Dialog, DialogClose, DialogContent, DialogDescription,
  DialogOverlay, DialogTitle, DialogTrigger, Input, Label, UiDensity, button_class,
};

#[component]
pub fn DialogFormDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut name = use_signal(|| "dioxus-shadcn".to_string());

  rsx! {
    Dialog { open: open(), on_open_change: move |next| open.set(next),
      DialogTrigger {
        class: button_class(ButtonVariant::Outline, ButtonSize::Md, UiDensity::Comfortable, ""),
        "Rename project"
      }
      p { class: "mt-3 text-sm text-muted-foreground", "Project: {name}" }
      DialogOverlay {}
      DialogContent {
        DialogTitle { "Rename project" }
        DialogDescription { "Focus stays inside the dialog until it closes." }
        div { class: "grid gap-2",
          Label { r#for: "dialog-form-name", "Name" }
          Input { id: "dialog-form-name", value: name(), on_value_change: move |value| name.set(value) }
        }
        div { class: "flex justify-end",
          Button { onclick: move |_| open.set(false), "Save" }
        }
        DialogClose { "×" }
      }
    }
  }
}
