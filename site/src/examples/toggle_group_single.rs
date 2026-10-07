use dioxus::prelude::*;
use dioxus_shadcn::{ToggleGroup, ToggleGroupItem};

#[component]
pub fn ToggleGroupSingleDemo() -> Element {
  let mut alignment = use_signal(|| "left".to_string());

  rsx! {
    // The group owns the pressed item; the app only shows it.
    ToggleGroup {
      "aria-label": "Text alignment",
      default_value: "left",
      on_value_change: move |value| alignment.set(value),
      ToggleGroupItem { value: "left", "Left" }
      ToggleGroupItem { value: "center", "Center" }
      ToggleGroupItem { value: "right", "Right" }
      ToggleGroupItem { value: "justify", disabled: true, "Justify" }
    }
    p { class: "mt-3 text-sm text-muted-foreground",
      if alignment().is_empty() { "Alignment: none" } else { "Alignment: {alignment}" }
    }
  }
}
