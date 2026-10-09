# Pricing

A pricing page: three plans side by side, a Monthly or Yearly switch, a
highlighted plan, and each plan's features and choose button.

```bash
dxui add pricing
```

Components: Badge, Button, Card, Separator, Toggle Group.

## Behavior

- `PricingBlock` takes `on_choose`, called with a `PlanChoice` (`plan`, the
  plan's id, and `billing`, `Billing::Monthly` or `Billing::Yearly`) when a
  plan's button is pressed.
- The switch changes every price and its period at once. Pressing the
  pressed item again keeps the current period instead of clearing it.
- The plans come from the `PLANS` constant: an id, a name, a description, a
  monthly and a yearly price, features, and whether it is highlighted.
  Replace them with your own.

## Accessibility Notes

The switch is a Toggle Group named "Billing period"; its items are buttons
with `aria-pressed` and arrow-key focus. Each plan is a region named by its
title, so a screen reader lists the plans and their prices read in order.
The check marks before the features are hidden from assistive technology.
