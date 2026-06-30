# Chart

Chart provides source-copy friendly SVG composition parts for simple line, bar,
and area charts.

## Source Copy

```bash
dxui add chart
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["chart"] }
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
- `chart_line_path`
- `chart_area_path`
- `chart_bar_rects`
- `chart_fallback_rows`
- `chart_summary`

## Accessibility Notes

Use `ChartSvg` with a stable title ID and description ID. Provide
`ChartFallbackTable` or an adjacent table for non-trivial data. Series must be
identified by text, not color alone. Tooltip content is supplemental; pointer
tracking, keyboard cursor exploration, formatting, localization, and external
chart backends remain app-owned.
