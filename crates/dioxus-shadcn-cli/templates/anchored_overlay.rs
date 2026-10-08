use std::cell::Cell;
use std::rc::Rc;

use super::element_id::next_element_id;
use super::overlay::{DismissBehavior, OverlayAlign, OverlaySide};
use super::script::{Script, component_script};
use super::layer::use_layer;
use dioxus::prelude::*;

// Places anchored content with the flip and shift rules of
// `compute_overlay_placement`, done in the page to avoid a round trip per
// layout change, and reports Escape and outside interactions to Rust.
component_script!(anchored_overlay_script = r#"
export async function run(dioxus) {
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
  // A submenu (RFC 0067) asks for the right side, meaning its trigger's inline
  // end, so it opens to the left in right-to-left layouts.
  const mirrored =
    content.hasAttribute("data-dxui-submenu") && anchor !== null && getComputedStyle(anchor).direction === "rtl";
  const preferred = mirrored ? opposite[preferredSide] || preferredSide : preferredSide;
  const vertical = (side) => side === "top" || side === "bottom";
  const cross = (start, anchorSize, size) =>
    align === "start" ? start : align === "end" ? start + anchorSize - size : start + (anchorSize - size) / 2;
  const clamp = (value, size, viewportSize) =>
    Math.min(Math.max(value, padding), Math.max(padding, viewportSize - padding - size));
  const place = () => {
    const rect = anchorRect();
    if (!rect || !(preferred in opposite)) return;
    // Fixed positioning and the anchor width (a minimum width for lists) can
    // change the content's size, so apply them before measuring.
    Object.assign(content.style, { position: "fixed", margin: "0" });
    content.style.setProperty("--dxui-anchor-width", `${rect.width}px`);
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
    const required = (vertical(preferred) ? height : width) + Math.max(offset, 0);
    let side = preferred;
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
  content.style.removeProperty("--dxui-anchor-width");
}
"#);

/// Where anchored content goes relative to the element with id `anchor_id`, or
/// to the viewport point `anchor_point` when there is no anchor element.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AnchoredPlacement {
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

/// Whether a message from `anchored_overlay_script` should close the overlay.
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
  let scope_id = use_hook(|| format!("dxui-anchored-{}", next_element_id()));
  let was_open = use_hook(|| Rc::new(Cell::new(false)));
  // Counted among the open overlays, so a dialog under it leaves Escape to
  // it; menus and submenus share Escape through their own scripts.
  use_layer(open);
  let effect_scope_id = scope_id.clone();

  use_effect(use_reactive(
    (&open, &placement, &dismiss, &on_open_change),
    move |(open, placement, dismiss, on_open_change)| {
      if open == was_open.replace(open) || !open {
        return;
      }
      let script = anchored_overlay_script::start();
      // A send error means the page already finished the script; nothing to place.
      let _ = script.send((
        effect_scope_id.as_str(),
        placement.anchor_id.as_deref(),
        side_name(placement.side),
        align_name(placement.align),
        placement.side_offset,
        placement.anchor_point,
      ));
      spawn(async move {
        while let Ok(message) = script.recv::<String>().await {
          if let Some(handler) = on_open_change.filter(|_| dismisses(&dismiss, &message)) {
            handler.call(false);
          }
        }
      });
    },
  ));

  scope_id
}
