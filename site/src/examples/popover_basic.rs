use dioxus::prelude::*;
use dioxus_shadcn::{
  ButtonSize, ButtonVariant, Input, Label, Popover, PopoverContent, PopoverDescription,
  PopoverHeader, PopoverTitle, PopoverTrigger, UiDensity, button_class,
};

#[component]
pub fn PopoverBasicDemo() -> Element {
  let mut width = use_signal(|| "100%".to_string());

  rsx! {
    Popover {
      PopoverTrigger {
        class: button_class(ButtonVariant::Outline, ButtonSize::Md, UiDensity::Comfortable, ""),
        "Dimensions"
      }
      PopoverContent {
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
}
