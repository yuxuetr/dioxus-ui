use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, Drawer, DrawerClose, DrawerContent, DrawerDescription,
  DrawerFooter, DrawerHeader, DrawerOverlay, DrawerTitle, DrawerTrigger, button_class, use_density,
};

#[component]
pub fn DrawerBasicDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut goal = use_signal(|| 350);

  rsx! {
    Drawer { open: open(), on_open_change: move |next| open.set(next),
      DrawerTrigger {
        class: button_class(ButtonVariant::Outline, ButtonSize::Md, use_density(), ""),
        "Set daily goal"
      }
      DrawerOverlay {}
      DrawerContent {
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
          DrawerClose { "Cancel" }
        }
      }
    }
  }
}
