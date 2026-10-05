use dioxus::prelude::*;
use dioxus_shadcn::{FileInput, Label};

#[component]
pub fn Demo() -> Element {
  let mut names = use_signal(Vec::<String>::new);

  rsx! {
    div { class: "grid max-w-sm gap-2",
      Label { r#for: "file-input-documents", "Documents" }
      FileInput {
        id: "file-input-documents",
        multiple: true,
        accept: ".pdf,.md,.txt",
        onchange: move |event: FormEvent| {
          names.set(event.files().iter().map(|file| file.name()).collect());
        },
      }
      if names().is_empty() {
        p { class: "text-sm text-muted-foreground", "PDF, Markdown, or text files." }
      } else {
        ul { class: "list-disc ps-5 text-sm",
          for name in names() {
            li { key: "{name}", "{name}" }
          }
        }
      }
    }
  }
}
