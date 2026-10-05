use dioxus::prelude::*;
use super::utils::{classes, use_dismiss_timer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SonnerPlacement {
  TopLeft,
  TopCenter,
  TopRight,
  BottomLeft,
  BottomCenter,
  BottomRight,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SonnerVariant {
  Default,
  Success,
  Info,
  Warning,
  Error,
  Loading,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SonnerItem {
  pub id: String,
  pub title: String,
  pub description: Option<String>,
  pub variant: SonnerVariant,
  pub duration_ms: u64,
  pub dismissible: bool,
}

impl SonnerItem {
  pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      title: title.into(),
      description: None,
      variant: SonnerVariant::Default,
      duration_ms: 5000,
      dismissible: true,
    }
  }

  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }

  pub const fn with_variant(mut self, variant: SonnerVariant) -> Self {
    self.variant = variant;
    self
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SonnerQueue {
  pub items: Vec<SonnerItem>,
  pub limit: usize,
}

impl SonnerQueue {
  pub fn new(limit: usize) -> Self {
    Self {
      items: Vec::new(),
      limit,
    }
  }

  pub fn push(self, item: SonnerItem) -> Self {
    sonner_queue_push(self, item)
  }

  pub fn dismiss(self, id: &str) -> Self {
    sonner_queue_dismiss(self, id)
  }
}

pub const SONNER_VIEWPORT_BASE_CLASS: &str = "fixed z-50 flex max-h-screen w-full flex-col gap-2 p-4 sm:max-w-sm";
pub const SONNER_TOAST_BASE_CLASS: &str = "pointer-events-auto relative grid w-full grid-cols-[auto_1fr_auto] items-start gap-3 overflow-hidden rounded-md border p-4 shadow-lg transition-all";
pub const SONNER_ICON_BASE_CLASS: &str = "mt-0.5 h-2.5 w-2.5 rounded-full";
pub const SONNER_CONTENT_BASE_CLASS: &str = "grid gap-1";
pub const SONNER_TITLE_BASE_CLASS: &str = "text-sm font-semibold leading-none tracking-normal";
pub const SONNER_DESCRIPTION_BASE_CLASS: &str = "text-sm";
pub const SONNER_ACTION_BASE_CLASS: &str = "inline-flex h-8 shrink-0 items-center justify-center rounded-md border border-zinc-200 bg-transparent px-3 text-sm font-medium transition-colors hover:bg-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";
pub const SONNER_CLOSE_BASE_CLASS: &str = "inline-flex h-7 w-7 items-center justify-center rounded-md text-zinc-500 transition-colors hover:bg-zinc-100 hover:text-zinc-950 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";

pub fn sonner_placement_attribute(placement: SonnerPlacement) -> &'static str {
  match placement {
    SonnerPlacement::TopLeft => "top-left",
    SonnerPlacement::TopCenter => "top-center",
    SonnerPlacement::TopRight => "top-right",
    SonnerPlacement::BottomLeft => "bottom-left",
    SonnerPlacement::BottomCenter => "bottom-center",
    SonnerPlacement::BottomRight => "bottom-right",
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SonnerDismissReason {
  Action,
  Close,
  Timeout,
  Programmatic,
}

pub fn sonner_dismiss_reason_attribute(reason: SonnerDismissReason) -> &'static str {
  match reason {
    SonnerDismissReason::Action => "action",
    SonnerDismissReason::Close => "close",
    SonnerDismissReason::Timeout => "timeout",
    SonnerDismissReason::Programmatic => "programmatic",
  }
}

pub fn sonner_variant_attribute(variant: SonnerVariant) -> &'static str {
  match variant {
    SonnerVariant::Default => "default",
    SonnerVariant::Success => "success",
    SonnerVariant::Info => "info",
    SonnerVariant::Warning => "warning",
    SonnerVariant::Error => "error",
    SonnerVariant::Loading => "loading",
  }
}

pub fn sonner_queue_push(queue: SonnerQueue, item: SonnerItem) -> SonnerQueue {
  let mut items = queue.items;

  if let Some(index) = items.iter().position(|current| current.id == item.id) {
    items.remove(index);
  }

  items.push(item);

  SonnerQueue {
    items: sonner_queue_limit(items, queue.limit),
    limit: queue.limit,
  }
}

pub fn sonner_queue_dismiss(queue: SonnerQueue, id: &str) -> SonnerQueue {
  SonnerQueue {
    items: queue
      .items
      .into_iter()
      .filter(|item| item.id != id)
      .collect(),
    limit: queue.limit,
  }
}

pub fn sonner_queue_limit(items: Vec<SonnerItem>, limit: usize) -> Vec<SonnerItem> {
  if limit == 0 {
    return Vec::new();
  }

  let len = items.len();

  if len <= limit {
    items
  } else {
    items.into_iter().skip(len - limit).collect()
  }
}

pub const fn sonner_is_expired(elapsed_ms: u64, duration_ms: u64) -> bool {
  duration_ms > 0 && elapsed_ms >= duration_ms
}

pub fn sonner_viewport_class(placement: SonnerPlacement, class: &str) -> String {
  let placement_class = match placement {
    SonnerPlacement::TopLeft => "left-0 top-0 sm:left-0",
    SonnerPlacement::TopCenter => "top-0 sm:left-1/2 sm:-translate-x-1/2",
    SonnerPlacement::TopRight => "right-0 top-0 sm:right-0",
    SonnerPlacement::BottomLeft => "bottom-0 left-0 sm:left-0",
    SonnerPlacement::BottomCenter => "bottom-0 sm:left-1/2 sm:-translate-x-1/2",
    SonnerPlacement::BottomRight => "bottom-0 right-0 sm:right-0",
  };

  classes([
    Some(SONNER_VIEWPORT_BASE_CLASS),
    Some(placement_class),
    Some(class),
  ])
}

pub fn sonner_toast_class(variant: SonnerVariant, class: &str) -> String {
  let variant_class = match variant {
    SonnerVariant::Default => "border-zinc-200 bg-white text-zinc-950",
    SonnerVariant::Success => "border-green-200 bg-green-50 text-green-950",
    SonnerVariant::Info => "border-blue-200 bg-blue-50 text-blue-950",
    SonnerVariant::Warning => "border-amber-200 bg-amber-50 text-amber-950",
    SonnerVariant::Error => "border-red-200 bg-red-50 text-red-950",
    SonnerVariant::Loading => "border-zinc-200 bg-zinc-50 text-zinc-950",
  };

  classes([
    Some(SONNER_TOAST_BASE_CLASS),
    Some(variant_class),
    Some(class),
  ])
}

pub fn sonner_icon_class(variant: SonnerVariant, class: &str) -> String {
  let variant_class = match variant {
    SonnerVariant::Default => "bg-zinc-400",
    SonnerVariant::Success => "bg-green-600",
    SonnerVariant::Info => "bg-blue-600",
    SonnerVariant::Warning => "bg-amber-500",
    SonnerVariant::Error => "bg-red-600",
    SonnerVariant::Loading => "bg-zinc-400 animate-pulse",
  };

  classes([
    Some(SONNER_ICON_BASE_CLASS),
    Some(variant_class),
    Some(class),
  ])
}

pub fn sonner_content_class(class: &str) -> String {
  classes([Some(SONNER_CONTENT_BASE_CLASS), Some(class)])
}

pub fn sonner_title_class(class: &str) -> String {
  classes([Some(SONNER_TITLE_BASE_CLASS), Some(class)])
}

pub fn sonner_description_class(variant: SonnerVariant, class: &str) -> String {
  let variant_class = match variant {
    SonnerVariant::Default | SonnerVariant::Loading => "text-zinc-600",
    SonnerVariant::Success => "text-green-800",
    SonnerVariant::Info => "text-blue-800",
    SonnerVariant::Warning => "text-amber-800",
    SonnerVariant::Error => "text-red-800",
  };

  classes([
    Some(SONNER_DESCRIPTION_BASE_CLASS),
    Some(variant_class),
    Some(class),
  ])
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
    SonnerVariant::Default | SonnerVariant::Success | SonnerVariant::Info | SonnerVariant::Loading => {
      "polite"
    }
  }
}

/// Persistent live region; render it once and add toasts inside it so
/// assistive technology announces them.
#[component]
pub fn SonnerViewport(
  #[props(default = SonnerPlacement::BottomRight)] placement: SonnerPlacement,
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
  #[props(default = SonnerVariant::Default)] variant: SonnerVariant,
  #[props(default = true)] open: bool,
  #[props(default)] class: String,
  #[props(default = 5000)] duration_ms: u64,
  #[props(default)] on_dismiss: Option<EventHandler<SonnerDismissReason>>,
  children: Element,
) -> Element {
  let class = sonner_toast_class(variant, &class);
  let dismiss_timer = use_dismiss_timer(open, duration_ms, on_dismiss, SonnerDismissReason::Timeout);

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
  #[props(default = SonnerVariant::Default)] variant: SonnerVariant,
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
  #[props(default = SonnerVariant::Default)] variant: SonnerVariant,
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
