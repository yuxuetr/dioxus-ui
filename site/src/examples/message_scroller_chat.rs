use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonVariant, MessageScroller, MessageScrollerBottomAnchor, MessageScrollerContent,
  MessageScrollerIntent, MessageScrollerJumpButton, MessageScrollerUnreadMarker,
  MessageScrollerViewport,
};

#[component]
pub fn MessageScrollerChatDemo() -> Element {
  let mut messages = use_signal(|| (1..=8).map(|n| format!("Message {n}")).collect::<Vec<_>>());
  let mut unread = use_signal(|| false);

  rsx! {
    Button {
      variant: ButtonVariant::Outline,
      onclick: move |_| {
        let next = messages().len() + 1;
        messages.write().push(format!("Message {next}"));
        unread.set(true);
      },
      "Receive a message"
    }
    MessageScroller {
      class: "mt-3 h-56 max-w-sm rounded-md border border-border",
      intent: MessageScrollerIntent::Follow,
      has_unread: unread(),
      MessageScrollerViewport {
        MessageScrollerContent { class: "grid gap-2 p-3",
          for message in messages() {
            div { key: "{message}", class: "rounded-md bg-muted px-3 py-2 text-sm", "{message}" }
          }
          MessageScrollerUnreadMarker { visible: unread(), "New messages" }
          MessageScrollerBottomAnchor {}
        }
      }
      MessageScrollerJumpButton { visible: unread(), onclick: move |_| unread.set(false), "Jump to latest" }
    }
  }
}
