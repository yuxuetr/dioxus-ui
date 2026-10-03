use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToastPlacement {
  TopLeft,
  TopCenter,
  TopRight,
  BottomLeft,
  BottomCenter,
  BottomRight,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToastVariant {
  Default,
  Success,
  Info,
  Warning,
  Error,
  Loading,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToastDismissReason {
  Action,
  Close,
  Timeout,
  Programmatic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToastItem {
  pub id: String,
  pub title: String,
  pub description: Option<String>,
  pub variant: ToastVariant,
  pub duration_ms: u64,
  pub dismissible: bool,
}

impl ToastItem {
  pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      title: title.into(),
      description: None,
      variant: ToastVariant::Default,
      duration_ms: 5000,
      dismissible: true,
    }
  }

  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }

  pub const fn with_variant(mut self, variant: ToastVariant) -> Self {
    self.variant = variant;
    self
  }

  pub const fn with_duration_ms(mut self, duration_ms: u64) -> Self {
    self.duration_ms = duration_ms;
    self
  }

  pub const fn with_dismissible(mut self, dismissible: bool) -> Self {
    self.dismissible = dismissible;
    self
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToastQueue {
  pub items: Vec<ToastItem>,
  pub limit: usize,
}

impl ToastQueue {
  pub fn new(limit: usize) -> Self {
    Self {
      items: Vec::new(),
      limit,
    }
  }

  pub fn with_items(items: Vec<ToastItem>, limit: usize) -> Self {
    Self {
      items: toast_queue_limit(items, limit),
      limit,
    }
  }

  pub fn push(self, item: ToastItem) -> Self {
    toast_queue_push(self, item)
  }

  pub fn dismiss(self, id: &str) -> Self {
    toast_queue_dismiss(self, id)
  }
}

pub const TOAST_VIEWPORT_BASE_CLASS: &str = "fixed z-50 flex max-h-screen w-full flex-col gap-2 p-4 sm:max-w-sm";
pub const TOAST_ROOT_BASE_CLASS: &str = "pointer-events-auto relative grid w-full gap-1 overflow-hidden rounded-md border bg-white p-4 pr-10 text-zinc-950 shadow-lg transition-all data-state-closed:opacity-0 data-state-open:opacity-100";
pub const TOAST_TITLE_BASE_CLASS: &str = "text-sm font-semibold leading-none tracking-normal";
pub const TOAST_DESCRIPTION_BASE_CLASS: &str = "text-sm text-zinc-600";
pub const TOAST_ACTION_BASE_CLASS: &str = "inline-flex h-8 shrink-0 items-center justify-center rounded-md border border-zinc-200 bg-transparent px-3 text-sm font-medium transition-colors hover:bg-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";
pub const TOAST_CLOSE_BASE_CLASS: &str = "absolute right-2 top-2 inline-flex h-7 w-7 items-center justify-center rounded-md text-zinc-500 transition-colors hover:bg-zinc-100 hover:text-zinc-950 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";

pub fn toast_placement_attribute(placement: ToastPlacement) -> &'static str {
  match placement {
    ToastPlacement::TopLeft => "top-left",
    ToastPlacement::TopCenter => "top-center",
    ToastPlacement::TopRight => "top-right",
    ToastPlacement::BottomLeft => "bottom-left",
    ToastPlacement::BottomCenter => "bottom-center",
    ToastPlacement::BottomRight => "bottom-right",
  }
}

pub fn toast_variant_attribute(variant: ToastVariant) -> &'static str {
  match variant {
    ToastVariant::Default => "default",
    ToastVariant::Success => "success",
    ToastVariant::Info => "info",
    ToastVariant::Warning => "warning",
    ToastVariant::Error => "error",
    ToastVariant::Loading => "loading",
  }
}

pub fn toast_dismiss_reason_attribute(reason: ToastDismissReason) -> &'static str {
  match reason {
    ToastDismissReason::Action => "action",
    ToastDismissReason::Close => "close",
    ToastDismissReason::Timeout => "timeout",
    ToastDismissReason::Programmatic => "programmatic",
  }
}

pub fn toast_queue_push(queue: ToastQueue, item: ToastItem) -> ToastQueue {
  let mut items = queue.items;

  if let Some(index) = items.iter().position(|current| current.id == item.id) {
    items.remove(index);
  }

  items.push(item);

  ToastQueue {
    items: toast_queue_limit(items, queue.limit),
    limit: queue.limit,
  }
}

pub fn toast_queue_dismiss(queue: ToastQueue, id: &str) -> ToastQueue {
  ToastQueue {
    items: queue
      .items
      .into_iter()
      .filter(|item| item.id != id)
      .collect(),
    limit: queue.limit,
  }
}

pub fn toast_queue_limit(items: Vec<ToastItem>, limit: usize) -> Vec<ToastItem> {
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

pub const fn toast_is_expired(elapsed_ms: u64, duration_ms: u64) -> bool {
  duration_ms > 0 && elapsed_ms >= duration_ms
}

pub fn toast_viewport_class(placement: ToastPlacement, class: &str) -> String {
  let placement_class = match placement {
    ToastPlacement::TopLeft => "left-0 top-0 sm:left-0",
    ToastPlacement::TopCenter => "top-0 sm:left-1/2 sm:-translate-x-1/2",
    ToastPlacement::TopRight => "right-0 top-0 sm:right-0",
    ToastPlacement::BottomLeft => "bottom-0 left-0 sm:left-0",
    ToastPlacement::BottomCenter => "bottom-0 sm:left-1/2 sm:-translate-x-1/2",
    ToastPlacement::BottomRight => "bottom-0 right-0 sm:right-0",
  };

  classes([
    Some(TOAST_VIEWPORT_BASE_CLASS),
    Some(placement_class),
    Some(class),
  ])
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

  classes([
    Some(TOAST_ROOT_BASE_CLASS),
    Some(variant_class),
    Some(class),
  ])
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

  classes([
    Some(TOAST_DESCRIPTION_BASE_CLASS),
    Some(variant_class),
    Some(class),
  ])
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
  #[props(default = ToastPlacement::BottomRight)] placement: ToastPlacement,
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

#[component]
pub fn ToastRoot(
  #[props(default = ToastVariant::Default)] variant: ToastVariant,
  #[props(default = true)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = toast_root_class(variant, &class);

  rsx! {
    div {
      role: "status",
      class,
      hidden: !open,
      "aria-live": toast_live_attribute(variant),
      "data-state": if open { "open" } else { "closed" },
      "data-variant": toast_variant_attribute(variant),
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
  #[props(default = ToastVariant::Default)] variant: ToastVariant,
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
  children: Element,
) -> Element {
  let class = toast_action_class(disabled, &class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      {children}
    }
  }
}

#[component]
pub fn ToastClose(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = toast_close_class(disabled, &class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-label": "Close notification",
      {children}
    }
  }
}
