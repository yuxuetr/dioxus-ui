//! Data, domain, and scale helpers for the styled `Chart` components: linear
//! scales, color tokens, and the accessible labels and fallback table rows.

/// One data point: an x position and a y value that may be missing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartPoint {
  /// Position along the x axis.
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
  /// Stable identifier, used for keys and fallback table rows.
  pub id: String,
  /// Human-readable name shown in legends and accessible labels.
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
  /// Lower end of the interval.
  pub min: f64,
  /// Upper end of the interval.
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
  /// Success color.
  Success,
  /// Warning color.
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

pub(crate) fn chart_domain_normalize(min: f64, max: f64) -> ChartDomain {
  let min = finite_or_default(min, 0.0);
  let max = finite_or_default(max, min);

  if min <= max { ChartDomain::new(min, max) } else { ChartDomain::new(max, min) }
}

/// The range with non-finite ends replaced, keeping its direction.
fn chart_range_finite(range: ChartDomain) -> ChartDomain {
  let min = finite_or_default(range.min, 0.0);
  ChartDomain::new(min, finite_or_default(range.max, min))
}

fn midpoint(domain: ChartDomain) -> f64 {
  domain.min + (domain.max - domain.min) / 2.0
}

fn finite_or_default(value: f64, default: f64) -> f64 {
  if value.is_finite() { value } else { default }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calculates_domain_from_finite_values() {
    assert_eq!(chart_domain(&[3.0, -2.0, f64::NAN, 7.0]), ChartDomain::new(-2.0, 7.0));
    assert_eq!(chart_domain(&[]), ChartDomain::new(0.0, 0.0));
  }

  #[test]
  fn calculates_series_domains_with_missing_values() {
    let series = vec![
      ChartSeries::new(
        "revenue",
        "Revenue",
        vec![ChartPoint::new(0.0, 10.0), ChartPoint::missing(1.0)],
      ),
      ChartSeries::new("cost", "Cost", vec![ChartPoint::new(2.0, -4.0)]),
    ];

    assert_eq!(chart_series_x_domain(&series), ChartDomain::new(0.0, 2.0));
    assert_eq!(chart_series_y_domain(&series), ChartDomain::new(-4.0, 10.0));
  }

  #[test]
  fn scales_values_across_ranges() {
    let scale = ChartScale::new(ChartDomain::new(0.0, 100.0), ChartDomain::new(0.0, 1.0));

    assert_eq!(scale.scale(25.0), 0.25);
    assert_eq!(
      chart_scale_value(10.0, ChartDomain::new(10.0, 10.0), ChartDomain::new(0.0, 100.0)),
      50.0
    );
  }

  #[test]
  fn an_inverted_range_flips_the_axis() {
    // SVG y grows downward, so a y range from 280 to 32 draws larger values higher.
    let scale = ChartScale::new(ChartDomain::new(0.0, 24.0), ChartDomain::new(280.0, 32.0));

    assert_eq!(scale.scale(0.0), 280.0);
    assert_eq!(scale.scale(24.0), 32.0);
    assert_eq!(scale.scale(12.0), 156.0);
    assert!(scale.scale(18.0) < scale.scale(12.0));
    assert_eq!(scale.scale(f64::NAN), 280.0);
  }

  #[test]
  fn normalizes_reversed_or_non_finite_domains() {
    assert_eq!(ChartDomain::new(10.0, 2.0).normalized(), ChartDomain::new(2.0, 10.0));
    assert_eq!(chart_domain_normalize(f64::NAN, f64::INFINITY), ChartDomain::new(0.0, 0.0));
  }

  #[test]
  fn maps_color_tokens() {
    assert_eq!(chart_color_class(ChartColorToken::Warning), "text-warning");
    assert_eq!(chart_color_attribute(ChartColorToken::Destructive), "destructive");
  }

  #[test]
  fn summarizes_series_and_missing_values() {
    let series = vec![
      ChartSeries::new(
        "revenue",
        "Revenue",
        vec![ChartPoint::new(0.0, 10.0), ChartPoint::missing(1.0)],
      ),
      ChartSeries::new("cost", "Cost", vec![ChartPoint::new(0.0, 4.0)]),
    ];

    assert_eq!(chart_summary(&series), "2 series, 3 points, 1 missing values");
  }

  #[test]
  fn creates_color_independent_series_and_value_labels() {
    let series = ChartSeries::new("revenue", "Revenue", vec![]);

    assert_eq!(chart_series_label(&series, ChartColorToken::Primary), "Revenue (primary)");
    assert_eq!(chart_value_label("Revenue", "Q1", Some(42.0)), "Revenue at Q1: 42");
    assert_eq!(chart_value_label("Revenue", "Q2", None), "Revenue at Q2: missing");
  }

  #[test]
  fn creates_fallback_rows_for_table_rendering() {
    let series = vec![ChartSeries::new(
      "revenue",
      "Revenue",
      vec![ChartPoint::new(0.0, 10.0), ChartPoint::missing(1.0)],
    )];

    let rows = chart_fallback_rows(&series);

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].series_label, "Revenue");
    assert_eq!(rows[0].x_label, "0");
    assert_eq!(rows[0].y_label, "10");
    assert!(!rows[0].missing);
    assert_eq!(rows[1].y_label, "missing");
    assert!(rows[1].missing);
  }
}
