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
rsx! {
  // A plain button.
  Fab { "aria-label": "Compose", icon: rsx! { PlusIcon {} }, onclick: move |_| compose() }

  // A speed dial.
  Fab {
    "aria-label": "Create",
    icon: rsx! { PlusIcon {} },
    FabAction { label: "Photo", onclick: move |_| take_photo(), CameraIcon {} }
    FabAction { label: "Note", onclick: move |_| new_note(), NoteIcon {} }
  }
}
```

The button is fixed to the bottom inline-end corner, above the home
indicator; `fixed: false` keeps it in the flow. Without children it is a
plain button and `onclick` runs. With `FabAction` children it is a speed dial
that owns whether it is open (see
[RFC 0077](../rfcs/0077-component-owned-state.md)): a press toggles it, the
actions stack above the button while open, and an action's press runs its
`onclick` and closes the dial. Pass `open` and `on_open_change` to control
it, or `default_open` to start it open; `on_open_change` hears every change.
`icon` is the button's content.

## Accessibility Notes

Name the button with `aria-label`. As a speed dial it sets `aria-expanded`
and `aria-controls`, the actions are a group that renders only while open, so
closed actions are out of the tab order, and Escape closes the dial and
returns focus to the button. Each `FabAction` shows its label, which names it
(see [RFC 0061](../rfcs/0061-mobile-navigation.md)).
