use dioxus::prelude::*;
use dioxus_shadcn::{
  Label, Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator,
  SelectTrigger,
};

const FRUITS: [(&str, &str, bool); 4] =
  [("apple", "Apple", false), ("banana", "Banana", false), ("cherry", "Cherry", true), ("grape", "Grape", false)];

#[component]
pub fn SelectBasicDemo() -> Element {
  let mut value = use_signal(|| "banana".to_string());
  let label = FRUITS.iter().find(|(id, _, _)| *id == value()).map(|(_, label, _)| *label).unwrap_or("None");

  rsx! {
    Select { id: "select-basic-trigger", value: value(), on_value_change: move |next| value.set(next),
      div { class: "grid max-w-xs gap-2",
        Label { r#for: "select-basic-trigger", "Fruit" }
        SelectTrigger { span { class: "truncate", "{label}" } }
      }
      SelectContent {
        SelectGroup {
          SelectLabel { "Fruits" }
          for (id, label, disabled) in FRUITS {
            SelectItem { key: "{id}", value: id, disabled, "{label}" }
          }
        }
        SelectSeparator {}
        SelectItem { value: "none", "None" }
      }
    }
  }
}
