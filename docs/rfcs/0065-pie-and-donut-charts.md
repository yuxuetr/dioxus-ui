# RFC 0065: Pie and Donut Charts

- Status: Accepted
- Created: 2026-10-05

## Summary

Add `ChartPieSeries`, which draws pie and donut slices from values, the
`chart_pie_arcs` geometry it uses, and `Chart1` to `Chart5` color tokens that
read the stylesheet's `--chart-1` to `--chart-5`.

## Current State

The 0.1.0 release notes limit Chart to line, bar, and area series. A share of
a whole, such as traffic by source or a budget split, is the most common
chart those cannot draw. The stylesheet already defines `--chart-1` to
`--chart-5`, and every theme preset maps them
([RFC 0057](0057-theme-presets.md)), but `ChartColorToken` has no way to use
them; its six tokens are status and text colors, too few and too alike for
neighboring slices.

## Decision

### Colors

`ChartColorToken` gains `Chart1` to `Chart5`, mapped to `text-chart-1` to
`text-chart-5`. They suit any series, not only slices.

### Geometry

`chart_pie_arcs(values, center, radius, inner_radius)` returns one
`ChartArc` per value: its SVG path, its start and end angles, and its
fraction of the total. Slices start at the top and run clockwise. A value
that is negative, zero, or not finite counts as zero and gets an empty path;
a zero total draws nothing. A slice of the whole circle is drawn as two half
arcs, since one SVG arc cannot start and end at the same point. With
`inner_radius` above zero the slices are a ring, whose full-circle case uses
the even-odd rule to cut the hole.

### `ChartPieSeries`

Takes `slices: Vec<ChartSlice>` (an id, a label, a value, and a color),
`inner_radius` (0 for a pie), and the center and radius in the SVG's view
box. Each slice is a `path` filled with its color and outlined with the
background color, so neighbors stay apart, and carries `data-slice` and
`data-fraction`. It renders inside `ChartSvg` like the other series, which
names the chart with its title and description; a `ChartLegend` and
`ChartFallbackTable` give the values in text.

## Alternatives

- **Generate colors per slice.** Generated hues would not follow a theme;
  five tokens follow every preset, and apps can merge small slices.
- **Conic gradients.** CSS draws a pie with one element but cannot give each
  slice its own element, outline, or data attribute.

## Verification

- Unit tests for fractions, angles, the empty and full-circle cases, and
  donut paths.
- SSR tests for the slice paths and attributes.
- A site example of a donut with a legend and fallback table, audited in both
  themes and every preset.
