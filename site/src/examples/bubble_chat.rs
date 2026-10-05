use dioxus::prelude::*;
use dioxus_ui::{Bubble, BubbleAlign, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant};

#[component]
pub fn Demo() -> Element {
  rsx! {
    BubbleGroup { class: "max-w-md",
      Bubble { variant: BubbleVariant::Secondary,
        BubbleContent { variant: BubbleVariant::Secondary, "Can we ship the token migration today?" }
      }
      Bubble { align: BubbleAlign::End,
        BubbleContent { "Yes, the release gate is green." }
        BubbleReactions { "+2 agreed" }
      }
      Bubble { variant: BubbleVariant::Tinted,
        BubbleContent { variant: BubbleVariant::Tinted, "I'll update the changelog." }
      }
      Bubble { variant: BubbleVariant::Destructive,
        BubbleContent { variant: BubbleVariant::Destructive, "This message could not be delivered." }
      }
    }
  }
}
