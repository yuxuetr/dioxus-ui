use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonVariant, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader,
  SheetOverlay, SheetSide, SheetTitle,
};

#[component]
pub fn Demo() -> Element {
  let mut side = use_signal(|| None::<SheetSide>);
  let open = side().is_some();

  rsx! {
    div { class: "flex flex-wrap gap-2",
      for (label, value) in [("Left", SheetSide::Left), ("Right", SheetSide::Right), ("Top", SheetSide::Top), ("Bottom", SheetSide::Bottom)] {
        Button { key: "{label}", variant: ButtonVariant::Outline, onclick: move |_| side.set(Some(value)), "{label}" }
      }
    }
    SheetOverlay { open, on_open_change: move |_| side.set(None) }
    SheetContent {
      open,
      side: side().unwrap_or_default(),
      on_open_change: move |_| side.set(None),
      SheetHeader {
        SheetTitle { "Edit profile" }
        SheetDescription { "Make changes to your profile here." }
      }
      SheetFooter {
        Button { onclick: move |_| side.set(None), "Save changes" }
      }
      SheetClose { on_open_change: move |_| side.set(None), "×" }
    }
  }
}
