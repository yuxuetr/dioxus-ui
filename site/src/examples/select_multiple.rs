use dioxus::prelude::*;
use dioxus_shadcn::{Select, SelectContent, SelectItem, SelectTrigger};

const LANGUAGES: [(&str, &str); 5] =
  [("rust", "Rust"), ("go", "Go"), ("ts", "TypeScript"), ("python", "Python"), ("zig", "Zig")];

#[component]
pub fn SelectMultipleDemo() -> Element {
  let mut values = use_signal(|| vec!["rust".to_string()]);
  let summary = match values().len() {
    0 => "Pick languages".to_string(),
    1 | 2 => LANGUAGES
      .iter()
      .filter(|(value, _)| values().iter().any(|chosen| chosen == value))
      .map(|(_, label)| *label)
      .collect::<Vec<_>>()
      .join(", "),
    count => format!("{count} languages"),
  };

  rsx! {
    div { class: "w-64",
      Select { multiple: true, values: values(), on_values_change: move |next| values.set(next),
        SelectTrigger { "aria-label": "Languages", span { class: "truncate", "{summary}" } }
        SelectContent {
          for (value, label) in LANGUAGES {
            SelectItem { key: "{value}", value, "{label}" }
          }
        }
      }
    }
  }
}
