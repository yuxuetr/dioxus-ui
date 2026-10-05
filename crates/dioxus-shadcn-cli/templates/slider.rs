use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use super::utils::classes;

static NEXT_SLIDER_ID: AtomicUsize = AtomicUsize::new(0);

// Runs for the slider's lifetime. A primary-button press captures the pointer
// and sends the value under it, and so does each move until release. Bounds
// and step are read from the root at event time, so prop changes apply
// without a restart.
// Keep in sync with `SLIDER_POINTER_SCRIPT` in `dioxus-shadcn`'s `slider.rs`.
pub(crate) const SLIDER_POINTER_SCRIPT: &str = r#"
const scopeId = await dioxus.recv();
const root = document.querySelector(`[data-dxui-slider="${scopeId}"]`);
if (!root) return;
const disabled = () => root.getAttribute("aria-disabled") === "true";
const valueAt = (event) => {
  const min = Number(root.getAttribute("aria-valuemin"));
  const max = Number(root.getAttribute("aria-valuemax"));
  const step = Number(root.dataset.step) || 1;
  const rect = root.getBoundingClientRect();
  // A vertical slider grows from the bottom.
  const vertical = root.dataset.orientation === "vertical";
  const size = vertical ? rect.height : rect.width;
  if (size <= 0 || max <= min) return min;
  const offset = vertical ? rect.bottom - event.clientY : event.clientX - rect.left;
  const ratio = Math.min(1, Math.max(0, offset / size));
  const value = min + Math.round((ratio * (max - min)) / step) * step;
  return Math.min(max, Math.max(min, value));
};
const send = (event) => {
  const value = valueAt(event);
  if (value !== Number(root.getAttribute("aria-valuenow"))) dioxus.send(value);
};
const onPointerDown = (event) => {
  if (disabled() || event.button !== 0) return;
  // Keeps the press from selecting text; focus moves explicitly instead.
  event.preventDefault();
  root.setPointerCapture(event.pointerId);
  root.focus();
  send(event);
};
const onPointerMove = (event) => {
  if (root.hasPointerCapture(event.pointerId)) send(event);
};
const onPointerUp = (event) => {
  if (root.hasPointerCapture(event.pointerId)) root.releasePointerCapture(event.pointerId);
};
let finish;
const ended = new Promise((resolve) => {
  finish = resolve;
});
const observer = new MutationObserver(() => {
  if (!root.isConnected) finish();
});
observer.observe(document.documentElement, { subtree: true, childList: true });
root.addEventListener("pointerdown", onPointerDown);
root.addEventListener("pointermove", onPointerMove);
root.addEventListener("pointerup", onPointerUp);
root.addEventListener("pointercancel", onPointerUp);
await ended;
observer.disconnect();
"#;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderState {
  pub value: f64,
  pub min: f64,
  pub max: f64,
  pub step: f64,
  pub page_step: f64,
}

impl SliderState {
  pub fn new(value: f64, min: f64, max: f64, step: f64) -> Self {
    let (min, max) = ordered_bounds(min, max);
    let step = positive_or_default(step, 1.0);
    let value = snap_value(value, min, max, step);

    Self {
      value,
      min,
      max,
      step,
      page_step: step * 10.0,
    }
  }

  pub fn with_value(self, value: f64) -> Self {
    Self {
      value: snap_value(value, self.min, self.max, self.step),
      ..self
    }
  }

  pub fn moved(self, movement: SliderKeyMove) -> Self {
    let value = match movement {
      SliderKeyMove::Decrement => self.value - self.step,
      SliderKeyMove::Increment => self.value + self.step,
      SliderKeyMove::PageDecrement => self.value - self.page_step,
      SliderKeyMove::PageIncrement => self.value + self.page_step,
      SliderKeyMove::Home => self.min,
      SliderKeyMove::End => self.max,
    };

    self.with_value(value)
  }

  pub fn percent(self) -> f64 {
    if self.max <= self.min {
      return 0.0;
    }

    ((self.value - self.min) / (self.max - self.min) * 100.0).clamp(0.0, 100.0)
  }

  pub fn aria_attributes(self) -> SliderAriaAttributes {
    SliderAriaAttributes {
      aria_valuemin: self.min,
      aria_valuemax: self.max,
      aria_valuenow: self.value,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SliderKeyMove {
  Decrement,
  Increment,
  PageDecrement,
  PageIncrement,
  Home,
  End,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderAriaAttributes {
  pub aria_valuemin: f64,
  pub aria_valuemax: f64,
  pub aria_valuenow: f64,
}

pub const SLIDER_ROOT_BASE_CLASS: &str =
  "relative flex touch-none select-none items-center disabled:opacity-50";
pub const SLIDER_TRACK_BASE_CLASS: &str =
  "relative grow overflow-hidden rounded-full bg-muted";
pub const SLIDER_RANGE_BASE_CLASS: &str = "absolute rounded-full bg-primary";
pub const SLIDER_THUMB_BASE_CLASS: &str = "block h-5 w-5 rounded-full border-2 border-primary bg-background shadow transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

pub fn slider_root_class(class: &str) -> String {
  classes([Some(SLIDER_ROOT_BASE_CLASS), Some(class)])
}

pub fn slider_track_class(class: &str) -> String {
  classes([Some(SLIDER_TRACK_BASE_CLASS), Some(class)])
}

pub fn slider_range_class(class: &str) -> String {
  classes([Some(SLIDER_RANGE_BASE_CLASS), Some(class)])
}

pub fn slider_thumb_class(class: &str) -> String {
  classes([Some(SLIDER_THUMB_BASE_CLASS), Some(class)])
}

pub fn slider_state(value: f64, min: f64, max: f64, step: f64) -> SliderState {
  SliderState::new(value, min, max, step)
}

pub fn slider_percent(value: f64, min: f64, max: f64, step: f64) -> f64 {
  slider_state(value, min, max, step).percent()
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SliderOrientation {
  #[default]
  Horizontal,
  Vertical,
}

impl SliderOrientation {
  pub fn attribute(self) -> &'static str {
    match self {
      Self::Horizontal => "horizontal",
      Self::Vertical => "vertical",
    }
  }
}

pub fn slider_range_style(orientation: SliderOrientation, percent: f64) -> String {
  let percent = percent.clamp(0.0, 100.0);

  match orientation {
    SliderOrientation::Horizontal => format!("left: 0%; width: {percent}%;"),
    SliderOrientation::Vertical => format!("bottom: 0%; height: {percent}%;"),
  }
}

/// Centers the thumb on the value along the root. The position is inline so
/// it does not depend on compiled Tailwind classes.
pub fn slider_thumb_style(orientation: SliderOrientation, percent: f64) -> String {
  let percent = percent.clamp(0.0, 100.0);

  match orientation {
    SliderOrientation::Horizontal => {
      format!("position: absolute; left: {percent}%; top: 50%; transform: translate(-50%, -50%);")
    }
    SliderOrientation::Vertical => {
      format!("position: absolute; bottom: {percent}%; left: 50%; transform: translate(-50%, 50%);")
    }
  }
}

/// Classes that size a vertical slider's root, track, and range along its
/// height.
fn slider_orientation_classes(
  orientation: SliderOrientation,
) -> (Option<&'static str>, Option<&'static str>, Option<&'static str>) {
  match orientation {
    SliderOrientation::Horizontal => (Some("w-full"), Some("h-2 w-full"), Some("h-full")),
    SliderOrientation::Vertical => {
      (Some("h-full w-5 flex-col"), Some("h-full w-2"), Some("w-full"))
    }
  }
}

/// Maps a `KeyboardEvent.key` name to a slider move, following the WAI-ARIA
/// slider pattern. Returns `None` for keys the slider leaves alone.
pub fn slider_key_move(key: &str) -> Option<SliderKeyMove> {
  match key {
    "ArrowRight" | "ArrowUp" => Some(SliderKeyMove::Increment),
    "ArrowLeft" | "ArrowDown" => Some(SliderKeyMove::Decrement),
    "PageUp" => Some(SliderKeyMove::PageIncrement),
    "PageDown" => Some(SliderKeyMove::PageDecrement),
    "Home" => Some(SliderKeyMove::Home),
    "End" => Some(SliderKeyMove::End),
    _ => None,
  }
}

pub fn slider_aria_attributes(
  value: f64,
  min: f64,
  max: f64,
  step: f64,
) -> SliderAriaAttributes {
  slider_state(value, min, max, step).aria_attributes()
}

fn snap_value(value: f64, min: f64, max: f64, step: f64) -> f64 {
  let clamped = slider_clamp(value, min, max);
  let steps = ((clamped - min) / step).round();
  let snapped = min + steps * step;

  slider_clamp(snapped, min, max)
}

fn slider_clamp(value: f64, min: f64, max: f64) -> f64 {
  let (min, max) = ordered_bounds(min, max);

  if value.is_finite() {
    value.clamp(min, max)
  } else {
    min
  }
}

fn ordered_bounds(min: f64, max: f64) -> (f64, f64) {
  let min = finite_or_default(min, 0.0);
  let max = finite_or_default(max, min);

  if min <= max {
    (min, max)
  } else {
    (max, min)
  }
}

fn positive_or_default(value: f64, default: f64) -> f64 {
  if value.is_finite() && value > 0.0 {
    value
  } else {
    default
  }
}

fn finite_or_default(value: f64, default: f64) -> f64 {
  if value.is_finite() {
    value
  } else {
    default
  }
}

/// A controlled slider. Arrow, Page Up, Page Down, Home, and End keys and a
/// pointer press or drag call `on_value_change` with the new snapped value
/// when it differs from `value`; the app passes it back as `value`. Other
/// attributes, such as `aria-label` and `aria-valuetext`, are passed to the
/// root. A disabled slider ignores keys and the pointer.
#[component]
pub fn Slider(
  #[props(default)] value: f64,
  #[props(default = 0.0)] min: f64,
  #[props(default = 100.0)] max: f64,
  #[props(default = 1.0)] step: f64,
  #[props(default)] orientation: SliderOrientation,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] track_class: String,
  #[props(default)] range_class: String,
  #[props(default)] thumb_class: String,
  #[props(default)] on_value_change: Option<EventHandler<f64>>,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
  let state = slider_state(value, min, max, step);
  let scope_id = use_slider_pointer(state, disabled, on_value_change);
  let percent = state.percent();
  let aria = state.aria_attributes();
  let (root_orientation, track_orientation, range_orientation) =
    slider_orientation_classes(orientation);
  let root_class = slider_root_class(&classes([root_orientation, Some(class.as_str())]));
  let track_class = slider_track_class(&classes([track_orientation, Some(track_class.as_str())]));
  let range_class = slider_range_class(&classes([range_orientation, Some(range_class.as_str())]));
  let thumb_class = slider_thumb_class(&thumb_class);
  let range_style = slider_range_style(orientation, percent);
  let thumb_style = slider_thumb_style(orientation, percent);

  rsx! {
    div {
      role: "slider",
      class: root_class,
      tabindex: if disabled { "-1" } else { "0" },
      "aria-disabled": disabled.to_string(),
      "aria-orientation": orientation.attribute(),
      "data-orientation": orientation.attribute(),
      "aria-valuemin": aria.aria_valuemin.to_string(),
      "aria-valuemax": aria.aria_valuemax.to_string(),
      "aria-valuenow": aria.aria_valuenow.to_string(),
      "data-value": state.value.to_string(),
      "data-step": state.step.to_string(),
      "data-disabled": disabled.to_string(),
      "data-dxui-slider": scope_id,
      onkeydown: move |event: KeyboardEvent| {
        if disabled {
          return;
        }
        let Some(movement) = slider_key_move(&event.key().to_string()) else {
          return;
        };
        // Arrow, Page, Home, and End keys would otherwise scroll the page.
        event.prevent_default();
        let next = state.moved(movement).value;
        if next != state.value {
          if let Some(handler) = on_value_change {
            handler.call(next);
          }
        }
      },
      ..attributes,
      div {
        class: track_class,
        div {
          class: range_class,
          style: range_style,
        }
      }
      span {
        class: thumb_class,
        style: thumb_style,
      }
    }
  }
}

/// Runs the pointer script for the slider's lifetime and returns the value
/// for the root's `data-dxui-slider` attribute. Values from the script are
/// snapped against the latest props before reaching `on_value_change`.
fn use_slider_pointer(
  state: SliderState,
  disabled: bool,
  on_value_change: Option<EventHandler<f64>>,
) -> String {
  let scope_id =
    use_hook(|| format!("dxui-slider-{}", NEXT_SLIDER_ID.fetch_add(1, Ordering::Relaxed)));
  // The receive loop outlives this render, so it reads the latest props here.
  let mut latest = use_hook(|| CopyValue::new((state, disabled)));
  latest.set((state, disabled));
  let effect_scope_id = scope_id.clone();

  use_effect(move || {
    let mut eval = document::eval(SLIDER_POINTER_SCRIPT);
    // A send error means the page already finished the script; nothing to track.
    let _ = eval.send(effect_scope_id.as_str());
    spawn(async move {
      while let Ok(value) = eval.recv::<f64>().await {
        let (state, disabled) = latest();
        let next = state.with_value(value).value;
        if disabled || next == state.value {
          continue;
        }
        if let Some(handler) = on_value_change {
          handler.call(next);
        }
      }
    });
  });

  scope_id
}
