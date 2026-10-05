use dioxus::prelude::*;
use dioxus_shadcn::{
  Bubble, BubbleAlign, BubbleContent, BubbleVariant, Message, MessageAlign, MessageAvatar,
  MessageContent, MessageFooter, MessageGroup, MessageHeader,
};

#[component]
pub fn Demo() -> Element {
  rsx! {
    MessageGroup { class: "max-w-lg",
      Message {
        MessageAvatar { "AL" }
        MessageContent {
          MessageHeader { "Ada · 10:02" }
          Bubble { variant: BubbleVariant::Secondary,
            BubbleContent { variant: BubbleVariant::Secondary, "Did the dark theme check pass?" }
          }
        }
      }
      Message { align: MessageAlign::End,
        MessageAvatar { "GH" }
        MessageContent { align: MessageAlign::End,
          Bubble { align: BubbleAlign::End, BubbleContent { "All 67 routes, both themes." } }
          MessageFooter { "Read" }
        }
      }
    }
  }
}
