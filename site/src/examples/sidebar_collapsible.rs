use dioxus::prelude::*;
use dioxus_shadcn::{
  Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel, SidebarHeader,
  SidebarItem, SidebarProvider, SidebarTrigger,
};

#[component]
pub fn SidebarCollapsibleDemo() -> Element {
  let mut mobile_open = use_signal(|| false);
  let mut section = use_signal(|| "inbox");

  rsx! {
    // The provider owns collapsed; the app controls the off-canvas panel so
    // that choosing a folder closes it.
    SidebarProvider {
      off_canvas: true,
      mobile_open: mobile_open(),
      on_mobile_open_change: move |next| mobile_open.set(next),
      div { class: "flex h-72 overflow-hidden rounded-md border border-border",
        Sidebar { "aria-label": "Mail", shortcut: 'b',
          SidebarHeader { span { class: "text-sm font-semibold", "Acme Mail" } }
          SidebarContent {
            SidebarGroup { role: "group", "aria-labelledby": "sidebar-collapsible-label",
              SidebarGroupLabel { id: "sidebar-collapsible-label", "Folders" }
              for (value, label) in [("inbox", "Inbox"), ("drafts", "Drafts"), ("sent", "Sent")] {
                SidebarItem {
                  key: "{value}",
                  active: section() == value,
                  onclick: move |_| {
                    section.set(value);
                    mobile_open.set(false);
                  },
                  "{label}"
                }
              }
              SidebarItem { disabled: true, "Archive" }
            }
          }
          SidebarFooter { span { class: "text-xs text-muted-foreground", "Signed in as ada" } }
        }
        div { class: "flex-1 p-4",
          SidebarTrigger { "aria-label": "Toggle sidebar", "☰" }
          p { class: "mt-3 text-sm", "Showing {section}" }
          p { class: "mt-1 text-xs text-muted-foreground", "Ctrl or ⌘ + B toggles the sidebar." }
        }
      }
    }
  }
}
