use dioxus::prelude::*;
use dioxus_shadcn::{Fab, FabAction};

#[component]
fn Icon(path: &'static str) -> Element {
  rsx! {
    svg {
      view_box: "0 0 24 24",
      fill: "none",
      stroke: "currentColor",
      stroke_width: "2",
      stroke_linecap: "round",
      stroke_linejoin: "round",
      path { d: path }
    }
  }
}

#[component]
pub fn FabSpeedDialDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut last = use_signal(|| "Nothing yet".to_string());
  let mut choose = move |action: &str| {
    last.set(format!("Created a {action}"));
    open.set(false);
  };

  let trigger_icon = if open() { "M6 6l12 12M18 6L6 18" } else { "M12 5v14M5 12h14" };

  rsx! {
    // A phone-sized frame; in an app the Fab is fixed to the viewport.
    div { class: "relative mx-auto flex h-80 w-72 flex-col justify-between overflow-hidden rounded-xl border border-border p-4",
      p { class: "text-sm text-muted-foreground", "{last}" }
      Fab {
        class: "self-end",
        fixed: false,
        "aria-label": "Create",
        icon: rsx! { Icon { path: trigger_icon } },
        open: open(),
        on_open_change: move |next| open.set(next),
        FabAction { label: "Photo", onclick: move |_| choose("photo"),
          Icon { path: "M4 8h3l2-3h6l2 3h3v11H4zM12 17a4 4 0 1 0 0-8 4 4 0 0 0 0 8z" }
        }
        FabAction { label: "Note", onclick: move |_| choose("note"),
          Icon { path: "M5 4h10l4 4v12H5zM9 12h6M9 16h6" }
        }
        FabAction { label: "Event", onclick: move |_| choose("event"),
          Icon { path: "M4 6h16v14H4zM4 10h16M8 3v4M16 3v4" }
        }
      }
    }
  }
}
