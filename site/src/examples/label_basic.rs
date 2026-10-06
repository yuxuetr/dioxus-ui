use dioxus::prelude::*;
use dioxus_shadcn::{Input, Label, Switch};

#[component]
pub fn LabelBasicDemo() -> Element {
  let mut notify = use_signal(|| false);

  rsx! {
    div { class: "grid max-w-sm gap-4",
      div { class: "grid gap-2",
        Label { r#for: "label-basic-name", "Display name" }
        Input { id: "label-basic-name", placeholder: "Ada Lovelace" }
      }
      div { class: "flex items-center gap-2",
        Switch {
          id: "label-basic-notify",
          checked: notify(),
          on_checked_change: move |checked| notify.set(checked),
        }
        Label { r#for: "label-basic-notify", "Email notifications" }
      }
    }
  }
}
