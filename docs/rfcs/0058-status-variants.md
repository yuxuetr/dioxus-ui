# RFC 0058: Status Variants

- Status: Accepted
- Created: 2026-10-05

## Summary

Add `Success`, `Warning`, and `Info` variants to Alert and Badge, and the
`--success-foreground`, `--warning-foreground`, and `--info-foreground`
tokens that solid status surfaces need for readable text.

## Current State

The stylesheet defines `--success`, `--warning`, and `--info`
([RFC 0051](0051-semantic-color-tokens.md)), but only Toast, Sonner, Bubble,
and Attachment read them, as borders, dots, and tints. Alert has `Default`
and `Destructive`; Badge has `Default`, `Secondary`, `Destructive`, and
`Outline`. An app that shows a "Paid" badge or a "Saved" alert restyles the
component through `class`. daisyUI has these variants for both components.

Measured in the default theme on 2026-10-05:

| Color | White text | Dark text | As text on the background |
| --- | --- | --- | --- |
| light `--success` | 3.08:1 | 6.18:1 | 3.22:1 |
| light `--warning` | 3.07:1 | 6.22:1 | 3.20:1 |
| light `--info` | 5.03:1 | 3.79:1 | 5.25:1 |
| dark `--success` | 2.12:1 | 8.97:1 | 8.97:1 |
| dark `--warning` | 2.05:1 | 9.32:1 | 9.32:1 |
| dark `--info` | 3.60:1 | 5.29:1 | 5.29:1 |

No single text color works on all three, and light success and warning
cannot be text on the page.

## Decision

### Tokens

| Token | Light | Dark |
| --- | --- | --- |
| `--success-foreground` | `oklch(0.141 0.005 285.823)` | `oklch(0.141 0.005 285.823)` |
| `--warning-foreground` | `oklch(0.141 0.005 285.823)` | `oklch(0.141 0.005 285.823)` |
| `--info-foreground` | `oklch(0.985 0 0)` | `oklch(0.141 0.005 285.823)` |

`@theme inline` maps them to `text-success-foreground` and the rest. Presets
take them from daisyUI's `success-content`, `warning-content`, and
`info-content`, adjusted like the other foregrounds
([RFC 0057](0057-theme-presets.md)).

### Badge

`Success`, `Warning`, and `Info` are solid, like `Default` and `Destructive`:
`border-transparent bg-success text-success-foreground`.

### Alert

The status color cannot be the text color (3.2:1 for light success), so the
status variants tint the surface and border and keep the foreground text:
`border-success/50 bg-success/10 text-foreground`. `Destructive` keeps
`text-destructive`, which passes.

### Gate

`npm run verify:theme-presets` adds each foreground on its status color, and
`foreground` on each status color at 10% over `card`.

## Migration

Adding enum variants breaks exhaustive `match` expressions on `AlertVariant`
and `BadgeVariant`; add an arm or a wildcard. Apps whose stylesheet predates
0.2.0 add the three tokens to `:root` and `.dark` and the three
`--color-*-foreground` lines to `@theme inline`; without them the status
badges draw text in the inherited color.

## Verification

- SSR tests for each new variant's classes in the crate.
- Site examples for the new variants, audited by `npm run verify:site` in
  both themes and every preset.
- `npm run verify:theme-presets` with the new pairs.
