use super::element_id::next_element_id;
use super::utils::classes;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LayoutOrientation {
  #[default]
  Horizontal,
  Vertical,
}

pub fn layout_orientation_attribute(orientation: LayoutOrientation) -> &'static str {
  match orientation {
    LayoutOrientation::Horizontal => "horizontal",
    LayoutOrientation::Vertical => "vertical",
  }
}

pub fn resizable_clamp(size: f64, min_size: f64, max_size: f64) -> f64 {
  let (min_size, max_size) = ordered_bounds(min_size, max_size);
  let size = finite_or_default(size, min_size);

  size.clamp(min_size, max_size)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResizablePanelState {
  pub size: f64,
  pub min_size: f64,
  pub max_size: f64,
  pub collapsed: bool,
}

impl ResizablePanelState {
  pub fn new(size: f64, min_size: f64, max_size: f64) -> Self {
    let (min_size, max_size) = ordered_bounds(min_size, max_size);
    let size = resizable_clamp(size, min_size, max_size);

    Self { size, min_size, max_size, collapsed: false }
  }

  pub fn with_size(self, size: f64) -> Self {
    Self { size: resizable_clamp(size, self.min_size, self.max_size), ..self }
  }
}

pub fn resizable_resize_pair(
  first: ResizablePanelState,
  second: ResizablePanelState,
  delta: f64,
) -> (ResizablePanelState, ResizablePanelState) {
  let delta = finite_or_default(delta, 0.0);
  let first_size = resizable_clamp(first.size + delta, first.min_size, first.max_size);
  let consumed_delta = first_size - first.size;
  let second_size = resizable_clamp(second.size - consumed_delta, second.min_size, second.max_size);
  let second_consumed_delta = second.size - second_size;
  let first_size =
    resizable_clamp(first.size + second_consumed_delta, first.min_size, first.max_size);

  (first.with_size(first_size), second.with_size(second_size))
}

fn finite_or_default(value: f64, default: f64) -> f64 {
  if value.is_finite() { value } else { default }
}

fn ordered_bounds(min_size: f64, max_size: f64) -> (f64, f64) {
  let min_size = finite_or_default(min_size, 0.0);
  let max_size = finite_or_default(max_size, min_size);

  if min_size <= max_size { (min_size, max_size) } else { (max_size, min_size) }
}

// Runs for the handle's lifetime. A primary-button press captures the pointer
// and records the pointer position and `aria-valuenow`; each move sends the
// target size of the panel before the handle, from the pointer offset over the
// group (the handle's parent) size along `data-orientation`. Sending targets
// instead of per-move deltas keeps the edge under the pointer after a limit.
// Keep in sync with `RESIZABLE_HANDLE_SCRIPT` in `dioxus-shadcn`'s `resizable.rs`.
pub(crate) const RESIZABLE_HANDLE_SCRIPT: &str = r#"
const scopeId = await dioxus.recv();
const handle = document.querySelector(`[data-dxui-resizable-handle="${scopeId}"]`);
if (!handle) return;
const vertical = () => handle.dataset.orientation === "vertical";
const position = (event) => (vertical() ? event.clientY : event.clientX);
let start = null;
const onPointerDown = (event) => {
  if (handle.getAttribute("aria-disabled") === "true" || event.button !== 0) return;
  // Keeps the press from selecting text; focus moves explicitly instead.
  event.preventDefault();
  handle.setPointerCapture(event.pointerId);
  handle.focus();
  start = { pointer: position(event), value: Number(handle.getAttribute("aria-valuenow")) };
};
const onPointerMove = (event) => {
  if (!start || !handle.hasPointerCapture(event.pointerId)) return;
  const rect = handle.parentElement.getBoundingClientRect();
  const size = vertical() ? rect.height : rect.width;
  if (size <= 0) return;
  dioxus.send(start.value + ((position(event) - start.pointer) / size) * 100);
};
const onPointerUp = (event) => {
  if (handle.hasPointerCapture(event.pointerId)) handle.releasePointerCapture(event.pointerId);
  start = null;
};
let finish;
const ended = new Promise((resolve) => {
  finish = resolve;
});
const observer = new MutationObserver(() => {
  if (!handle.isConnected) finish();
});
observer.observe(document.documentElement, { subtree: true, childList: true });
handle.addEventListener("pointerdown", onPointerDown);
handle.addEventListener("pointermove", onPointerMove);
handle.addEventListener("pointerup", onPointerUp);
handle.addEventListener("pointercancel", onPointerUp);
await ended;
observer.disconnect();
"#;

pub const RESIZABLE_PANEL_GROUP_BASE_CLASS: &str =
  "flex h-full w-full data-[orientation=vertical]:flex-col";
pub const RESIZABLE_PANEL_BASE_CLASS: &str = "min-w-0 overflow-hidden";
pub const RESIZABLE_HANDLE_BASE_CLASS: &str = "relative flex w-px cursor-col-resize touch-none items-center justify-center bg-border after:absolute after:inset-y-0 after:left-1/2 after:w-1 after:-translate-x-1/2 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring data-[orientation=vertical]:h-px data-[orientation=vertical]:w-full data-[orientation=vertical]:cursor-row-resize data-[orientation=vertical]:after:inset-x-0 data-[orientation=vertical]:after:top-1/2 data-[orientation=vertical]:after:h-1 data-[orientation=vertical]:after:w-full data-[orientation=vertical]:after:-translate-y-1/2 data-[disabled=true]:opacity-50";

pub fn resizable_panel_group_class(orientation: LayoutOrientation, class: &str) -> String {
  let orientation_class = match orientation {
    LayoutOrientation::Horizontal => "flex-row",
    LayoutOrientation::Vertical => "flex-col",
  };

  classes([Some(RESIZABLE_PANEL_GROUP_BASE_CLASS), Some(orientation_class), Some(class)])
}

pub fn resizable_panel_class(collapsed: bool, class: &str) -> String {
  classes([Some(RESIZABLE_PANEL_BASE_CLASS), collapsed.then_some("hidden"), Some(class)])
}

pub fn resizable_handle_class(disabled: bool, class: &str) -> String {
  classes([
    Some(RESIZABLE_HANDLE_BASE_CLASS),
    disabled.then_some("pointer-events-none opacity-50"),
    Some(class),
  ])
}

pub fn resizable_panel_style(size: f64, min_size: f64, max_size: f64) -> String {
  let size = resizable_clamp(size, min_size, max_size);

  format!("flex-basis: {size}%;")
}

/// ARIA orients a separator by its line, which runs across the group.
pub fn resizable_separator_orientation(orientation: LayoutOrientation) -> &'static str {
  match orientation {
    LayoutOrientation::Horizontal => "vertical",
    LayoutOrientation::Vertical => "horizontal",
  }
}

/// The size delta a key requests for the panel before the handle, or `None`
/// for keys the handle ignores.
pub fn resizable_handle_key_delta(
  key: &str,
  orientation: LayoutOrientation,
  value: f64,
  min: f64,
  max: f64,
  step: f64,
) -> Option<f64> {
  match (orientation, key) {
    (LayoutOrientation::Horizontal, "ArrowRight") | (LayoutOrientation::Vertical, "ArrowDown") => {
      Some(step)
    }
    (LayoutOrientation::Horizontal, "ArrowLeft") | (LayoutOrientation::Vertical, "ArrowUp") => {
      Some(-step)
    }
    (_, "Home") => Some(min - value),
    (_, "End") => Some(max - value),
    _ => None,
  }
}

#[component]
pub fn ResizablePanelGroup(
  #[props(default)] orientation: LayoutOrientation,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = resizable_panel_group_class(orientation, &class);

  rsx! {
    div {
      class,
      "data-orientation": layout_orientation_attribute(orientation),
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn ResizablePanel(
  #[props(default = 50.0)] size: f64,
  #[props(default = 0.0)] min_size: f64,
  #[props(default = 100.0)] max_size: f64,
  #[props(default)] collapsed: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = resizable_panel_class(collapsed, &class);
  let style = resizable_panel_style(size, min_size, max_size);

  rsx! {
    div {
      class,
      style,
      hidden: collapsed,
      "data-collapsed": collapsed.to_string(),
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn ResizableHandle(
  #[props(default)] orientation: LayoutOrientation,
  #[props(default)] disabled: bool,
  #[props(default = 50.0)] value: f64,
  #[props(default = 0.0)] min: f64,
  #[props(default = 100.0)] max: f64,
  #[props(default = 10.0)] step: f64,
  #[props(default)] on_resize: Option<EventHandler<f64>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
  let class = resizable_handle_class(disabled, &class);
  let scope_id = use_resizable_handle_pointer(value, min, max, disabled, on_resize);

  rsx! {
    div {
      role: "separator",
      class,
      tabindex: (!disabled).then_some("0"),
      "aria-disabled": disabled.to_string(),
      "aria-orientation": resizable_separator_orientation(orientation),
      "aria-valuenow": value.to_string(),
      "aria-valuemin": min.to_string(),
      "aria-valuemax": max.to_string(),
      "data-disabled": disabled.to_string(),
      "data-orientation": layout_orientation_attribute(orientation),
      "data-dxui-resizable-handle": scope_id,
      onkeydown: move |event: KeyboardEvent| {
        if disabled {
          return;
        }
        let key = event.key().to_string();
        let Some(delta) = resizable_handle_key_delta(&key, orientation, value, min, max, step) else {
          return;
        };
        // Arrow, Home, and End keys would otherwise scroll the page.
        event.prevent_default();
        if delta != 0.0 {
          if let Some(handler) = on_resize {
            handler.call(delta);
          }
        }
      },
      ..attributes,
    }
  }
}

/// Runs the pointer script for the handle's lifetime and returns the value for
/// its `data-dxui-resizable-handle` attribute. Targets from the script are
/// clamped against the latest props and reported as a delta from `value`.
fn use_resizable_handle_pointer(
  value: f64,
  min: f64,
  max: f64,
  disabled: bool,
  on_resize: Option<EventHandler<f64>>,
) -> String {
  let scope_id = use_hook(|| format!("dxui-resizable-handle-{}", next_element_id()));
  // The receive loop outlives this render, so it reads the latest props here.
  let mut latest = use_hook(|| CopyValue::new((value, min, max, disabled)));
  latest.set((value, min, max, disabled));
  let effect_scope_id = scope_id.clone();

  use_effect(move || {
    let mut eval = document::eval(RESIZABLE_HANDLE_SCRIPT);
    // A send error means the page already finished the script; nothing to track.
    let _ = eval.send(effect_scope_id.as_str());
    spawn(async move {
      while let Ok(target) = eval.recv::<f64>().await {
        let (value, min, max, disabled) = latest();
        let delta = resizable_clamp(target, min, max) - value;
        if disabled || delta == 0.0 {
          continue;
        }
        if let Some(handler) = on_resize {
          handler.call(delta);
        }
      }
    });
  });

  scope_id
}
