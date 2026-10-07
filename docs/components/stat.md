# Stat

Stat shows key numbers, such as revenue or sign-ups, with a title, an
optional description, and an optional figure.

## Source Copy

```bash
dxui add stat
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["stat"] }
```

## API Surface

- `StatGroup`
- `StatGroupOrientation`
- `Stat`
- `StatTitle`
- `StatValue`
- `StatDescription`
- `StatFigure`
- `stat_group_class`
- `stat_class`
- `stat_title_class`
- `stat_value_class`
- `stat_description_class`
- `stat_figure_class`

```rust
rsx! {
  StatGroup {
    Stat {
      StatTitle { "Revenue" }
      StatValue { "$45,231" }
      StatDescription { "+20% from last month" }
    }
  }
}
```

`StatGroupOrientation::Horizontal`, the default, puts dividers between the
stats and scrolls sideways when narrow; `Vertical` stacks them. `StatFigure`
takes a second column spanning the rows, wherever it sits in source.

## Accessibility Notes

`StatGroup` is a `dl`, each `Stat` a `div` in it, `StatTitle` a `dt`, and the
value, description, and figure `dd`s, so assistive technology reads each
value with its title. Keep `Stat` inside a `StatGroup` and the title first.
Mark a decorative figure icon `aria-hidden` (see
[RFC 0059](../rfcs/0059-display-components.md)).
