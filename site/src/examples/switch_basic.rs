use dioxus::prelude::*;
use dioxus_shadcn::{Label, Switch};

#[component]
pub fn Demo() -> Element {
  let mut wifi = use_signal(|| true);

  rsx! {
    div { class: "grid gap-3",
      div { class: "flex items-center gap-2",
        Switch {
          id: "switch-basic-wifi",
          checked: wifi(),
          on_checked_change: move |checked| wifi.set(checked),
        }
        Label { r#for: "switch-basic-wifi", "Wi-Fi" }
      }
      div { class: "flex items-center gap-2",
        Switch { id: "switch-basic-airplane", checked: true, disabled: true }
        Label { r#for: "switch-basic-airplane", "Airplane mode (managed)" }
      }
    }
  }
}
