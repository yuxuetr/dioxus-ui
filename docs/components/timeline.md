# Timeline

Timeline lists events in order along a connector line, with an optional time
and a marker for each.

## Source Copy

```bash
dxui add timeline
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["timeline"] }
```

## API Surface

- `Timeline`
- `TimelineOrientation`
- `TimelineItem`
- `TimelineTime`
- `TimelineMarker`
- `TimelineContent`
- `timeline_class`
- `timeline_item_class`
- `timeline_time_class`
- `timeline_marker_class`
- `timeline_content_class`

```rust
rsx! {
  Timeline {
    TimelineItem {
      TimelineTime { datetime: "2026-10-05", "Oct 5" }
      TimelineMarker {}
      TimelineContent { "Published 0.1.0" }
    }
  }
}
```

`TimelineOrientation::Vertical`, the default, puts the time beside the
marker; `Horizontal` puts it above and the content below. The connector runs
between markers only. `TimelineMarker` draws a primary dot without children;
pass an icon to replace it, and a class such as `text-success` to color it.

## Accessibility Notes

`Timeline` is an `ol` and each item an `li`, so assistive technology reads
the events in order with their count. `TimelineTime` is a `time` element;
pass `datetime`. The marker is `aria-hidden`, so put any state it shows in
the content text too (see [RFC 0059](../rfcs/0059-display-components.md)).
