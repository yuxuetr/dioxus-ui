use dioxus::prelude::*;
use dioxus_ui::{ToggleGroup, ToggleGroupItem, toggle_group_single_selection};

#[component]
pub fn Demo() -> Element {
  let mut alignment = use_signal(|| Some("left".to_string()));
  let pressed = move |value: &str| alignment().as_deref() == Some(value);

  rsx! {
    ToggleGroup {
      "aria-label": "Text alignment",
      on_toggle: move |value: String| {
        alignment.set(toggle_group_single_selection(alignment().as_deref(), &value))
      },
      ToggleGroupItem { value: "left", pressed: pressed("left"), "Left" }
      ToggleGroupItem { value: "center", pressed: pressed("center"), "Center" }
      ToggleGroupItem { value: "right", pressed: pressed("right"), "Right" }
      ToggleGroupItem { value: "justify", pressed: pressed("justify"), disabled: true, "Justify" }
    }
    p { class: "mt-3 text-sm text-muted-foreground",
      "Alignment: {alignment().unwrap_or_else(|| \"none\".to_string())}"
    }
  }
}
