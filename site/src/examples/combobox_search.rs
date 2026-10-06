use dioxus::prelude::*;
use dioxus_shadcn::{
  Combobox, ComboboxContent, ComboboxEmpty, ComboboxInput, ComboboxItem, ComboboxList, Label,
};

const FRAMEWORKS: [(&str, &str); 5] =
  [("dioxus", "Dioxus"), ("leptos", "Leptos"), ("yew", "Yew"), ("sycamore", "Sycamore"), ("iced", "Iced")];

#[component]
pub fn ComboboxSearchDemo() -> Element {
  let mut query = use_signal(String::new);
  let matches = move || {
    FRAMEWORKS
      .iter()
      .filter(|(_, label)| label.to_lowercase().contains(&query().to_lowercase()))
      .copied()
      .collect::<Vec<_>>()
  };

  rsx! {
    Combobox {
      id: "combobox-search-input",
      // Show the chosen framework's label in the input.
      on_value_change: move |next: String| {
        if let Some((_, label)) = FRAMEWORKS.iter().find(|(id, _)| *id == next) {
          query.set(label.to_string());
        }
      },
      div { class: "grid max-w-xs gap-2",
        Label { r#for: "combobox-search-input", "Framework" }
        ComboboxInput {
          class: "rounded-md border border-input",
          value: query(),
          placeholder: "Search frameworks",
          oninput: move |event: FormEvent| query.set(event.value()),
        }
      }
      ComboboxContent {
        ComboboxList {
          if matches().is_empty() {
            ComboboxEmpty { "No framework found." }
          }
          for (id, label) in matches() {
            ComboboxItem { key: "{id}", value: id, "{label}" }
          }
        }
      }
    }
  }
}
