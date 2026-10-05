use dioxus::prelude::*;
use dioxus_shadcn::{Marker, MarkerContent, MarkerIcon, MarkerVariant};

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "grid max-w-md gap-3",
      Marker {
        MarkerIcon {
          svg { class: "h-4 w-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
            circle { cx: "12", cy: "12", r: "9" }
            path { d: "M12 8h.01M11 12h1v4h1" }
          }
        }
        MarkerContent { "Ada joined the conversation" }
      }
      Marker { variant: MarkerVariant::Border,
        MarkerIcon {
          svg { class: "h-4 w-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
            path { d: "M12 17v5M9 3h6l-1 7 4 3H6l4-3z" }
          }
        }
        MarkerContent { "Pinned by Grace" }
      }
      Marker { variant: MarkerVariant::Separator,
        MarkerContent { "Today" }
      }
    }
  }
}
