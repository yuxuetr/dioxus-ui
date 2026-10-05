use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  ToastDismissReason as SonnerDismissReason, ToastItem as SonnerItem,
  ToastPlacement as SonnerPlacement, ToastQueue as SonnerQueue, ToastVariant as SonnerVariant,
  toast_dismiss_reason_attribute as sonner_dismiss_reason_attribute,
  toast_is_expired as sonner_is_expired, toast_placement_attribute as sonner_placement_attribute,
  toast_queue_dismiss as sonner_queue_dismiss, toast_queue_limit as sonner_queue_limit,
  toast_queue_push as sonner_queue_push, toast_variant_attribute as sonner_variant_attribute,
};

use crate::dismiss_timer::use_dismiss_timer;

pub const SONNER_VIEWPORT_BASE_CLASS: &str =
  "fixed z-50 flex max-h-screen w-full flex-col gap-2 p-4 sm:max-w-sm";
pub const SONNER_TOAST_BASE_CLASS: &str = "pointer-events-auto relative grid w-full grid-cols-[auto_1fr_auto] items-start gap-3 overflow-hidden rounded-md border p-4 shadow-lg transition-all";
pub const SONNER_ICON_BASE_CLASS: &str = "mt-0.5 h-2.5 w-2.5 rounded-full";
pub const SONNER_CONTENT_BASE_CLASS: &str = "grid gap-1";
pub const SONNER_TITLE_BASE_CLASS: &str = "text-sm font-semibold leading-none tracking-normal";
pub const SONNER_DESCRIPTION_BASE_CLASS: &str = "text-sm";
pub const SONNER_ACTION_BASE_CLASS: &str = "inline-flex h-8 shrink-0 items-center justify-center rounded-md border border-border bg-transparent px-3 text-sm font-medium transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
pub const SONNER_CLOSE_BASE_CLASS: &str = "inline-flex h-7 w-7 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

pub fn sonner_viewport_class(placement: SonnerPlacement, class: &str) -> String {
  let placement_class = match placement {
    SonnerPlacement::TopLeft => "left-0 top-0 sm:left-0",
    SonnerPlacement::TopCenter => "top-0 sm:left-1/2 sm:-translate-x-1/2",
    SonnerPlacement::TopRight => "right-0 top-0 sm:right-0",
    SonnerPlacement::BottomLeft => "bottom-0 left-0 sm:left-0",
    SonnerPlacement::BottomCenter => "bottom-0 sm:left-1/2 sm:-translate-x-1/2",
    SonnerPlacement::BottomRight => "bottom-0 right-0 sm:right-0",
  };

  classes([Some(SONNER_VIEWPORT_BASE_CLASS), Some(placement_class), Some(class)])
}

pub fn sonner_toast_class(variant: SonnerVariant, class: &str) -> String {
  let variant_class = match variant {
    SonnerVariant::Default => "border-border bg-popover text-popover-foreground",
    SonnerVariant::Success => "border-success/50 bg-popover text-popover-foreground",
    SonnerVariant::Info => "border-info/50 bg-popover text-popover-foreground",
    SonnerVariant::Warning => "border-warning/50 bg-popover text-popover-foreground",
    SonnerVariant::Error => "border-destructive/50 bg-popover text-popover-foreground",
    SonnerVariant::Loading => "border-border bg-popover text-popover-foreground",
  };

  classes([Some(SONNER_TOAST_BASE_CLASS), Some(variant_class), Some(class)])
}

pub fn sonner_icon_class(variant: SonnerVariant, class: &str) -> String {
  let variant_class = match variant {
    SonnerVariant::Default => "bg-muted-foreground",
    SonnerVariant::Success => "bg-success",
    SonnerVariant::Info => "bg-info",
    SonnerVariant::Warning => "bg-warning",
    SonnerVariant::Error => "bg-destructive",
    SonnerVariant::Loading => "bg-muted-foreground animate-pulse",
  };

  classes([Some(SONNER_ICON_BASE_CLASS), Some(variant_class), Some(class)])
}

pub fn sonner_content_class(class: &str) -> String {
  classes([Some(SONNER_CONTENT_BASE_CLASS), Some(class)])
}

pub fn sonner_title_class(class: &str) -> String {
  classes([Some(SONNER_TITLE_BASE_CLASS), Some(class)])
}

pub fn sonner_description_class(variant: SonnerVariant, class: &str) -> String {
  let variant_class = match variant {
    SonnerVariant::Default | SonnerVariant::Loading => "text-muted-foreground",
    SonnerVariant::Success => "text-muted-foreground",
    SonnerVariant::Info => "text-muted-foreground",
    SonnerVariant::Warning => "text-muted-foreground",
    SonnerVariant::Error => "text-muted-foreground",
  };

  classes([Some(SONNER_DESCRIPTION_BASE_CLASS), Some(variant_class), Some(class)])
}

pub fn sonner_action_class(disabled: bool, class: &str) -> String {
  classes([
    Some(SONNER_ACTION_BASE_CLASS),
    disabled.then_some("pointer-events-none opacity-50"),
    Some(class),
  ])
}

pub fn sonner_close_class(disabled: bool, class: &str) -> String {
  classes([
    Some(SONNER_CLOSE_BASE_CLASS),
    disabled.then_some("pointer-events-none opacity-50"),
    Some(class),
  ])
}

pub fn sonner_live_attribute(variant: SonnerVariant) -> &'static str {
  match variant {
    SonnerVariant::Error | SonnerVariant::Warning => "assertive",
    SonnerVariant::Default
    | SonnerVariant::Success
    | SonnerVariant::Info
    | SonnerVariant::Loading => "polite",
  }
}

/// Persistent live region; render it once and add toasts inside it so
/// assistive technology announces them.
#[component]
pub fn SonnerViewport(
  #[props(default)] placement: SonnerPlacement,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = sonner_viewport_class(placement, &class);

  rsx! {
    div {
      role: "region",
      class,
      "aria-label": "Notifications",
      "aria-live": "polite",
      "data-placement": sonner_placement_attribute(placement),
      {children}
    }
  }
}

/// Calls `on_dismiss(Timeout)` after `duration_ms` of open time, pausing while
/// the pointer is over the toast or focus is inside it. `0` disables the timer.
#[component]
pub fn SonnerToast(
  #[props(default)] variant: SonnerVariant,
  #[props(default = true)] open: bool,
  #[props(default)] class: String,
  #[props(default = 5000)] duration_ms: u64,
  #[props(default)] on_dismiss: Option<EventHandler<SonnerDismissReason>>,
  children: Element,
) -> Element {
  let class = sonner_toast_class(variant, &class);
  let dismiss_timer =
    use_dismiss_timer(open, duration_ms, on_dismiss, SonnerDismissReason::Timeout);

  rsx! {
    div {
      role: "status",
      class,
      hidden: !open,
      "aria-live": sonner_live_attribute(variant),
      "data-state": if open { "open" } else { "closed" },
      "data-variant": sonner_variant_attribute(variant),
      "data-dxui-dismiss-timer": dismiss_timer,
      {children}
    }
  }
}

#[component]
pub fn SonnerIcon(
  #[props(default)] variant: SonnerVariant,
  #[props(default)] class: String,
) -> Element {
  let class = sonner_icon_class(variant, &class);

  rsx! {
    span {
      class,
      "aria-hidden": "true",
    }
  }
}

#[component]
pub fn SonnerContent(#[props(default)] class: String, children: Element) -> Element {
  let class = sonner_content_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn SonnerTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = sonner_title_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn SonnerDescription(
  #[props(default)] variant: SonnerVariant,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = sonner_description_class(variant, &class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn SonnerAction(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] on_dismiss: Option<EventHandler<SonnerDismissReason>>,
  children: Element,
) -> Element {
  let class = sonner_action_class(disabled, &class);

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
          handler.call(SonnerDismissReason::Action);
        }
      },
      {children}
    }
  }
}

#[component]
pub fn SonnerClose(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] on_dismiss: Option<EventHandler<SonnerDismissReason>>,
  children: Element,
) -> Element {
  let class = sonner_close_class(disabled, &class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-label": "Close notification",
      onclick: move |_| {
        if let Some(handler) = on_dismiss {
          handler.call(SonnerDismissReason::Close);
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
  fn sonner_toast_class_reflects_variant() {
    let actual = sonner_toast_class(SonnerVariant::Success, "shadow-none");

    assert!(actual.contains(SONNER_TOAST_BASE_CLASS));
    assert!(actual.contains("border-success/50 bg-popover text-popover-foreground"));
    assert!(actual.ends_with("shadow-none"));
  }

  #[test]
  fn sonner_icon_and_description_reflect_variant() {
    let icon = sonner_icon_class(SonnerVariant::Loading, "");
    let description = sonner_description_class(SonnerVariant::Error, "");

    assert!(icon.contains("bg-muted-foreground animate-pulse"));
    assert!(description.contains("text-muted-foreground"));
  }

  #[test]
  fn sonner_primitives_are_reexported() {
    let queue =
      SonnerQueue::new(1).push(SonnerItem::new("one", "One")).push(SonnerItem::new("two", "Two"));

    assert_eq!(queue.items[0].id, "two");
    assert_eq!(sonner_variant_attribute(SonnerVariant::Info), "info");
    assert!(sonner_is_expired(5000, 5000));
  }
}
