# Popover

Popover provides primitive overlay configuration with styled content, header,
title, and description parts.

## Source Copy

```bash
dxui add popover
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["popover"] }
```

## API Surface

- `PopoverContent`
- `PopoverHeader`
- `PopoverTitle`
- `PopoverDescription`
- `PopoverPrimitiveConfig`
- `popover_content_class`

## Accessibility Notes

Use popovers for supplemental interactive content. Configure dismissal behavior
carefully so keyboard and pointer users can close the surface predictably.
