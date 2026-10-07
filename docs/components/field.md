# Field

Field provides form layout composition around existing controls. It owns spacing
and state attributes only; control IDs, labels, descriptions, validation logic,
and messages remain app-owned.

## Source Copy

```bash
dxui add field
```

This generates:

```text
src/components/ui/field.rs
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["field"] }
```

## API Surface

- `Field`
- `FieldLabel`
- `FieldDescription`
- `FieldError`
- `FieldGroup`
- class helpers for every part

## Accessibility Notes

- `Field` exposes invalid and disabled state through data attributes.
- `FieldLabel` renders a native `label`. Pass `r#for` with the control's
  `id`, or wrap the control; an empty `for` is left out. Other attributes,
  such as `id`, pass through.
- `FieldDescription` and `FieldError` pass other attributes through, so give
  them an `id` and point the control's `aria-describedby` at it. Apps own
  `aria-describedby`, `aria-invalid`, and live-region behavior on the actual
  form control.
