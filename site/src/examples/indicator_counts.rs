use dioxus::prelude::*;
use dioxus_shadcn::{
  Avatar, AvatarFallback, Badge, BadgeVariant, Button, ButtonVariant, Indicator, IndicatorItem,
  IndicatorPlacement, Status, StatusSize, StatusVariant,
};

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "flex flex-wrap items-center gap-8 p-2",
      Indicator {
        Button { variant: ButtonVariant::Outline, "aria-label": "Inbox, 3 unread", "Inbox" }
        IndicatorItem { "aria-hidden": "true",
          Badge { class: "rounded-full px-1.5", variant: BadgeVariant::Destructive, "3" }
        }
      }
      Indicator {
        Avatar { AvatarFallback { "JD" } }
        IndicatorItem { placement: IndicatorPlacement::BottomEnd,
          Status {
            class: "ring-2 ring-background",
            variant: StatusVariant::Success,
            size: StatusSize::Lg,
            label: "Online",
          }
        }
      }
      Indicator {
        div { class: "grid size-20 place-items-center rounded-md border border-border text-sm text-muted-foreground",
          "Content"
        }
        IndicatorItem { placement: IndicatorPlacement::TopStart,
          Badge { variant: BadgeVariant::Info, "New" }
        }
      }
    }
  }
}
