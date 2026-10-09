use dioxus::prelude::*;
use dioxus_shadcn::{Tree, TreeItem};

#[component]
pub fn TreeFilesDemo() -> Element {
  let mut selected = use_signal(|| "main".to_string());

  rsx! {
    div { class: "grid gap-3",
      Tree {
        class: "w-64 rounded-md border p-2",
        "aria-label": "Project files",
        default_expanded: vec!["src".to_string()],
        selected: selected(),
        on_selected_change: move |value: String| selected.set(value),
        TreeItem {
          value: "src",
          group: rsx! {
            TreeItem { value: "main", "main.rs" }
            TreeItem {
              value: "components",
              group: rsx! {
                TreeItem { value: "button", "button.rs" }
                TreeItem { value: "tree", "tree.rs" }
              },
              "components"
            }
          },
          "src"
        }
        TreeItem {
          value: "assets",
          group: rsx! {
            TreeItem { value: "css", "main.css" }
          },
          "assets"
        }
        TreeItem { value: "cargo", "Cargo.toml" }
      }
      p { class: "text-sm text-muted-foreground", "Selected: {selected}" }
    }
  }
}
