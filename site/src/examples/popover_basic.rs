use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonVariant, Input, Label, PopoverContent, PopoverDescription, PopoverHeader, PopoverTitle};

#[component]
pub fn PopoverBasicDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut width = use_signal(|| "100%".to_string());

  rsx! {
    Button {
      id: "popover-basic-trigger",
      variant: ButtonVariant::Outline,
      "aria-expanded": "{open}",
      onclick: move |_| open.toggle(),
      "Dimensions"
    }
    PopoverContent {
      open: open(),
      anchor_id: "popover-basic-trigger",
      on_open_change: move |next| open.set(next),
      PopoverHeader {
        PopoverTitle { "Dimensions" }
        PopoverDescription { "Set the dimensions for the layer." }
      }
      div { class: "mt-3 grid gap-2",
        Label { r#for: "popover-basic-width", "Width" }
        Input { id: "popover-basic-width", value: width(), on_value_change: move |value| width.set(value) }
      }
    }
  }
}
