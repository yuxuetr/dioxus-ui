//! Slider: a controlled numeric value on a horizontal or vertical track, and
//! `RangeSlider`, a controlled pair of values with two thumbs.
use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
use dioxus_shadcn_primitives::{SliderKeyMove, SliderState};

use crate::density::{density_control_class, density_hit_area_class, use_density, with_density};
use crate::element_id::next_element_id;

// Runs for the slider's lifetime. A primary-button press captures the pointer
// and sends the value under it, and so does each move until release. Bounds
// and step are read from the root at event time, so prop changes apply
// without a restart.
// Keep in sync with `SLIDER_POINTER_SCRIPT` in the CLI `slider.rs` template.
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

const SLIDER_ROOT_BASE_CLASS: &str =
  "relative flex touch-none select-none items-center disabled:opacity-50";
const SLIDER_TRACK_BASE_CLASS: &str = "relative grow overflow-hidden rounded-full bg-muted";
const SLIDER_RANGE_BASE_CLASS: &str = "absolute rounded-full bg-primary";
const SLIDER_THUMB_BASE_CLASS: &str = "block h-5 w-5 rounded-full border-2 border-primary bg-background shadow transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

fn slider_root_class(class: &str) -> String {
  merge_classes(classes([Some(SLIDER_ROOT_BASE_CLASS)]), class)
}

fn slider_track_class(class: &str) -> String {
  merge_classes(classes([Some(SLIDER_TRACK_BASE_CLASS)]), class)
}

fn slider_range_class(class: &str) -> String {
  merge_classes(classes([Some(SLIDER_RANGE_BASE_CLASS)]), class)
}

fn slider_thumb_class(class: &str) -> String {
  merge_classes(classes([Some(SLIDER_THUMB_BASE_CLASS)]), class)
}

fn slider_state(value: f64, min: f64, max: f64, step: f64) -> SliderState {
  SliderState::new(value, min, max, step)
}

/// The direction a slider's track runs.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SliderOrientation {
  /// Runs left to right.
  #[default]
  Horizontal,
  /// Runs bottom to top.
  Vertical,
}

impl SliderOrientation {
  /// The `aria-orientation` and `data-orientation` value: `horizontal` or `vertical`.
  pub fn attribute(self) -> &'static str {
    match self {
      Self::Horizontal => "horizontal",
      Self::Vertical => "vertical",
    }
  }
}

fn slider_range_style(orientation: SliderOrientation, percent: f64) -> String {
  let percent = percent.clamp(0.0, 100.0);

  match orientation {
    SliderOrientation::Horizontal => format!("left: 0%; width: {percent}%;"),
    SliderOrientation::Vertical => format!("bottom: 0%; height: {percent}%;"),
  }
}

/// Centers the thumb on the value along the root. The position is inline so
/// it does not depend on compiled Tailwind classes.
fn slider_thumb_style(orientation: SliderOrientation, percent: f64) -> String {
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
  let root_class = slider_root_class(&merge_classes(
    classes([root_orientation, Some(density_control_class(use_density()))]),
    class.as_str(),
  ));
  let track_class =
    slider_track_class(&merge_classes(classes([track_orientation]), track_class.as_str()));
  let range_class =
    slider_range_class(&merge_classes(classes([range_orientation]), range_class.as_str()));
  let thumb_class =
    slider_thumb_class(&with_density(density_hit_area_class(use_density()), &thumb_class));
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
  let scope_id = use_hook(|| format!("dxui-slider-{}", next_element_id()));
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

// Runs for the range slider's lifetime. A primary-button press picks the
// nearer thumb (the upper one when the press is above both), captures the
// pointer, and sends `[thumb, value]` for the press and each move until
// release.
// Keep in sync with `RANGE_SLIDER_POINTER_SCRIPT` in the CLI `slider.rs` template.
pub(crate) const RANGE_SLIDER_POINTER_SCRIPT: &str = r#"
const scopeId = await dioxus.recv();
const root = document.querySelector(`[data-dxui-range-slider="${scopeId}"]`);
if (!root) return;
const thumbs = () => Array.from(root.querySelectorAll('[role="slider"]'));
const valueAt = (event) => {
  const min = Number(root.dataset.min);
  const max = Number(root.dataset.max);
  const step = Number(root.dataset.step) || 1;
  const rect = root.getBoundingClientRect();
  const vertical = root.dataset.orientation === "vertical";
  const size = vertical ? rect.height : rect.width;
  if (size <= 0 || max <= min) return min;
  const offset = vertical ? rect.bottom - event.clientY : event.clientX - rect.left;
  const ratio = Math.min(1, Math.max(0, offset / size));
  return Math.min(max, Math.max(min, min + Math.round((ratio * (max - min)) / step) * step));
};
let active = 0;
const onPointerDown = (event) => {
  if (root.dataset.disabled === "true" || event.button !== 0) return;
  event.preventDefault();
  const value = valueAt(event);
  const [low, high] = thumbs().map((thumb) => Number(thumb.getAttribute("aria-valuenow")));
  active = Math.abs(value - low) < Math.abs(value - high) || value < low ? 0 : 1;
  root.setPointerCapture(event.pointerId);
  thumbs()[active]?.focus();
  dioxus.send([active, value]);
};
const onPointerMove = (event) => {
  if (root.hasPointerCapture(event.pointerId)) dioxus.send([active, valueAt(event)]);
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

/// The values after moving thumb `thumb` (0 for the lower, 1 for the upper)
/// to `target`: snapped to `step` within `min..=max`, and kept `min_gap`
/// away from the other thumb. The input pair is snapped and ordered first.
fn range_slider_values(
  values: (f64, f64),
  thumb: usize,
  target: f64,
  min: f64,
  max: f64,
  step: f64,
  min_gap: f64,
) -> (f64, f64) {
  let state = slider_state(min, min, max, step);
  let snap = |value: f64| state.with_value(value).value;
  let (low, high) = (snap(values.0.min(values.1)), snap(values.0.max(values.1)));
  let gap = min_gap.max(0.0).min(state.max - state.min);
  if thumb == 0 {
    (snap(target).min(high - gap).max(state.min), high)
  } else {
    (low, snap(target).max(low + gap).min(state.max))
  }
}

/// A controlled slider with two thumbs, each a `slider` that keyboard and
/// pointer move as on `Slider`; a press moves the nearer thumb. The thumbs
/// stay `min_steps_between` steps apart. `on_value_change` receives the new
/// `(low, high)` pair. `start_label` and `end_label` name the thumbs; other
/// attributes are passed to the root, a `group`.
#[component]
pub fn RangeSlider(
  #[props(default)] value: (f64, f64),
  #[props(default = 0.0)] min: f64,
  #[props(default = 100.0)] max: f64,
  #[props(default = 1.0)] step: f64,
  #[props(default)] min_steps_between: u32,
  #[props(default)] orientation: SliderOrientation,
  #[props(default)] disabled: bool,
  #[props(default = "Minimum".to_string())] start_label: String,
  #[props(default = "Maximum".to_string())] end_label: String,
  #[props(default)] class: String,
  #[props(default)] track_class: String,
  #[props(default)] range_class: String,
  #[props(default)] thumb_class: String,
  #[props(default)] on_value_change: Option<EventHandler<(f64, f64)>>,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
  let state = slider_state(min, min, max, step);
  let gap = f64::from(min_steps_between) * state.step;
  let values = range_slider_values(value, 0, value.0.min(value.1), min, max, step, gap);
  let scope_id = use_hook(|| format!("dxui-range-slider-{}", next_element_id()));
  // The receive loop outlives this render, so it reads the latest props here.
  let mut latest = use_hook(|| CopyValue::new((values, min, max, step, gap, disabled)));
  latest.set((values, min, max, step, gap, disabled));
  let effect_scope_id = scope_id.clone();
  use_effect(move || {
    let mut eval = document::eval(RANGE_SLIDER_POINTER_SCRIPT);
    // A send error means the page already finished the script; nothing to track.
    let _ = eval.send(effect_scope_id.as_str());
    spawn(async move {
      while let Ok((thumb, target)) = eval.recv::<(usize, f64)>().await {
        let (values, min, max, step, gap, disabled) = latest();
        let next = range_slider_values(values, thumb, target, min, max, step, gap);
        if disabled || next == values {
          continue;
        }
        if let Some(handler) = on_value_change {
          handler.call(next);
        }
      }
    });
  });

  let low = slider_state(values.0, min, max, step).percent();
  let high = slider_state(values.1, min, max, step).percent();
  let (root_orientation, track_orientation, range_orientation) =
    slider_orientation_classes(orientation);
  let root_class = slider_root_class(&merge_classes(
    classes([root_orientation, Some(density_control_class(use_density()))]),
    class.as_str(),
  ));
  let track_class =
    slider_track_class(&merge_classes(classes([track_orientation]), track_class.as_str()));
  let range_class =
    slider_range_class(&merge_classes(classes([range_orientation]), range_class.as_str()));
  let thumb_class =
    slider_thumb_class(&with_density(density_hit_area_class(use_density()), &thumb_class));
  let range_style = match orientation {
    SliderOrientation::Horizontal => format!("left: {low}%; width: {}%;", high - low),
    SliderOrientation::Vertical => format!("bottom: {low}%; height: {}%;", high - low),
  };
  let thumbs = [
    (0, values.0, state.min, values.1 - gap, start_label, low),
    (1, values.1, values.0 + gap, state.max, end_label, high),
  ];

  rsx! {
    div {
      role: "group",
      class: root_class,
      "aria-disabled": disabled.to_string(),
      "data-orientation": orientation.attribute(),
      "data-min": state.min.to_string(),
      "data-max": state.max.to_string(),
      "data-step": state.step.to_string(),
      "data-disabled": disabled.to_string(),
      "data-dxui-range-slider": scope_id,
      ..attributes,
      div {
        class: track_class,
        div {
          class: range_class,
          style: range_style,
        }
      }
      for (thumb, now, low_bound, high_bound, label, percent) in thumbs {
        span {
          key: "{thumb}",
          role: "slider",
          class: thumb_class.clone(),
          style: slider_thumb_style(orientation, percent),
          tabindex: if disabled { "-1" } else { "0" },
          "aria-label": label,
          "aria-orientation": orientation.attribute(),
          "aria-valuemin": low_bound.to_string(),
          "aria-valuemax": high_bound.to_string(),
          "aria-valuenow": now.to_string(),
          "aria-disabled": disabled.to_string(),
          onkeydown: move |event: KeyboardEvent| {
            if disabled {
              return;
            }
            let Some(movement) = slider_key_move(&event.key().to_string()) else {
              return;
            };
            // Arrow, Page, Home, and End keys would otherwise scroll the page.
            event.prevent_default();
            let target = slider_state(now, min, max, step).moved(movement).value;
            let next = range_slider_values(values, thumb, target, min, max, step, gap);
            if next != values {
              if let Some(handler) = on_value_change {
                handler.call(next);
              }
            }
          },
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn range_slider_values_snap_and_keep_the_gap() {
    // Each thumb snaps and stays inside the bounds.
    assert_eq!(range_slider_values((20.0, 80.0), 0, 33.0, 0.0, 100.0, 5.0, 0.0), (35.0, 80.0));
    assert_eq!(range_slider_values((20.0, 80.0), 1, 140.0, 0.0, 100.0, 5.0, 0.0), (20.0, 100.0));
    assert_eq!(range_slider_values((20.0, 80.0), 0, -10.0, 0.0, 100.0, 5.0, 0.0), (0.0, 80.0));
    // A thumb stops the gap away from the other one.
    assert_eq!(range_slider_values((20.0, 80.0), 0, 95.0, 0.0, 100.0, 5.0, 10.0), (70.0, 80.0));
    assert_eq!(range_slider_values((20.0, 80.0), 1, 0.0, 0.0, 100.0, 5.0, 10.0), (20.0, 30.0));
    // Without a gap the thumbs may meet.
    assert_eq!(range_slider_values((20.0, 80.0), 1, 20.0, 0.0, 100.0, 5.0, 0.0), (20.0, 20.0));
    // A reversed pair is ordered first.
    assert_eq!(range_slider_values((80.0, 20.0), 1, 90.0, 0.0, 100.0, 5.0, 0.0), (20.0, 90.0));
  }

  #[test]
  fn ssr_range_slider_bounds_each_thumb_by_the_other() {
    fn app() -> Element {
      rsx! {
        RangeSlider { value: (20.0, 60.0), step: 5.0, min_steps_between: 2, "aria-label": "Price" }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains(r#"role="group""#));
    assert!(html.contains(r#"aria-label="Price""#));
    assert_eq!(html.matches(r#"role="slider""#).count(), 2);
    assert!(html.contains(
      r#"aria-label="Minimum" aria-orientation="horizontal" aria-valuemin="0" aria-valuemax="50" aria-valuenow="20""#
    ));
    assert!(html.contains(
      r#"aria-label="Maximum" aria-orientation="horizontal" aria-valuemin="30" aria-valuemax="100" aria-valuenow="60""#
    ));
    assert!(html.contains("left: 20%; width: 40%;"));
  }

  #[test]
  fn slider_key_move_follows_the_slider_pattern() {
    assert_eq!(slider_key_move("ArrowRight"), Some(SliderKeyMove::Increment));
    assert_eq!(slider_key_move("ArrowUp"), Some(SliderKeyMove::Increment));
    assert_eq!(slider_key_move("ArrowLeft"), Some(SliderKeyMove::Decrement));
    assert_eq!(slider_key_move("ArrowDown"), Some(SliderKeyMove::Decrement));
    assert_eq!(slider_key_move("PageUp"), Some(SliderKeyMove::PageIncrement));
    assert_eq!(slider_key_move("PageDown"), Some(SliderKeyMove::PageDecrement));
    assert_eq!(slider_key_move("Home"), Some(SliderKeyMove::Home));
    assert_eq!(slider_key_move("End"), Some(SliderKeyMove::End));
    assert_eq!(slider_key_move("Enter"), None);
  }

  #[test]
  fn slider_classes_append_user_classes() {
    assert!(slider_root_class("mt-2").contains(SLIDER_ROOT_BASE_CLASS));
    assert!(slider_track_class("h-3").ends_with("h-3"));
    assert!(slider_range_class("bg-zinc-900").ends_with("bg-zinc-900"));
    assert!(slider_thumb_class("h-6").ends_with("h-6"));
  }

  #[test]
  fn slider_styles_clamp_percent() {
    let horizontal = SliderOrientation::Horizontal;
    let vertical = SliderOrientation::Vertical;

    assert_eq!(slider_range_style(horizontal, 125.0), "left: 0%; width: 100%;");
    assert_eq!(slider_range_style(vertical, 40.0), "bottom: 0%; height: 40%;");
    assert_eq!(
      slider_thumb_style(horizontal, -10.0),
      "position: absolute; left: 0%; top: 50%; transform: translate(-50%, -50%);"
    );
    assert_eq!(
      slider_thumb_style(vertical, 40.0),
      "position: absolute; bottom: 40%; left: 50%; transform: translate(-50%, 50%);"
    );
  }
}
