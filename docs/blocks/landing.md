# Landing

A product page: a header with navigation, a hero with two actions, a grid of
six features, a customer quote, an early-access email form, and a footer.

```bash
dxui add landing
```

Components: Avatar, Badge, Button, Card, Input, Separator.

## Behavior

- `LandingBlock` takes `on_get_started`, called by the header's and the
  hero's main buttons, and `on_subscribe`, called with the email the
  early-access form accepted.
- The form checks that the email contains `@` when submitted. A failed
  check marks the input invalid and shows its error, which clears as soon as
  the email is edited; an accepted email replaces the form with a thank-you
  status.
- The hero, the features (the `FEATURES` constant), the quote, and the
  footer are layout in the block, not components: change the copy, the
  sections, and their order in place.

## Accessibility Notes

The header and footer navigation are named "Main" and "Footer", and the
page content sits in `main`. The feature and early-access sections are
named by their headings. The email input is named "Email", described by its
error after a failed check, and the thank-you message is a `status`, so it
is announced. The feature icons are hidden from assistive technology.
