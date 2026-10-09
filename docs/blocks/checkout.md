# Checkout

A checkout page: contact, shipping address, shipping method, and card
fields beside an order summary whose total follows the shipping choice.

```bash
dxui add checkout
```

Components: Button, Card, Field, Input, Label, Native Select, Radio Group,
Separator.

## Behavior

- `CheckoutBlock` takes `on_place_order`, called with an `Order` (`email`,
  `name`, `address`, `city`, `postal_code`, `country`, `shipping`, and
  `total_cents`) when Pay passes the checks. The card details are not in
  it: hand them to your payment provider's own fields or SDK.
- The checks run on Pay: an email with `@`, every address field, a country,
  a card number of 12 to 19 digits (spaces allowed), an expiry as `MM/YY`,
  and a 3 or 4 digit code. A failed check marks the field invalid and shows
  its error.
- The shipping method changes the summary's shipping line, its total, and
  the Pay button's amount.
- A placed order replaces the page with a confirmation. Replace `ITEMS`,
  `COUNTRIES`, and `SHIPPING`, and the card fields, with your own.

## Accessibility Notes

The form is named "Checkout" and each section is headed. Every input has a
label and an `autocomplete` token (shipping address parts, `cc-number`,
`cc-exp`, `cc-csc`) that browsers fill from saved details; the card and code
inputs ask for a numeric keyboard. The shipping methods are a radio group
named by their heading. Errors are described on their fields, the summary
is a complementary region named "Order summary", and the confirmation is a
`status`.
