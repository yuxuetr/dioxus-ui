use dioxus::prelude::*;
use super::utils::classes;

pub const CHART_BASE_CLASS: &str = "relative w-full text-zinc-950";
pub const CHART_SVG_BASE_CLASS: &str = "h-auto w-full overflow-visible";
pub const CHART_TITLE_BASE_CLASS: &str = "text-sm font-medium text-zinc-950";
pub const CHART_DESCRIPTION_BASE_CLASS: &str = "text-sm text-zinc-600";
pub const CHART_LEGEND_BASE_CLASS: &str = "mt-3 flex flex-wrap items-center gap-3 text-sm text-zinc-700";
pub const CHART_FALLBACK_TABLE_BASE_CLASS: &str = "mt-4 w-full caption-bottom text-sm";
pub const CHART_TOOLTIP_SLOT_BASE_CLASS: &str = "pointer-events-none absolute z-20 rounded-md border border-zinc-200 bg-white px-3 py-2 text-sm text-zinc-950 shadow-md";
pub const CHART_LINE_SERIES_BASE_CLASS: &str = "fill-none stroke-current";
pub const CHART_AREA_SERIES_BASE_CLASS: &str = "fill-current stroke-current";
pub const CHART_BAR_SERIES_BASE_CLASS: &str = "fill-current";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartPoint {
  pub x: f64,
  pub y: Option<f64>,
}

impl ChartPoint {
  pub const fn new(x: f64, y: f64) -> Self {
    Self { x, y: Some(y) }
  }

  pub const fn missing(x: f64) -> Self {
    Self { x, y: None }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChartSeries {
  pub id: String,
  pub label: String,
  pub points: Vec<ChartPoint>,
}

impl ChartSeries {
  pub fn new(
    id: impl Into<String>,
    label: impl Into<String>,
    points: Vec<ChartPoint>,
  ) -> Self {
    Self {
      id: id.into(),
      label: label.into(),
      points,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartDomain {
  pub min: f64,
  pub max: f64,
}

impl ChartDomain {
  pub const fn new(min: f64, max: f64) -> Self {
    Self { min, max }
  }

  pub fn normalized(self) -> Self {
    chart_domain_normalize(self.min, self.max)
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartScale {
  pub domain: ChartDomain,
  pub range: ChartDomain,
}

impl ChartScale {
  pub fn new(domain: ChartDomain, range: ChartDomain) -> Self {
    Self {
      domain: domain.normalized(),
      range: range.normalized(),
    }
  }

  pub fn scale(self, value: f64) -> f64 {
    chart_scale_value(value, self.domain, self.range)
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ChartColorToken {
  #[default]
  Primary,
  Secondary,
  Success,
  Warning,
  Destructive,
  Neutral,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChartFallbackRow {
  pub series_id: String,
  pub series_label: String,
  pub x_label: String,
  pub y_label: String,
  pub missing: bool,
}

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
  classes([
    Some(CHART_TOOLTIP_SLOT_BASE_CLASS),
    (!visible).then_some("hidden"),
    Some(class),
  ])
}

pub fn chart_line_series_class(color: ChartColorToken, class: &str) -> String {
  classes([
    Some(CHART_LINE_SERIES_BASE_CLASS),
    Some(chart_color_class(color)),
    Some(class),
  ])
}

pub fn chart_area_series_class(color: ChartColorToken, class: &str) -> String {
  classes([
    Some(CHART_AREA_SERIES_BASE_CLASS),
    Some(chart_color_class(color)),
    Some(class),
  ])
}

pub fn chart_bar_series_class(color: ChartColorToken, class: &str) -> String {
  classes([
    Some(CHART_BAR_SERIES_BASE_CLASS),
    Some(chart_color_class(color)),
    Some(class),
  ])
}

pub fn chart_domain(values: &[f64]) -> ChartDomain {
  let mut min = f64::INFINITY;
  let mut max = f64::NEG_INFINITY;

  for value in values.iter().copied().filter(|value| value.is_finite()) {
    min = min.min(value);
    max = max.max(value);
  }

  if min.is_finite() && max.is_finite() {
    chart_domain_normalize(min, max)
  } else {
    ChartDomain::new(0.0, 0.0)
  }
}

pub fn chart_series_x_domain(series: &[ChartSeries]) -> ChartDomain {
  let values = series
    .iter()
    .flat_map(|series| series.points.iter().map(|point| point.x))
    .collect::<Vec<_>>();

  chart_domain(&values)
}

pub fn chart_series_y_domain(series: &[ChartSeries]) -> ChartDomain {
  let values = series
    .iter()
    .flat_map(|series| series.points.iter().filter_map(|point| point.y))
    .collect::<Vec<_>>();

  chart_domain(&values)
}

pub fn chart_scale_value(value: f64, domain: ChartDomain, range: ChartDomain) -> f64 {
  let domain = domain.normalized();
  let range = range.normalized();

  if !value.is_finite() {
    return range.min;
  }

  let span = domain.max - domain.min;

  if span == 0.0 {
    return midpoint(range);
  }

  let ratio = (value - domain.min) / span;

  range.min + ratio * (range.max - range.min)
}

pub fn chart_color_class(token: ChartColorToken) -> &'static str {
  match token {
    ChartColorToken::Primary => "text-blue-600",
    ChartColorToken::Secondary => "text-zinc-600",
    ChartColorToken::Success => "text-green-600",
    ChartColorToken::Warning => "text-amber-600",
    ChartColorToken::Destructive => "text-red-600",
    ChartColorToken::Neutral => "text-zinc-900",
  }
}

pub fn chart_color_attribute(token: ChartColorToken) -> &'static str {
  match token {
    ChartColorToken::Primary => "primary",
    ChartColorToken::Secondary => "secondary",
    ChartColorToken::Success => "success",
    ChartColorToken::Warning => "warning",
    ChartColorToken::Destructive => "destructive",
    ChartColorToken::Neutral => "neutral",
  }
}

pub fn chart_summary(series: &[ChartSeries]) -> String {
  let series_count = series.len();
  let point_count = series.iter().map(|series| series.points.len()).sum::<usize>();
  let missing_count = series
    .iter()
    .flat_map(|series| series.points.iter())
    .filter(|point| point.y.is_none())
    .count();

  format!(
    "{series_count} series, {point_count} points, {missing_count} missing values"
  )
}

pub fn chart_series_label(series: &ChartSeries, token: ChartColorToken) -> String {
  format!(
    "{} ({})",
    series.label,
    chart_color_attribute(token)
  )
}

pub fn chart_value_label(series_label: &str, x_label: &str, y: Option<f64>) -> String {
  match y {
    Some(y) => format!("{series_label} at {x_label}: {y}"),
    None => format!("{series_label} at {x_label}: missing"),
  }
}

pub fn chart_fallback_rows(series: &[ChartSeries]) -> Vec<ChartFallbackRow> {
  series
    .iter()
    .flat_map(|series| {
      series.points.iter().map(|point| ChartFallbackRow {
        series_id: series.id.clone(),
        series_label: series.label.clone(),
        x_label: chart_number_label(point.x),
        y_label: point
          .y
          .map(chart_number_label)
          .unwrap_or_else(|| "missing".to_string()),
        missing: point.y.is_none(),
      })
    })
    .collect()
}

pub fn chart_number_label(value: f64) -> String {
  if value.is_finite() {
    value.to_string()
  } else {
    "missing".to_string()
  }
}

pub fn chart_domain_normalize(min: f64, max: f64) -> ChartDomain {
  let min = finite_or_default(min, 0.0);
  let max = finite_or_default(max, min);

  if min <= max {
    ChartDomain::new(min, max)
  } else {
    ChartDomain::new(max, min)
  }
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
      (
        chart_number_label(x_scale.scale(point.x)),
        chart_number_label(y_scale.scale(baseline)),
      )
    })
  });
  let first_present = series.points.iter().find_map(|point| {
    point.y.map(|_| {
      (
        chart_number_label(x_scale.scale(point.x)),
        chart_number_label(y_scale.scale(baseline)),
      )
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

fn midpoint(domain: ChartDomain) -> f64 {
  domain.min + (domain.max - domain.min) / 2.0
}

fn finite_or_default(value: f64, default: f64) -> f64 {
  if value.is_finite() {
    value
  } else {
    default
  }
}

fn non_negative_finite(value: f64) -> f64 {
  if value.is_finite() {
    value.max(0.0)
  } else {
    0.0
  }
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
pub fn ChartTitle(
  id: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
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
pub fn ChartDescription(
  id: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
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
