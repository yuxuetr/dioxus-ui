use dioxus::prelude::*;
use dioxus_ui::{Spinner, SpinnerSize};

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "flex items-center gap-4",
      Spinner { size: SpinnerSize::Sm }
      Spinner { size: SpinnerSize::Md, label: "Saving" }
      Spinner { size: SpinnerSize::Lg, label: "Uploading" }
    }
  }
}
