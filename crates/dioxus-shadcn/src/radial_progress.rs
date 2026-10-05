use dioxus::prelude::*;
use dioxus_shadcn_core::classes;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RadialProgressSize {
  Sm,
  #[default]
  Md,
  Lg,
}

impl RadialProgressSize {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "size-12 text-xs",
      Self::Md => "size-20 text-sm",
      Self::Lg => "size-28 text-base",
    }
  }

  /// The ring's stroke width in the 100-unit view box.
  pub const fn stroke_width(self) -> f64 {
    match self {
      Self::Sm => 10.0,
      Self::Md => 8.0,
      Self::Lg => 7.0,
    }
  }
}

pub const RADIAL_PROGRESS_BASE_CLASS: &str =
  "relative inline-grid shrink-0 place-items-center font-medium tabular-nums";
pub const RADIAL_PROGRESS_TRACK_CLASS: &str = "stroke-muted";
pub const RADIAL_PROGRESS_INDICATOR_CLASS: &str =
  "stroke-primary transition-[stroke-dashoffset] duration-300";
// Children fill the slot; while it is empty the default percentage shows.
pub const RADIAL_PROGRESS_SLOT_CLASS: &str = "peer relative empty:hidden";
pub const RADIAL_PROGRESS_DEFAULT_LABEL_CLASS: &str = "relative hidden peer-empty:inline";

pub fn radial_progress_class(size: RadialProgressSize, class: &str) -> String {
  classes([Some(RADIAL_PROGRESS_BASE_CLASS), Some(size.class()), Some(class)])
}

/// The value clamped to 0 through 100; anything that is not a number is 0.
pub fn radial_progress_value(value: f64) -> f64 {
  if value.is_nan() { 0.0 } else { value.clamp(0.0, 100.0) }
}

/// The ring's radius and the dash offset that leaves `value` percent drawn.
pub fn radial_progress_geometry(value: f64, stroke_width: f64) -> (f64, f64, f64) {
  let radius = 50.0 - stroke_width / 2.0;
  let circumference = 2.0 * std::f64::consts::PI * radius;
  let offset = circumference * (1.0 - radial_progress_value(value) / 100.0);
  (radius, circumference, offset)
}

/// A circular progress bar for a value from 0 to 100. Children replace the
/// centered `{value}%` label. Name it with `aria-label` or
/// `aria-labelledby`.
#[component]
pub fn RadialProgress(
  value: f64,
  #[props(default)] size: RadialProgressSize,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = radial_progress_class(size, &class);
  let value = radial_progress_value(value);
  let stroke_width = size.stroke_width();
  let (radius, circumference, offset) = radial_progress_geometry(value, stroke_width);
  let rounded = value.round();

  rsx! {
    div {
      class,
      role: "progressbar",
      "aria-valuemin": "0",
      "aria-valuemax": "100",
      "aria-valuenow": "{rounded}",
      ..attributes,
      // The ring starts at the top and runs clockwise.
      svg {
        class: "absolute inset-0 size-full -rotate-90",
        view_box: "0 0 100 100",
        fill: "none",
        "aria-hidden": "true",
        circle {
          class: RADIAL_PROGRESS_TRACK_CLASS,
          cx: "50",
          cy: "50",
          r: "{radius}",
          stroke_width: "{stroke_width}",
        }
        circle {
          class: RADIAL_PROGRESS_INDICATOR_CLASS,
          cx: "50",
          cy: "50",
          r: "{radius}",
          stroke_width: "{stroke_width}",
          stroke_linecap: "round",
          stroke_dasharray: "{circumference}",
          stroke_dashoffset: "{offset}",
        }
      }
      span { class: RADIAL_PROGRESS_SLOT_CLASS, {children} }
      span { class: RADIAL_PROGRESS_DEFAULT_LABEL_CLASS, "{rounded}%" }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_radial_progress_is_a_progressbar() {
    fn app() -> Element {
      rsx! { RadialProgress { value: 72.4, "aria-label": "Storage used" } }
    }
    let html = render(app);

    assert!(html.contains("role=\"progressbar\""));
    assert!(html.contains("aria-valuemin=\"0\""));
    assert!(html.contains("aria-valuemax=\"100\""));
    assert!(html.contains("aria-valuenow=\"72\""));
    assert!(html.contains("aria-label=\"Storage used\""));
    assert!(html.contains(">72%</span>"));
  }

  #[test]
  fn value_is_clamped() {
    assert_eq!(radial_progress_value(-5.0), 0.0);
    assert_eq!(radial_progress_value(140.0), 100.0);
    assert_eq!(radial_progress_value(f64::NAN), 0.0);
  }

  #[test]
  fn geometry_draws_the_value() {
    let (radius, circumference, offset) = radial_progress_geometry(25.0, 8.0);

    assert_eq!(radius, 46.0);
    assert!((offset - circumference * 0.75).abs() < 1e-9);
    assert_eq!(radial_progress_geometry(100.0, 8.0).2, 0.0);
    assert!((radial_progress_geometry(0.0, 8.0).2 - circumference).abs() < 1e-9);
  }
}
