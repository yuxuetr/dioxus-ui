use dioxus::prelude::*;
use dioxus_shadcn::{Label, TagsInput};

#[component]
pub fn TagsInputTopicsDemo() -> Element {
  let mut topics = use_signal(|| vec!["dioxus".to_string(), "rust".to_string()]);

  rsx! {
    div { class: "grid max-w-md gap-2",
      Label { r#for: "tags-input-topics", "Topics" }
      TagsInput {
        id: "tags-input-topics",
        placeholder: "Add a topic and press Enter",
        tags: topics(),
        on_tags_change: move |tags| topics.set(tags),
      }
      p { class: "text-sm text-muted-foreground", "{topics().len()} topics" }
    }
  }
}
