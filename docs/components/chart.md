# Chart

Chart provides source-copy friendly SVG composition parts for simple line, bar,
and area charts.

## Source Copy

```bash
dxui add chart
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["chart"] }
```

## API Surface

- `ChartRoot`
- `ChartSvg`
- `ChartTitle`
- `ChartDescription`
- `ChartLegend`
- `ChartFallbackTable`
- `ChartTooltipSlot`
- `ChartLineSeries`
- `ChartBarSeries`
- `ChartAreaSeries`
- `ChartPoint`
- `ChartSeries`
- `ChartDomain`
- `ChartScale`
- `ChartColorToken`
- `ChartFallbackRow` (`ChartFallbackTable`'s `rows`)
- `ChartBarRect` (what `chart_bar_rects` returns)
- `chart_line_path`
- `chart_area_path`
- `chart_bar_rects`
- `chart_fallback_rows`
- `chart_summary`
- `chart_domain`, `chart_scale_value`, `chart_view_box`
- `ChartPieSeries`, `ChartSlice`, `ChartArc` (what `chart_pie_arcs` returns)
- `chart_pie_arcs`
- `CHART_COLOR_CLASSES`

### Pie and donut charts

`ChartPieSeries` draws `slices`, each a `ChartSlice` with an id, a label, a
value, and a color, as shares of their total, from the top and clockwise:

```rust
ChartSvg { view_box: chart_view_box(200.0, 200.0), title_id: "traffic-title", description_id: "traffic-description",
  ChartPieSeries {
    inner_radius: 60.0,
    slices: vec![
      ChartSlice::new("direct", "Direct", 4200.0, ChartColorToken::Chart1),
      ChartSlice::new("search", "Search", 3100.0, ChartColorToken::Chart2),
    ],
  }
}
```

`center` (default `(100.0, 100.0)`) and `radius` (default 96) are in view box
units; `inner_radius` above zero makes a donut, and `gap` (default 2) outlines
slices with the background color. Values that are negative, zero, or not
finite draw nothing. `ChartColorToken::Chart1` to `Chart5` read the
`--chart-1` to `--chart-5` tokens, which every theme preset sets (see
[RFC 0065](../rfcs/0065-pie-and-donut-charts.md)).

`chart_color_class` lives in the primitives crate, which Tailwind's `@source`
does not scan in crate mode; `CHART_COLOR_CLASSES` spells its classes out in
this crate so they are generated.


## Accessibility Notes

Use `ChartSvg` with a stable title ID and description ID. Provide
`ChartFallbackTable` or an adjacent table for non-trivial data. Series must be
identified by text, not color alone: give pie slices a legend with their
labels, and their values in `ChartFallbackTable`. Tooltip content is supplemental; pointer
tracking, keyboard cursor exploration, formatting, localization, and external
chart backends remain app-owned.
