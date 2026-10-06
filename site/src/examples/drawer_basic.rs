use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonVariant, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter,
  DrawerHeader, DrawerOverlay, DrawerTitle,
};

#[component]
pub fn DrawerBasicDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut goal = use_signal(|| 350);

  rsx! {
    Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(true), "Set daily goal" }
    DrawerOverlay { open: open(), on_open_change: move |next| open.set(next) }
    DrawerContent { open: open(), on_open_change: move |next| open.set(next),
      DrawerHeader {
        DrawerTitle { "Move goal" }
        DrawerDescription { "Set your daily activity goal." }
      }
      div { class: "flex items-center justify-center gap-4",
        Button { variant: ButtonVariant::Outline, "aria-label": "Decrease", onclick: move |_| goal -= 10, "-" }
        span { class: "text-4xl font-bold", "{goal}" }
        Button { variant: ButtonVariant::Outline, "aria-label": "Increase", onclick: move |_| goal += 10, "+" }
      }
      DrawerFooter {
        Button { onclick: move |_| open.set(false), "Submit" }
        DrawerClose { on_open_change: move |next| open.set(next), "Cancel" }
      }
    }
  }
}
