# Components

This page is the documentation-site seed for `dioxus-ui` components.

Current preview mode is command-line smoke output from the example crates. A
future docs site should replace this with visual Dioxus Web/Desktop previews.

For shadcn/ui coverage planning, see the [parity matrix](parity.md),
[complex component batches](complex-batches.md), and
[interaction batch 1 specification](interaction-batch-1.md). For the next
overlay milestone, see the [overlay variant API plan](overlay-variants.md).

For accessibility expectations, see the
[accessibility contract checklist](accessibility.md).

## Install by Source Copy

```bash
dxui init
dxui add button
```

## Install by Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["button"] }
```

## Component Catalog

| Component | CLI | Feature | Status |
| --- | --- | --- | --- |
| [Accordion](accordion.md) | `dxui add accordion` | `accordion` | Controlled styled parts |
| [Alert](alert.md) | `dxui add alert` | `alert` | Styled parts |
| [Alert Dialog](alert-dialog.md) | `dxui add alert-dialog` | `alert-dialog` | Dialog-backed styled parts |
| [Avatar](avatar.md) | `dxui add avatar` | `avatar` | Styled parts |
| [Badge](badge.md) | `dxui add badge` | `badge` | Styled |
| [Button](button.md) | `dxui add button` | `button` | Styled |
| [Card](card.md) | `dxui add card` | `card` | Styled parts |
| [Checkbox](checkbox.md) | `dxui add checkbox` | `checkbox` | Controlled styled part |
| [Dialog](dialog.md) | `dxui add dialog` | `dialog` | Primitive config + styled parts |
| [Drawer](drawer.md) | `dxui add drawer` | `drawer` | Dialog-backed bottom drawer |
| [Dropdown](dropdown.md) | `dxui add dropdown` | `dropdown` | Primitive config + styled parts |
| [Hover Card](hover-card.md) | `dxui add hover-card` | `hover-card` | Popover-backed preview content |
| [Input](input.md) | `dxui add input` | `input` | Styled |
| [Label](label.md) | `dxui add label` | `label` | Styled |
| [Pagination](pagination.md) | `dxui add pagination` | `pagination` | Styled parts |
| [Popover](popover.md) | `dxui add popover` | `popover` | Primitive config + styled parts |
| [Progress](progress.md) | `dxui add progress` | `progress` | Styled |
| [Radio Group](radio-group.md) | `dxui add radio-group` | `radio-group` | Primitive-backed styled parts |
| [Select](select.md) | `dxui add select` | `select` | Primitive config + styled parts |
| [Separator](separator.md) | `dxui add separator` | `separator` | Styled |
| [Sheet](sheet.md) | `dxui add sheet` | `sheet` | Dialog-backed side panel |
| [Skeleton](skeleton.md) | `dxui add skeleton` | `skeleton` | Styled |
| [Slider](slider.md) | `dxui add slider` | `slider` | Primitive-backed styled part |
| [Spinner](spinner.md) | `dxui add spinner` | `spinner` | Styled |
| [Switch](switch.md) | `dxui add switch` | `switch` | Controlled styled part |
| [Table](table.md) | `dxui add table` | `table` | Styled parts |
| [Tabs](tabs.md) | `dxui add tabs` | `tabs` | Controlled styled parts |
| [Textarea](textarea.md) | `dxui add textarea` | `textarea` | Styled |
| [Toggle](toggle.md) | `dxui add toggle` | `toggle` | Controlled styled part |
| [Toggle Group](toggle-group.md) | `dxui add toggle-group` | `toggle-group` | Primitive-backed styled parts |
| [Tooltip](tooltip.md) | `dxui add tooltip` | `tooltip` | Primitive config + styled part |

## Preview Commands

```bash
cargo run -p dioxus-ui-web-demo
cargo run -p dioxus-ui-desktop-demo
```

## CLI Smoke Commands

```bash
cargo run -p dioxus-ui-cli -- list
cargo run -p dioxus-ui-cli -- init --root /tmp/dxui-preview
cargo run -p dioxus-ui-cli -- add button --root /tmp/dxui-preview
cargo run -p dioxus-ui-cli -- add dialog --root /tmp/dxui-preview
```

## Next Preview Milestone

The next docs milestone should add a real Dioxus Web preview app that renders:

- component state matrix
- density variants
- disabled and invalid states
- overlay open/closed states
- generated source examples
