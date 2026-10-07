use dioxus::prelude::*;
use dioxus_shadcn::{
  ButtonSize, ButtonVariant, Dropdown, DropdownContent, DropdownGroup, DropdownItem, DropdownLabel,
  DropdownSeparator, DropdownSub, DropdownSubContent, DropdownSubTrigger, DropdownTrigger,
  button_class, use_density,
};

#[component]
pub fn DropdownActionsDemo() -> Element {
  let mut action = use_signal(|| "none");

  rsx! {
    Dropdown {
      DropdownTrigger {
        class: button_class(ButtonVariant::Outline, ButtonSize::Md, use_density(), ""),
        "Actions"
      }
      p { class: "mt-3 text-sm text-muted-foreground", "Last action: {action}" }
      DropdownContent {
        DropdownGroup {
          DropdownLabel { "Project" }
          DropdownItem { onclick: move |_| action.set("edit"), "Edit" }
          DropdownItem { onclick: move |_| action.set("duplicate"), "Duplicate" }
          DropdownSub {
            DropdownSubTrigger { "Share" }
            DropdownSubContent {
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
}
