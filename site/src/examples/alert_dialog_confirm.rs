use dioxus::prelude::*;
use dioxus_ui::{
  AlertDialogAction, AlertDialogActionVariant, AlertDialogCancel, AlertDialogContent,
  AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogOverlay,
  AlertDialogTitle, Button, ButtonVariant,
};

#[component]
pub fn Demo() -> Element {
  let mut open = use_signal(|| false);
  let mut result = use_signal(|| "pending");

  rsx! {
    Button { variant: ButtonVariant::Destructive, onclick: move |_| open.set(true), "Delete project" }
    p { class: "mt-3 text-sm text-muted-foreground", "Result: {result}" }
    AlertDialogOverlay { open: open() }
    AlertDialogContent { open: open(), on_open_change: move |next| open.set(next),
      AlertDialogHeader {
        AlertDialogTitle { "Delete project?" }
        AlertDialogDescription { "This permanently deletes the project and its deployments." }
      }
      AlertDialogFooter {
        AlertDialogCancel {
          on_open_change: move |next| {
            result.set("cancelled");
            open.set(next);
          },
          "Cancel"
        }
        AlertDialogAction {
          variant: AlertDialogActionVariant::Destructive,
          onclick: move |_| result.set("deleted"),
          on_open_change: move |next| open.set(next),
          "Delete"
        }
      }
    }
  }
}
