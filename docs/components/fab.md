# Fab

Fab is a floating action button for a screen's main action, and can open a
speed dial of related actions.

## Source Copy

```bash
dxui add fab
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["fab"] }
```

## API Surface

- `Fab`
- `FabAction`
- `fab_class`
- `fab_action_class`

```rust
let mut open = use_signal(|| false);

rsx! {
  // A plain button.
  Fab { "aria-label": "Compose", icon: rsx! { PlusIcon {} }, onclick: move |_| compose() }

  // A speed dial.
  Fab {
    "aria-label": "Create",
    icon: rsx! { PlusIcon {} },
    open: open(),
    on_open_change: move |next| open.set(next),
    FabAction { label: "Photo", onclick: move |_| { take_photo(); open.set(false); }, CameraIcon {} }
    FabAction { label: "Note", onclick: move |_| { new_note(); open.set(false); }, NoteIcon {} }
  }
}
```

The button is fixed to the bottom inline-end corner, above the home
indicator; `fixed: false` keeps it in the flow. Without `on_open_change` it
is a plain button and `onclick` runs. With it, a press calls
`on_open_change`, `open` shows the actions stacked above the button, and the
app closes the dial in each action's `onclick`. `icon` is the button's
content.

## Accessibility Notes

Name the button with `aria-label`. As a speed dial it sets `aria-expanded`
and `aria-controls`, the actions are a group that renders only while open, so
closed actions are out of the tab order, and Escape closes the dial and
returns focus to the button. Each `FabAction` shows its label, which names it
(see [RFC 0061](../rfcs/0061-mobile-navigation.md)).
