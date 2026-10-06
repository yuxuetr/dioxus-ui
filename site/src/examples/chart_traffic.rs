use dioxus::prelude::*;
use dioxus_shadcn::{
  ChartColorToken, ChartDescription, ChartFallbackRow, ChartFallbackTable, ChartLegend,
  ChartPieSeries, ChartRoot, ChartSlice, ChartSvg, ChartTitle, chart_color_class, chart_view_box,
};

#[component]
pub fn ChartTrafficDemo() -> Element {
  let slices = vec![
    ChartSlice::new("direct", "Direct", 4_200.0, ChartColorToken::Chart1),
    ChartSlice::new("search", "Search", 3_100.0, ChartColorToken::Chart2),
    ChartSlice::new("social", "Social", 1_600.0, ChartColorToken::Chart3),
    ChartSlice::new("email", "Email", 900.0, ChartColorToken::Chart4),
    ChartSlice::new("other", "Other", 400.0, ChartColorToken::Chart5),
  ];
  let total = slices.iter().map(|slice| slice.value).sum::<f64>();
  let rows = slices
    .iter()
    .map(|slice| ChartFallbackRow {
      series_id: slice.id.clone(),
      series_label: slice.label.clone(),
      x_label: format!("{}%", (slice.value / total * 100.0).round()),
      y_label: slice.value.to_string(),
      missing: false,
    })
    .collect::<Vec<_>>();

  rsx! {
    ChartRoot { class: "max-w-sm",
      ChartTitle { id: "chart-traffic-title", "Traffic sources" }
      ChartDescription { id: "chart-traffic-description", "10,200 visits last month, by source." }
      div { class: "relative mx-auto mt-4 w-48",
        ChartSvg {
          view_box: chart_view_box(200.0, 200.0),
          title_id: "chart-traffic-title",
          description_id: "chart-traffic-description",
          ChartPieSeries { slices: slices.clone(), inner_radius: 60.0 }
        }
        // The total sits in the donut's hole.
        div { class: "pointer-events-none absolute inset-0 grid place-content-center text-center",
          span { class: "text-2xl font-bold tabular-nums", "10.2k" }
          span { class: "text-xs text-muted-foreground", "visits" }
        }
      }
      ChartLegend { class: "justify-center",
        for slice in slices {
          span { key: "{slice.id}", class: "inline-flex items-center gap-2",
            span { class: "size-2 rounded-full bg-current {chart_color_class(slice.color)}" }
            "{slice.label}"
          }
        }
      }
      ChartFallbackTable { rows, caption: "Visits by source" }
    }
  }
}
