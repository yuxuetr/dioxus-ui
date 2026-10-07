use dioxus::prelude::*;
use dioxus_shadcn::{
  AlertDialog, AlertDialogAction, AlertDialogActionVariant, AlertDialogCancel, AlertDialogContent,
  AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogOverlay,
  AlertDialogTitle, AlertDialogTrigger, ButtonSize, ButtonVariant, button_class, use_density,
};

#[component]
pub fn AlertDialogConfirmDemo() -> Element {
  let mut result = use_signal(|| "pending");

  rsx! {
    AlertDialog {
      AlertDialogTrigger {
        class: button_class(ButtonVariant::Destructive, ButtonSize::Md, use_density(), ""),
        "Delete project"
      }
      p { class: "mt-3 text-sm text-muted-foreground", "Result: {result}" }
      AlertDialogOverlay {}
      AlertDialogContent {
        AlertDialogHeader {
          AlertDialogTitle { "Delete project?" }
          AlertDialogDescription { "This permanently deletes the project and its deployments." }
        }
        AlertDialogFooter {
          AlertDialogCancel { onclick: move |_| result.set("cancelled"), "Cancel" }
          AlertDialogAction {
            variant: AlertDialogActionVariant::Destructive,
            onclick: move |_| result.set("deleted"),
            "Delete"
          }
        }
      }
    }
  }
}
