# Marker

Marker provides inline status, bordered row, and labeled separator layouts for
message or activity interfaces.

## Source Copy

```bash
dxui add marker
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["marker"] }
```

## API Surface

- `Marker`
- `MarkerIcon`
- `MarkerContent`
- `MarkerVariant`
- `marker_class`
- `marker_icon_class`
- `marker_content_class`

## Accessibility Notes

`MarkerIcon` is decorative by default. Pass `decorative: false` only when the
icon has app-provided accessible meaning.

For streaming, progress, or async status, apps should explicitly set status or
live-region semantics where needed. Marker does not announce updates by itself.

## Ownership Boundaries

Marker owns inline status, bordered row, and separator styling. Search indexing,
citation resolution, tool execution state, transcript compaction, and popover
details remain app-owned.
