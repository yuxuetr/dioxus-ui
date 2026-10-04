use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  ToastDismissReason, ToastItem, ToastPlacement, ToastQueue, ToastVariant,
  toast_dismiss_reason_attribute, toast_is_expired, toast_placement_attribute, toast_queue_dismiss,
  toast_queue_limit, toast_queue_push, toast_variant_attribute,
};

use crate::dismiss_timer::use_dismiss_timer;

pub const TOAST_VIEWPORT_BASE_CLASS: &str =
  "fixed z-50 flex max-h-screen w-full flex-col gap-2 p-4 sm:max-w-sm";
pub const TOAST_ROOT_BASE_CLASS: &str = "pointer-events-auto relative grid w-full gap-1 overflow-hidden rounded-md border bg-white p-4 pr-10 text-zinc-950 shadow-lg transition-all data-state-closed:opacity-0 data-state-open:opacity-100";
pub const TOAST_TITLE_BASE_CLASS: &str = "text-sm font-semibold leading-none tracking-normal";
pub const TOAST_DESCRIPTION_BASE_CLASS: &str = "text-sm text-zinc-600";
pub const TOAST_ACTION_BASE_CLASS: &str = "inline-flex h-8 shrink-0 items-center justify-center rounded-md border border-zinc-200 bg-transparent px-3 text-sm font-medium transition-colors hover:bg-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";
pub const TOAST_CLOSE_BASE_CLASS: &str = "absolute right-2 top-2 inline-flex h-7 w-7 items-center justify-center rounded-md text-zinc-500 transition-colors hover:bg-zinc-100 hover:text-zinc-950 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";

pub fn toast_viewport_class(placement: ToastPlacement, class: &str) -> String {
  let placement_class = match placement {
    ToastPlacement::TopLeft => "left-0 top-0 sm:left-0",
    ToastPlacement::TopCenter => "top-0 sm:left-1/2 sm:-translate-x-1/2",
    ToastPlacement::TopRight => "right-0 top-0 sm:right-0",
    ToastPlacement::BottomLeft => "bottom-0 left-0 sm:left-0",
    ToastPlacement::BottomCenter => "bottom-0 sm:left-1/2 sm:-translate-x-1/2",
    ToastPlacement::BottomRight => "bottom-0 right-0 sm:right-0",
  };

  classes([Some(TOAST_VIEWPORT_BASE_CLASS), Some(placement_class), Some(class)])
}

pub fn toast_root_class(variant: ToastVariant, class: &str) -> String {
  let variant_class = match variant {
    ToastVariant::Default => "border-zinc-200",
    ToastVariant::Success => "border-green-200 text-green-950",
    ToastVariant::Info => "border-blue-200 text-blue-950",
    ToastVariant::Warning => "border-amber-200 text-amber-950",
    ToastVariant::Error => "border-red-200 text-red-950",
    ToastVariant::Loading => "border-zinc-200 text-zinc-950",
  };

  classes([Some(TOAST_ROOT_BASE_CLASS), Some(variant_class), Some(class)])
}

pub fn toast_title_class(class: &str) -> String {
  classes([Some(TOAST_TITLE_BASE_CLASS), Some(class)])
}

pub fn toast_description_class(variant: ToastVariant, class: &str) -> String {
  let variant_class = match variant {
    ToastVariant::Default | ToastVariant::Loading => "text-zinc-600",
    ToastVariant::Success => "text-green-800",
    ToastVariant::Info => "text-blue-800",
    ToastVariant::Warning => "text-amber-800",
    ToastVariant::Error => "text-red-800",
  };

  classes([Some(TOAST_DESCRIPTION_BASE_CLASS), Some(variant_class), Some(class)])
}

pub fn toast_action_class(disabled: bool, class: &str) -> String {
  classes([
    Some(TOAST_ACTION_BASE_CLASS),
    disabled.then_some("pointer-events-none opacity-50"),
    Some(class),
  ])
}

pub fn toast_close_class(disabled: bool, class: &str) -> String {
  classes([
    Some(TOAST_CLOSE_BASE_CLASS),
    disabled.then_some("pointer-events-none opacity-50"),
    Some(class),
  ])
}

pub fn toast_live_attribute(variant: ToastVariant) -> &'static str {
  match variant {
    ToastVariant::Error | ToastVariant::Warning => "assertive",
    ToastVariant::Default | ToastVariant::Success | ToastVariant::Info | ToastVariant::Loading => {
      "polite"
    }
  }
}

#[component]
pub fn ToastViewport(
  #[props(default)] placement: ToastPlacement,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = toast_viewport_class(placement, &class);

  rsx! {
    div {
      class,
      "data-placement": toast_placement_attribute(placement),
      {children}
    }
  }
}

/// Calls `on_dismiss(Timeout)` after `duration_ms` of open time, pausing while
/// the pointer is over the toast or focus is inside it. `0` disables the timer.
#[component]
pub fn ToastRoot(
  #[props(default)] variant: ToastVariant,
  #[props(default = true)] open: bool,
  #[props(default)] class: String,
  #[props(default = 5000)] duration_ms: u64,
  #[props(default)] on_dismiss: Option<EventHandler<ToastDismissReason>>,
  children: Element,
) -> Element {
  let class = toast_root_class(variant, &class);
  let dismiss_timer = use_dismiss_timer(open, duration_ms, on_dismiss, ToastDismissReason::Timeout);

  rsx! {
    div {
      role: "status",
      class,
      hidden: !open,
      "aria-live": toast_live_attribute(variant),
      "data-state": if open { "open" } else { "closed" },
      "data-variant": toast_variant_attribute(variant),
      "data-dxui-dismiss-timer": dismiss_timer,
      {children}
    }
  }
}

#[component]
pub fn ToastTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = toast_title_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn ToastDescription(
  #[props(default)] variant: ToastVariant,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = toast_description_class(variant, &class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn ToastAction(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] on_dismiss: Option<EventHandler<ToastDismissReason>>,
  children: Element,
) -> Element {
  let class = toast_action_class(disabled, &class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      onclick: move |event| {
        if let Some(handler) = onclick {
          handler.call(event);
        }
        if let Some(handler) = on_dismiss {
          handler.call(ToastDismissReason::Action);
        }
      },
      {children}
    }
  }
}

#[component]
pub fn ToastClose(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] on_dismiss: Option<EventHandler<ToastDismissReason>>,
  children: Element,
) -> Element {
  let class = toast_close_class(disabled, &class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-label": "Close notification",
      onclick: move |_| {
        if let Some(handler) = on_dismiss {
          handler.call(ToastDismissReason::Close);
        }
      },
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn toast_viewport_class_reflects_placement() {
    let actual = toast_viewport_class(ToastPlacement::TopCenter, "max-w-md");

    assert!(actual.contains(TOAST_VIEWPORT_BASE_CLASS));
    assert!(actual.contains("top-0 sm:left-1/2 sm:-translate-x-1/2"));
    assert!(actual.ends_with("max-w-md"));
  }

  #[test]
  fn toast_root_and_description_classes_reflect_variant() {
    let root = toast_root_class(ToastVariant::Error, "border-2");
    let description = toast_description_class(ToastVariant::Error, "");

    assert!(root.contains(TOAST_ROOT_BASE_CLASS));
    assert!(root.contains("border-red-200 text-red-950"));
    assert!(description.contains("text-red-800"));
  }

  #[test]
  fn toast_primitives_are_reexported() {
    let queue =
      ToastQueue::new(1).push(ToastItem::new("one", "One")).push(ToastItem::new("two", "Two"));

    assert_eq!(queue.items[0].id, "two");
    assert_eq!(toast_variant_attribute(ToastVariant::Success), "success");
    assert!(toast_is_expired(5000, 5000));
  }
}
