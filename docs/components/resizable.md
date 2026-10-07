# Resizable

Resizable provides controlled panel group, panel, and handle parts. The app
owns the panel sizes in percent; the handle reports keyboard and pointer
resizes as a delta.

## Source Copy

```bash
dxui add resizable
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["resizable"] }
```

## API Surface

- `ResizablePanelGroup`
- `ResizablePanel`
- `ResizableHandle`
- `LayoutOrientation`
- `ResizablePanelState`
- `resizable_clamp`
- `resizable_resize_pair`

## Resizing

`ResizableHandle` takes the size of the panel before it as `value` and its
limits, and reports a delta in percent through `on_resize`. Apply it with
`resizable_resize_pair`:

```rust
let mut panels = use_signal(|| {
  (ResizablePanelState::new(30.0, 20.0, 80.0), ResizablePanelState::new(70.0, 20.0, 80.0))
});

rsx! {
  ResizablePanelGroup {
    ResizablePanel { id: "files", size: panels().0.size, "Files" }
    ResizableHandle {
      "aria-controls": "files",
      "aria-label": "Resize files",
      value: panels().0.size,
      min: 20.0,
      max: 80.0,
      on_resize: move |delta| {
        let (first, second) = panels();
        panels.set(resizable_resize_pair(first, second, delta));
      },
    }
    ResizablePanel { size: panels().1.size, "Editor" }
  }
}
```

- The handle is a Tab stop unless disabled. In a horizontal group ArrowRight
  and ArrowLeft move it by `step` (default 10); in a vertical group ArrowDown
  and ArrowUp do. Home and End move it to `min` and `max`.
- A primary-button drag captures the pointer and tracks it, measured against
  the handle's parent, so the handle must be a direct child of the group.
  The edge stays under the pointer after a limit stops the resize.
- `aria-valuenow`, `aria-valuemin`, and `aria-valuemax` mirror `value`,
  `min`, and `max`. Pass `aria-controls` with the panel's `id` and an
  `aria-label`.
- A disabled handle reports nothing and leaves the Tab order.
- Global and element attributes pass through the group, panels, and handle.

## Accessibility Notes

Handles follow the APG window splitter pattern: a focusable separator with
value attributes. `aria-orientation` describes the separator line, so it is
`vertical` between side-by-side panels. Right-to-left groups, keyboard
collapse, and persisted layouts stay with the app.
