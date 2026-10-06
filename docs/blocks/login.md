# Login

A sign-in page: a card with email and password fields, a remember-me
checkbox, a submit button, and a second provider button.

```bash
dxui add login
```

Components: Button, Card, Checkbox, Field, Input, Label, Separator.

## Behavior

- `LoginBlock` takes `on_sign_in`, called with a `SignIn` (`email`,
  `password`, `remember`) when a submit passes the checks, and
  `on_provider` for the "Continue with SSO" button.
- The checks run on submit: the email must contain `@` and the password must
  have at least 8 characters. A failed check marks the field invalid and
  shows its error, linked to the input with `aria-describedby`.
- Replace the checks and the sign-up link with your own; the block keeps no
  state outside its signals.

## Accessibility Notes

Each input has a label, an `autocomplete` value password managers read, and
`aria-invalid` with a described error when a check fails. The form submits
with Enter from either field. The separator between the two sign-in paths is
decorative.
