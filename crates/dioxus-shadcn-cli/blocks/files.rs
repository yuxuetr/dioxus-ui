use dioxus::html::HasFileData;
use dioxus::prelude::*;

use crate::components::ui::breadcrumb::{
  Breadcrumb, BreadcrumbItem, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::components::ui::context_menu::{
  ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
};
use crate::components::ui::data_table::{
  DataTable, DataTableCell, DataTableContainer, DataTableHeaderCell, DataTableRow,
};
use crate::components::ui::tree::{Tree, TreeItem};

struct Folder {
  id: &'static str,
  name: &'static str,
  parent: Option<&'static str>,
}

const FOLDERS: &[Folder] = &[
  Folder { id: "home", name: "My files", parent: None },
  Folder { id: "design", name: "Design", parent: Some("home") },
  Folder { id: "exports", name: "Exports", parent: Some("design") },
  Folder { id: "docs", name: "Docs", parent: Some("home") },
  Folder { id: "shared", name: "Shared", parent: None },
];

/// A file in a folder; the app stores the bytes.
#[derive(Clone, Debug, PartialEq)]
pub struct FileEntry {
  pub name: String,
  pub folder: String,
  pub size: u64,
  pub modified: String,
}

fn file(name: &str, folder: &str, size: u64, modified: &str) -> FileEntry {
  FileEntry {
    name: name.to_string(),
    folder: folder.to_string(),
    size,
    modified: modified.to_string(),
  }
}

fn sample_files() -> Vec<FileEntry> {
  vec![
    file("Roadmap.pdf", "home", 482_113, "Oct 8"),
    file("Budget.xlsx", "home", 96_204, "Oct 6"),
    file("Logo.svg", "design", 7_310, "Oct 2"),
    file("Mockups.fig", "design", 12_882_001, "Sep 30"),
    file("Hero.png", "exports", 1_204_550, "Sep 29"),
    file("Onboarding.md", "docs", 5_118, "Sep 21"),
    file("Contract.pdf", "shared", 233_870, "Sep 12"),
  ]
}

fn folder_name(id: &str) -> &'static str {
  FOLDERS.iter().find(|folder| folder.id == id).map_or("", |folder| folder.name)
}

/// The folder and its ancestors, outermost first.
fn folder_path(id: &str) -> Vec<&'static Folder> {
  let mut path = Vec::new();
  let mut current = FOLDERS.iter().find(|folder| folder.id == id);
  while let Some(folder) = current {
    path.insert(0, folder);
    current = folder.parent.and_then(|parent| FOLDERS.iter().find(|folder| folder.id == parent));
  }
  path
}

fn format_size(bytes: u64) -> String {
  match bytes {
    0..1_024 => format!("{bytes} B"),
    1_024..1_048_576 => format!("{:.1} KB", bytes as f64 / 1_024.0),
    _ => format!("{:.1} MB", bytes as f64 / 1_048_576.0),
  }
}

#[component]
fn FolderNode(id: &'static str) -> Element {
  let children: Vec<&'static Folder> =
    FOLDERS.iter().filter(|folder| folder.parent == Some(id)).collect();
  let name = folder_name(id);
  if children.is_empty() {
    return rsx! {
      TreeItem { value: id, "{name}" }
    };
  }
  rsx! {
    TreeItem {
      value: id,
      group: rsx! {
        for child in children {
          FolderNode { key: "{child.id}", id: child.id }
        }
      },
      "{name}"
    }
  }
}

/// A file manager: a folder tree, breadcrumbs, an upload area that takes
/// dropped or picked files, and the open folder's files in a table whose
/// context menu deletes the row it was opened on. `on_upload` hears the
/// files added to a folder and `on_delete` a deleted file; replace `FOLDERS`
/// and `sample_files` with your data.
#[component]
pub fn FilesBlock(
  #[props(default)] on_upload: Option<EventHandler<Vec<FileEntry>>>,
  #[props(default)] on_delete: Option<EventHandler<FileEntry>>,
) -> Element {
  let mut files = use_signal(sample_files);
  let mut folder = use_signal(|| "home".to_string());
  let mut expanded = use_signal(|| vec!["home".to_string()]);
  let mut dragging = use_signal(|| false);
  let mut menu_target = use_signal(|| None::<String>);

  let mut open_folder = move |id: &str| {
    // Opening a folder from the breadcrumbs shows it in the tree as well.
    let ancestors: Vec<String> =
      folder_path(id).iter().map(|folder| folder.id.to_string()).collect();
    expanded.with_mut(|open| {
      for ancestor in &ancestors[..ancestors.len().saturating_sub(1)] {
        if !open.contains(ancestor) {
          open.push(ancestor.clone());
        }
      }
    });
    folder.set(id.to_string());
  };
  let mut upload = move |names: Vec<(String, u64)>| {
    if names.is_empty() {
      return;
    }
    let added: Vec<FileEntry> =
      names.into_iter().map(|(name, size)| file(&name, &folder(), size, "Just now")).collect();
    files.with_mut(|all| all.extend(added.clone()));
    if let Some(handler) = on_upload {
      handler.call(added);
    }
  };

  let path = folder_path(&folder());
  let shown: Vec<FileEntry> =
    files().into_iter().filter(|entry| entry.folder == folder()).collect();

  rsx! {
    div { class: "flex min-h-[32rem] flex-col bg-background text-foreground md:flex-row",
      nav { "aria-label": "Folders", class: "border-b p-2 md:w-60 md:shrink-0 md:border-e md:border-b-0",
        Tree {
          "aria-label": "Folders",
          expanded: expanded(),
          on_expanded_change: move |open: Vec<String>| expanded.set(open),
          selected: folder(),
          on_selected_change: move |id: String| folder.set(id),
          for root in FOLDERS.iter().filter(|folder| folder.parent.is_none()) {
            FolderNode { key: "{root.id}", id: root.id }
          }
        }
      }
      section { "aria-label": "Folder contents", class: "grid min-w-0 flex-1 content-start gap-4 p-4",
        div { class: "flex flex-wrap items-center justify-between gap-3",
          Breadcrumb {
            BreadcrumbList {
              for (index, step) in path.iter().enumerate() {
                BreadcrumbItem { key: "{step.id}",
                  if index + 1 == path.len() {
                    BreadcrumbPage { "{step.name}" }
                  } else {
                    button {
                      class: "transition-colors hover:text-foreground",
                      onclick: {
                        let id = step.id;
                        move |_| open_folder(id)
                      },
                      "{step.name}"
                    }
                  }
                }
                if index + 1 < path.len() {
                  BreadcrumbSeparator {}
                }
              }
            }
          }
          label {
            class: "inline-flex h-9 cursor-pointer items-center rounded-md border border-input px-3 text-sm font-medium hover:bg-accent focus-within:ring-2 focus-within:ring-ring",
            input {
              class: "sr-only",
              r#type: "file",
              multiple: true,
              onchange: move |event| upload(event.files().iter().map(|file| (file.name(), file.size())).collect()),
            }
            "Upload"
          }
        }
        div {
          class: if dragging() { "rounded-md border-2 border-dashed border-primary bg-accent/40 p-4 text-center text-sm" } else { "rounded-md border-2 border-dashed border-border p-4 text-center text-sm text-muted-foreground" },
          role: "region",
          "aria-label": "Upload area",
          ondragover: move |event| {
            event.prevent_default();
            dragging.set(true);
          },
          ondragleave: move |_| dragging.set(false),
          ondrop: move |event| {
            event.prevent_default();
            dragging.set(false);
            upload(event.files().iter().map(|file| (file.name(), file.size())).collect());
          },
          "Drop files here to add them to {folder_name(&folder())}"
        }
        ContextMenu {
          ContextMenuTrigger { class: "min-w-0",
            DataTable {
              DataTableContainer {
                table { class: "w-full text-sm",
                  caption { class: "sr-only", "Files in {folder_name(&folder())}" }
                  thead {
                    tr {
                      DataTableHeaderCell { "Name" }
                      DataTableHeaderCell { "Size" }
                      DataTableHeaderCell { "Modified" }
                    }
                  }
                  tbody {
                    if shown.is_empty() {
                      tr {
                        td { class: "p-6 text-center text-muted-foreground", colspan: "3", "This folder is empty." }
                      }
                    }
                    for entry in shown {
                      DataTableRow { key: "{entry.name}",
                        DataTableCell { class: "font-medium",
                          // DataTableCell passes no handlers, so the name reports the right-click.
                          span {
                            class: "block",
                            oncontextmenu: {
                              let name = entry.name.clone();
                              move |_| menu_target.set(Some(name.clone()))
                            },
                            "{entry.name}"
                          }
                        }
                        DataTableCell { "{format_size(entry.size)}" }
                        DataTableCell { "{entry.modified}" }
                      }
                    }
                  }
                }
              }
            }
          }
          ContextMenuContent {
            if let Some(name) = menu_target() {
              ContextMenuItem {
                destructive: true,
                onclick: {
                  let name = name.clone();
                  move |_| {
                  let current = folder();
                  let mut removed = None;
                  files.with_mut(|all| {
                    if let Some(index) = all.iter().position(|entry| entry.name == name && entry.folder == current) {
                      removed = Some(all.remove(index));
                    }
                  });
                  if let (Some(handler), Some(entry)) = (on_delete, removed) {
                    handler.call(entry);
                  }
                  menu_target.set(None);
                  }
                },
                "Delete {name}"
              }
            } else {
              ContextMenuItem { disabled: true, "Right-click a file name" }
            }
          }
        }
      }
    }
  }
}
