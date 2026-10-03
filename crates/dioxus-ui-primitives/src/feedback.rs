#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToastPlacement {
  TopLeft,
  TopCenter,
  TopRight,
  BottomLeft,
  BottomCenter,
  #[default]
  BottomRight,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToastVariant {
  #[default]
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
    Self { items: Vec::new(), limit }
  }

  pub fn with_items(items: Vec<ToastItem>, limit: usize) -> Self {
    Self { items: toast_queue_limit(items, limit), limit }
  }

  pub fn push(self, item: ToastItem) -> Self {
    toast_queue_push(self, item)
  }

  pub fn dismiss(self, id: &str) -> Self {
    toast_queue_dismiss(self, id)
  }
}

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

  ToastQueue { items: toast_queue_limit(items, queue.limit), limit: queue.limit }
}

pub fn toast_queue_dismiss(queue: ToastQueue, id: &str) -> ToastQueue {
  ToastQueue {
    items: queue.items.into_iter().filter(|item| item.id != id).collect(),
    limit: queue.limit,
  }
}

pub fn toast_queue_limit(items: Vec<ToastItem>, limit: usize) -> Vec<ToastItem> {
  if limit == 0 {
    return Vec::new();
  }

  let len = items.len();

  if len <= limit { items } else { items.into_iter().skip(len - limit).collect() }
}

pub const fn toast_is_expired(elapsed_ms: u64, duration_ms: u64) -> bool {
  duration_ms > 0 && elapsed_ms >= duration_ms
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn maps_feedback_attributes() {
    assert_eq!(toast_placement_attribute(ToastPlacement::TopCenter), "top-center");
    assert_eq!(toast_variant_attribute(ToastVariant::Warning), "warning");
    assert_eq!(toast_dismiss_reason_attribute(ToastDismissReason::Timeout), "timeout");
  }

  #[test]
  fn pushes_toasts_and_keeps_newest_items_with_limit() {
    let queue = ToastQueue::new(2)
      .push(ToastItem::new("one", "One"))
      .push(ToastItem::new("two", "Two"))
      .push(ToastItem::new("three", "Three"));

    assert_eq!(
      queue.items.iter().map(|item| item.id.as_str()).collect::<Vec<_>>(),
      vec!["two", "three"]
    );
  }

  #[test]
  fn replaces_existing_toast_by_id() {
    let queue = ToastQueue::new(3)
      .push(ToastItem::new("one", "One"))
      .push(ToastItem::new("one", "Updated").with_variant(ToastVariant::Success));

    assert_eq!(queue.items.len(), 1);
    assert_eq!(queue.items[0].title, "Updated");
    assert_eq!(queue.items[0].variant, ToastVariant::Success);
  }

  #[test]
  fn dismisses_toast_by_id() {
    let queue = ToastQueue::new(3)
      .push(ToastItem::new("one", "One"))
      .push(ToastItem::new("two", "Two"))
      .dismiss("one");

    assert_eq!(queue.items.len(), 1);
    assert_eq!(queue.items[0].id, "two");
  }

  #[test]
  fn handles_zero_limit_and_timeout_expiry() {
    assert!(toast_queue_limit(vec![ToastItem::new("one", "One")], 0).is_empty());
    assert!(!toast_is_expired(5000, 0));
    assert!(!toast_is_expired(4999, 5000));
    assert!(toast_is_expired(5000, 5000));
  }
}
