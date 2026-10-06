use dioxus::prelude::*;
use dioxus_shadcn::{Swap, SwapEffect};

#[component]
fn Icon(path: &'static str) -> Element {
  rsx! {
    svg {
      class: "size-5",
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
pub fn SwapIconsDemo() -> Element {
  let mut menu = use_signal(|| false);
  let mut liked = use_signal(|| false);
  let mut sound = use_signal(|| true);

  rsx! {
    div { class: "flex items-center gap-4",
      Swap {
        class: "size-10 border border-border hover:bg-accent",
        "aria-label": "Menu",
        effect: SwapEffect::Rotate,
        active: menu(),
        on_active_change: move |active| menu.set(active),
        on: rsx! { Icon { path: "M6 6l12 12M18 6L6 18" } },
        off: rsx! { Icon { path: "M4 7h16M4 12h16M4 17h16" } },
      }
      Swap {
        class: "size-10 border border-border hover:bg-accent",
        "aria-label": "Like",
        active: liked(),
        on_active_change: move |active| liked.set(active),
        on: rsx! { span { class: "text-destructive", Icon { path: "M12 20s-7-4.5-7-10a4 4 0 0 1 7-2.6A4 4 0 0 1 19 10c0 5.5-7 10-7 10z" } } },
        off: rsx! { Icon { path: "M12 20s-7-4.5-7-10a4 4 0 0 1 7-2.6A4 4 0 0 1 19 10c0 5.5-7 10-7 10z" } },
      }
      Swap {
        class: "h-10 border border-border px-3 text-sm hover:bg-accent",
        effect: SwapEffect::Flip,
        active: sound(),
        on_active_change: move |active| sound.set(active),
        on: rsx! { "Sound on" },
        off: rsx! { "Muted" },
      }
    }
  }
}
