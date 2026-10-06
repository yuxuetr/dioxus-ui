use dioxus::prelude::*;
use dioxus_shadcn::{
  Menubar, MenubarCheckboxItem, MenubarContent, MenubarItem, MenubarMenu, MenubarSeparator,
  MenubarShortcut, MenubarTrigger,
};

#[component]
pub fn MenubarEditorDemo() -> Element {
  let mut action = use_signal(|| "none");
  let mut word_wrap = use_signal(|| true);

  rsx! {
    Menubar { "aria-label": "Editor",
      MenubarMenu { value: "file",
        MenubarTrigger { "File" }
        MenubarContent {
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
        MenubarTrigger { "Edit" }
        MenubarContent {
          MenubarItem { onclick: move |_| action.set("undo"), "Undo" }
          MenubarItem { onclick: move |_| action.set("redo"), "Redo" }
        }
      }
      MenubarMenu { value: "view",
        MenubarTrigger { "View" }
        MenubarContent {
          MenubarCheckboxItem { checked: word_wrap(), onclick: move |_| word_wrap.toggle(), "Word wrap" }
        }
      }
    }
    p { class: "mt-3 text-sm text-muted-foreground", "Last action: {action}" }
  }
}
