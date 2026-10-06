use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

pub const TAGS_INPUT_BASE_CLASS: &str = "flex min-h-10 w-full flex-wrap items-center gap-1.5 rounded-md border bg-background px-2 py-1.5 text-sm transition-colors focus-within:ring-2";
pub const TAGS_INPUT_LIST_CLASS: &str = "flex flex-wrap gap-1.5";
pub const TAGS_INPUT_TAG_CLASS: &str = "inline-flex items-center gap-1 rounded-md bg-secondary py-0.5 ps-2 pe-1 text-xs font-medium text-secondary-foreground";
pub const TAGS_INPUT_REMOVE_CLASS: &str = "inline-flex size-4 items-center justify-center rounded-sm text-secondary-foreground hover:bg-background/60 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none";
pub const TAGS_INPUT_FIELD_CLASS: &str = "min-w-24 flex-1 bg-transparent py-0.5 outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed";

pub fn tags_input_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-within:ring-destructive"
  } else {
    "border-input focus-within:ring-ring"
  };

  merge_classes(classes([Some(TAGS_INPUT_BASE_CLASS), Some(invalid_class)]), class)
}

/// The tags with `draft` added, trimmed, unless it is empty or already there.
pub fn tags_input_add(tags: &[String], draft: &str) -> Option<Vec<String>> {
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
pub fn tags_input_commit(tags: &[String], text: &str) -> (Vec<String>, String) {
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

pub fn tags_input_remove(tags: &[String], index: usize) -> Vec<String> {
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
                class: TAGS_INPUT_REMOVE_CLASS,
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
        class: TAGS_INPUT_FIELD_CLASS,
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

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
  }

  #[test]
  fn ssr_tags_are_a_list_with_named_remove_buttons() {
    fn app() -> Element {
      rsx! { TagsInput { tags: vec!["rust".to_string(), "ui".to_string()], "aria-label": "Topics" } }
    }
    let html = render(app);

    assert!(html.contains("<ul class=\"flex flex-wrap gap-1.5\">"));
    assert_eq!(html.matches("<li").count(), 2);
    assert!(html.contains("aria-label=\"Remove rust\""));
    assert!(html.contains("aria-label=\"Topics\""));
  }

  #[test]
  fn adding_skips_empty_and_duplicate_tags() {
    let tags = strings(&["rust"]);

    assert_eq!(tags_input_add(&tags, "  ui "), Some(strings(&["rust", "ui"])));
    assert_eq!(tags_input_add(&tags, "rust"), None);
    assert_eq!(tags_input_add(&tags, "   "), None);
  }

  #[test]
  fn commas_commit_every_segment_before_the_last() {
    let (tags, draft) = tags_input_commit(&strings(&["a"]), "b, c,a, d");

    assert_eq!(tags, strings(&["a", "b", "c"]));
    assert_eq!(draft, "d");
    assert_eq!(tags_input_commit(&[], "plain"), (Vec::new(), "plain".to_string()));
  }

  #[test]
  fn removing_drops_one_tag() {
    assert_eq!(tags_input_remove(&strings(&["a", "b", "c"]), 1), strings(&["a", "c"]));
    assert_eq!(tags_input_remove(&strings(&["a"]), 3), strings(&["a"]));
  }
}
