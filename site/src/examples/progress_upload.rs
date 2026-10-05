use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonVariant, Label, Progress};

#[component]
pub fn Demo() -> Element {
  let mut uploaded = use_signal(|| 3.0_f32);

  rsx! {
    div { class: "grid max-w-sm gap-3",
      Label { id: "progress-upload-label", "Uploaded {uploaded} of 5 files" }
      Progress {
        value: uploaded() / 5.0 * 100.0,
        "aria-labelledby": "progress-upload-label",
        "aria-valuetext": "{uploaded} of 5 files",
      }
      div { class: "flex gap-2",
        Button { variant: ButtonVariant::Outline, disabled: uploaded() <= 0.0, onclick: move |_| uploaded -= 1.0, "Remove" }
        Button { variant: ButtonVariant::Outline, disabled: uploaded() >= 5.0, onclick: move |_| uploaded += 1.0, "Add" }
      }
    }
  }
}
