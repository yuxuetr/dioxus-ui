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
  fn normalizes_reversed_or_non_finite_domains() {
    assert_eq!(ChartDomain::new(10.0, 2.0).normalized(), ChartDomain::new(2.0, 10.0));
    assert_eq!(
      chart_domain_normalize(f64::NAN, f64::INFINITY),
      ChartDomain::new(0.0, 0.0)
    );
  }

  #[test]
  fn maps_color_tokens() {
    assert_eq!(chart_color_class(ChartColorToken::Warning), "text-amber-600");
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

    assert_eq!(
      chart_series_label(&series, ChartColorToken::Primary),
      "Revenue (primary)"
    );
    assert_eq!(
      chart_value_label("Revenue", "Q1", Some(42.0)),
      "Revenue at Q1: 42"
    );
    assert_eq!(
      chart_value_label("Revenue", "Q2", None),
      "Revenue at Q2: missing"
    );
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
