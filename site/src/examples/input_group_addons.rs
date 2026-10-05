use dioxus::prelude::*;
use dioxus_ui::{InputGroup, InputGroupAddon, InputGroupAddonPosition, InputGroupControl};

#[component]
pub fn Demo() -> Element {
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
      InputGroup { invalid: true,
        InputGroupAddon { "@" }
        InputGroupControl {
          input { class: "h-10 w-full px-3 outline-none", "aria-label": "Handle", "aria-invalid": "true", value: "taken" }
        }
      }
    }
  }
}
