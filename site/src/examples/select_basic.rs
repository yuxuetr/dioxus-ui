use dioxus::prelude::*;
use dioxus_shadcn::{
  Label, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator, SelectTrigger,
  SelectValue,
};

const FRUITS: [(&str, &str, bool); 4] =
  [("apple", "Apple", false), ("banana", "Banana", false), ("cherry", "Cherry", true), ("grape", "Grape", false)];

#[component]
pub fn Demo() -> Element {
  let mut open = use_signal(|| false);
  let mut value = use_signal(|| "banana".to_string());
  let label = FRUITS.iter().find(|(id, _, _)| *id == value()).map(|(_, label, _)| *label).unwrap_or("Pick a fruit");

  rsx! {
    div { class: "grid max-w-xs gap-2",
      Label { r#for: "select-basic-trigger", "Fruit" }
      SelectTrigger {
        id: "select-basic-trigger",
        open: open(),
        on_open_change: move |next| open.set(next),
        SelectValue { "{label}" }
      }
    }
    SelectContent {
      open: open(),
      anchor_id: "select-basic-trigger",
      on_open_change: move |next| open.set(next),
      on_value_change: move |next| value.set(next),
      SelectGroup {
        SelectLabel { "Fruits" }
        for (id, label, disabled) in FRUITS {
          SelectItem { key: "{id}", value: id, selected: value() == id, disabled, "{label}" }
        }
      }
      SelectSeparator {}
      SelectItem { value: "none", selected: value() == "none", "None" }
    }
  }
}
