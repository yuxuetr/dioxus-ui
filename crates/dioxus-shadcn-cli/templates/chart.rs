//! Chart: SVG parts for simple line, bar, area, and pie charts, with a table
//! fallback for the data.
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

const CHART_BASE_CLASS: &str = "relative w-full text-foreground";
const CHART_SVG_BASE_CLASS: &str = "h-auto w-full overflow-visible";
const CHART_TITLE_BASE_CLASS: &str = "text-sm font-medium text-foreground";
const CHART_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";
const CHART_LEGEND_BASE_CLASS: &str =
  "mt-3 flex flex-wrap items-center gap-3 text-sm text-muted-foreground";
const CHART_FALLBACK_TABLE_BASE_CLASS: &str = "mt-4 w-full caption-bottom text-sm";
const CHART_TOOLTIP_SLOT_BASE_CLASS: &str = "pointer-events-none absolute z-20 rounded-md border border-border bg-popover px-3 py-2 text-sm text-popover-foreground shadow-md";
const CHART_LINE_SERIES_BASE_CLASS: &str = "fill-none stroke-current";
const CHART_AREA_SERIES_BASE_CLASS: &str = "fill-current stroke-current";
const CHART_BAR_SERIES_BASE_CLASS: &str = "fill-current";

/// One data point: an x position and a y value that may be missing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartPoint {
  pub x: f64,
  /// Value along the y axis; `None` marks a gap in the series.
  pub y: Option<f64>,
}

impl ChartPoint {
  /// A point with a value.
  pub const fn new(x: f64, y: f64) -> Self {
    Self { x, y: Some(y) }
  }

  /// A point at `x` with no value, drawn as a gap and listed as missing.
  pub const fn missing(x: f64) -> Self {
    Self { x, y: None }
  }
}

/// A named line or set of bars in a chart.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartSeries {
  pub id: String,
  pub label: String,
  /// Data points in drawing order.
  pub points: Vec<ChartPoint>,
}

impl ChartSeries {
  /// Builds a series from its id, label, and points.
  pub fn new(id: impl Into<String>, label: impl Into<String>, points: Vec<ChartPoint>) -> Self {
    Self { id: id.into(), label: label.into(), points }
  }
}

/// A closed numeric interval, used both for data domains and pixel ranges.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartDomain {
  pub min: f64,
  pub max: f64,
}

impl ChartDomain {
  /// Builds an interval as given, without reordering or validating its ends.
  pub const fn new(min: f64, max: f64) -> Self {
    Self { min, max }
  }

  /// The same interval with non-finite ends replaced (min by 0, max by min)
  /// and the ends swapped if reversed.
  pub fn normalized(self) -> Self {
    chart_domain_normalize(self.min, self.max)
  }
}

/// A linear mapping from data values to pixel positions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartScale {
  /// Data interval, normalized so `min <= max`.
  pub domain: ChartDomain,
  /// Output interval in pixels; may run high to low to flip an axis.
  pub range: ChartDomain,
}

impl ChartScale {
  /// Maps `domain` onto `range`, which keeps its direction: a range from 280
  /// to 32 puts larger values higher in an SVG, whose y grows downward.
  pub fn new(domain: ChartDomain, range: ChartDomain) -> Self {
    Self { domain: domain.normalized(), range: chart_range_finite(range) }
  }

  /// Maps a data value to a pixel position; see [`chart_scale_value`].
  pub fn scale(self, value: f64) -> f64 {
    chart_scale_value(value, self.domain, self.range)
  }
}

/// A theme color for a chart series, mapped to a text class and a data
/// attribute; the SVG draws with `currentColor`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ChartColorToken {
  /// The theme's primary color.
  #[default]
  Primary,
  /// Muted foreground color.
  Secondary,
  Success,
  Warning,
  /// Destructive color.
  Destructive,
  /// Plain foreground color.
  Neutral,
  /// The `--chart-1` to `--chart-5` palette, for series and slices that sit
  /// side by side (RFC 0065).
  Chart1,
  /// The `--chart-2` palette color.
  Chart2,
  /// The `--chart-3` palette color.
  Chart3,
  /// The `--chart-4` palette color.
  Chart4,
  /// The `--chart-5` palette color.
  Chart5,
}

/// One row of the table that stands in for a chart for screen readers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChartFallbackRow {
  /// Id of the series the point belongs to.
  pub series_id: String,
  /// Label of the series the point belongs to.
  pub series_label: String,
  /// The point's x value as text.
  pub x_label: String,
  /// The point's y value as text, or `"missing"`.
  pub y_label: String,
  /// Whether the point has no y value.
  pub missing: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ChartBarRect {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

fn chart_class(class: &str) -> String {
  merge_classes(classes([Some(CHART_BASE_CLASS)]), class)
}

fn chart_svg_class(class: &str) -> String {
  merge_classes(classes([Some(CHART_SVG_BASE_CLASS)]), class)
}

fn chart_title_class(class: &str) -> String {
  merge_classes(classes([Some(CHART_TITLE_BASE_CLASS)]), class)
}

fn chart_description_class(class: &str) -> String {
  merge_classes(classes([Some(CHART_DESCRIPTION_BASE_CLASS)]), class)
}

fn chart_legend_class(class: &str) -> String {
  merge_classes(classes([Some(CHART_LEGEND_BASE_CLASS)]), class)
}

fn chart_fallback_table_class(class: &str) -> String {
  merge_classes(classes([Some(CHART_FALLBACK_TABLE_BASE_CLASS)]), class)
}

fn chart_tooltip_slot_class(visible: bool, class: &str) -> String {
  merge_classes(classes([Some(CHART_TOOLTIP_SLOT_BASE_CLASS), (!visible).then_some("hidden")]), class)
}

fn chart_line_series_class(color: ChartColorToken, class: &str) -> String {
  merge_classes(classes([Some(CHART_LINE_SERIES_BASE_CLASS), Some(chart_color_class(color))]), class)
}

fn chart_area_series_class(color: ChartColorToken, class: &str) -> String {
  merge_classes(classes([Some(CHART_AREA_SERIES_BASE_CLASS), Some(chart_color_class(color))]), class)
}

fn chart_bar_series_class(color: ChartColorToken, class: &str) -> String {
  merge_classes(classes([Some(CHART_BAR_SERIES_BASE_CLASS), Some(chart_color_class(color))]), class)
}

/// The smallest interval containing every finite value, ignoring NaN and
/// infinities; `0..0` when there are no finite values.
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

/// The interval spanned by every point's x across all series.
pub fn chart_series_x_domain(series: &[ChartSeries]) -> ChartDomain {
  let values =
    series.iter().flat_map(|series| series.points.iter().map(|point| point.x)).collect::<Vec<_>>();

  chart_domain(&values)
}

/// The interval spanned by every present y across all series; missing
/// values are skipped.
pub fn chart_series_y_domain(series: &[ChartSeries]) -> ChartDomain {
  let values = series
    .iter()
    .flat_map(|series| series.points.iter().filter_map(|point| point.y))
    .collect::<Vec<_>>();

  chart_domain(&values)
}

/// Maps `value` from `domain` onto `range`, from `range.min` at the domain's
/// low end to `range.max` at its high end, so an inverted range flips the
/// axis.
pub fn chart_scale_value(value: f64, domain: ChartDomain, range: ChartDomain) -> f64 {
  let domain = domain.normalized();
  let range = chart_range_finite(range);

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

/// The range with non-finite ends replaced, keeping its direction.
fn chart_range_finite(range: ChartDomain) -> ChartDomain {
  let min = finite_or_default(range.min, 0.0);
  ChartDomain::new(min, finite_or_default(range.max, min))
}

/// The Tailwind text color class for a color token, such as `text-chart-1`.
pub fn chart_color_class(token: ChartColorToken) -> &'static str {
  match token {
    ChartColorToken::Primary => "text-primary",
    ChartColorToken::Secondary => "text-muted-foreground",
    ChartColorToken::Success => "text-success",
    ChartColorToken::Warning => "text-warning",
    ChartColorToken::Destructive => "text-destructive",
    ChartColorToken::Neutral => "text-foreground",
    ChartColorToken::Chart1 => "text-chart-1",
    ChartColorToken::Chart2 => "text-chart-2",
    ChartColorToken::Chart3 => "text-chart-3",
    ChartColorToken::Chart4 => "text-chart-4",
    ChartColorToken::Chart5 => "text-chart-5",
  }
}

/// The `data-*` attribute value for a color token, such as `chart-1`.
pub fn chart_color_attribute(token: ChartColorToken) -> &'static str {
  match token {
    ChartColorToken::Primary => "primary",
    ChartColorToken::Secondary => "secondary",
    ChartColorToken::Success => "success",
    ChartColorToken::Warning => "warning",
    ChartColorToken::Destructive => "destructive",
    ChartColorToken::Neutral => "neutral",
    ChartColorToken::Chart1 => "chart-1",
    ChartColorToken::Chart2 => "chart-2",
    ChartColorToken::Chart3 => "chart-3",
    ChartColorToken::Chart4 => "chart-4",
    ChartColorToken::Chart5 => "chart-5",
  }
}

/// A one-line description of the chart's size for `aria-label`s, like
/// `2 series, 10 points, 1 missing values`.
pub fn chart_summary(series: &[ChartSeries]) -> String {
  let series_count = series.len();
  let point_count = series.iter().map(|series| series.points.len()).sum::<usize>();
  let missing_count =
    series.iter().flat_map(|series| series.points.iter()).filter(|point| point.y.is_none()).count();

  format!("{series_count} series, {point_count} points, {missing_count} missing values")
}

/// A series label with its color name appended, like `Revenue (primary)`.
pub fn chart_series_label(series: &ChartSeries, token: ChartColorToken) -> String {
  format!("{} ({})", series.label, chart_color_attribute(token))
}

/// An accessible label for one value, like `Revenue at Jan: 42`, or
/// `missing` in place of the value when `y` is `None`.
pub fn chart_value_label(series_label: &str, x_label: &str, y: Option<f64>) -> String {
  match y {
    Some(y) => format!("{series_label} at {x_label}: {y}"),
    None => format!("{series_label} at {x_label}: missing"),
  }
}

/// One fallback table row per point across all series, in order.
pub fn chart_fallback_rows(series: &[ChartSeries]) -> Vec<ChartFallbackRow> {
  series
    .iter()
    .flat_map(|series| {
      series.points.iter().map(|point| ChartFallbackRow {
        series_id: series.id.clone(),
        series_label: series.label.clone(),
        x_label: chart_number_label(point.x),
        y_label: point.y.map(chart_number_label).unwrap_or_else(|| "missing".to_string()),
        missing: point.y.is_none(),
      })
    })
    .collect()
}

/// Formats a number for labels; NaN and infinities read as `missing`.
pub fn chart_number_label(value: f64) -> String {
  if value.is_finite() { value.to_string() } else { "missing".to_string() }
}

pub fn chart_domain_normalize(min: f64, max: f64) -> ChartDomain {
  let min = finite_or_default(min, 0.0);
  let max = finite_or_default(max, min);

  if min <= max { ChartDomain::new(min, max) } else { ChartDomain::new(max, min) }
}

/// The SVG `viewBox` for a chart `width` by `height`; a negative or non-finite size
/// counts as 0.
pub fn chart_view_box(width: f64, height: f64) -> String {
  format!(
    "0 0 {} {}",
    chart_number_label(non_negative_finite(width)),
    chart_number_label(non_negative_finite(height))
  )
}

fn chart_line_path(series: &ChartSeries, x_scale: ChartScale, y_scale: ChartScale) -> String {
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

fn chart_area_path(
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

fn chart_bar_rects(
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
  if value.is_finite() { value } else { default }
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

const CHART_PIE_SERIES_BASE_CLASS: &str = "stroke-background";

/// One pie or donut slice to draw.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartSlice {
  /// Identifies the slice among its siblings.
  pub id: String,
  /// The name shown for the slice.
  pub label: String,
  /// The slice's amount; its share of the total sets its angle.
  pub value: f64,
  /// The color the slice is filled with.
  pub color: ChartColorToken,
}

impl ChartSlice {
  /// A slice from its id, label, value, and color.
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
struct ChartArc {
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
fn chart_pie_arcs(
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

fn chart_pie_series_class(class: &str) -> String {
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
