# Resizable

Resizable provides controlled panel group, panel, and handle parts. It does not
own pointer dragging or DOM measurement in the first implementation.

## Source Copy

```bash
dxui add resizable
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["resizable"] }
```

## API Surface

- `ResizablePanelGroup`
- `ResizablePanel`
- `ResizableHandle`
- `LayoutOrientation`
- `ResizablePanelState`
- `resizable_panel_style`
- `resizable_clamp`
- `resizable_resize_pair`

## Accessibility Notes

Handles use separator semantics with orientation and disabled attributes.
Applications own keyboard resizing, pointer dragging, and persistence until
runtime measurement adapters are added.
