use dioxus::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback};
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::input::Input;
use crate::components::ui::item::{Item, ItemContent, ItemDescription, ItemTitle};
use crate::components::ui::resizable::{
  ResizableHandle, ResizablePanel, ResizablePanelGroup, ResizablePanelState, resizable_resize_pair,
};
use crate::components::ui::scroll_area::{ScrollArea, ScrollAreaViewport};
use crate::components::ui::separator::Separator;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Folder {
  Inbox,
  Sent,
  Archive,
}

impl Folder {
  const ALL: [Folder; 3] = [Folder::Inbox, Folder::Sent, Folder::Archive];

  fn label(self) -> &'static str {
    match self {
      Folder::Inbox => "Inbox",
      Folder::Sent => "Sent",
      Folder::Archive => "Archive",
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
struct Mail {
  id: u32,
  from: &'static str,
  subject: &'static str,
  body: &'static str,
  date: &'static str,
  folder: Folder,
  unread: bool,
}

fn sample_mail() -> Vec<Mail> {
  vec![
    Mail {
      id: 1,
      from: "Priya Natarajan",
      subject: "Quarterly numbers",
      body: "The numbers for the quarter are in. Revenue is up 12 percent and churn is flat. Can we go through the details on Thursday?",
      date: "9:41",
      folder: Folder::Inbox,
      unread: true,
    },
    Mail {
      id: 2,
      from: "Billing",
      subject: "Invoice 2026-10 is ready",
      body: "Your invoice for October is ready. It will be charged to the card ending in 4242 on the 15th.",
      date: "Yesterday",
      folder: Folder::Inbox,
      unread: true,
    },
    Mail {
      id: 3,
      from: "Tomás Herrera",
      subject: "Design review notes",
      body: "Thanks for the walkthrough. Two notes: the empty state needs a clearer action, and the table could use a sticky header.",
      date: "Mon",
      folder: Folder::Inbox,
      unread: false,
    },
    Mail {
      id: 4,
      from: "Lena Fischer",
      subject: "Offsite travel",
      body: "Flights are booked for the 21st. The hotel is a ten minute walk from the venue.",
      date: "Oct 2",
      folder: Folder::Inbox,
      unread: false,
    },
    Mail {
      id: 5,
      from: "You",
      subject: "Re: Release checklist",
      body: "Looks good to me. I added the migration note and the rollback steps.",
      date: "Oct 1",
      folder: Folder::Sent,
      unread: false,
    },
  ]
}

fn initials(name: &str) -> String {
  name.split_whitespace().filter_map(|word| word.chars().next()).take(2).collect()
}

/// A mail screen: folders with unread counts, a searchable message list, and
/// a reading pane, the last two in resizable panels. Opening a message marks
/// it read and Archive moves it out of the inbox. `on_reply` hears Reply with
/// the message id; replace `sample_mail` with your own data.
#[component]
pub fn InboxBlock(#[props(default)] on_reply: Option<EventHandler<u32>>) -> Element {
  let mut mail = use_signal(sample_mail);
  let mut folder = use_signal(|| Folder::Inbox);
  let mut query = use_signal(String::new);
  let mut open = use_signal(|| Some(1_u32));
  let mut panels = use_signal(|| {
    (ResizablePanelState::new(40.0, 25.0, 60.0), ResizablePanelState::new(60.0, 40.0, 75.0))
  });

  let needle = query().to_lowercase();
  let visible: Vec<Mail> = mail()
    .into_iter()
    .filter(|message| message.folder == folder())
    .filter(|message| {
      needle.is_empty()
        || message.subject.to_lowercase().contains(&needle)
        || message.from.to_lowercase().contains(&needle)
        || message.body.to_lowercase().contains(&needle)
    })
    .collect();
  let opened = open().and_then(|id| mail().into_iter().find(|message| message.id == id));

  rsx! {
    div { class: "flex h-screen min-h-[32rem] flex-col bg-background text-foreground md:flex-row",
      nav {
        "aria-label": "Folders",
        class: "flex gap-1 border-b p-2 md:w-48 md:flex-col md:border-e md:border-b-0",
        for entry in Folder::ALL {
          {
            let unread = mail().iter().filter(|message| message.folder == entry && message.unread).count();
            rsx! {
              Button {
                key: "{entry.label()}",
                class: "justify-between md:w-full",
                variant: if folder() == entry { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
                size: ButtonSize::Sm,
                "aria-current": (folder() == entry).then_some("page"),
                onclick: move |_| {
                  folder.set(entry);
                  open.set(None);
                },
                "{entry.label()}"
                if unread > 0 {
                  Badge { variant: BadgeVariant::Secondary, "{unread}" }
                }
              }
            }
          }
        }
      }
      ResizablePanelGroup { class: "min-h-0 flex-1",
        ResizablePanel { id: "inbox-list", class: "flex min-w-0 flex-col", size: panels().0.size,
          div { class: "grid gap-3 p-3",
            h1 { class: "text-lg font-semibold", "{folder().label()}" }
            Input {
              r#type: "search",
              placeholder: "Search mail",
              "aria-label": "Search mail",
              value: query(),
              on_value_change: move |value| query.set(value),
            }
          }
          Separator { decorative: true }
          ScrollArea { class: "min-h-0 flex-1",
            ScrollAreaViewport { class: "h-full", tabindex: "-1",
              if visible.is_empty() {
                p { class: "p-6 text-center text-sm text-muted-foreground", "No messages." }
              }
              ul { "aria-label": "Messages", class: "grid gap-1 p-2",
                for message in visible {
                  li { key: "{message.id}",
                    button {
                      class: "block w-full rounded-md text-left focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring",
                      "aria-current": (open() == Some(message.id)).then_some("true"),
                      onclick: move |_| {
                        open.set(Some(message.id));
                        mail.with_mut(|all| {
                          if let Some(found) = all.iter_mut().find(|m| m.id == message.id) {
                            found.unread = false;
                          }
                        });
                      },
                      Item { selected: open() == Some(message.id),
                        ItemContent {
                          div { class: "flex items-center justify-between gap-2",
                            ItemTitle { class: if message.unread { "font-semibold" } else { "font-normal" },
                              "{message.from}"
                            }
                            span { class: "shrink-0 text-xs text-muted-foreground", "{message.date}" }
                          }
                          p { class: "truncate text-sm",
                            if message.unread {
                              span { class: "sr-only", "Unread: " }
                            }
                            "{message.subject}"
                          }
                          ItemDescription { "{message.body}" }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
        ResizableHandle {
          "aria-controls": "inbox-list",
          "aria-label": "Resize message list",
          value: panels().0.size,
          min: 25.0,
          max: 60.0,
          on_resize: move |delta| {
            let (first, second) = panels();
            panels.set(resizable_resize_pair(first, second, delta));
          },
        }
        ResizablePanel { class: "min-w-0", size: panels().1.size,
          section { "aria-label": "Reading pane", class: "flex h-full flex-col",
            if let Some(message) = opened {
              div { class: "flex flex-wrap items-center gap-2 p-3",
                Button {
                  size: ButtonSize::Sm,
                  variant: ButtonVariant::Outline,
                  onclick: move |_| {
                    if let Some(handler) = on_reply {
                      handler.call(message.id);
                    }
                  },
                  "Reply"
                }
                if message.folder != Folder::Archive {
                  Button {
                    size: ButtonSize::Sm,
                    variant: ButtonVariant::Ghost,
                    onclick: move |_| {
                      mail.with_mut(|all| {
                        if let Some(found) = all.iter_mut().find(|m| m.id == message.id) {
                          found.folder = Folder::Archive;
                        }
                      });
                      open.set(None);
                    },
                    "Archive"
                  }
                }
              }
              Separator { decorative: true }
              article { class: "grid gap-4 overflow-y-auto p-6",
                div { class: "flex items-start gap-3",
                  Avatar {
                    AvatarFallback { "{initials(message.from)}" }
                  }
                  div { class: "grid gap-0.5",
                    h2 { class: "text-lg font-semibold", "{message.subject}" }
                    p { class: "text-sm text-muted-foreground", "{message.from} \u{b7} {message.date}" }
                  }
                }
                p { class: "leading-relaxed", "{message.body}" }
              }
            } else {
              p { class: "m-auto p-6 text-sm text-muted-foreground", "Select a message to read it." }
            }
          }
        }
      }
    }
  }
}
