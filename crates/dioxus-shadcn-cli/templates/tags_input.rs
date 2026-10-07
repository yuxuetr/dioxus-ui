//! Tags input: collects a list of short values, such as topics or email recipients,
//! shown as removable chips.
use super::utils::{classes, merge_classes};
use super::density::{density_control_class, density_hit_area_class, use_density, with_density};
use dioxus::prelude::*;

const TAGS_INPUT_BASE_CLASS: &str = "flex min-h-10 w-full flex-wrap items-center gap-1.5 rounded-md border bg-background px-2 py-1.5 text-sm transition-colors focus-within:ring-2";
const TAGS_INPUT_LIST_CLASS: &str = "flex flex-wrap gap-1.5";
const TAGS_INPUT_TAG_CLASS: &str = "inline-flex items-center gap-1 rounded-md bg-secondary py-0.5 ps-2 pe-1 text-xs font-medium text-secondary-foreground";
const TAGS_INPUT_REMOVE_CLASS: &str = "inline-flex size-4 items-center justify-center rounded-sm text-secondary-foreground hover:bg-background/60 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none";
const TAGS_INPUT_FIELD_CLASS: &str = "min-w-24 flex-1 bg-transparent py-0.5 outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed";

/// Classes for the field: base classes, the invalid or normal border and focus ring,
/// then `class` merged over them.
pub fn tags_input_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-within:ring-destructive"
  } else {
    "border-input focus-within:ring-ring"
  };

  merge_classes(classes([Some(TAGS_INPUT_BASE_CLASS), Some(invalid_class)]), class)
}

/// The tags with `draft` added, trimmed, unless it is empty or already there.
fn tags_input_add(tags: &[String], draft: &str) -> Option<Vec<String>> {
  let tag = draft.trim();
  if tag.is_empty() || tags.iter().any(|existing| existing == tag) {
    return None;
  }
  let mut next = tags.to_vec();
  next.push(tag.to_string());
  Some(next)
}

/// Typed or pasted text with commas: every segment before the last comma
/// becomes a tag, and the rest stays as the draft.
fn tags_input_commit(tags: &[String], text: &str) -> (Vec<String>, String) {
  let Some((complete, draft)) = text.rsplit_once(',') else {
    return (tags.to_vec(), text.to_string());
  };
  let mut next = tags.to_vec();
  for segment in complete.split(',') {
    if let Some(added) = tags_input_add(&next, segment) {
      next = added;
    }
  }
  (next, draft.trim_start().to_string())
}

fn tags_input_remove(tags: &[String], index: usize) -> Vec<String> {
  tags
    .iter()
    .enumerate()
    .filter(|(position, _)| *position != index)
    .map(|(_, tag)| tag.clone())
    .collect()
}

/// Tags as removable chips before a text input. Enter or a comma adds the
/// draft, Backspace in an empty input removes the last tag, and changes call
/// `on_tags_change`. Each remove button is named "{remove_label} {tag}".
/// Other attributes, such as `id`, `placeholder`, and `aria-label`, go to
/// the input.
#[component]
pub fn TagsInput(
  tags: Vec<String>,
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
  #[props(default = "Remove".to_string())] remove_label: String,
  #[props(default)] class: String,
  #[props(default)] on_tags_change: Option<EventHandler<Vec<String>>>,
  #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
  let class = tags_input_class(invalid, &class);
  let density = use_density();
  let mut draft = use_signal(String::new);
  let emit = move |next: Vec<String>| {
    if let Some(handler) = on_tags_change {
      handler.call(next);
    }
  };
  let input_tags = tags.clone();
  let key_tags = tags.clone();

  rsx! {
    div { class,
      if !tags.is_empty() {
        ul { class: TAGS_INPUT_LIST_CLASS,
          for (index, tag) in tags.iter().enumerate() {
            li { key: "{tag}", class: TAGS_INPUT_TAG_CLASS,
              span { "{tag}" }
              button {
                class: with_density(density_hit_area_class(density), TAGS_INPUT_REMOVE_CLASS),
                r#type: "button",
                disabled,
                "aria-label": "{remove_label} {tag}",
                onclick: {
                  let tags = tags.clone();
                  move |_| emit(tags_input_remove(&tags, index))
                },
                "×"
              }
            }
          }
        }
      }
      input {
        class: with_density(density_control_class(density), TAGS_INPUT_FIELD_CLASS),
        r#type: "text",
        autocomplete: "off",
        disabled,
        value: draft(),
        "aria-invalid": invalid.then_some("true"),
        oninput: move |event| {
          let (next, rest) = tags_input_commit(&input_tags, &event.value());
          if next.len() != input_tags.len() {
            emit(next);
          }
          draft.set(rest);
        },
        onkeydown: move |event| match event.key() {
          Key::Enter => {
            // Adding a tag must not submit a surrounding form.
            event.prevent_default();
            if let Some(next) = tags_input_add(&key_tags, &draft.peek()) {
              emit(next);
            }
            draft.set(String::new());
          }
          Key::Backspace if draft.peek().is_empty() && !key_tags.is_empty() => {
            emit(tags_input_remove(&key_tags, key_tags.len() - 1));
          }
          _ => {}
        },
        ..attributes,
      }
    }
  }
}
