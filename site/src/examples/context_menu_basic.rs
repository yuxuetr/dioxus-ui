use dioxus::prelude::*;
use dioxus_shadcn::{
  ContextMenuCheckboxItem, ContextMenuContent, ContextMenuItem, ContextMenuSeparator,
  ContextMenuShortcut,
};

#[component]
pub fn Demo() -> Element {
  let mut open = use_signal(|| false);
  let mut point = use_signal(|| (0.0, 0.0));
  let mut bookmarked = use_signal(|| false);
  let mut action = use_signal(|| "none");

  rsx! {
    div {
      class: "flex h-32 items-center justify-center rounded-md border border-dashed border-input text-sm text-muted-foreground",
      oncontextmenu: move |event| {
        event.prevent_default();
        let position = event.client_coordinates();
        point.set((position.x, position.y));
        open.set(true);
      },
      "Right-click here"
    }
    p { class: "mt-3 text-sm text-muted-foreground", "Last action: {action}" }
    ContextMenuContent {
      open: open(),
      anchor_point: point(),
      on_open_change: move |next| open.set(next),
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
