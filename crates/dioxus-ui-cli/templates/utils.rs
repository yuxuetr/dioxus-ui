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

/// Keeps a component's default `aria-label` only when the app passed none.
/// The browser applies the later spread value anyway, but SSR writes both
/// attributes and an HTML parser keeps the first.
pub fn default_aria_label(attributes: &[Attribute], label: &'static str) -> Option<&'static str> {
  (!attributes.iter().any(|attribute| attribute.name == "aria-label")).then_some(label)
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
// Roving grids keep inactive items at tabindex -1; they are not Tab stops.
const focusables = () =>
  Array.from(scope.querySelectorAll(selector)).filter((el) => el.tabIndex >= 0 && el.getClientRects().length > 0);
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
(scope.querySelector("[data-dxui-autofocus]") ?? focusables()[0] ?? scope).focus();
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
// Focus still inside the hidden scope, or already blurred to the body, goes
// back; focus anywhere else was moved on purpose, such as by an outside click,
// and stays there.
const active = document.activeElement;
const focusLeft = active !== null && active !== document.body && !scope.contains(active);
if (!focusLeft && previous instanceof HTMLElement && previous.isConnected) previous.focus();
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
const [scopeId, anchorId, preferredSide, align, offset, point] = await dioxus.recv();
const content = document.querySelector(`[data-dxui-anchored="${scopeId}"]`);
if (!content) return;
const anchor = anchorId ? document.getElementById(anchorId) : null;
// Without an anchor element, a viewport point (a context menu's pointer
// position) acts as a zero-size anchor.
const anchorRect = () =>
  anchor
    ? anchor.getBoundingClientRect()
    : point
      ? { left: point[0], top: point[1], right: point[0], bottom: point[1], width: 0, height: 0 }
      : null;
const padding = 8;
const opposite = { top: "bottom", bottom: "top", left: "right", right: "left" };
const vertical = (side) => side === "top" || side === "bottom";
const cross = (start, anchorSize, size) =>
  align === "start" ? start : align === "end" ? start + anchorSize - size : start + (anchorSize - size) / 2;
const clamp = (value, size, viewportSize) =>
  Math.min(Math.max(value, padding), Math.max(padding, viewportSize - padding - size));
const place = () => {
  const rect = anchorRect();
  if (!rect || !(preferredSide in opposite)) return;
  // Fixed positioning can change the content's size, so apply it before measuring.
  Object.assign(content.style, { position: "fixed", margin: "0" });
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

/// Where anchored content goes relative to the element with id `anchor_id`, or
/// to the viewport point `anchor_point` when there is no anchor element.
#[derive(Clone, Debug, PartialEq)]
pub struct AnchoredPlacement {
  pub anchor_id: Option<String>,
  pub anchor_point: Option<(f64, f64)>,
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
        placement.anchor_point,
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

// Select, Combobox, and Command keep focus on the anchor (trigger or input) and track
// the highlighted option through `aria-activedescendant`; menus move DOM focus
// to the highlighted item instead. Items are read from the DOM on every key so
// items re-rendered while filtering are picked up. Sends the chosen item's
// `data-value`, empty for menu items.
// Keep in sync with `LISTBOX_SCRIPT` in the CLI `utils.rs` template.
pub const LISTBOX_SCRIPT: &str = r#"
const [scopeId, anchorId, mode] = await dioxus.recv();
const listbox = document.querySelector(`[data-dxui-listbox="${scopeId}"]`);
if (!listbox) return;
const anchor = anchorId ? document.getElementById(anchorId) : null;
const previous = document.activeElement;
const isMenu = mode === "menu";
const jumps = mode !== "combobox";
// Space and typeahead are for modes where no input takes the typing.
const searchesByKey = mode === "select" || isMenu;
// Command goes back to the first option when the query changes.
const resetsOnInput = mode === "command";
const itemSelector = isMenu
  ? '[role="menuitem"], [role="menuitemcheckbox"], [role="menuitemradio"]'
  : '[role="option"]';
const enabled = (option) =>
  option.dataset.disabled !== "true" && option.getAttribute("aria-disabled") !== "true";
const options = () => {
  const all = Array.from(listbox.querySelectorAll(itemSelector));
  all.forEach((option, index) => {
    if (!option.id) option.id = `${scopeId}-option-${index}`;
    if (isMenu && !option.hasAttribute("tabindex")) option.tabIndex = -1;
  });
  return all.filter(enabled);
};
let highlighted = null;
const highlight = (option) => {
  if (highlighted && highlighted !== option) delete highlighted.dataset.highlighted;
  highlighted = option;
  if (option) {
    option.dataset.highlighted = "";
    if (isMenu) {
      // The menu may still be in normal flow before placement makes it fixed;
      // scrolling the page then would move it away from its anchor point.
      option.focus({ preventScroll: true });
    } else {
      option.scrollIntoView({ block: "nearest" });
      if (anchor) anchor.setAttribute("aria-activedescendant", option.id);
    }
  } else if (anchor && !isMenu) {
    anchor.removeAttribute("aria-activedescendant");
  }
};
const initial = () => {
  if (mode === "combobox") return null;
  const list = options();
  const selected = isMenu ? null : list.find((option) => option.getAttribute("aria-selected") === "true");
  return selected || list[0] || null;
};
const choose = (option) => {
  if (option && enabled(option)) dioxus.send(option.dataset.value || "");
};
// A menu item is activated by clicking it, so its own click handler runs and
// the click listener below reports the choice.
const activate = (option) => (isMenu ? option.click() : choose(option));
let buffer = "";
let bufferedAt = 0;
const typing = () => buffer !== "" && performance.now() - bufferedAt <= 500;
const typeahead = (character) => {
  buffer = typing() ? buffer + character : character;
  bufferedAt = performance.now();
  // Repeating one letter cycles through its matches; a longer prefix keeps
  // the current option while it still matches.
  const repeated = Array.from(buffer).every((letter) => letter === buffer[0]);
  const query = repeated ? buffer[0] : buffer;
  const list = options();
  const current = list.indexOf(highlighted);
  const start = current < 0 ? 0 : current + (repeated ? 1 : 0);
  for (let offset = 0; offset < list.length; offset += 1) {
    const option = list[(start + offset) % list.length];
    if (option.textContent.trim().toLowerCase().startsWith(query)) return highlight(option);
  }
};
const onKeyDown = (event) => {
  const list = options();
  const index = list.indexOf(highlighted);
  const last = list.length - 1;
  let handled = true;
  if (event.key === "ArrowDown") {
    const next = isMenu ? (index + 1) % list.length : Math.min(index + 1, last);
    highlight(list[next] || null);
  } else if (event.key === "ArrowUp") {
    const next = index < 0 ? last : isMenu ? (index - 1 + list.length) % list.length : Math.max(index - 1, 0);
    highlight(list[next] || null);
  } else if (jumps && event.key === "Home") {
    highlight(list[0] || null);
  } else if (jumps && event.key === "End") {
    highlight(list[last] || null);
  } else if (event.key === "Enter" && highlighted) {
    activate(highlighted);
  } else if (searchesByKey && event.key === " " && highlighted && !typing()) {
    activate(highlighted);
  } else if (searchesByKey && event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey) {
    typeahead(event.key.toLowerCase());
  } else {
    handled = false;
  }
  if (handled) event.preventDefault();
};
const optionFrom = (target) => {
  const option = target instanceof Element ? target.closest(itemSelector) : null;
  return option && listbox.contains(option) && enabled(option) ? option : null;
};
const onPointerMove = (event) => {
  const option = optionFrom(event.target);
  if (option && option !== highlighted) highlight(option);
};
// Keep focus where the keys are read (the anchor, or the highlighted menu
// item) when the pointer presses inside.
const onPointerDown = (event) => event.preventDefault();
const onClick = (event) => choose(optionFrom(event.target));
const keySource = isMenu ? listbox : anchor;
// Resets at once, and again on the next DOM change, which is the app's
// re-render of the filtered list.
let resetPending = false;
const onInput = () => {
  resetPending = true;
  highlight(initial());
};
let finish;
const ended = new Promise((resolve) => {
  finish = resolve;
});
const observer = new MutationObserver(() => {
  if (!listbox.isConnected || listbox.hidden) return finish();
  if (resetPending) {
    resetPending = false;
    highlight(initial());
  } else if (highlighted && !options().includes(highlighted)) {
    highlight(initial());
  }
});
await new Promise((resolve) => requestAnimationFrame(resolve));
if (!listbox.isConnected || listbox.hidden) return;
highlight(initial());
observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["hidden", "data-disabled", "aria-disabled"] });
if (keySource) keySource.addEventListener("keydown", onKeyDown);
if (resetsOnInput && anchor) anchor.addEventListener("input", onInput);
listbox.addEventListener("pointermove", onPointerMove);
listbox.addEventListener("pointerdown", onPointerDown);
listbox.addEventListener("click", onClick);
await ended;
observer.disconnect();
if (keySource) keySource.removeEventListener("keydown", onKeyDown);
if (resetsOnInput && anchor) anchor.removeEventListener("input", onInput);
listbox.removeEventListener("pointermove", onPointerMove);
listbox.removeEventListener("pointerdown", onPointerDown);
listbox.removeEventListener("click", onClick);
highlight(null);
if (isMenu) {
  // Focus still inside the hidden menu, or blurred to the body, goes back to
  // the anchor (a menubar may have switched menus since this one opened), or
  // without one to where it was before opening; focus moved to another
  // control stays there.
  const active = document.activeElement;
  const focusLeft = active !== null && active !== document.body && !listbox.contains(active);
  const returnTo = anchor || previous;
  if (!focusLeft && returnTo instanceof HTMLElement && returnTo.isConnected) returnTo.focus();
}
"#;

/// How the listbox script reads keys and shows the highlighted item.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListboxMode {
  /// Select: starts on the selected option, handles Home, End, Space, and
  /// typeahead.
  Select,
  /// Combobox: starts without a highlight and leaves typing to the input.
  Combobox,
  /// Command: like Select for Home and End, but leaves typing to the input
  /// and goes back to the first option when the query changes.
  Command,
  /// Menu: focuses the first item, moves DOM focus with wrapping arrows, Home,
  /// End, and typeahead, activates items by clicking them, and returns focus
  /// on close.
  Menu,
}

impl ListboxMode {
  fn name(self) -> &'static str {
    match self {
      Self::Select => "select",
      Self::Combobox => "combobox",
      Self::Command => "command",
      Self::Menu => "menu",
    }
  }

  /// Select, Combobox, and Command read keys from their anchor; menus read them from
  /// the focused item and also run without an anchor.
  fn needs_anchor(self) -> bool {
    match self {
      Self::Select => true,
      Self::Combobox => true,
      Self::Command => true,
      Self::Menu => false,
    }
  }
}

/// Runs the listbox script each time `open` turns true, with an `anchor_id`
/// unless the mode is a menu. Choosing an option calls `on_value_change(value)` and then
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
      if open == was_open.replace(open) || !open || (anchor_id.is_none() && mode.needs_anchor()) {
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

static NEXT_ROVING_GROUP_ID: AtomicUsize = AtomicUsize::new(0);

// Runs for the group's lifetime and reads items from the DOM on every event.
// The root sets `data-dxui-roving-orientation` (horizontal, vertical, or both),
// `data-dxui-roving-loop`, and `data-dxui-roving-activation` ("focus" when
// selection follows focus, "manual" when items carry a selection that does
// not). With `data-dxui-roving-tab-stops="all"` the script
// leaves `tabindex` alone, so every enabled item stays in the Tab order. Sends
// the `data-value` of a clicked item, and of a newly focused item when
// selection follows focus.
// Keep in sync with `ROVING_GROUP_SCRIPT` in the CLI `utils.rs` template.
pub const ROVING_GROUP_SCRIPT: &str = r#"
const scopeId = await dioxus.recv();
const root = document.querySelector(`[data-dxui-roving-group="${scopeId}"]`);
if (!root) return;
const itemSelector = "[data-dxui-roving-item]";
// Items inside a nested group belong to that group.
const items = () =>
  Array.from(root.querySelectorAll(itemSelector)).filter(
    (item) => item.closest("[data-dxui-roving-group]") === root,
  );
const enabledItems = () => items().filter((item) => !item.disabled);
const followsFocus = () => root.dataset.dxuiRovingActivation === "focus";
const hasSelection = () => ["focus", "manual"].includes(root.dataset.dxuiRovingActivation);
const singleTabStop = () => root.dataset.dxuiRovingTabStops !== "all";
const selected = (item) =>
  ["aria-selected", "aria-checked", "aria-pressed"].some((name) => item.getAttribute(name) === "true");
let lastFocused = null;
// One Tab stop: where items carry a selection, the selected item comes
// first; otherwise the item that last had focus does.
const setTabStop = () => {
  if (!singleTabStop()) return;
  const enabled = enabledItems();
  const chosen = enabled.find(selected);
  const last = enabled.includes(lastFocused) ? lastFocused : null;
  const stop = (hasSelection() ? chosen || last : last || chosen) || enabled[0];
  items().forEach((item) => {
    item.tabIndex = item === stop ? 0 : -1;
  });
};
const keys = {
  horizontal: [["ArrowRight"], ["ArrowLeft"]],
  vertical: [["ArrowDown"], ["ArrowUp"]],
  both: [["ArrowRight", "ArrowDown"], ["ArrowLeft", "ArrowUp"]],
};
// Returns the item to focus, the current item at an end without looping, or
// null for keys the group does not handle.
const step = (item, key) => {
  const enabled = enabledItems();
  const index = enabled.indexOf(item);
  if (index < 0) return null;
  const [nextKeys, previousKeys] = keys[root.dataset.dxuiRovingOrientation] || keys.both;
  const loop = root.dataset.dxuiRovingLoop !== "false";
  const last = enabled.length - 1;
  if (nextKeys.includes(key)) return index < last ? enabled[index + 1] : loop ? enabled[0] : item;
  if (previousKeys.includes(key)) return index > 0 ? enabled[index - 1] : loop ? enabled[last] : item;
  if (key === "Home") return enabled[0];
  if (key === "End") return enabled[last];
  return null;
};
// In a right-to-left layout, ArrowLeft points at the next item.
const visualKey = (key) => {
  if (getComputedStyle(root).direction !== "rtl") return key;
  if (key === "ArrowLeft") return "ArrowRight";
  if (key === "ArrowRight") return "ArrowLeft";
  return key;
};
const onKeyDown = (event) => {
  if (event.defaultPrevented || event.altKey || event.ctrlKey || event.metaKey) return;
  if (!items().includes(event.target)) return;
  const next = step(event.target, visualKey(event.key));
  if (!next) return;
  event.preventDefault();
  if (next === event.target) return;
  next.focus();
  if (followsFocus()) dioxus.send(next.dataset.value || "");
};
const onClick = (event) => {
  const item = event.target instanceof Element ? event.target.closest(itemSelector) : null;
  if (item && !item.disabled && items().includes(item)) dioxus.send(item.dataset.value || "");
};
const onFocusIn = (event) => {
  if (!items().includes(event.target)) return;
  lastFocused = event.target;
  if (!singleTabStop()) return;
  event.target.tabIndex = 0;
  items().forEach((item) => {
    if (item !== event.target) item.tabIndex = -1;
  });
};
// Leaving the group moves the Tab stop back to its preferred item. The
// browser has already picked the next focus target.
const onFocusOut = (event) => {
  if (items().includes(event.target) && !items().includes(event.relatedTarget)) setTabStop();
};
let finish;
const ended = new Promise((resolve) => {
  finish = resolve;
});
const observer = new MutationObserver(() => {
  if (!root.isConnected) return finish();
  // Focus stays the Tab stop until selection catches up with it.
  if (!items().includes(document.activeElement)) setTabStop();
});
setTabStop();
observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["aria-selected", "aria-checked", "aria-pressed", "disabled"] });
root.addEventListener("keydown", onKeyDown);
root.addEventListener("click", onClick);
root.addEventListener("focusin", onFocusIn);
root.addEventListener("focusout", onFocusOut);
await ended;
observer.disconnect();
"#;

/// Builds a trigger or panel id for a group part. Characters that are not
/// ASCII letters, digits, `-`, or `_` become `-` so the id is a valid IDREF.
pub fn group_part_id(base_id: &str, part: &str, value: &str) -> String {
  let value: String = value
    .chars()
    .map(|character| {
      if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
        character
      } else {
        '-'
      }
    })
    .collect();

  format!("{base_id}-{part}-{value}")
}

/// Runs the roving group script for the component's lifetime. Clicks, and
/// focus moves where the root sets `data-dxui-roving-activation="focus"`, call
/// `on_activate` with the item's `data-value`.
///
/// Returns the value for the root's `data-dxui-roving-group` attribute.
pub fn use_roving_group(on_activate: Option<EventHandler<String>>) -> String {
  let scope_id = use_hook(|| {
    format!("dxui-roving-group-{}", NEXT_ROVING_GROUP_ID.fetch_add(1, Ordering::Relaxed))
  });
  let effect_scope_id = scope_id.clone();

  use_effect(move || {
    let mut eval = document::eval(ROVING_GROUP_SCRIPT);
    // A send error means the page already finished the script; nothing to track.
    let _ = eval.send(effect_scope_id.as_str());
    spawn(async move {
      while let Ok(value) = eval.recv::<String>().await {
        if let Some(handler) = on_activate {
          handler.call(value);
        }
      }
    });
  });

  scope_id
}

static NEXT_HOVER_OPEN_ID: AtomicUsize = AtomicUsize::new(0);

// Runs for the root's lifetime and finds the trigger and content on every
// event. Sends "open" or "close" only when the request changes the content's
// `data-state`. Parts inside a nested root belong to that root.
// Keep in sync with `HOVER_OPEN_SCRIPT` in the CLI `utils.rs` template.
pub const HOVER_OPEN_SCRIPT: &str = r#"
const [scopeId, openDelayMs, closeDelayMs, closeOnPress, describes] = await dioxus.recv();
const root = document.querySelector(`[data-dxui-hover-open="${scopeId}"]`);
if (!root) return;
const part = (name) =>
  Array.from(root.querySelectorAll(`[data-dxui-hover-${name}]`)).find(
    (element) => element.closest("[data-dxui-hover-open]") === root,
  );
const trigger = () => part("trigger");
const content = () => part("content");
const isOpen = () => content()?.dataset.state === "open";
const inside = (element, node) => node instanceof Node && !!element && element.contains(node);
const insideParts = (node) => inside(trigger(), node) || inside(content(), node);
let openTimer = null;
let closeTimer = null;
let hovered = false;
// A press's own focus does not open. With `closeOnPress`, hover opens only
// when the pointer enters the trigger, so after a press it stays closed until
// the pointer leaves and returns.
let pressing = false;
const clearTimers = () => {
  clearTimeout(openTimer);
  clearTimeout(closeTimer);
  openTimer = closeTimer = null;
};
const request = (open) => {
  clearTimers();
  if (open !== isOpen()) dioxus.send(open ? "open" : "close");
};
// Moving between the trigger and the content, across the gap within the close
// delay, counts as staying.
const onPointerOver = (event) => {
  if (event.pointerType === "touch") return;
  if (!insideParts(event.target) || insideParts(event.relatedTarget)) return;
  hovered = true;
  clearTimeout(closeTimer);
  if (isOpen() || !inside(trigger(), event.target)) return;
  clearTimeout(openTimer);
  openTimer = setTimeout(() => request(true), openDelayMs);
};
const onPointerOut = (event) => {
  if (event.pointerType === "touch") return;
  if (!insideParts(event.target) || insideParts(event.relatedTarget)) return;
  hovered = false;
  clearTimeout(openTimer);
  if (!isOpen()) return;
  clearTimeout(closeTimer);
  closeTimer = setTimeout(() => request(false), closeDelayMs);
};
const onPointerDown = (event) => {
  if (!inside(trigger(), event.target)) return;
  pressing = true;
  if (closeOnPress) request(false);
};
const onPointerUp = () => {
  pressing = false;
};
const onFocusIn = (event) => {
  if (inside(trigger(), event.target) && !pressing) request(true);
};
// Focus moving between the parts keeps it open. Focus leaving while the
// pointer rests on the parts, such as a press on the content's text, leaves
// closing to the pointer.
const onFocusOut = (event) => {
  if (insideParts(event.target) && !insideParts(event.relatedTarget) && !hovered) request(false);
};
const describe = () => {
  const button = trigger();
  const described = content();
  if (!describes || !button) return;
  if (described && described.id && isOpen()) button.setAttribute("aria-describedby", described.id);
  else button.removeAttribute("aria-describedby");
};
let finish;
const ended = new Promise((resolve) => {
  finish = resolve;
});
const observer = new MutationObserver(() => {
  if (!root.isConnected) return finish();
  describe();
});
describe();
observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["data-state"] });
root.addEventListener("pointerover", onPointerOver);
root.addEventListener("pointerout", onPointerOut);
root.addEventListener("pointerdown", onPointerDown);
root.addEventListener("focusin", onFocusIn);
root.addEventListener("focusout", onFocusOut);
document.addEventListener("pointerup", onPointerUp, true);
document.addEventListener("pointercancel", onPointerUp, true);
await ended;
clearTimers();
observer.disconnect();
document.removeEventListener("pointerup", onPointerUp, true);
document.removeEventListener("pointercancel", onPointerUp, true);
"#;

/// How a hover-open root times and links its parts.
#[derive(Clone, Copy, PartialEq)]
pub struct HoverOpenOptions {
  pub open_delay_ms: u32,
  pub close_delay_ms: u32,
  /// A press on the trigger closes the content, and it stays closed while the
  /// pointer rests on the trigger.
  pub close_on_press: bool,
  /// The trigger's `aria-describedby` names the content while it is open.
  pub describe_trigger: bool,
}

/// Runs the hover-open script for the component's lifetime. Hover and focus
/// requests call `on_open_change`; `options` are read when the root mounts.
///
/// Returns the value for the root's `data-dxui-hover-open` attribute. The
/// trigger carries `data-dxui-hover-trigger` and the content
/// `data-dxui-hover-content`.
pub fn use_hover_open(
  on_open_change: Option<EventHandler<bool>>,
  options: HoverOpenOptions,
) -> String {
  let scope_id =
    use_hook(|| format!("dxui-hover-{}", NEXT_HOVER_OPEN_ID.fetch_add(1, Ordering::Relaxed)));
  let effect_scope_id = scope_id.clone();

  use_effect(move || {
    let mut eval = document::eval(HOVER_OPEN_SCRIPT);
    // A send error means the page already finished the script; nothing to track.
    let _ = eval.send((
      effect_scope_id.as_str(),
      options.open_delay_ms,
      options.close_delay_ms,
      options.close_on_press,
      options.describe_trigger,
    ));
    spawn(async move {
      while let Ok(message) = eval.recv::<String>().await {
        if let Some(handler) = on_open_change {
          handler.call(message == "open");
        }
      }
    });
  });

  scope_id
}
