use dioxus::prelude::*;
use dioxus_shadcn::{
  Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel, SidebarHeader,
  SidebarItem, SidebarTrigger,
};

#[component]
pub fn Demo() -> Element {
  let mut collapsed = use_signal(|| false);
  let mut section = use_signal(|| "inbox");

  rsx! {
    div { class: "flex h-72 overflow-hidden rounded-md border border-border",
      Sidebar { id: "sidebar-collapsible", "aria-label": "Mail", collapsed: collapsed(),
        SidebarHeader { span { class: "text-sm font-semibold", "Acme Mail" } }
        SidebarContent {
          SidebarGroup { role: "group", "aria-labelledby": "sidebar-collapsible-label",
            SidebarGroupLabel { id: "sidebar-collapsible-label", "Folders" }
            for (value, label) in [("inbox", "Inbox"), ("drafts", "Drafts"), ("sent", "Sent")] {
              SidebarItem { key: "{value}", active: section() == value, onclick: move |_| section.set(value), "{label}" }
            }
            SidebarItem { disabled: true, "Archive" }
          }
        }
        SidebarFooter { span { class: "text-xs text-muted-foreground", "Signed in as ada" } }
      }
      div { class: "flex-1 p-4",
        SidebarTrigger {
          "aria-controls": "sidebar-collapsible",
          collapsed: collapsed(),
          on_collapsed_change: move |next| collapsed.set(next),
          "☰"
        }
        p { class: "mt-3 text-sm", "Showing {section}" }
      }
    }
  }
}
