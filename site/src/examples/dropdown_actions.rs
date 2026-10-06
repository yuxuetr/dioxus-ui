use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonVariant, DropdownContent, DropdownGroup, DropdownItem, DropdownLabel,
  DropdownSeparator, DropdownSub, DropdownSubContent, DropdownSubTrigger,
};

#[component]
pub fn Demo() -> Element {
  let mut open = use_signal(|| false);
  let mut share = use_signal(|| false);
  let mut action = use_signal(|| "none");

  rsx! {
    Button {
      id: "dropdown-actions-trigger",
      variant: ButtonVariant::Outline,
      "aria-haspopup": "menu",
      "aria-expanded": "{open}",
      onclick: move |_| open.toggle(),
      "Actions"
    }
    p { class: "mt-3 text-sm text-muted-foreground", "Last action: {action}" }
    DropdownContent {
      open: open(),
      anchor_id: "dropdown-actions-trigger",
      on_open_change: move |next| open.set(next),
      DropdownGroup {
        DropdownLabel { "Project" }
        DropdownItem { onclick: move |_| action.set("edit"), "Edit" }
        DropdownItem { onclick: move |_| action.set("duplicate"), "Duplicate" }
        DropdownSub { on_open_change: move |next| share.set(next),
          DropdownSubTrigger { open: share(), "Share" }
          DropdownSubContent { open: share(),
            DropdownItem { onclick: move |_| action.set("copy link"), "Copy link" }
            DropdownItem { onclick: move |_| action.set("email"), "Email" }
          }
        }
        DropdownItem { disabled: true, "Archive" }
      }
      DropdownSeparator {}
      DropdownItem { destructive: true, onclick: move |_| action.set("delete"), "Delete" }
    }
  }
}
