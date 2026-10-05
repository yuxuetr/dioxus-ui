use dioxus::prelude::*;
use dioxus_ui::{Avatar, AvatarFallback, AvatarImage};

// An inline image keeps the example working offline.
const PHOTO: &str = "data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20viewBox=%270%200%2040%2040%27%3E%3Crect%20width=%2740%27%20height=%2740%27%20fill=%27%233b82f6%27/%3E%3Ccircle%20cx=%2720%27%20cy=%2716%27%20r=%277%27%20fill=%27white%27/%3E%3Crect%20x=%279%27%20y=%2726%27%20width=%2722%27%20height=%2714%27%20rx=%277%27%20fill=%27white%27/%3E%3C/svg%3E";

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "flex items-center gap-4",
      Avatar {
        AvatarImage { src: PHOTO, alt: "Dioxus Labs" }
        AvatarFallback { "DL" }
      }
      Avatar { AvatarFallback { "AL" } }
      Avatar { class: "h-12 w-12", AvatarFallback { "GH" } }
    }
  }
}
