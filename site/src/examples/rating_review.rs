use dioxus::prelude::*;
use dioxus_shadcn::{Label, Rating};

#[component]
pub fn Demo() -> Element {
  let mut quality = use_signal(|| 4_u8);
  let mut service = use_signal(|| 0_u8);

  rsx! {
    div { class: "grid gap-4",
      div { class: "grid gap-1",
        Label { id: "rating-quality-label", "Quality" }
        Rating {
          "aria-labelledby": "rating-quality-label",
          value: quality(),
          on_value_change: move |value| quality.set(value),
        }
      }
      div { class: "grid gap-1",
        Label { id: "rating-service-label", "Service" }
        Rating {
          "aria-labelledby": "rating-service-label",
          max: 10,
          value: service(),
          on_value_change: move |value| service.set(value),
        }
      }
      p { class: "text-sm text-muted-foreground", "Quality {quality}/5, service {service}/10" }
    }
  }
}
