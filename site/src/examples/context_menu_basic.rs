use dioxus::prelude::*;
use dioxus_shadcn::{
  ContextMenu, ContextMenuCheckboxItem, ContextMenuContent, ContextMenuItem, ContextMenuSeparator,
  ContextMenuShortcut, ContextMenuTrigger,
};

#[component]
pub fn ContextMenuBasicDemo() -> Element {
  let mut bookmarked = use_signal(|| false);
  let mut action = use_signal(|| "none");

  rsx! {
    ContextMenu {
      ContextMenuTrigger {
        class: "flex h-32 items-center justify-center rounded-md border border-dashed border-input text-sm text-muted-foreground",
        "Right-click here"
      }
      p { class: "mt-3 text-sm text-muted-foreground", "Last action: {action}" }
      ContextMenuContent {
        ContextMenuItem { onclick: move |_| action.set("back"),
          "Back"
          ContextMenuShortcut { "Alt ←" }
        }
        ContextMenuItem { disabled: true, "Forward" }
        ContextMenuCheckboxItem { checked: bookmarked(), onclick: move |_| bookmarked.toggle(), "Bookmark" }
        ContextMenuSeparator {}
        ContextMenuItem { destructive: true, onclick: move |_| action.set("delete"), "Delete" }
      }
    }
  }
}
