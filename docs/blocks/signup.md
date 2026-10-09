# Signup

An account creation page: a card with name, email, and password fields, a
password strength meter, a terms checkbox, and a submit button.

```bash
dxui add signup
```

Components: Button, Card, Checkbox, Field, Input, Label, Progress.

## Behavior

- `SignupBlock` takes `on_sign_up`, called with a `SignUp` (`name`, `email`,
  `password`) when a submit passes the checks.
- The meter follows each keystroke. A password earns a point each for 8 and
  12 characters, mixed case, a digit, and a symbol, and reads Weak (0 or 1),
  Fair (2), Good (3), or Strong (4 or 5).
- The checks run on submit: a name, an email containing `@`, a password of
  at least 8 characters rated Fair or better, and the terms accepted. A
  failed check marks the field invalid and shows its error.
- Replace the checks, the strength rule, and the sign-in link with your own;
  the block keeps no state outside its signals.

## Accessibility Notes

Each input has a label and an `autocomplete` value; the password uses
`new-password` so password managers offer to generate one. The meter is a
`progressbar` named "Password strength" whose `aria-valuetext` reads the
level, and the password input is described by the level text and, after a
failed check, by its error. The terms checkbox carries `aria-invalid` and its
error the same way.
