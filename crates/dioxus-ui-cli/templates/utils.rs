use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

pub fn classes<I, S>(parts: I) -> String
where
  I: IntoIterator<Item = Option<S>>,
  S: AsRef<str>,
{
  let mut output = String::new();

  for part in parts.into_iter().flatten() {
    for token in part.as_ref().split_whitespace() {
      if !output.is_empty() {
        output.push(' ');
      }

      output.push_str(token);
    }
  }

  output
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UiDensity {
  Compact,
  #[default]
  Comfortable,
  Touch,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum FocusStrategy {
  #[default]
  FirstFocusable,
  Container,
  None,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum FocusReturn {
  #[default]
  Trigger,
  None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DismissBehavior {
  pub escape_key: bool,
  pub outside_pointer: bool,
  pub focus_outside: bool,
}

impl DismissBehavior {
  pub const fn dialog_default() -> Self {
    Self {
      escape_key: true,
      outside_pointer: false,
      focus_outside: false,
    }
  }

  pub const fn popover_default() -> Self {
    Self {
      escape_key: true,
      outside_pointer: true,
      focus_outside: true,
    }
  }

  pub const fn tooltip_default() -> Self {
    Self {
      escape_key: true,
      outside_pointer: false,
      focus_outside: false,
    }
  }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum PortalTarget {
  Body,
  Selector(String),
  #[default]
  Inline,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OverlaySide {
  Top,
  Right,
  Bottom,
  Left,
  #[default]
  Inline,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OverlayAlign {
  Start,
  #[default]
  Center,
  End,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DialogPrimitiveConfig {
  pub open: bool,
  pub focus_strategy: FocusStrategy,
  pub focus_return: FocusReturn,
  pub dismiss: DismissBehavior,
  pub portal_target: PortalTarget,
}

impl DialogPrimitiveConfig {
  pub fn controlled(open: bool) -> Self {
    Self {
      open,
      focus_strategy: FocusStrategy::default(),
      focus_return: FocusReturn::default(),
      dismiss: DismissBehavior::dialog_default(),
      portal_target: PortalTarget::default(),
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PopoverPrimitiveConfig {
  pub open: bool,
  pub focus_strategy: FocusStrategy,
  pub focus_return: FocusReturn,
  pub dismiss: DismissBehavior,
  pub portal_target: PortalTarget,
  pub side: OverlaySide,
  pub align: OverlayAlign,
}

impl PopoverPrimitiveConfig {
  pub fn controlled(open: bool) -> Self {
    Self {
      open,
      focus_strategy: FocusStrategy::None,
      focus_return: FocusReturn::Trigger,
      dismiss: DismissBehavior::popover_default(),
      portal_target: PortalTarget::default(),
      side: OverlaySide::Bottom,
      align: OverlayAlign::Center,
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TooltipPrimitiveConfig {
  pub open: bool,
  pub delay_ms: u16,
  pub focus_strategy: FocusStrategy,
  pub dismiss: DismissBehavior,
  pub portal_target: PortalTarget,
  pub side: OverlaySide,
}

impl TooltipPrimitiveConfig {
  pub fn controlled(open: bool) -> Self {
    Self {
      open,
      delay_ms: 700,
      focus_strategy: FocusStrategy::None,
      dismiss: DismissBehavior::tooltip_default(),
      portal_target: PortalTarget::default(),
      side: OverlaySide::Top,
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectPrimitiveConfig {
  pub open: bool,
  pub value: Option<String>,
  pub focus_strategy: FocusStrategy,
  pub dismiss: DismissBehavior,
  pub portal_target: PortalTarget,
  pub side: OverlaySide,
  pub align: OverlayAlign,
}

impl SelectPrimitiveConfig {
  pub fn controlled(open: bool, value: Option<String>) -> Self {
    Self {
      open,
      value,
      focus_strategy: FocusStrategy::Container,
      dismiss: DismissBehavior::popover_default(),
      portal_target: PortalTarget::default(),
      side: OverlaySide::Bottom,
      align: OverlayAlign::Start,
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DropdownPrimitiveConfig {
  pub open: bool,
  pub focus_strategy: FocusStrategy,
  pub dismiss: DismissBehavior,
  pub portal_target: PortalTarget,
  pub side: OverlaySide,
  pub align: OverlayAlign,
}

impl DropdownPrimitiveConfig {
  pub fn controlled(open: bool) -> Self {
    Self {
      open,
      focus_strategy: FocusStrategy::Container,
      dismiss: DismissBehavior::popover_default(),
      portal_target: PortalTarget::default(),
      side: OverlaySide::Bottom,
      align: OverlayAlign::End,
    }
  }
}

static NEXT_FOCUS_SCOPE_ID: AtomicUsize = AtomicUsize::new(0);

// Runs until the scope element is hidden or removed, so focus is restored both
// when `open` turns false and when the app stops rendering the content.
pub const MODAL_FOCUS_SCOPE_SCRIPT: &str = r#"
const scope = document.querySelector('[data-dxui-focus-scope="__SCOPE_ID__"]');
if (!scope) return;
const previous = document.activeElement;
const selector = 'a[href], button:not([disabled]), input:not([disabled]):not([type="hidden"]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"]), [contenteditable="true"]';
const focusables = () => Array.from(scope.querySelectorAll(selector)).filter((el) => el.getClientRects().length > 0);
const onKeyDown = (event) => {
  if (event.key !== "Tab") return;
  const items = focusables();
  if (items.length === 0) {
    event.preventDefault();
    scope.focus();
    return;
  }
  const first = items[0];
  const last = items[items.length - 1];
  const active = document.activeElement;
  if (event.shiftKey && (active === first || active === scope)) {
    event.preventDefault();
    last.focus();
  } else if (!event.shiftKey && active === last) {
    event.preventDefault();
    first.focus();
  }
};
await new Promise((resolve) => requestAnimationFrame(resolve));
if (!scope.isConnected || scope.hidden) return;
scope.addEventListener("keydown", onKeyDown);
(focusables()[0] ?? scope).focus();
await new Promise((resolve) => {
  const observer = new MutationObserver(() => {
    if (!scope.isConnected || scope.hidden) {
      observer.disconnect();
      resolve();
    }
  });
  observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["hidden"] });
});
scope.removeEventListener("keydown", onKeyDown);
if (previous instanceof HTMLElement && previous.isConnected) previous.focus();
"#;

pub fn modal_focus_scope_script(scope_id: &str) -> String {
  MODAL_FOCUS_SCOPE_SCRIPT.replace("__SCOPE_ID__", scope_id)
}

/// Starts the modal focus scope each time `open` turns true.
///
/// Returns the value for the content's `data-dxui-focus-scope` attribute.
pub fn use_modal_focus_scope(open: bool) -> String {
  let scope_id =
    use_hook(|| format!("dxui-focus-{}", NEXT_FOCUS_SCOPE_ID.fetch_add(1, Ordering::Relaxed)));
  let script = modal_focus_scope_script(&scope_id);

  use_effect(use_reactive((&open,), move |(open,)| {
    if open {
      document::eval(&script);
    }
  }));

  scope_id
}

static NEXT_ANCHORED_ID: AtomicUsize = AtomicUsize::new(0);

// Places anchored content with the flip and shift rules of
// `compute_overlay_placement`, done in the page to avoid a round trip per
// layout change, and reports Escape and outside interactions to Rust.
// Keep in sync with `ANCHORED_OVERLAY_SCRIPT` in the CLI `utils.rs` template.
pub const ANCHORED_OVERLAY_SCRIPT: &str = r#"
const [scopeId, anchorId, preferredSide, align, offset] = await dioxus.recv();
const content = document.querySelector(`[data-dxui-anchored="${scopeId}"]`);
if (!content) return;
const anchor = anchorId ? document.getElementById(anchorId) : null;
const padding = 8;
const opposite = { top: "bottom", bottom: "top", left: "right", right: "left" };
const vertical = (side) => side === "top" || side === "bottom";
const cross = (start, anchorSize, size) =>
  align === "start" ? start : align === "end" ? start + anchorSize - size : start + (anchorSize - size) / 2;
const clamp = (value, size, viewportSize) =>
  Math.min(Math.max(value, padding), Math.max(padding, viewportSize - padding - size));
const place = () => {
  if (!anchor || !(preferredSide in opposite)) return;
  // Fixed positioning can change the content's size, so apply it before measuring.
  Object.assign(content.style, { position: "fixed", margin: "0" });
  const rect = anchor.getBoundingClientRect();
  const width = content.offsetWidth;
  const height = content.offsetHeight;
  const viewportWidth = document.documentElement.clientWidth;
  const viewportHeight = document.documentElement.clientHeight;
  const space = {
    top: rect.top - padding,
    bottom: viewportHeight - padding - rect.bottom,
    left: rect.left - padding,
    right: viewportWidth - padding - rect.right,
  };
  const required = (vertical(preferredSide) ? height : width) + Math.max(offset, 0);
  let side = preferredSide;
  if (space[side] < required && space[opposite[side]] >= space[side]) side = opposite[side];
  let x;
  let y;
  if (vertical(side)) {
    x = clamp(cross(rect.left, rect.width, width), width, viewportWidth);
    y = side === "top" ? rect.top - height - offset : rect.bottom + offset;
  } else {
    x = side === "left" ? rect.left - width - offset : rect.right + offset;
    y = clamp(cross(rect.top, rect.height, height), height, viewportHeight);
  }
  Object.assign(content.style, { left: `${Math.round(x)}px`, top: `${Math.round(y)}px` });
  content.dataset.side = side;
};
const inside = (target) => content.contains(target) || (anchor !== null && anchor.contains(target));
const onPointerDown = (event) => {
  if (!inside(event.target)) dioxus.send("pointer-outside");
};
const onFocusIn = (event) => {
  if (!inside(event.target)) dioxus.send("focus-outside");
};
const onKeyDown = (event) => {
  if (event.key === "Escape") dioxus.send("escape");
};
await new Promise((resolve) => requestAnimationFrame(resolve));
if (!content.isConnected || content.hidden) return;
place();
window.addEventListener("resize", place);
window.addEventListener("scroll", place, true);
document.addEventListener("pointerdown", onPointerDown, true);
document.addEventListener("focusin", onFocusIn);
document.addEventListener("keydown", onKeyDown);
await new Promise((resolve) => {
  const observer = new MutationObserver(() => {
    if (!content.isConnected || content.hidden) {
      observer.disconnect();
      resolve();
    }
  });
  observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["hidden"] });
});
window.removeEventListener("resize", place);
window.removeEventListener("scroll", place, true);
document.removeEventListener("pointerdown", onPointerDown, true);
document.removeEventListener("focusin", onFocusIn);
document.removeEventListener("keydown", onKeyDown);
Object.assign(content.style, { position: "", margin: "", left: "", top: "" });
"#;

/// Where anchored content goes relative to the element with id `anchor_id`.
#[derive(Clone, Debug, PartialEq)]
pub struct AnchoredPlacement {
  pub anchor_id: Option<String>,
  pub side: OverlaySide,
  pub align: OverlayAlign,
  pub side_offset: i32,
}

fn side_name(side: OverlaySide) -> &'static str {
  match side {
    OverlaySide::Top => "top",
    OverlaySide::Right => "right",
    OverlaySide::Bottom => "bottom",
    OverlaySide::Left => "left",
    OverlaySide::Inline => "inline",
  }
}

fn align_name(align: OverlayAlign) -> &'static str {
  match align {
    OverlayAlign::Start => "start",
    OverlayAlign::Center => "center",
    OverlayAlign::End => "end",
  }
}

/// Whether a message from `ANCHORED_OVERLAY_SCRIPT` should close the overlay.
fn dismisses(dismiss: &DismissBehavior, message: &str) -> bool {
  match message {
    "escape" => dismiss.escape_key,
    "pointer-outside" => dismiss.outside_pointer,
    "focus-outside" => dismiss.focus_outside,
    _ => false,
  }
}

/// Positions content next to its anchor and requests close on the dismissal
/// paths enabled in `dismiss`, each time `open` turns true. Placement and
/// dismissal settings are read when the overlay opens.
///
/// Returns the value for the content's `data-dxui-anchored` attribute.
pub fn use_anchored_overlay(
  open: bool,
  placement: AnchoredPlacement,
  dismiss: DismissBehavior,
  on_open_change: Option<EventHandler<bool>>,
) -> String {
  let scope_id =
    use_hook(|| format!("dxui-anchored-{}", NEXT_ANCHORED_ID.fetch_add(1, Ordering::Relaxed)));
  let was_open = use_hook(|| Rc::new(Cell::new(false)));
  let effect_scope_id = scope_id.clone();

  use_effect(use_reactive(
    (&open, &placement, &dismiss, &on_open_change),
    move |(open, placement, dismiss, on_open_change)| {
      if open == was_open.replace(open) || !open {
        return;
      }
      let mut eval = document::eval(ANCHORED_OVERLAY_SCRIPT);
      // A send error means the page already finished the script; nothing to place.
      let _ = eval.send((
        effect_scope_id.as_str(),
        placement.anchor_id.as_deref(),
        side_name(placement.side),
        align_name(placement.align),
        placement.side_offset,
      ));
      spawn(async move {
        while let Ok(message) = eval.recv::<String>().await {
          if let Some(handler) = on_open_change.filter(|_| dismisses(&dismiss, &message)) {
            handler.call(false);
          }
        }
      });
    },
  ));

  scope_id
}

static NEXT_DISMISS_TIMER_ID: AtomicUsize = AtomicUsize::new(0);

// Counts down only while the pointer is outside the toast and focus is not
// inside it, and reports "timeout" once the remaining time runs out. Exits
// quietly when the toast is hidden or removed first.
// Keep in sync with `DISMISS_TIMER_SCRIPT` in the CLI `utils.rs` template.
pub const DISMISS_TIMER_SCRIPT: &str = r#"
const [scopeId, duration] = await dioxus.recv();
const toast = document.querySelector(`[data-dxui-dismiss-timer="${scopeId}"]`);
if (!toast) return;
await new Promise((resolve) => requestAnimationFrame(resolve));
if (!toast.isConnected || toast.hidden) return;
let remaining = duration;
let startedAt = 0;
let timer = null;
let hovered = false;
let focused = false;
let finish;
const ended = new Promise((resolve) => {
  finish = resolve;
});
const resume = () => {
  if (timer !== null || hovered || focused) return;
  startedAt = performance.now();
  timer = setTimeout(() => finish("timeout"), remaining);
};
const pause = () => {
  if (timer === null) return;
  clearTimeout(timer);
  timer = null;
  remaining -= performance.now() - startedAt;
};
const onPointerEnter = () => {
  hovered = true;
  pause();
};
const onPointerLeave = () => {
  hovered = false;
  resume();
};
const onFocusIn = () => {
  focused = true;
  pause();
};
const onFocusOut = (event) => {
  if (toast.contains(event.relatedTarget)) return;
  focused = false;
  resume();
};
const observer = new MutationObserver(() => {
  if (!toast.isConnected || toast.hidden) finish("closed");
});
observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["hidden"] });
toast.addEventListener("pointerenter", onPointerEnter);
toast.addEventListener("pointerleave", onPointerLeave);
toast.addEventListener("focusin", onFocusIn);
toast.addEventListener("focusout", onFocusOut);
toast.dataset.timer = "running";
resume();
const reason = await ended;
clearTimeout(timer);
observer.disconnect();
toast.removeEventListener("pointerenter", onPointerEnter);
toast.removeEventListener("pointerleave", onPointerLeave);
toast.removeEventListener("focusin", onFocusIn);
toast.removeEventListener("focusout", onFocusOut);
delete toast.dataset.timer;
if (reason === "timeout") dioxus.send("timeout");
"#;

/// Starts the dismiss countdown each time `open` turns true with a non-zero
/// `duration_ms`, and calls `on_dismiss(timeout_reason)` when it runs out. The
/// duration is read when the toast opens.
///
/// Returns the value for the toast's `data-dxui-dismiss-timer` attribute.
pub fn use_dismiss_timer<R: Clone + 'static>(
  open: bool,
  duration_ms: u64,
  on_dismiss: Option<EventHandler<R>>,
  timeout_reason: R,
) -> String {
  let scope_id =
    use_hook(|| format!("dxui-timer-{}", NEXT_DISMISS_TIMER_ID.fetch_add(1, Ordering::Relaxed)));
  let was_open = use_hook(|| Rc::new(Cell::new(false)));
  let effect_scope_id = scope_id.clone();

  use_effect(use_reactive(
    (&open, &duration_ms, &on_dismiss),
    move |(open, duration_ms, on_dismiss)| {
      if open == was_open.replace(open) || !open || duration_ms == 0 {
        return;
      }
      let timeout_reason = timeout_reason.clone();
      let mut eval = document::eval(DISMISS_TIMER_SCRIPT);
      // A send error means the page already finished the script; nothing to time.
      let _ = eval.send((effect_scope_id.as_str(), duration_ms));
      spawn(async move {
        let timed_out = matches!(eval.recv::<String>().await, Ok(message) if message == "timeout");
        if let Some(handler) = on_dismiss.filter(|_| timed_out) {
          handler.call(timeout_reason);
        }
      });
    },
  ));

  scope_id
}

static NEXT_LISTBOX_ID: AtomicUsize = AtomicUsize::new(0);

// Keeps focus on the anchor (Select trigger or Combobox input) and tracks the
// highlighted option through `aria-activedescendant`. Options are read from
// the DOM on every key so items re-rendered while filtering are picked up.
// Sends the chosen option's `data-value`.
// Keep in sync with `LISTBOX_SCRIPT` in the CLI `utils.rs` template.
pub const LISTBOX_SCRIPT: &str = r#"
const [scopeId, anchorId, mode] = await dioxus.recv();
const listbox = document.querySelector(`[data-dxui-listbox="${scopeId}"]`);
if (!listbox) return;
const anchor = anchorId ? document.getElementById(anchorId) : null;
const isSelect = mode === "select";
const enabled = (option) =>
  option.dataset.disabled !== "true" && option.getAttribute("aria-disabled") !== "true";
const options = () => {
  const all = Array.from(listbox.querySelectorAll('[role="option"]'));
  all.forEach((option, index) => {
    if (!option.id) option.id = `${scopeId}-option-${index}`;
  });
  return all.filter(enabled);
};
let highlighted = null;
const highlight = (option) => {
  if (highlighted && highlighted !== option) delete highlighted.dataset.highlighted;
  highlighted = option;
  if (option) {
    option.dataset.highlighted = "";
    option.scrollIntoView({ block: "nearest" });
    if (anchor) anchor.setAttribute("aria-activedescendant", option.id);
  } else if (anchor) {
    anchor.removeAttribute("aria-activedescendant");
  }
};
const initial = () => {
  if (!isSelect) return null;
  const list = options();
  return list.find((option) => option.getAttribute("aria-selected") === "true") || list[0] || null;
};
const choose = (option) => {
  if (option && enabled(option)) dioxus.send(option.dataset.value || "");
};
let buffer = "";
let bufferedAt = 0;
const typing = () => buffer !== "" && performance.now() - bufferedAt <= 500;
const typeahead = (character) => {
  buffer = typing() ? buffer + character : character;
  bufferedAt = performance.now();
  const list = options();
  const start = list.indexOf(highlighted) + 1;
  for (let offset = 0; offset < list.length; offset += 1) {
    const option = list[(start + offset) % list.length];
    if (option.textContent.trim().toLowerCase().startsWith(buffer)) return highlight(option);
  }
};
const onKeyDown = (event) => {
  const list = options();
  const index = list.indexOf(highlighted);
  let handled = true;
  if (event.key === "ArrowDown") {
    highlight(list[Math.min(index + 1, list.length - 1)] || null);
  } else if (event.key === "ArrowUp") {
    highlight(list[index < 0 ? list.length - 1 : Math.max(index - 1, 0)] || null);
  } else if (isSelect && event.key === "Home") {
    highlight(list[0] || null);
  } else if (isSelect && event.key === "End") {
    highlight(list[list.length - 1] || null);
  } else if (event.key === "Enter" && highlighted) {
    choose(highlighted);
  } else if (isSelect && event.key === " " && !typing()) {
    choose(highlighted);
  } else if (isSelect && event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey) {
    typeahead(event.key.toLowerCase());
  } else {
    handled = false;
  }
  if (handled) event.preventDefault();
};
const optionFrom = (target) => {
  const option = target instanceof Element ? target.closest('[role="option"]') : null;
  return option && listbox.contains(option) && enabled(option) ? option : null;
};
const onPointerMove = (event) => {
  const option = optionFrom(event.target);
  if (option && option !== highlighted) highlight(option);
};
// Keep focus on the anchor so keys keep reaching it after a click.
const onPointerDown = (event) => event.preventDefault();
const onClick = (event) => choose(optionFrom(event.target));
let finish;
const ended = new Promise((resolve) => {
  finish = resolve;
});
const observer = new MutationObserver(() => {
  if (!listbox.isConnected || listbox.hidden) return finish();
  if (highlighted && !options().includes(highlighted)) highlight(initial());
});
await new Promise((resolve) => requestAnimationFrame(resolve));
if (!listbox.isConnected || listbox.hidden) return;
highlight(initial());
observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["hidden", "data-disabled", "aria-disabled"] });
if (anchor) anchor.addEventListener("keydown", onKeyDown);
listbox.addEventListener("pointermove", onPointerMove);
listbox.addEventListener("pointerdown", onPointerDown);
listbox.addEventListener("click", onClick);
await ended;
observer.disconnect();
if (anchor) anchor.removeEventListener("keydown", onKeyDown);
listbox.removeEventListener("pointermove", onPointerMove);
listbox.removeEventListener("pointerdown", onPointerDown);
listbox.removeEventListener("click", onClick);
highlight(null);
"#;

/// How the listbox script treats keys on the anchor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListboxMode {
  /// Select: starts on the selected option, handles Home, End, Space, and
  /// typeahead.
  Select,
}

impl ListboxMode {
  fn name(self) -> &'static str {
    match self {
          Self::Select => "select",
    }
  }
}

/// Runs the listbox script each time `open` turns true with an `anchor_id`.
/// Choosing an option calls `on_value_change(value)` and then
/// `on_open_change(false)`.
///
/// Returns the value for the content's `data-dxui-listbox` attribute.
pub fn use_listbox(
  open: bool,
  anchor_id: Option<String>,
  mode: ListboxMode,
  on_value_change: Option<EventHandler<String>>,
  on_open_change: Option<EventHandler<bool>>,
) -> String {
  let scope_id =
    use_hook(|| format!("dxui-listbox-{}", NEXT_LISTBOX_ID.fetch_add(1, Ordering::Relaxed)));
  let was_open = use_hook(|| Rc::new(Cell::new(false)));
  let effect_scope_id = scope_id.clone();

  use_effect(use_reactive(
    (&open, &anchor_id, &on_value_change, &on_open_change),
    move |(open, anchor_id, on_value_change, on_open_change)| {
      if open == was_open.replace(open) || !open || anchor_id.is_none() {
        return;
      }
      let mut eval = document::eval(LISTBOX_SCRIPT);
      // A send error means the page already finished the script; nothing to track.
      let _ = eval.send((effect_scope_id.as_str(), anchor_id.as_deref(), mode.name()));
      spawn(async move {
        while let Ok(value) = eval.recv::<String>().await {
          if let Some(handler) = on_value_change {
            handler.call(value);
          }
          if let Some(handler) = on_open_change {
            handler.call(false);
          }
        }
      });
    },
  ));

  scope_id
}
