use dioxus::prelude::*;
use dioxus_ui::{ButtonGroup, ButtonGroupItem, ButtonGroupOrientation};

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "flex flex-wrap items-start gap-6",
      ButtonGroup { aria_label: "Text alignment",
        ButtonGroupItem { "Left" }
        ButtonGroupItem { "Center" }
        ButtonGroupItem { "Right" }
      }
      ButtonGroup { aria_label: "History", orientation: ButtonGroupOrientation::Vertical,
        ButtonGroupItem { "Undo" }
        ButtonGroupItem { "Redo" }
        ButtonGroupItem { disabled: true, "Clear" }
      }
    }
  }
}
