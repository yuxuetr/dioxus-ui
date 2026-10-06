use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonVariant, Sheet, SheetClose, SheetContent, SheetDescription, SheetFooter,
  SheetHeader, SheetOverlay, SheetSide, SheetTitle,
};

#[component]
pub fn SheetSideDemo() -> Element {
  let mut side = use_signal(|| None::<SheetSide>);

  rsx! {
    div { class: "flex flex-wrap gap-2",
      for (label, value) in [("Left", SheetSide::Left), ("Right", SheetSide::Right), ("Top", SheetSide::Top), ("Bottom", SheetSide::Bottom)] {
        Button { key: "{label}", variant: ButtonVariant::Outline, onclick: move |_| side.set(Some(value)), "{label}" }
      }
    }
    Sheet {
      open: side().is_some(),
      on_open_change: move |next: bool| {
        if !next {
          side.set(None);
        }
      },
      SheetOverlay {}
      SheetContent { side: side().unwrap_or_default(),
        SheetHeader {
          SheetTitle { "Edit profile" }
          SheetDescription { "Make changes to your profile here." }
        }
        SheetFooter {
          Button { onclick: move |_| side.set(None), "Save changes" }
        }
        SheetClose { "×" }
      }
    }
  }
}
