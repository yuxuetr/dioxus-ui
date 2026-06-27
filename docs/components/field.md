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
dioxus-ui = { version = "0.1", default-features = false, features = ["field"] }
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
- `FieldLabel` renders a native `label`; apps own `for` or wrapping
  association.
- Apps own `aria-describedby`, `aria-invalid`, and live-region behavior on the
  actual form control.
