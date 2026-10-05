use dioxus::prelude::*;
use dioxus_ui::{
  Button, ButtonVariant, SonnerClose, SonnerContent, SonnerDescription, SonnerIcon, SonnerTitle,
  SonnerToast, SonnerVariant, SonnerViewport,
};

const VARIANTS: [(&str, SonnerVariant, &str); 4] = [
  ("Success", SonnerVariant::Success, "Event created"),
  ("Info", SonnerVariant::Info, "New version available"),
  ("Warning", SonnerVariant::Warning, "Storage almost full"),
  ("Error", SonnerVariant::Error, "Upload failed"),
];

#[component]
pub fn Demo() -> Element {
  let mut shown = use_signal(|| None::<usize>);

  rsx! {
    div { class: "flex flex-wrap gap-2",
      for (index, (label, _, _)) in VARIANTS.iter().enumerate() {
        Button { key: "{label}", variant: ButtonVariant::Outline, onclick: move |_| shown.set(Some(index)), "{label}" }
      }
    }
    SonnerViewport {
      if let Some((_, variant, title)) = shown().and_then(|index| VARIANTS.get(index)).copied() {
        SonnerToast { key: "{title}", variant, on_dismiss: move |_| shown.set(None),
          SonnerIcon { variant }
          SonnerContent {
            SonnerTitle { "{title}" }
            SonnerDescription { variant, "Dismisses after five seconds." }
          }
          SonnerClose { on_dismiss: move |_| shown.set(None), "×" }
        }
      }
    }
  }
}
