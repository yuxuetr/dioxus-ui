use dioxus::prelude::*;
use dioxus_ui::{ComboboxContent, ComboboxEmpty, ComboboxInput, ComboboxItem, ComboboxList, Label};

const FRAMEWORKS: [(&str, &str); 5] =
  [("dioxus", "Dioxus"), ("leptos", "Leptos"), ("yew", "Yew"), ("sycamore", "Sycamore"), ("iced", "Iced")];

#[component]
pub fn Demo() -> Element {
  let mut query = use_signal(String::new);
  let mut open = use_signal(|| false);
  let mut value = use_signal(String::new);
  let matches = move || {
    FRAMEWORKS
      .iter()
      .filter(|(_, label)| label.to_lowercase().contains(&query().to_lowercase()))
      .copied()
      .collect::<Vec<_>>()
  };

  rsx! {
    div { class: "grid max-w-xs gap-2",
      Label { r#for: "combobox-search-input", "Framework" }
      ComboboxInput {
        id: "combobox-search-input",
        class: "rounded-md border border-input",
        value: query(),
        open: open(),
        placeholder: "Search frameworks",
        oninput: move |event: FormEvent| {
          query.set(event.value());
          open.set(true);
        },
        on_open_change: move |next| open.set(next),
      }
    }
    ComboboxContent {
      open: open(),
      anchor_id: "combobox-search-input",
      on_open_change: move |next| open.set(next),
      on_value_change: move |next: String| {
        if let Some((_, label)) = FRAMEWORKS.iter().find(|(id, _)| *id == next) {
          query.set(label.to_string());
        }
        value.set(next);
      },
      ComboboxList {
        if matches().is_empty() {
          ComboboxEmpty { "No framework found." }
        }
        for (id, label) in matches() {
          ComboboxItem { key: "{id}", value: id, selected: value() == id, "{label}" }
        }
      }
    }
  }
}
