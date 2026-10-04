use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use dioxus_ui_primitives::{DismissBehavior, OverlayAlign, OverlaySide};

static NEXT_ANCHORED_ID: AtomicUsize = AtomicUsize::new(0);

// Places anchored content with the flip and shift rules of
// `compute_overlay_placement`, done in the page to avoid a round trip per
// layout change, and reports Escape and outside interactions to Rust.
// Keep in sync with `ANCHORED_OVERLAY_SCRIPT` in the CLI `utils.rs` template.
pub(crate) const ANCHORED_OVERLAY_SCRIPT: &str = r#"
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
pub(crate) struct AnchoredPlacement {
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
pub(crate) fn use_anchored_overlay(
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn anchored_messages_follow_dismiss_behavior() {
    let popover = DismissBehavior::popover_default();
    let tooltip = DismissBehavior::tooltip_default();

    assert!(dismisses(&popover, "escape"));
    assert!(dismisses(&popover, "pointer-outside"));
    assert!(dismisses(&popover, "focus-outside"));
    assert!(dismisses(&tooltip, "escape"));
    assert!(!dismisses(&tooltip, "pointer-outside"));
    assert!(!dismisses(&popover, "unknown"));
  }
}
