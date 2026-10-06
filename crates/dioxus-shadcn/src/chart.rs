use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
pub use dioxus_shadcn_primitives::{
  ChartColorToken, ChartDomain, ChartFallbackRow, ChartPoint, ChartScale, ChartSeries,
  chart_color_attribute, chart_color_class, chart_domain, chart_domain_normalize,
  chart_fallback_rows, chart_number_label, chart_scale_value, chart_series_label,
  chart_series_x_domain, chart_series_y_domain, chart_summary, chart_value_label,
};

pub const CHART_BASE_CLASS: &str = "relative w-full text-foreground";
pub const CHART_SVG_BASE_CLASS: &str = "h-auto w-full overflow-visible";
pub const CHART_TITLE_BASE_CLASS: &str = "text-sm font-medium text-foreground";
pub const CHART_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";
pub const CHART_LEGEND_BASE_CLASS: &str =
  "mt-3 flex flex-wrap items-center gap-3 text-sm text-muted-foreground";
pub const CHART_FALLBACK_TABLE_BASE_CLASS: &str = "mt-4 w-full caption-bottom text-sm";
pub const CHART_TOOLTIP_SLOT_BASE_CLASS: &str = "pointer-events-none absolute z-20 rounded-md border border-border bg-popover px-3 py-2 text-sm text-popover-foreground shadow-md";
pub const CHART_LINE_SERIES_BASE_CLASS: &str = "fill-none stroke-current";
pub const CHART_AREA_SERIES_BASE_CLASS: &str = "fill-current stroke-current";
pub const CHART_BAR_SERIES_BASE_CLASS: &str = "fill-current";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartBarRect {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

pub fn chart_class(class: &str) -> String {
  merge_classes(classes([Some(CHART_BASE_CLASS)]), class)
}

pub fn chart_svg_class(class: &str) -> String {
  merge_classes(classes([Some(CHART_SVG_BASE_CLASS)]), class)
}

pub fn chart_title_class(class: &str) -> String {
  merge_classes(classes([Some(CHART_TITLE_BASE_CLASS)]), class)
}

pub fn chart_description_class(class: &str) -> String {
  merge_classes(classes([Some(CHART_DESCRIPTION_BASE_CLASS)]), class)
}

pub fn chart_legend_class(class: &str) -> String {
  merge_classes(classes([Some(CHART_LEGEND_BASE_CLASS)]), class)
}

pub fn chart_fallback_table_class(class: &str) -> String {
  merge_classes(classes([Some(CHART_FALLBACK_TABLE_BASE_CLASS)]), class)
}

pub fn chart_tooltip_slot_class(visible: bool, class: &str) -> String {
  merge_classes(
    classes([Some(CHART_TOOLTIP_SLOT_BASE_CLASS), (!visible).then_some("hidden")]),
    class,
  )
}

pub fn chart_line_series_class(color: ChartColorToken, class: &str) -> String {
  merge_classes(
    classes([Some(CHART_LINE_SERIES_BASE_CLASS), Some(chart_color_class(color))]),
    class,
  )
}

pub fn chart_area_series_class(color: ChartColorToken, class: &str) -> String {
  merge_classes(
    classes([Some(CHART_AREA_SERIES_BASE_CLASS), Some(chart_color_class(color))]),
    class,
  )
}

pub fn chart_bar_series_class(color: ChartColorToken, class: &str) -> String {
  merge_classes(classes([Some(CHART_BAR_SERIES_BASE_CLASS), Some(chart_color_class(color))]), class)
}

pub fn chart_view_box(width: f64, height: f64) -> String {
  format!(
    "0 0 {} {}",
    chart_number_label(non_negative_finite(width)),
    chart_number_label(non_negative_finite(height))
  )
}

pub fn chart_line_path(series: &ChartSeries, x_scale: ChartScale, y_scale: ChartScale) -> String {
  let mut segments = Vec::new();

  for point in &series.points {
    if let Some(y) = point.y {
      let command = if segments.is_empty() { "M" } else { "L" };
      segments.push(format!(
        "{command} {} {}",
        chart_number_label(x_scale.scale(point.x)),
        chart_number_label(y_scale.scale(y))
      ));
    }
  }

  segments.join(" ")
}

pub fn chart_area_path(
  series: &ChartSeries,
  x_scale: ChartScale,
  y_scale: ChartScale,
  baseline: f64,
) -> String {
  let line = chart_line_path(series, x_scale, y_scale);
  let last_present = series.points.iter().rev().find_map(|point| {
    point.y.map(|_| {
      (chart_number_label(x_scale.scale(point.x)), chart_number_label(y_scale.scale(baseline)))
    })
  });
  let first_present = series.points.iter().find_map(|point| {
    point.y.map(|_| {
      (chart_number_label(x_scale.scale(point.x)), chart_number_label(y_scale.scale(baseline)))
    })
  });

  match (first_present, last_present) {
    (Some((first_x, first_y)), Some((last_x, last_y))) => {
      format!("{line} L {last_x} {last_y} L {first_x} {first_y} Z")
    }
    _ => String::new(),
  }
}

pub fn chart_bar_rects(
  series: &ChartSeries,
  x_scale: ChartScale,
  y_scale: ChartScale,
  baseline: f64,
  bar_width: f64,
) -> Vec<ChartBarRect> {
  let baseline_y = y_scale.scale(baseline);
  let bar_width = non_negative_finite(bar_width);

  series
    .points
    .iter()
    .filter_map(|point| {
      point.y.map(|y| {
        let scaled_y = y_scale.scale(y);

        ChartBarRect {
          x: x_scale.scale(point.x) - bar_width / 2.0,
          y: scaled_y.min(baseline_y),
          width: bar_width,
          height: (baseline_y - scaled_y).abs(),
        }
      })
    })
    .collect()
}

fn non_negative_finite(value: f64) -> f64 {
  if value.is_finite() { value.max(0.0) } else { 0.0 }
}

#[component]
pub fn ChartRoot(#[props(default)] class: String, children: Element) -> Element {
  let class = chart_class(&class);

  rsx! {
    figure {
      class,
      {children}
    }
  }
}

#[component]
pub fn ChartSvg(
  #[props(default = chart_view_box(640.0, 320.0))] view_box: String,
  #[props(default = "chart-title".to_string())] title_id: String,
  #[props(default = "chart-description".to_string())] description_id: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = chart_svg_class(&class);
  let labelled_by = format!("{title_id} {description_id}");

  rsx! {
    svg {
      role: "img",
      class,
      view_box,
      "aria-labelledby": labelled_by,
      preserve_aspect_ratio: "xMidYMid meet",
      {children}
    }
  }
}

#[component]
pub fn ChartTitle(id: String, #[props(default)] class: String, children: Element) -> Element {
  let class = chart_title_class(&class);

  rsx! {
    title {
      id,
      class,
      {children}
    }
  }
}

#[component]
pub fn ChartDescription(id: String, #[props(default)] class: String, children: Element) -> Element {
  let class = chart_description_class(&class);

  rsx! {
    desc {
      id,
      class,
      {children}
    }
  }
}

#[component]
pub fn ChartLegend(#[props(default)] class: String, children: Element) -> Element {
  let class = chart_legend_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn ChartFallbackTable(
  rows: Vec<ChartFallbackRow>,
  #[props(default = "Chart data".to_string())] caption: String,
  #[props(default)] class: String,
) -> Element {
  let class = chart_fallback_table_class(&class);

  rsx! {
    table {
      class,
      caption {
        {caption}
      }
      tbody {
        for row in rows {
          tr {
            "data-missing": row.missing.to_string(),
            th {
              scope: "row",
              {row.series_label}
            }
            td {
              {row.x_label}
            }
            td {
              {row.y_label}
            }
          }
        }
      }
    }
  }
}

#[component]
pub fn ChartTooltipSlot(
  #[props(default)] visible: bool,
  #[props(default)] x: f64,
  #[props(default)] y: f64,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = chart_tooltip_slot_class(visible, &class);
  let style = format!("left: {}px; top: {}px;", non_negative_finite(x), non_negative_finite(y));

  rsx! {
    div {
      class,
      style,
      "data-visible": visible.to_string(),
      {children}
    }
  }
}

#[component]
pub fn ChartLineSeries(
  series: ChartSeries,
  x_scale: ChartScale,
  y_scale: ChartScale,
  #[props(default)] color: ChartColorToken,
  #[props(default = 2.0)] stroke_width: f64,
  #[props(default)] class: String,
) -> Element {
  let class = chart_line_series_class(color, &class);
  let d = chart_line_path(&series, x_scale, y_scale);
  let stroke_width = chart_number_label(non_negative_finite(stroke_width));

  rsx! {
    path {
      class,
      d,
      "data-chart-type": "line",
      "data-series": series.id,
      fill: "none",
      stroke_width,
    }
  }
}

#[component]
pub fn ChartAreaSeries(
  series: ChartSeries,
  x_scale: ChartScale,
  y_scale: ChartScale,
  #[props(default)] color: ChartColorToken,
  #[props(default)] baseline: f64,
  #[props(default = 0.16)] opacity: f64,
  #[props(default)] class: String,
) -> Element {
  let class = chart_area_series_class(color, &class);
  let d = chart_area_path(&series, x_scale, y_scale, baseline);
  let opacity = chart_number_label(opacity.clamp(0.0, 1.0));

  rsx! {
    path {
      class,
      d,
      "data-chart-type": "area",
      "data-series": series.id,
      opacity,
    }
  }
}

#[component]
pub fn ChartBarSeries(
  series: ChartSeries,
  x_scale: ChartScale,
  y_scale: ChartScale,
  #[props(default)] color: ChartColorToken,
  #[props(default)] baseline: f64,
  #[props(default = 16.0)] bar_width: f64,
  #[props(default)] class: String,
) -> Element {
  let class = chart_bar_series_class(color, &class);
  let rects = chart_bar_rects(&series, x_scale, y_scale, baseline, bar_width);

  rsx! {
    g {
      class,
      "data-chart-type": "bar",
      "data-series": series.id,
      for rect in rects {
        rect {
          x: chart_number_label(rect.x),
          y: chart_number_label(rect.y),
          width: chart_number_label(rect.width),
          height: chart_number_label(rect.height),
        }
      }
    }
  }
}

pub const CHART_PIE_SERIES_BASE_CLASS: &str = "stroke-background";

/// Every class `chart_color_class` returns. Apps point Tailwind's `@source`
/// at this crate, not at the primitives crate that maps the tokens, so the
/// classes are spelled out here for Tailwind to find.
pub const CHART_COLOR_CLASSES: [&str; 11] = [
  "text-primary",
  "text-muted-foreground",
  "text-success",
  "text-warning",
  "text-destructive",
  "text-foreground",
  "text-chart-1",
  "text-chart-2",
  "text-chart-3",
  "text-chart-4",
  "text-chart-5",
];

/// One pie or donut slice to draw.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartSlice {
  pub id: String,
  pub label: String,
  pub value: f64,
  pub color: ChartColorToken,
}

impl ChartSlice {
  pub fn new(
    id: impl Into<String>,
    label: impl Into<String>,
    value: f64,
    color: ChartColorToken,
  ) -> Self {
    Self { id: id.into(), label: label.into(), value, color }
  }
}

/// A slice's SVG path, its angles in radians from the top, clockwise, and
/// its share of the total.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartArc {
  pub path: String,
  pub start_angle: f64,
  pub end_angle: f64,
  pub fraction: f64,
}

fn arc_number(value: f64) -> String {
  let rounded = (value * 100.0).round() / 100.0;
  // Avoid "-0" in paths.
  if rounded == 0.0 { "0".to_string() } else { rounded.to_string() }
}

fn arc_point(center: (f64, f64), radius: f64, angle: f64) -> String {
  // Angles count from the top, so sine and cosine swap roles.
  let x = center.0 + radius * angle.sin();
  let y = center.1 - radius * angle.cos();
  format!("{} {}", arc_number(x), arc_number(y))
}

fn ring_path(center: (f64, f64), radius: f64, inner: f64, start: f64, end: f64) -> String {
  let large = if end - start > std::f64::consts::PI { 1 } else { 0 };
  let (r, ir) = (arc_number(radius), arc_number(inner));
  if inner > 0.0 {
    format!(
      "M {} A {r} {r} 0 {large} 1 {} L {} A {ir} {ir} 0 {large} 0 {} Z",
      arc_point(center, radius, start),
      arc_point(center, radius, end),
      arc_point(center, inner, end),
      arc_point(center, inner, start),
    )
  } else {
    format!(
      "M {} {} L {} A {r} {r} 0 {large} 1 {} Z",
      arc_number(center.0),
      arc_number(center.1),
      arc_point(center, radius, start),
      arc_point(center, radius, end),
    )
  }
}

// One SVG arc cannot start and end at the same point, so a whole circle is
// two half arcs; a ring adds the inner circle, cut out by the even-odd rule.
fn full_circle_path(center: (f64, f64), radius: f64, inner: f64) -> String {
  let circle = |radius: f64| {
    let r = arc_number(radius);
    format!(
      "M {} A {r} {r} 0 1 1 {} A {r} {r} 0 1 1 {} Z",
      arc_point(center, radius, 0.0),
      arc_point(center, radius, std::f64::consts::PI),
      arc_point(center, radius, 0.0),
    )
  };
  if inner > 0.0 { format!("{} {}", circle(radius), circle(inner)) } else { circle(radius) }
}

/// The slices for `values` around `center`: each takes its share of the
/// total, from the top, clockwise. Values that are negative, zero, or not
/// finite count as zero and get an empty path; a zero total draws nothing.
/// `inner_radius` above zero makes a donut.
pub fn chart_pie_arcs(
  values: &[f64],
  center: (f64, f64),
  radius: f64,
  inner_radius: f64,
) -> Vec<ChartArc> {
  let sizes = values
    .iter()
    .map(|value| if value.is_finite() && *value > 0.0 { *value } else { 0.0 })
    .collect::<Vec<_>>();
  let total = sizes.iter().sum::<f64>();
  let radius = non_negative_finite(radius);
  let inner = non_negative_finite(inner_radius).min(radius);
  let full = std::f64::consts::TAU;
  let mut start = 0.0;

  sizes
    .into_iter()
    .map(|size| {
      let fraction = if total > 0.0 { size / total } else { 0.0 };
      let end = start + fraction * full;
      let path = if fraction <= 0.0 {
        String::new()
      } else if fraction >= 1.0 - 1e-9 {
        full_circle_path(center, radius, inner)
      } else {
        ring_path(center, radius, inner, start, end)
      };
      let arc = ChartArc { path, start_angle: start, end_angle: end, fraction };
      start = end;
      arc
    })
    .collect()
}

pub fn chart_pie_series_class(class: &str) -> String {
  merge_classes(classes([Some(CHART_PIE_SERIES_BASE_CLASS)]), class)
}

/// Pie or donut slices, inside a `ChartSvg`. `center` and `radius` are in the
/// view box's units; `inner_radius` above zero makes a donut. Each slice is
/// filled with its color and outlined with the background, and carries
/// `data-slice` and `data-fraction`.
#[component]
pub fn ChartPieSeries(
  slices: Vec<ChartSlice>,
  #[props(default = (100.0, 100.0))] center: (f64, f64),
  #[props(default = 96.0)] radius: f64,
  #[props(default)] inner_radius: f64,
  #[props(default = 2.0)] gap: f64,
  #[props(default)] class: String,
) -> Element {
  let class = chart_pie_series_class(&class);
  let values = slices.iter().map(|slice| slice.value).collect::<Vec<_>>();
  let arcs = chart_pie_arcs(&values, center, radius, inner_radius);
  let gap = arc_number(non_negative_finite(gap));

  rsx! {
    g { class, "data-chart-type": "pie", stroke_width: gap,
      for (slice, arc) in slices.iter().zip(arcs) {
        if !arc.path.is_empty() {
          path {
            key: "{slice.id}",
            class: "fill-current {chart_color_class(slice.color)}",
            d: arc.path,
            fill_rule: "evenodd",
            "data-slice": slice.id.clone(),
            "data-fraction": arc_number(arc.fraction),
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn chart_color_classes_cover_every_token() {
    for token in [
      ChartColorToken::Primary,
      ChartColorToken::Secondary,
      ChartColorToken::Success,
      ChartColorToken::Warning,
      ChartColorToken::Destructive,
      ChartColorToken::Neutral,
      ChartColorToken::Chart1,
      ChartColorToken::Chart2,
      ChartColorToken::Chart3,
      ChartColorToken::Chart4,
      ChartColorToken::Chart5,
    ] {
      assert!(CHART_COLOR_CLASSES.contains(&chart_color_class(token)), "{token:?}");
    }
  }

  #[test]
  fn pie_arcs_split_the_circle_by_share() {
    let arcs = chart_pie_arcs(&[1.0, 3.0], (100.0, 100.0), 100.0, 0.0);

    assert_eq!(arcs.len(), 2);
    assert_eq!(arcs[0].fraction, 0.25);
    assert!((arcs[0].end_angle - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
    assert_eq!(arcs[0].path, "M 100 100 L 100 0 A 100 100 0 0 1 200 100 Z");
    assert_eq!(arcs[1].path, "M 100 100 L 200 100 A 100 100 0 1 1 100 0 Z");
  }

  #[test]
  fn pie_arcs_skip_empty_values_and_totals() {
    let arcs = chart_pie_arcs(&[0.0, -2.0, f64::NAN, 5.0], (0.0, 0.0), 10.0, 0.0);

    assert!(arcs[..3].iter().all(|arc| arc.path.is_empty() && arc.fraction == 0.0));
    assert_eq!(arcs[3].fraction, 1.0);
    assert_eq!(arcs[3].path, "M 0 -10 A 10 10 0 1 1 0 10 A 10 10 0 1 1 0 -10 Z");
    assert!(
      chart_pie_arcs(&[0.0, 0.0], (0.0, 0.0), 10.0, 0.0).iter().all(|arc| arc.path.is_empty())
    );
  }

  #[test]
  fn donut_arcs_trace_a_ring() {
    let arcs = chart_pie_arcs(&[1.0, 1.0], (50.0, 50.0), 50.0, 30.0);

    assert_eq!(arcs[0].path, "M 50 0 A 50 50 0 0 1 50 100 L 50 80 A 30 30 0 0 0 50 20 Z");
    let full = chart_pie_arcs(&[2.0], (50.0, 50.0), 50.0, 30.0);
    assert_eq!(full[0].path.matches('M').count(), 2);
  }

  #[test]
  fn ssr_pie_series_draws_a_path_per_slice() {
    fn app() -> Element {
      rsx! {
        ChartPieSeries {
          inner_radius: 60.0,
          slices: vec![
            ChartSlice::new("direct", "Direct", 3.0, ChartColorToken::Chart1),
            ChartSlice::new("search", "Search", 1.0, ChartColorToken::Chart2),
            ChartSlice::new("none", "None", 0.0, ChartColorToken::Chart3),
          ],
        }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains("data-chart-type=\"pie\""));
    assert_eq!(html.matches("<path").count(), 2);
    assert!(html.contains("text-chart-1"));
    assert!(html.contains("data-fraction=\"0.75\""));
    assert!(!html.contains("data-slice=\"none\""));
  }

  fn sample_series() -> ChartSeries {
    ChartSeries::new(
      "revenue",
      "Revenue",
      vec![ChartPoint::new(0.0, 10.0), ChartPoint::missing(1.0), ChartPoint::new(2.0, 30.0)],
    )
  }

  #[test]
  fn chart_classes_append_user_classes() {
    assert!(chart_class("max-w-lg").ends_with("max-w-lg"));
    assert!(chart_svg_class("aspect-video").contains(CHART_SVG_BASE_CLASS));
    assert!(chart_tooltip_slot_class(false, "left-0").contains("hidden"));
  }

  #[test]
  fn chart_view_box_normalizes_non_finite_values() {
    assert_eq!(chart_view_box(640.0, 320.0), "0 0 640 320");
    assert_eq!(chart_view_box(f64::NAN, -1.0), "0 0 0 0");
  }

  #[test]
  fn chart_line_and_area_paths_skip_missing_points() {
    let series = sample_series();
    let x_scale = ChartScale::new(ChartDomain::new(0.0, 2.0), ChartDomain::new(0.0, 200.0));
    let y_scale = ChartScale::new(ChartDomain::new(0.0, 30.0), ChartDomain::new(300.0, 0.0));

    // The y range runs from 300 at the bottom to 0 at the top.
    assert_eq!(chart_line_path(&series, x_scale, y_scale), "M 0 200 L 200 0");
    assert_eq!(
      chart_area_path(&series, x_scale, y_scale, 0.0),
      "M 0 200 L 200 0 L 200 300 L 0 300 Z"
    );
  }

  #[test]
  fn chart_bar_rects_use_baseline_and_width() {
    let series = sample_series();
    let x_scale = ChartScale::new(ChartDomain::new(0.0, 2.0), ChartDomain::new(0.0, 200.0));
    let y_scale = ChartScale::new(ChartDomain::new(0.0, 30.0), ChartDomain::new(300.0, 0.0));
    let rects = chart_bar_rects(&series, x_scale, y_scale, 0.0, 20.0);

    assert_eq!(rects.len(), 2);
    assert_eq!(rects[0].x, -10.0);
    // The bar rises from the baseline at the bottom (y 300) to its value.
    assert_eq!(rects[0].y, 200.0);
    assert_eq!(rects[0].width, 20.0);
    assert_eq!(rects[0].height, 100.0);
  }
}
