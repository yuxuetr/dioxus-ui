use super::utils::{classes, merge_classes};
use super::density::{density_control_class, use_density, with_density};
use dioxus::prelude::*;

// Dim the field only when the input is disabled, not when a button is at a
// bound.
pub const NUMBER_INPUT_BASE_CLASS: &str = "flex h-10 w-full items-stretch overflow-hidden rounded-md border bg-background text-sm transition-colors focus-within:ring-2 has-[input:disabled]:cursor-not-allowed has-[input:disabled]:opacity-50";
pub const NUMBER_INPUT_FIELD_CLASS: &str = "min-w-0 flex-1 bg-transparent px-2 text-center tabular-nums outline-none disabled:cursor-not-allowed";
pub const NUMBER_INPUT_BUTTON_CLASS: &str = "flex w-10 shrink-0 items-center justify-center text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground disabled:pointer-events-none disabled:opacity-50";

pub fn number_input_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-within:ring-destructive"
  } else {
    "border-input focus-within:ring-ring"
  };

  merge_classes(classes([Some(NUMBER_INPUT_BASE_CLASS), Some(invalid_class)]), class)
}

/// The value kept within the optional bounds.
pub fn number_input_clamp(value: f64, min: Option<f64>, max: Option<f64>) -> f64 {
  let value = min.map_or(value, |min| value.max(min));
  max.map_or(value, |max| value.min(max))
}

/// Rounds to the step's decimals, so 0.1 + 0.2 reads 0.3.
pub fn number_input_round(value: f64, step: f64) -> f64 {
  let step_text = step.to_string();
  let decimals = step_text.split_once('.').map_or(0, |(_, fraction)| fraction.len()).min(12);
  let scale = 10_f64.powi(decimals as i32);
  (value * scale).round() / scale
}

/// One step up (`direction` 1) or down (-1), rounded and clamped.
pub fn number_input_step(
  value: f64,
  step: f64,
  direction: f64,
  min: Option<f64>,
  max: Option<f64>,
) -> f64 {
  number_input_clamp(number_input_round(value + step * direction, step), min, max)
}

/// The text for a value: no trailing `.0`.
pub fn number_input_format(value: f64) -> String {
  value.to_string()
}

/// A number from typed text, if it is a complete, finite number.
pub fn number_input_parse(text: &str) -> Option<f64> {
  text.trim().parse::<f64>().ok().filter(|value| value.is_finite())
}

/// A number field with decrement and increment buttons. The input keeps the
/// text being typed and calls `on_value_change` with each complete number;
/// on blur it clamps to `min` and `max`. ArrowUp and ArrowDown step, and Home
/// and End go to `min` and `max`. Other attributes, such as `id`, `name`, and
/// `aria-label`, go to the input.
#[component]
pub fn NumberInput(
  value: f64,
  #[props(default)] min: Option<f64>,
  #[props(default)] max: Option<f64>,
  #[props(default = 1.0)] step: f64,
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
  #[props(default = "Decrease".to_string())] decrement_label: String,
  #[props(default = "Increase".to_string())] increment_label: String,
  #[props(default)] class: String,
  #[props(default)] on_value_change: Option<EventHandler<f64>>,
  #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
  let class = number_input_class(invalid, &class);
  let density = use_density();
  let mut text = use_signal(|| number_input_format(value));

  // Follow a new value from the app unless the text already means it, so
  // "1." stays while the value is 1.
  use_effect(use_reactive((&value,), move |(value,)| {
    if number_input_parse(&text.peek()) != Some(value) {
      text.set(number_input_format(value));
    }
  }));

  let mut commit = move |next: f64| {
    text.set(number_input_format(next));
    if next == value {
      return;
    }
    if let Some(handler) = on_value_change {
      handler.call(next);
    }
  };
  let at_min = min.is_some_and(|min| value <= min);
  let at_max = max.is_some_and(|max| value >= max);

  rsx! {
    div { class,
      button {
        class: with_density(density_control_class(density), NUMBER_INPUT_BUTTON_CLASS),
        r#type: "button",
        tabindex: "-1",
        "aria-label": decrement_label,
        disabled: disabled || at_min,
        onclick: move |_| commit(number_input_step(value, step, -1.0, min, max)),
        "−"
      }
      input {
        class: with_density(density_control_class(density), NUMBER_INPUT_FIELD_CLASS),
        r#type: "text",
        inputmode: "decimal",
        role: "spinbutton",
        autocomplete: "off",
        value: text(),
        disabled,
        "aria-valuenow": "{value}",
        "aria-valuemin": min.map(|min| min.to_string()),
        "aria-valuemax": max.map(|max| max.to_string()),
        "aria-invalid": invalid.then_some("true"),
        oninput: move |event| {
          let typed = event.value();
          if let (Some(handler), Some(number)) = (on_value_change, number_input_parse(&typed)) {
            handler.call(number);
          }
          text.set(typed);
        },
        onblur: move |_| {
          let typed = number_input_parse(&text.peek()).unwrap_or(value);
          commit(number_input_clamp(number_input_round(typed, step), min, max));
        },
        onkeydown: move |event| {
          let next = match event.key() {
            Key::ArrowUp => Some(number_input_step(value, step, 1.0, min, max)),
            Key::ArrowDown => Some(number_input_step(value, step, -1.0, min, max)),
            Key::Home => min,
            Key::End => max,
            _ => None,
          };
          if let Some(next) = next {
            event.prevent_default();
            commit(next);
          }
        },
        ..attributes,
      }
      button {
        class: with_density(density_control_class(density), NUMBER_INPUT_BUTTON_CLASS),
        r#type: "button",
        tabindex: "-1",
        "aria-label": increment_label,
        disabled: disabled || at_max,
        onclick: move |_| commit(number_input_step(value, step, 1.0, min, max)),
        "+"
      }
    }
  }
}
