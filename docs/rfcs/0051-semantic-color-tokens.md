# RFC 0051: Semantic Color Tokens

- Status: Accepted
- Created: 2026-10-05

## Summary

Components move from fixed Tailwind palette utilities (`bg-zinc-100`,
`ring-blue-600`) to the shadcn/ui semantic color tokens (`bg-secondary`,
`ring-ring`). The generated stylesheet defines the tokens in the shadcn/ui v4
layout. The dark theme then redefines only the tokens, and app classes keep
their colors. This supersedes RFC 0047's palette remap once the components
are migrated.

## Current State

As of M175:

- Color utilities in component class strings, counted with
  `(bg|text|border|ring|...)-(zinc|blue|red|white|green|amber|emerald|black)`:

  | Location | Uses | Files |
  | --- | --- | --- |
  | `crates/dioxus-ui/src` | 629 | 62 |
  | `crates/dioxus-ui-cli/templates` | 540 | 62 |
  | `crates/dioxus-ui-primitives/src` (chart colors) | 7 | 1 |
  | `crates/dioxus-ui-core/src` (class merge tests only) | 6 | 1 |

- The most common crate uses are `bg-zinc-100` (77), `text-zinc-950` (76),
  `border-zinc-200` (59), `bg-white` (51), `ring-blue-600` (47),
  `text-zinc-600` (29), and `text-zinc-500` (28). Blue is the brand color:
  the primary Button, the Badge default, checked Checkbox, Radio Group,
  Switch, Slider, and Progress fills, the Button link, and every focus ring.
  Red marks destructive actions and invalid fields. Green, amber, emerald,
  and blue soft surfaces mark the Toast, Sonner, and Attachment states.
- The CLI default stylesheet maps `--color-background` and
  `--color-foreground` to `--dxui-background` and `--dxui-foreground`, which
  no component uses. Its `.dark` block (RFC 0047) remaps about 80 palette
  variables, so app classes under `.dark` change color too.
- The drawn Checkbox (RFC 0045) paints its marks as data-URI SVG backgrounds
  with a literal white stroke.

## Decision

### Token set and layout

The stylesheet uses the shadcn/ui v4 layout: `:root` and `.dark` hold the
values, and `@theme inline` maps each token to a Tailwind color, so
`bg-primary` compiles to `background-color: var(--primary)`. The values are
the shadcn/ui zinc base color, with the deviations listed below.

| Token | Light | Dark |
| --- | --- | --- |
| `--background` | `oklch(1 0 0)` | `oklch(0.141 0.005 285.823)` |
| `--foreground` | `oklch(0.141 0.005 285.823)` | `oklch(0.985 0 0)` |
| `--card`, `--popover` | `oklch(1 0 0)` | `oklch(0.21 0.006 285.885)` |
| `--card-foreground`, `--popover-foreground` | `oklch(0.141 0.005 285.823)` | `oklch(0.985 0 0)` |
| `--primary` | `oklch(0.21 0.006 285.885)` | `oklch(0.92 0.004 286.32)` |
| `--primary-foreground` | `oklch(0.985 0 0)` | `oklch(0.21 0.006 285.885)` |
| `--secondary`, `--muted`, `--accent` | `oklch(0.967 0.001 286.375)` | `oklch(0.274 0.006 286.033)` |
| `--secondary-foreground`, `--accent-foreground` | `oklch(0.21 0.006 285.885)` | `oklch(0.985 0 0)` |
| `--muted-foreground` | `oklch(0.442 0.017 285.786)` * | `oklch(0.705 0.015 286.067)` |
| `--destructive` | `oklch(0.577 0.245 27.325)` | `oklch(0.704 0.191 22.216)` |
| `--destructive-foreground` * | `oklch(0.985 0 0)` | `oklch(0.141 0.005 285.823)` |
| `--border` | `oklch(0.92 0.004 286.32)` | `oklch(1 0 0 / 10%)` |
| `--input` | `oklch(0.92 0.004 286.32)` | `oklch(1 0 0 / 15%)` |
| `--ring` | `oklch(0.552 0.016 285.938)` * | `oklch(0.552 0.016 285.938)` |
| `--chart-1` to `--chart-5` | shadcn/ui zinc values | shadcn/ui zinc values |
| `--sidebar`, `--sidebar-foreground`, `--sidebar-primary`, `--sidebar-primary-foreground`, `--sidebar-accent`, `--sidebar-accent-foreground`, `--sidebar-border` | shadcn/ui zinc values | shadcn/ui zinc values |
| `--sidebar-ring` | `oklch(0.552 0.016 285.938)` * | `oklch(0.552 0.016 285.938)` |
| `--success` * | `oklch(0.627 0.194 149.214)` | `oklch(0.723 0.219 149.579)` |
| `--warning` * | `oklch(0.666 0.179 58.318)` | `oklch(0.769 0.188 70.08)` |
| `--info` * | `oklch(0.546 0.245 262.881)` | `oklch(0.623 0.214 259.815)` |
| `--radius` | `0.5rem` * | |

Deviations from shadcn/ui, marked * above:

- `--muted-foreground` is zinc-600 in the light theme, not zinc-500.
  shadcn/ui puts muted text on `bg-muted` (Tabs, Command), where zinc-500
  reaches only 4.4:1 and fails the 4.5:1 check that
  `npm run verify:runtime-interactions` runs. zinc-600 is what descriptions
  use today.
- `--ring` and `--sidebar-ring` are zinc-500 in the light theme, not
  zinc-400, so the focus ring keeps 3:1 against white (WCAG 1.4.11). zinc-400
  reaches about 2.6:1.
- `--destructive-foreground` is kept from shadcn/ui v3. v4 writes
  `text-white` and adds `dark:bg-destructive/60`; one token keeps component
  classes free of `dark:` variants, and dark text on the lighter dark-theme
  red keeps 4.5:1.
- `--success`, `--warning`, and `--info` are added for the Toast, Sonner,
  Attachment, and Chart states, which shadcn/ui does not cover. Each is one
  strong color: inline surfaces use it at `/10`, borders at `/30` or `/50`,
  and status dots and chart series use it whole. Text on those surfaces stays `foreground`
  or `muted-foreground`, so no status foreground token is needed.
- `--radius` is `0.5rem`. `@theme inline` derives `--radius-sm` to
  `--radius-xl` from it as shadcn/ui does, and `0.5rem` gives back
  Tailwind's default `rounded-sm` to `rounded-xl` sizes, so radii do not
  change.

The stylesheet also declares `@custom-variant dark (&:is(.dark *));` as
shadcn/ui does, so an app's own `dark:` utilities follow the same `dark`
class as the theme instead of the system preference.

### Mapping

Each palette role maps to one token class:

| Role today | Token class |
| --- | --- |
| page and control surfaces: `bg-white` | `bg-background` |
| Card, Alert, Table surfaces | `bg-card text-card-foreground` |
| floating surfaces (Popover, Dropdown, Select, Combobox, Command, Hover Card, Date Picker, Menubar, Context Menu, Chart tooltip) | `bg-popover text-popover-foreground` |
| main text: `text-zinc-950`, `text-zinc-900` | `text-foreground` |
| secondary text: `text-zinc-600`, `text-zinc-500`, `text-zinc-700`, `placeholder:text-zinc-500` | `text-muted-foreground` |
| disabled or decorative text: `text-zinc-400` | `text-muted-foreground` |
| hover, focus, and active item fills: `bg-zinc-100`, `hover:bg-zinc-100`, `bg-zinc-200` | `bg-accent text-accent-foreground` |
| quiet surfaces: `bg-zinc-50`, Skeleton, Slider and Progress tracks, Table footer | `bg-muted` |
| secondary Button and Badge: `bg-zinc-100 text-zinc-900` | `bg-secondary text-secondary-foreground` |
| borders: `border-zinc-200`, `border-zinc-300`, separators `bg-zinc-200` | `border-border` or `border-input` for form controls, `bg-border` for separators |
| brand fills: primary Button, Badge default, checked Checkbox, Radio Group, Switch, Slider range and thumb border, Progress, Toggle Group and Bubble default | `bg-primary text-primary-foreground`, `border-primary` |
| link text: `text-blue-600` | `text-primary` |
| focus rings: `ring-blue-600` | `ring-ring` |
| Tooltip: `bg-zinc-950 text-white` | `bg-primary text-primary-foreground` |
| pressed Toggle: `bg-zinc-900 text-white`, outline `bg-blue-50 text-blue-700` | `bg-accent text-accent-foreground`, as shadcn/ui |
| destructive fills: `bg-red-600 text-white hover:bg-red-700` | `bg-destructive text-destructive-foreground hover:bg-destructive/90` |
| invalid fields: `border-red-500`, `ring-red-500` | `border-destructive`, `ring-destructive` |
| destructive text and items: `text-red-600 focus:bg-red-50`, Alert destructive | `text-destructive focus:bg-destructive/10` |
| Bubble and Attachment states (inline) | `border-<state>/30 bg-<state>/10` with `text-foreground`; `<state>` is `success`, `warning`, `info`, or `destructive` |
| Toast and Sonner states (floating over content, so the surface stays opaque) | `bg-popover text-popover-foreground border-<state>/50`; Sonner dots `bg-<state>`; descriptions `text-muted-foreground` |
| Chart color tokens | `text-primary`, `text-muted-foreground`, `text-success`, `text-warning`, `text-destructive`, `text-foreground` |
| Sidebar surface, borders, item fills, and focus ring | `bg-sidebar text-sidebar-foreground`, `border-sidebar-border`, `bg-sidebar-accent text-sidebar-accent-foreground`, `ring-sidebar-ring` |
| modal overlays: `bg-black/50` | unchanged, as shadcn/ui |

Blue stops being the brand color: checked states and the primary Button turn
near-black in the light theme and near-white in the dark theme, as in
shadcn/ui. An app restores a blue brand by setting `--primary` and `--ring`.

The Checkbox marks stay data-URI backgrounds (RFC 0045), but a data URI
cannot read a CSS variable, and the white stroke disappears on the
near-white dark-theme primary. The marks therefore get a `dark:` twin with a
zinc-900 stroke, the dark `--primary-foreground`. Drawing the marks as an
inline SVG over the input would follow the token, but needs a wrapper
element, which breaks `peer-*` styling on a sibling Label. An app that sets
`--primary-foreground` to a color far from white or zinc-900 overrides the
marks through `class`; reevaluate when an app reports that.

### Stylesheet setup

- The CLI default `assets/dioxus-ui.css` gets the `:root`, `.dark`, and
  `@theme inline` token blocks and the custom variant. They replace the
  `--dxui-background` and `--dxui-foreground` variables; `bg-background` and
  `text-foreground` keep working.
- The Web and Desktop preview inputs carry the same blocks, and
  `npm run verify:css-inputs` fails when they drift from the CLI blocks.
- Until the components are migrated (M176.2 to M177), `.dark` keeps the
  RFC 0047 palette remap next to the token values, so migrated and
  unmigrated components both turn dark. M178 removes the remap; from then
  on `.dark` redefines only the tokens and app palette classes keep their
  colors.
- Crate-mode apps get the stylesheet the same way as source-copy apps: run
  `dxui init`, which writes only `assets/dioxus-ui.css` and an empty
  `src/components/ui/mod.rs`, or copy the token blocks from the theming
  docs.

### Breaking change

A crate-mode app whose stylesheet does not define the tokens loses the
component colors after upgrading: Tailwind generates no `bg-primary` without
the `@theme inline` mapping. Source-copy apps keep their copied components
until they add them again. The CHANGELOG records the change and the
migration step in M177.4.

### Palette gate

M178.2 adds a verifier that fails when a crate, primitives, or template
class string uses a palette color utility. Allowed: `bg-black/50` overlays,
`transparent`, `current`, and test assertions about app classes passed in
through `class`.

## Scope

In scope:

- the token blocks in the CLI and preview stylesheets, and their drift check
- migrating every component in the crate, the primitives chart colors, and
  the templates
- moving the dark theme onto the tokens and retiring the palette remap
- the palette gate and theming docs

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| A shadcn/ui `@layer base` that sets `body` to `bg-background text-foreground` and every border to `border-border` | Component classes name their colors, and the generated stylesheet should not restyle app elements | An app reports a page surface that does not follow the theme without its own class |
| Other base colors (slate, neutral, stone) or a theme switcher | One zinc theme matches today's look; apps redefine the tokens for others | The component site (M179) adds a theme picker |
| Status foreground tokens (`--success-foreground`) | No component puts text on a solid status fill | A component adds a solid status fill with text |
| Classes passed in by apps | App classes are the app's choice and stay palette or token as written | |
| The preview pages' own palette classes | They are fixtures, not components; the token theme check uses component surfaces | The preview theme check needs a fixture on tokens |

## Verification

- M176.2: `npm run verify:css-inputs` passes when the preview token blocks
  match the CLI blocks and fails when one value differs.
- M177: unit tests assert the token classes; the compiled preview
  stylesheet contains them; `npm run verify:runtime-interactions` keeps
  4.5:1 text contrast in both themes.
- M178: the palette gate fails on a reintroduced `bg-blue-600` in a
  component and passes on the migrated tree; the dark theme check passes
  without the palette remap.
