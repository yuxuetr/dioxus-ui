use dioxus::prelude::*;
use dioxus_shadcn::{ButtonGroup, ButtonGroupItem, ButtonGroupOrientation};

#[component]
pub fn ButtonGroupBasicDemo() -> Element {
  let mut align = use_signal(|| "left");
  let mut history = use_signal(|| vec!["Typed a heading"]);
  let mut undone = use_signal(Vec::<&str>::new);

  rsx! {
    div { class: "flex flex-wrap items-start gap-6",
      div { class: "grid gap-2",
        ButtonGroup { aria_label: "Text alignment",
          for (side, label) in [("left", "Left"), ("center", "Center"), ("right", "Right")] {
            ButtonGroupItem {
              key: "{side}",
              class: if align() == side { "bg-accent".to_string() } else { String::new() },
              "aria-pressed": (align() == side).to_string(),
              onclick: move |_| align.set(side),
              "{label}"
            }
          }
        }
        p { class: "text-sm text-muted-foreground", "Aligned {align}" }
      }
      ButtonGroup { aria_label: "History", orientation: ButtonGroupOrientation::Vertical,
        ButtonGroupItem {
          disabled: history().is_empty(),
          onclick: move |_| {
            if let Some(step) = history.write().pop() {
              undone.write().push(step);
            }
          },
          "Undo"
        }
        ButtonGroupItem {
          disabled: undone().is_empty(),
          onclick: move |_| {
            if let Some(step) = undone.write().pop() {
              history.write().push(step);
            }
          },
          "Redo"
        }
        ButtonGroupItem { disabled: true, "Clear" }
      }
    }
  }
}
