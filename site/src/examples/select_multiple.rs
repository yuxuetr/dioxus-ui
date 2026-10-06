use dioxus::prelude::*;
use dioxus_shadcn::{SelectContent, SelectItem, SelectTrigger, SelectValue};

const LANGUAGES: [(&str, &str); 5] =
  [("rust", "Rust"), ("go", "Go"), ("ts", "TypeScript"), ("python", "Python"), ("zig", "Zig")];

#[component]
pub fn SelectMultipleDemo() -> Element {
  let mut open = use_signal(|| false);
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
      SelectTrigger {
        id: "select-multiple-trigger",
        "aria-label": "Languages",
        open: open(),
        on_open_change: move |next| open.set(next),
        SelectValue { "{summary}" }
      }
      SelectContent {
        open: open(),
        multiple: true,
        anchor_id: "select-multiple-trigger",
        on_open_change: move |next| open.set(next),
        on_value_change: move |value: String| {
          let mut next = values();
          match next.iter().position(|chosen| *chosen == value) {
            Some(index) => {
              next.remove(index);
            }
            None => next.push(value),
          }
          values.set(next);
        },
        for (value, label) in LANGUAGES {
          SelectItem { key: "{value}", value, selected: values().iter().any(|chosen| chosen == value), "{label}" }
        }
      }
    }
  }
}
