use dioxus::html::HasFileData;
use dioxus::prelude::*;

use crate::components::ui::attachment::{
  Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription,
  AttachmentGroup, AttachmentMedia, AttachmentMediaVariant, AttachmentTitle,
};
use crate::components::ui::avatar::{Avatar, AvatarFallback};
use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::item::{Item, ItemContent, ItemDescription, ItemMedia, ItemTitle};
use crate::components::ui::message::{
  Message, MessageAlign, MessageAvatar, MessageContent, MessageFooter,
};
use crate::components::ui::message_scroller::{
  MessageScroller, MessageScrollerContent, MessageScrollerViewport,
};
use crate::components::ui::textarea::Textarea;

/// A file attached to a message, by name and size; the app uploads it.
#[derive(Clone, Debug, PartialEq)]
pub struct ChatFile {
  pub name: String,
  pub size: u64,
}

/// What sending reports: the conversation, the text, and the attached files.
#[derive(Clone, Debug, PartialEq)]
pub struct ChatSend {
  pub conversation: u32,
  pub text: String,
  pub files: Vec<ChatFile>,
}

#[derive(Clone, Debug, PartialEq)]
struct ChatMessage {
  id: u32,
  mine: bool,
  text: String,
  files: Vec<ChatFile>,
  time: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
struct Conversation {
  id: u32,
  name: &'static str,
  messages: Vec<ChatMessage>,
}

fn text_message(id: u32, mine: bool, text: &str, time: &'static str) -> ChatMessage {
  ChatMessage { id, mine, text: text.to_string(), files: Vec::new(), time }
}

fn sample_conversations() -> Vec<Conversation> {
  vec![
    Conversation {
      id: 1,
      name: "Amara Okafor",
      messages: vec![
        text_message(1, false, "Did the new build fix the login loop?", "10:02"),
        text_message(2, true, "It did. The token refresh was racing the redirect.", "10:04"),
        ChatMessage {
          id: 3,
          mine: false,
          text: "Great. Here are the logs from before, in case you want them.".to_string(),
          files: vec![ChatFile { name: "auth-trace.log".to_string(), size: 48_213 }],
          time: "10:05",
        },
      ],
    },
    Conversation {
      id: 2,
      name: "Design team",
      messages: vec![text_message(4, false, "Review moved to 3pm.", "Yesterday")],
    },
    Conversation {
      id: 3,
      name: "Kenji Watanabe",
      messages: vec![text_message(5, true, "Sent you the contract draft.", "Mon")],
    },
  ]
}

fn format_size(bytes: u64) -> String {
  match bytes {
    0..1_024 => format!("{bytes} B"),
    1_024..1_048_576 => format!("{:.1} KB", bytes as f64 / 1_024.0),
    _ => format!("{:.1} MB", bytes as f64 / 1_048_576.0),
  }
}

fn initials(name: &str) -> String {
  name.split_whitespace().filter_map(|word| word.chars().next()).take(2).collect()
}

/// A chat screen: a conversation list, the open conversation's messages, and
/// a composer that takes typed text and files picked or dropped on it. Enter
/// sends and Shift+Enter starts a new line. `on_send` hears each message;
/// replace `sample_conversations` with your data and upload the files there.
#[component]
pub fn ChatBlock(#[props(default)] on_send: Option<EventHandler<ChatSend>>) -> Element {
  let mut conversations = use_signal(sample_conversations);
  let mut open = use_signal(|| 1_u32);
  let mut draft = use_signal(String::new);
  let mut pending = use_signal(Vec::<ChatFile>::new);
  let mut dragging = use_signal(|| false);
  let mut next_id = use_signal(|| 100_u32);

  let current = conversations().into_iter().find(|conversation| conversation.id == open());
  let mut add_files = move |files: Vec<ChatFile>| pending.with_mut(|list| list.extend(files));
  let mut send = move || {
    let text = draft().trim().to_string();
    let files = pending();
    if text.is_empty() && files.is_empty() {
      return;
    }
    let id = next_id();
    next_id.set(id + 1);
    let message =
      ChatMessage { id, mine: true, text: text.clone(), files: files.clone(), time: "Now" };
    conversations.with_mut(|all| {
      if let Some(conversation) = all.iter_mut().find(|conversation| conversation.id == open()) {
        conversation.messages.push(message);
      }
    });
    draft.set(String::new());
    pending.set(Vec::new());
    if let Some(handler) = on_send {
      handler.call(ChatSend { conversation: open(), text, files });
    }
  };

  rsx! {
    div { class: "flex h-screen min-h-[32rem] bg-background text-foreground",
      nav { "aria-label": "Conversations", class: "hidden w-64 shrink-0 border-e md:block",
        ul { class: "grid gap-1 p-2",
          for conversation in conversations() {
            li { key: "{conversation.id}",
              button {
                class: "block w-full rounded-md text-left focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring",
                "aria-current": (open() == conversation.id).then_some("true"),
                onclick: move |_| {
                  open.set(conversation.id);
                  draft.set(String::new());
                  pending.set(Vec::new());
                },
                Item { selected: open() == conversation.id,
                  ItemMedia {
                    Avatar {
                      AvatarFallback { "{initials(conversation.name)}" }
                    }
                  }
                  ItemContent {
                    ItemTitle { "{conversation.name}" }
                    if let Some(last) = conversation.messages.last() {
                      ItemDescription { "{last.text}" }
                    }
                  }
                }
              }
            }
          }
        }
      }
      if let Some(conversation) = current {
        section { "aria-label": "{conversation.name}", class: "flex min-w-0 flex-1 flex-col",
          header { class: "flex items-center gap-3 border-b px-4 py-3",
            Avatar {
              AvatarFallback { "{initials(conversation.name)}" }
            }
            h1 { class: "font-semibold", "{conversation.name}" }
          }
          MessageScroller { class: "min-h-0 flex-1",
            // Reversed columns keep the newest message in view as messages arrive.
            MessageScrollerViewport {
              class: "flex h-full flex-col-reverse overflow-y-auto",
              role: "log",
              "aria-label": "Messages",
              MessageScrollerContent { class: "grid gap-4 p-4",
                for message in conversation.messages {
                  Message {
                    key: "{message.id}",
                    align: if message.mine { MessageAlign::End } else { MessageAlign::Start },
                    if !message.mine {
                      MessageAvatar {
                        Avatar {
                          AvatarFallback { "{initials(conversation.name)}" }
                        }
                      }
                    }
                    MessageContent { align: if message.mine { MessageAlign::End } else { MessageAlign::Start },
                      if !message.text.is_empty() {
                        Bubble {
                          variant: if message.mine { BubbleVariant::Default } else { BubbleVariant::Muted },
                          align: if message.mine { BubbleAlign::End } else { BubbleAlign::Start },
                          BubbleContent {
                            variant: if message.mine { BubbleVariant::Default } else { BubbleVariant::Muted },
                            "{message.text}"
                          }
                        }
                      }
                      for file in message.files {
                        Attachment { key: "{file.name}", class: "w-64",
                          AttachmentMedia { variant: AttachmentMediaVariant::Icon, FileIcon {} }
                          AttachmentContent {
                            AttachmentTitle { "{file.name}" }
                            AttachmentDescription { "{format_size(file.size)}" }
                          }
                        }
                      }
                      MessageFooter { "{message.time}" }
                    }
                  }
                }
              }
            }
          }
          form {
            class: if dragging() { "grid gap-2 border-t bg-accent/40 p-3 outline-2 outline-dashed outline-primary" } else { "grid gap-2 border-t p-3" },
            "aria-label": "Message composer",
            ondragover: move |event| {
              event.prevent_default();
              dragging.set(true);
            },
            ondragleave: move |_| dragging.set(false),
            ondrop: move |event| {
              event.prevent_default();
              dragging.set(false);
              add_files(
                event.files().iter().map(|file| ChatFile { name: file.name(), size: file.size() }).collect(),
              );
            },
            onsubmit: move |event| {
              event.prevent_default();
              send();
            },
            if !pending().is_empty() {
              div { role: "group", "aria-label": "Files to send",
              AttachmentGroup {
                for (index, file) in pending().into_iter().enumerate() {
                  Attachment { key: "{index}-{file.name}",
                    AttachmentMedia { variant: AttachmentMediaVariant::Icon, FileIcon {} }
                    AttachmentContent {
                      AttachmentTitle { "{file.name}" }
                      AttachmentDescription { "{format_size(file.size)}" }
                    }
                    AttachmentActions {
                      AttachmentAction {
                        "aria-label": "Remove {file.name}",
                        onclick: move |_| {
                          pending.with_mut(|list| {
                            if index < list.len() {
                              list.remove(index);
                            }
                          });
                        },
                        "\u{d7}"
                      }
                    }
                  }
                }
              }
              }
            }
            div { class: "flex items-end gap-2",
              label {
                class: "inline-flex h-10 cursor-pointer items-center rounded-md border border-input px-3 text-sm font-medium hover:bg-accent focus-within:ring-2 focus-within:ring-ring",
                input {
                  class: "sr-only",
                  r#type: "file",
                  multiple: true,
                  onchange: move |event| {
                    add_files(
                      event.files().iter().map(|file| ChatFile { name: file.name(), size: file.size() }).collect(),
                    );
                  },
                }
                "Attach"
              }
              // Textarea passes no handlers through, so Enter is read on its wrapper.
              div {
                class: "flex-1",
                onkeydown: move |event: KeyboardEvent| {
                  if event.key() == Key::Enter && !event.modifiers().shift() {
                    event.prevent_default();
                    send();
                  }
                },
                Textarea {
                  class: "min-h-10 resize-none",
                  rows: "1",
                  placeholder: "Write a message, or drop files here",
                  "aria-label": "Message",
                  value: draft(),
                  on_value_change: move |value| draft.set(value),
                }
              }
              Button { r#type: "submit", variant: ButtonVariant::Primary, "Send" }
            }
          }
        }
      }
    }
  }
}

#[component]
fn FileIcon() -> Element {
  rsx! {
    svg {
      class: "size-4",
      "aria-hidden": "true",
      view_box: "0 0 24 24",
      fill: "none",
      stroke: "currentColor",
      stroke_width: "2",
      stroke_linecap: "round",
      stroke_linejoin: "round",
      path { d: "M14 3H6a1 1 0 0 0-1 1v16a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V8zM14 3v5h5" }
    }
  }
}
