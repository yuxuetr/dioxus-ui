//! Toast state for the styled `Toast` and `Sonner` components: placements,
//! tones, and a capped queue where the newest toasts win.

/// Which viewport corner or edge toasts stack in.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToastPlacement {
  /// Top-left corner.
  TopLeft,
  /// Top edge, centered.
  TopCenter,
  /// Top-right corner.
  TopRight,
  /// Bottom-left corner.
  BottomLeft,
  /// Bottom edge, centered.
  BottomCenter,
  /// Bottom-right corner.
  #[default]
  BottomRight,
}

/// The tone of a toast, which picks its icon and colors.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToastVariant {
  /// Neutral message.
  #[default]
  Default,
  /// Completed action.
  Success,
  /// Informational notice.
  Info,
  /// Something needs attention.
  Warning,
  /// Failed action.
  Error,
  /// Work still in progress.
  Loading,
}

/// Why a toast went away.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToastDismissReason {
  /// The user clicked the toast's action button.
  Action,
  /// The user clicked the close button.
  Close,
  /// The toast's duration ran out.
  Timeout,
  /// Application code dismissed it.
  Programmatic,
}

/// One toast notification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToastItem {
  /// Identifier; pushing an item with an existing id replaces that toast.
  pub id: String,
  /// Headline text.
  pub title: String,
  /// Secondary text under the title; `None` shows none.
  pub description: Option<String>,
  /// Tone of the toast.
  pub variant: ToastVariant,
  /// How long the toast stays, in milliseconds; 0 keeps it until dismissed.
  pub duration_ms: u64,
  /// Whether the user can close it.
  pub dismissible: bool,
}

impl ToastItem {
  /// A dismissible default toast that stays for 5000 ms.
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

  /// Sets the secondary text.
  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }

  /// Sets the tone.
  pub const fn with_variant(mut self, variant: ToastVariant) -> Self {
    self.variant = variant;
    self
  }

  /// Sets how long it stays, in milliseconds; 0 keeps it until dismissed.
  pub const fn with_duration_ms(mut self, duration_ms: u64) -> Self {
    self.duration_ms = duration_ms;
    self
  }

  /// Sets whether the user can close it.
  pub const fn with_dismissible(mut self, dismissible: bool) -> Self {
    self.dismissible = dismissible;
    self
  }
}

/// The visible toasts, oldest first, capped at a limit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToastQueue {
  /// Visible toasts, oldest first.
  pub items: Vec<ToastItem>,
  /// Maximum number of visible toasts; 0 shows none.
  pub limit: usize,
}

impl ToastQueue {
  /// An empty queue that shows at most `limit` toasts.
  pub fn new(limit: usize) -> Self {
    Self { items: Vec::new(), limit }
  }

  /// A queue holding the newest `limit` of `items`.
  pub fn with_items(items: Vec<ToastItem>, limit: usize) -> Self {
    Self { items: toast_queue_limit(items, limit), limit }
  }

  /// Adds a toast; see [`toast_queue_push`].
  pub fn push(self, item: ToastItem) -> Self {
    toast_queue_push(self, item)
  }

  /// Removes the toast with `id`; see [`toast_queue_dismiss`].
  pub fn dismiss(self, id: &str) -> Self {
    toast_queue_dismiss(self, id)
  }
}

/// The `data-placement` value for a placement, such as `top-center`.
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

/// The `data-variant` value for a tone, such as `warning`.
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

/// The attribute value for a dismissal reason, such as `timeout`.
pub fn toast_dismiss_reason_attribute(reason: ToastDismissReason) -> &'static str {
  match reason {
    ToastDismissReason::Action => "action",
    ToastDismissReason::Close => "close",
    ToastDismissReason::Timeout => "timeout",
    ToastDismissReason::Programmatic => "programmatic",
  }
}

/// Appends `item` as the newest toast, first removing any toast with the
/// same id, then drops the oldest toasts beyond the limit.
pub fn toast_queue_push(queue: ToastQueue, item: ToastItem) -> ToastQueue {
  let mut items = queue.items;

  if let Some(index) = items.iter().position(|current| current.id == item.id) {
    items.remove(index);
  }

  items.push(item);

  ToastQueue { items: toast_queue_limit(items, queue.limit), limit: queue.limit }
}

/// Removes the toast with `id`; an unknown id leaves the queue unchanged.
pub fn toast_queue_dismiss(queue: ToastQueue, id: &str) -> ToastQueue {
  ToastQueue {
    items: queue.items.into_iter().filter(|item| item.id != id).collect(),
    limit: queue.limit,
  }
}

/// Keeps the newest `limit` items, dropping from the front; empty when
/// `limit` is 0.
pub fn toast_queue_limit(items: Vec<ToastItem>, limit: usize) -> Vec<ToastItem> {
  if limit == 0 {
    return Vec::new();
  }

  let len = items.len();

  if len <= limit { items } else { items.into_iter().skip(len - limit).collect() }
}

/// Whether a toast shown for `elapsed_ms` has outlived `duration_ms`; a
/// duration of 0 never expires.
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
