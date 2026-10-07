use crate::density::{density_control_class, use_density, with_density};
use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

pub const INPUT_BASE_CLASS: &str = "flex h-10 w-full rounded-md border bg-background px-3 py-2 text-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";

pub fn input_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  merge_classes(classes([Some(INPUT_BASE_CLASS), Some(invalid_class)]), class)
}

/// A controlled input. Each `input` event calls `on_value_change` with the new
/// text; the app passes it back as `value`. Other attributes, such as `id`,
/// `name`, and `aria-describedby`, are passed to the input.
#[component]
pub fn Input(
  #[props(default)] value: String,
  #[props(default)] placeholder: String,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
  let class = input_class(invalid, &with_density(density_control_class(use_density()), &class));

  rsx! {
    input {
      class,
      value,
      placeholder,
      disabled,
      "aria-invalid": invalid.to_string(),
      oninput: move |event: FormEvent| {
        if let Some(handler) = on_value_change {
          handler.call(event.value());
        }
      },
      ..attributes,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::density::DensityProvider;
  use dioxus_shadcn_core::UiDensity;

  #[test]
  fn touch_raises_the_input_to_a_touch_target_and_the_app_class_wins() {
    fn app() -> Element {
      rsx! {
        DensityProvider { density: UiDensity::Touch,
          Input { "aria-label": "Name" }
          Input { "aria-label": "Code", class: "min-h-8" }
        }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert_eq!(html.matches("min-h-11").count(), 1, "{html}");
    assert!(html.contains("min-h-8"));
  }

  #[test]
  fn input_class_adds_invalid_state() {
    let actual = input_class(true, "w-64");

    assert_eq!(
      actual,
      "flex h-10 rounded-md border bg-background px-3 py-2 text-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50 border-destructive focus-visible:ring-destructive w-64"
    );
    assert!(actual.contains("border-destructive focus-visible:ring-destructive"));
    assert!(actual.ends_with("w-64"));
  }
}
