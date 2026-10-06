use dioxus::prelude::*;
use dioxus_shadcn::{
  ChartAreaSeries, ChartColorToken, ChartDescription, ChartDomain, ChartFallbackTable,
  ChartLegend, ChartLineSeries, ChartPoint, ChartRoot, ChartScale, ChartSeries, ChartSvg,
  ChartTitle, chart_fallback_rows,
};

#[component]
pub fn ChartRevenueDemo() -> Element {
  let revenue = ChartSeries::new(
    "revenue",
    "Revenue",
    vec![
      ChartPoint::new(0.0, 12.0),
      ChartPoint::new(1.0, 18.0),
      ChartPoint::new(2.0, 15.0),
      ChartPoint::new(3.0, 24.0),
      ChartPoint::new(4.0, 22.0),
    ],
  );
  // Map data to the 640 by 320 view box, leaving room at the edges.
  let x = ChartScale::new(ChartDomain::new(0.0, 4.0), ChartDomain::new(40.0, 600.0));
  let y = ChartScale::new(ChartDomain::new(0.0, 24.0), ChartDomain::new(280.0, 32.0));
  let rows = chart_fallback_rows(std::slice::from_ref(&revenue));

  rsx! {
    ChartRoot { class: "max-w-xl",
      ChartTitle { id: "chart-revenue-title", "Revenue" }
      ChartDescription { id: "chart-revenue-description", "Monthly revenue in thousands, January to May." }
      ChartSvg { title_id: "chart-revenue-title", description_id: "chart-revenue-description",
        ChartAreaSeries { series: revenue.clone(), x_scale: x, y_scale: y, color: ChartColorToken::Primary, baseline: 0.0 }
        ChartLineSeries { series: revenue.clone(), x_scale: x, y_scale: y, color: ChartColorToken::Primary }
      }
      ChartLegend {
        span { class: "inline-flex items-center gap-2",
          span { class: "h-2 w-2 rounded-full bg-primary" }
          "Revenue"
        }
      }
      ChartFallbackTable { rows, caption: "Revenue by month" }
    }
  }
}
