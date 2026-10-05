use dioxus::prelude::*;
use dioxus_ui::{
  InputGroup, InputGroupAction, InputGroupAddon, InputGroupAddonPosition, InputGroupControl,
};

#[component]
pub fn Demo() -> Element {
  let mut query = use_signal(|| "dialog".to_string());

  rsx! {
    div { class: "grid max-w-sm gap-4",
      InputGroup {
        InputGroupAddon { "https://" }
        InputGroupControl {
          input { class: "h-10 w-full px-3 outline-none", "aria-label": "Website", placeholder: "example.com" }
        }
      }
      InputGroup {
        InputGroupControl {
          input { class: "h-10 w-full px-3 outline-none", "aria-label": "Amount", value: "120" }
        }
        InputGroupAddon { position: InputGroupAddonPosition::End, "USD" }
      }
      InputGroup {
        InputGroupControl {
          input {
            class: "h-10 w-full px-3 outline-none",
            "aria-label": "Search components",
            value: query(),
            oninput: move |event| query.set(event.value()),
          }
        }
        InputGroupAction {
          disabled: query().is_empty(),
          onclick: move |_| query.set(String::new()),
          "Clear"
        }
      }
      InputGroup { invalid: true,
        InputGroupAddon { "@" }
        InputGroupControl {
          input { class: "h-10 w-full px-3 outline-none", "aria-label": "Handle", "aria-invalid": "true", value: "taken" }
        }
      }
    }
  }
}
