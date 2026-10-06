use dioxus::prelude::*;
use dioxus_shadcn::{
  Timeline, TimelineContent, TimelineItem, TimelineMarker, TimelineOrientation, TimelineTime,
};

#[component]
pub fn TimelineReleaseDemo() -> Element {
  rsx! {
    div { class: "grid gap-10",
      Timeline { class: "max-w-md",
        TimelineItem {
          TimelineTime { datetime: "2026-09-12", "Sep 12" }
          TimelineMarker {}
          TimelineContent {
            p { class: "font-medium", "Design approved" }
            p { class: "text-sm text-muted-foreground", "RFCs for the component set were accepted." }
          }
        }
        TimelineItem {
          TimelineTime { datetime: "2026-09-28", "Sep 28" }
          TimelineMarker {}
          TimelineContent {
            p { class: "font-medium", "Release candidate" }
            p { class: "text-sm text-muted-foreground", "All gates passed on every renderer." }
          }
        }
        TimelineItem {
          TimelineTime { datetime: "2026-10-05", "Oct 5" }
          TimelineMarker { class: "text-success",
            svg {
              class: "size-4",
              view_box: "0 0 16 16",
              fill: "none",
              stroke: "currentColor",
              stroke_width: "2.5",
              stroke_linecap: "round",
              stroke_linejoin: "round",
              path { d: "M3.5 8.5l3 3 6-7" }
            }
          }
          TimelineContent {
            p { class: "font-medium", "Published" }
            p { class: "text-sm text-muted-foreground", "Version 0.1.0 is on crates.io." }
          }
        }
      }
      Timeline { orientation: TimelineOrientation::Horizontal,
        for (date, label) in [("2026-01-01", "Plan"), ("2026-04-01", "Build"), ("2026-07-01", "Test"), ("2026-10-01", "Ship")] {
          TimelineItem { key: "{date}",
            TimelineTime { datetime: date, "{label}" }
            TimelineMarker {}
            TimelineContent { class: "text-sm", "{date}" }
          }
        }
      }
    }
  }
}
