use dioxus::prelude::*;
use dioxus_shadcn::{
  Menubar, MenubarCheckboxItem, MenubarContent, MenubarItem, MenubarMenu, MenubarSeparator,
  MenubarShortcut, MenubarTrigger,
};

#[component]
pub fn MenubarEditorDemo() -> Element {
  let mut active = use_signal(|| None::<&'static str>);
  let mut action = use_signal(|| "none");
  let mut word_wrap = use_signal(|| true);
  let open = move |menu: &str| active() == Some(menu);

  rsx! {
    Menubar {
      "aria-label": "Editor",
      on_value_change: move |value: String| {
        active.set(["file", "edit", "view"].into_iter().find(|menu| *menu == value))
      },
      MenubarMenu { value: "file",
        MenubarTrigger {
          id: "menubar-editor-file",
          open: open("file"),
          on_open_change: move |next: bool| active.set(next.then_some("file")),
          "File"
        }
        MenubarContent {
          open: open("file"),
          anchor_id: "menubar-editor-file",
          on_open_change: move |_| active.set(None),
          MenubarItem { onclick: move |_| action.set("new"),
            "New file"
            MenubarShortcut { "Ctrl N" }
          }
          MenubarItem { onclick: move |_| action.set("open"), "Open" }
          MenubarSeparator {}
          MenubarItem { disabled: true, "Share" }
        }
      }
      MenubarMenu { value: "edit",
        MenubarTrigger {
          id: "menubar-editor-edit",
          open: open("edit"),
          on_open_change: move |next: bool| active.set(next.then_some("edit")),
          "Edit"
        }
        MenubarContent {
          open: open("edit"),
          anchor_id: "menubar-editor-edit",
          on_open_change: move |_| active.set(None),
          MenubarItem { onclick: move |_| action.set("undo"), "Undo" }
          MenubarItem { onclick: move |_| action.set("redo"), "Redo" }
        }
      }
      MenubarMenu { value: "view",
        MenubarTrigger {
          id: "menubar-editor-view",
          open: open("view"),
          on_open_change: move |next: bool| active.set(next.then_some("view")),
          "View"
        }
        MenubarContent {
          open: open("view"),
          anchor_id: "menubar-editor-view",
          on_open_change: move |_| active.set(None),
          MenubarCheckboxItem { checked: word_wrap(), onclick: move |_| word_wrap.toggle(), "Word wrap" }
        }
      }
    }
    p { class: "mt-3 text-sm text-muted-foreground", "Last action: {action}" }
  }
}
