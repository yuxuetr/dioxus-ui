use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonVariant, ToastAction, ToastClose, ToastDescription, ToastRoot, ToastTitle,
  ToastViewport, toast_dismiss_reason_attribute,
};

#[component]
pub fn ToastUndoDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut reason = use_signal(|| "none");

  rsx! {
    Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(true), "Archive message" }
    p { class: "mt-3 text-sm text-muted-foreground", "Last dismissal: {reason}" }
    ToastViewport {
      ToastRoot {
        open: open(),
        on_dismiss: move |next| {
          reason.set(toast_dismiss_reason_attribute(next));
          open.set(false);
        },
        ToastTitle { "Message archived" }
        ToastDescription { "It moves to the archive folder." }
        ToastAction {
          on_dismiss: move |next| {
            reason.set(toast_dismiss_reason_attribute(next));
            open.set(false);
          },
          "Undo"
        }
        ToastClose {
          on_dismiss: move |next| {
            reason.set(toast_dismiss_reason_attribute(next));
            open.set(false);
          },
          "×"
        }
      }
    }
  }
}
