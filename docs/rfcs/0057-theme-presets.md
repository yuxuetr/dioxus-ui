# RFC 0057: Theme Presets

- Status: Accepted
- Created: 2026-10-05

## Summary

Ship the daisyUI themes as theme presets: token sets scoped by
`[data-theme="name"]` that an app adds with `dxui theme add <name>` and
switches at runtime by setting one attribute. Foreground colors that miss
WCAG AA are darkened or lightened until they pass, and a gate keeps them
passing.

## Current State

`assets/dioxus-shadcn.css` defines one theme: the shadcn/ui zinc tokens in
`:root` and `.dark` ([RFC 0051](0051-semantic-color-tokens.md)). Rebranding
means editing about 35 values per color scheme by hand. daisyUI ships 35
themes switched by `data-theme`, and its palettes are MIT-licensed.

Measured on daisyUI's theme sources on 2026-10-05, 18 of the 35 themes have
at least one text pair below the 4.5:1 AA ratio, among base, primary,
secondary, and error with their content colors. The worst is `dark`
secondary at 3.05:1.

Checkbox draws its check mark as a data URI SVG, white by default and dark
under the `dark:` variant. A preset whose primary color is light, such as
`cupcake`, would get a white check on light teal.

## Decision

### A preset is a scoped token set

Each preset is one rule that sets every token `:root` sets, plus
`color-scheme`:

```css
[data-theme="cupcake"] {
  color-scheme: light;
  --radius: 1rem;
  --background: oklch(0.978 0.004 56.375);
  /* ... */
}
```

Setting `data-theme` on any element themes its subtree, so presets nest, and
an app switches theme by changing one attribute. A preset rule comes after
`.dark` in the stylesheet, so it wins on the element that has both. A preset
is a single color scheme, as in daisyUI; dark presets set
`color-scheme: dark` and need no `dark` class.

### Token mapping

| Token | daisyUI source |
| --- | --- |
| `background`, `card`, `popover` | `base-100` |
| `foreground`, `card-foreground`, `popover-foreground` | `base-content` |
| `primary`, `primary-foreground` | `primary`, `primary-content` |
| `secondary`, `secondary-foreground` | `secondary`, `secondary-content` |
| `muted`, `accent`, `sidebar-accent` | `base-200` |
| `muted-foreground` | `base-content` mixed toward `base-200` |
| `accent-foreground`, `sidebar-foreground`, `sidebar-accent-foreground` | `base-content` |
| `destructive`, `destructive-foreground` | `error`, `error-content` |
| `success`, `warning`, `info` | `success`, `warning`, `info` |
| `border`, `input`, `sidebar-border` | `base-300` |
| `ring`, `sidebar-ring` | `primary` |
| `chart-1` to `chart-5` | `primary`, `secondary`, `accent`, `info`, `success` |
| `sidebar` | `base-200` |
| `sidebar-primary`, `sidebar-primary-foreground` | `primary`, `primary-content` |
| `radius` | `radius-box` |

shadcn/ui's `accent` is the hover surface of menus and list items, not a brand
color, so it maps to `base-200`; daisyUI's `accent` becomes a chart color.
`muted-foreground` is `base-content` mixed toward `base-200` by the largest
step, at most 40%, that keeps 4.5:1 on `muted` and on `background`.

### Contrast adjustment

For each pair that `scripts/theme-presets.mjs` emits (foreground on
background, card, popover, muted, accent, and sidebar; primary, secondary,
destructive, and sidebar primary on their foregrounds), a foreground below
4.5:1 keeps its hue and chroma and moves its OKLCH lightness away from the
background in 0.01 steps until it passes. Black or white always reaches
4.58:1 on any color, so the loop always ends. The generated file lists the
adjusted tokens in its header comment.

`npm run verify:theme-presets` recomputes the ratios from the committed files
and fails below 4.5:1, so a hand edit cannot regress a preset.

### What ships

The 33 daisyUI themes other than `light` and `dark`, whose roles the default
theme already fills and whose names would read as the `dark` class. Each
preset is one file, `crates/dioxus-shadcn-cli/themes/<name>.css`, embedded in
the CLI like the templates, with daisyUI's copyright notice in its header.
`scripts/theme-presets.mjs <daisyui themes dir>` regenerates them from
daisyUI's `packages/daisyui/src/themes`; the presets were generated from
daisyUI commit `9adbeaa25981` (2026-10-05).

### CLI

- `dxui theme list` prints each preset with its color scheme.
- `dxui theme add <name>...` appends the presets to
  `assets/dioxus-shadcn.css` between `/* dxui theme: <name> */` markers and
  skips a preset already present, so running it twice changes nothing. It
  fails without touching the file when a name is unknown or the stylesheet is
  missing.

### Checkbox check mark

The check and the indeterminate dash become a `::before` mask filled with
`--primary-foreground`, so they follow any preset, and Checkbox no longer
uses the `dark:` variant. Apps keep the `dark:` variant for their own classes;
with a dark preset they add the `dark` class too if they rely on it.

### Site

The header gains a theme menu listing the default theme and every preset. The
Theming page shows each preset's swatches with the `dxui theme add` command,
and `npm run verify:site` audits contrast and axe-core in every preset on the
home page and one component page.

## Alternatives

- **Generate presets at runtime from daisyUI variables.** CSS `color-mix()`
  could derive `muted-foreground`, but the contrast gate needs literal values,
  and a mixed color cannot be adjusted per pair.
- **Ship daisyUI colors unchanged.** 18 presets would fail the site's own
  contrast check.
- **Dark presets set the `dark` class.** CSS cannot add a class; requiring
  both an attribute and a class doubles the switch for every app.

## Verification

- Unit tests for `dxui theme list` and `dxui theme add`, including the repeat
  and unknown-name cases.
- `npm run verify:theme-presets`, reverse-verified against a preset with a
  lowered foreground.
- The Checkbox runtime check reads the mask color in a light and a dark
  preset.
- `npm run verify:site` in every preset.
