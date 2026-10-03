use dioxus_ui_primitives::{
  ChartColorToken, ChartDomain, ChartPoint, ChartScale, ChartSeries, chart_color_attribute,
  chart_fallback_rows, chart_number_label, chart_series_label, chart_series_x_domain,
  chart_series_y_domain, chart_summary, chart_value_label,
};

const CHART_WIDTH: f64 = 640.0;
const CHART_HEIGHT: f64 = 320.0;
const CHART_PADDING: f64 = 32.0;
const BAR_WIDTH: f64 = 24.0;

#[derive(Clone, Debug, PartialEq)]
pub struct ChartFixtureRender {
  pub svg: String,
  pub fallback_table: String,
  pub summary: String,
  pub view_box: String,
  pub width: f64,
  pub height: f64,
  pub reduced_motion: bool,
  pub fallback_row_count: usize,
  pub missing_row_count: usize,
  pub bar_count: usize,
}

pub fn chart_fixture_states() -> Vec<String> {
  let render = render_chart_fixture(true);
  let has_title = render.svg.contains("runtime-chart-title");
  let has_description = render.svg.contains("runtime-chart-description");
  let has_line = render.svg.contains("data-chart-type=\"line\"");
  let has_area = render.svg.contains("data-chart-type=\"area\"");
  let has_bars = render.svg.contains("data-chart-type=\"bar\"");
  let has_animation = render.svg.contains("animate-");

  vec![
    "chart_fixture testid=runtime-chart-fixture expected=example-only result=ready".to_string(),
    format!(
      "chart_measurement testid=runtime-chart-measurement expected=640x320 result={}x{}",
      chart_number_label(render.width),
      chart_number_label(render.height)
    ),
    format!(
      "chart_responsive testid=runtime-chart-responsive expected=viewBox result={}",
      render.view_box
    ),
    format!(
      "chart_svg_semantics testid=runtime-chart-svg-semantics expected=title-description result={}",
      has_title && has_description
    ),
    format!(
      "chart_series_shapes testid=runtime-chart-series-shapes expected=line-bar-area result={}/{}/{}",
      has_line, has_bars, has_area
    ),
    format!(
      "chart_fallback_table testid=runtime-chart-fallback-table expected=rows result={} missing={}",
      render.fallback_row_count, render.missing_row_count
    ),
    format!(
      "chart_reduced_motion testid=runtime-chart-reduced-motion expected=no-animation result={}",
      render.reduced_motion && !has_animation
    ),
  ]
}

pub fn render_chart_fixture(reduced_motion: bool) -> ChartFixtureRender {
  let revenue_series = revenue_series();
  let cost_series = cost_series();
  let series = vec![revenue_series.clone(), cost_series.clone()];
  let x_domain = chart_series_x_domain(&series);
  let y_domain = include_zero(chart_series_y_domain(&series));
  let x_scale =
    ChartScale::new(x_domain, ChartDomain::new(CHART_PADDING, CHART_WIDTH - CHART_PADDING));
  let y_scale =
    ChartScale::new(y_domain, ChartDomain::new(CHART_HEIGHT - CHART_PADDING, CHART_PADDING));
  let fallback_rows = chart_fallback_rows(&series);
  let missing_row_count = fallback_rows.iter().filter(|row| row.missing).count();
  let view_box =
    format!("0 0 {} {}", chart_number_label(CHART_WIDTH), chart_number_label(CHART_HEIGHT));
  let summary = chart_summary(&series);
  let line_path = line_path(&revenue_series, x_scale, y_scale);
  let area_path = area_path(&revenue_series, x_scale, y_scale);
  let bars = bar_rects(&cost_series, x_scale, y_scale);
  let fallback_table = fallback_table_markup(&fallback_rows);
  let motion_attribute =
    if reduced_motion { "data-motion=\"reduced\"" } else { "data-motion=\"standard\"" };
  let svg = format!(
    "<figure data-testid=\"runtime-chart-fixture\" class=\"w-full max-w-3xl\" {motion_attribute}>\
<svg role=\"img\" aria-labelledby=\"runtime-chart-title runtime-chart-description\" \
viewBox=\"{view_box}\" class=\"h-auto w-full\" preserveAspectRatio=\"xMidYMid meet\">\
<title id=\"runtime-chart-title\">Quarterly revenue fixture</title>\
<desc id=\"runtime-chart-description\">{summary}</desc>\
<path data-chart-type=\"area\" d=\"{area_path}\" fill=\"currentColor\" opacity=\"0.16\" />\
<path data-chart-type=\"line\" d=\"{line_path}\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"3\" />\
{bars}\
</svg>{fallback_table}</figure>"
  );

  ChartFixtureRender {
    svg,
    fallback_table,
    summary,
    view_box,
    width: CHART_WIDTH,
    height: CHART_HEIGHT,
    reduced_motion,
    fallback_row_count: fallback_rows.len(),
    missing_row_count,
    bar_count: cost_series.points.iter().filter(|point| point.y.is_some()).count(),
  }
}

fn revenue_series() -> ChartSeries {
  ChartSeries::new(
    "revenue",
    chart_series_label(&ChartSeries::new("revenue", "Revenue", vec![]), ChartColorToken::Primary),
    vec![
      ChartPoint::new(0.0, 12.0),
      ChartPoint::new(1.0, 18.0),
      ChartPoint::missing(2.0),
      ChartPoint::new(3.0, 24.0),
    ],
  )
}

fn cost_series() -> ChartSeries {
  ChartSeries::new(
    "cost",
    chart_series_label(&ChartSeries::new("cost", "Cost", vec![]), ChartColorToken::Secondary),
    vec![
      ChartPoint::new(0.0, 8.0),
      ChartPoint::new(1.0, 11.0),
      ChartPoint::new(2.0, 10.0),
      ChartPoint::new(3.0, 13.0),
    ],
  )
}

fn include_zero(domain: ChartDomain) -> ChartDomain {
  ChartDomain::new(domain.min.min(0.0), domain.max.max(0.0))
}

fn line_path(series: &ChartSeries, x_scale: ChartScale, y_scale: ChartScale) -> String {
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

fn area_path(series: &ChartSeries, x_scale: ChartScale, y_scale: ChartScale) -> String {
  let line = line_path(series, x_scale, y_scale);
  let last_present = series.points.iter().rev().find_map(|point| {
    point
      .y
      .map(|_| (chart_number_label(x_scale.scale(point.x)), chart_number_label(y_scale.scale(0.0))))
  });
  let first_present = series.points.iter().find_map(|point| {
    point
      .y
      .map(|_| (chart_number_label(x_scale.scale(point.x)), chart_number_label(y_scale.scale(0.0))))
  });

  match (first_present, last_present) {
    (Some((first_x, first_y)), Some((last_x, last_y))) => {
      format!("{line} L {last_x} {last_y} L {first_x} {first_y} Z")
    }
    _ => String::new(),
  }
}

fn bar_rects(series: &ChartSeries, x_scale: ChartScale, y_scale: ChartScale) -> String {
  let baseline = y_scale.scale(0.0);

  series
    .points
    .iter()
    .filter_map(|point| {
      point.y.map(|y| {
        let x = x_scale.scale(point.x) - BAR_WIDTH / 2.0;
        let y_position = y_scale.scale(y).min(baseline);
        let height = (baseline - y_scale.scale(y)).abs();
        format!(
          "<rect data-chart-type=\"bar\" data-series=\"{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" />",
          chart_color_attribute(ChartColorToken::Secondary),
          chart_number_label(x),
          chart_number_label(y_position),
          chart_number_label(BAR_WIDTH),
          chart_number_label(height)
        )
      })
    })
    .collect::<Vec<_>>()
    .join("")
}

fn fallback_table_markup(rows: &[dioxus_ui_primitives::ChartFallbackRow]) -> String {
  let body = rows
    .iter()
    .map(|row| {
      let value_label = chart_value_label(&row.series_label, &row.x_label, value_from_row(row));
      format!(
        "<tr><th scope=\"row\">{}</th><td>{}</td><td>{}</td><td>{}</td></tr>",
        row.series_label, row.x_label, row.y_label, value_label
      )
    })
    .collect::<Vec<_>>()
    .join("");

  format!(
    "<table data-testid=\"runtime-chart-fallback-table\"><caption>Chart fallback data</caption>{body}</table>"
  )
}

fn value_from_row(row: &dioxus_ui_primitives::ChartFallbackRow) -> Option<f64> {
  if row.missing { None } else { row.y_label.parse::<f64>().ok() }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn chart_fixture_renders_accessible_svg_and_fallback_table() {
    let render = render_chart_fixture(true);

    assert!(render.svg.contains("role=\"img\""));
    assert!(render.svg.contains("runtime-chart-title"));
    assert!(render.svg.contains("runtime-chart-description"));
    assert!(render.fallback_table.contains("runtime-chart-fallback-table"));
    assert_eq!(render.fallback_row_count, 8);
    assert_eq!(render.missing_row_count, 1);
  }

  #[test]
  fn chart_fixture_covers_first_chart_shapes() {
    let render = render_chart_fixture(true);

    assert!(render.svg.contains("data-chart-type=\"line\""));
    assert!(render.svg.contains("data-chart-type=\"bar\""));
    assert!(render.svg.contains("data-chart-type=\"area\""));
    assert_eq!(render.bar_count, 4);
  }

  #[test]
  fn chart_fixture_exposes_measurement_responsive_and_motion_states() {
    let states = chart_fixture_states();

    assert!(states.iter().any(|state| state.contains("runtime-chart-measurement")));
    assert!(states.iter().any(|state| state.contains("result=640x320")));
    assert!(states.iter().any(|state| state.contains("runtime-chart-responsive")));
    assert!(states.iter().any(|state| state.contains("result=0 0 640 320")));
    assert!(states.iter().any(|state| state.contains("runtime-chart-reduced-motion")));
    assert!(states.iter().any(|state| state.contains("result=true")));
  }
}
