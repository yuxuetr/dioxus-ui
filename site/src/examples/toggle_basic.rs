use dioxus::prelude::*;
use dioxus_ui::{Toggle, ToggleVariant};

#[component]
pub fn Demo() -> Element {
  let mut bold = use_signal(|| false);
  let mut italic = use_signal(|| true);

  rsx! {
    div { class: "flex flex-wrap items-center gap-2",
      Toggle {
        "aria-label": "Bold",
        pressed: bold(),
        on_pressed_change: move |pressed| bold.set(pressed),
        "Bold"
      }
      Toggle {
        variant: ToggleVariant::Outline,
        "aria-label": "Italic",
        pressed: italic(),
        on_pressed_change: move |pressed| italic.set(pressed),
        "Italic"
      }
      Toggle { "aria-label": "Underline", disabled: true, "Underline" }
    }
  }
}
