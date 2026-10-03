use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  ChartColorToken, ChartDomain, ChartFallbackRow, ChartPoint, ChartScale, ChartSeries,
  chart_color_attribute, chart_color_class, chart_domain, chart_domain_normalize,
  chart_fallback_rows, chart_number_label, chart_scale_value, chart_series_label,
  chart_series_x_domain, chart_series_y_domain, chart_summary, chart_value_label,
};

pub const CHART_BASE_CLASS: &str = "relative w-full text-zinc-950";
pub const CHART_SVG_BASE_CLASS: &str = "h-auto w-full overflow-visible";
pub const CHART_TITLE_BASE_CLASS: &str = "text-sm font-medium text-zinc-950";
pub const CHART_DESCRIPTION_BASE_CLASS: &str = "text-sm text-zinc-600";
pub const CHART_LEGEND_BASE_CLASS: &str =
  "mt-3 flex flex-wrap items-center gap-3 text-sm text-zinc-700";
pub const CHART_FALLBACK_TABLE_BASE_CLASS: &str = "mt-4 w-full caption-bottom text-sm";
pub const CHART_TOOLTIP_SLOT_BASE_CLASS: &str = "pointer-events-none absolute z-20 rounded-md border border-zinc-200 bg-white px-3 py-2 text-sm text-zinc-950 shadow-md";
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
  classes([Some(CHART_BASE_CLASS), Some(class)])
}

pub fn chart_svg_class(class: &str) -> String {
  classes([Some(CHART_SVG_BASE_CLASS), Some(class)])
}

pub fn chart_title_class(class: &str) -> String {
  classes([Some(CHART_TITLE_BASE_CLASS), Some(class)])
}

pub fn chart_description_class(class: &str) -> String {
  classes([Some(CHART_DESCRIPTION_BASE_CLASS), Some(class)])
}

pub fn chart_legend_class(class: &str) -> String {
  classes([Some(CHART_LEGEND_BASE_CLASS), Some(class)])
}

pub fn chart_fallback_table_class(class: &str) -> String {
  classes([Some(CHART_FALLBACK_TABLE_BASE_CLASS), Some(class)])
}

pub fn chart_tooltip_slot_class(visible: bool, class: &str) -> String {
  classes([Some(CHART_TOOLTIP_SLOT_BASE_CLASS), (!visible).then_some("hidden"), Some(class)])
}

pub fn chart_line_series_class(color: ChartColorToken, class: &str) -> String {
  classes([Some(CHART_LINE_SERIES_BASE_CLASS), Some(chart_color_class(color)), Some(class)])
}

pub fn chart_area_series_class(color: ChartColorToken, class: &str) -> String {
  classes([Some(CHART_AREA_SERIES_BASE_CLASS), Some(chart_color_class(color)), Some(class)])
}

pub fn chart_bar_series_class(color: ChartColorToken, class: &str) -> String {
  classes([Some(CHART_BAR_SERIES_BASE_CLASS), Some(chart_color_class(color)), Some(class)])
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

#[cfg(test)]
mod tests {
  use super::*;

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

    assert_eq!(chart_line_path(&series, x_scale, y_scale), "M 0 100 L 200 300");
    assert_eq!(
      chart_area_path(&series, x_scale, y_scale, 0.0),
      "M 0 100 L 200 300 L 200 0 L 0 0 Z"
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
    assert_eq!(rects[0].y, 0.0);
    assert_eq!(rects[0].width, 20.0);
    assert_eq!(rects[0].height, 100.0);
  }
}
