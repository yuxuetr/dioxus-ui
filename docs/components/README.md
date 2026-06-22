# Components

This page is the documentation-site seed for `dioxus-ui` components.

Current preview mode is command-line smoke output from the example crates. A
future docs site should replace this with visual Dioxus Web/Desktop previews.

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
| [Badge](badge.md) | `dxui add badge` | `badge` | Styled |
| [Button](button.md) | `dxui add button` | `button` | Styled |
| [Checkbox](checkbox.md) | `dxui add checkbox` | `checkbox` | Controlled styled part |
| [Dialog](dialog.md) | `dxui add dialog` | `dialog` | Primitive config + styled parts |
| [Dropdown](dropdown.md) | `dxui add dropdown` | `dropdown` | Primitive config + styled parts |
| [Input](input.md) | `dxui add input` | `input` | Styled |
| [Label](label.md) | `dxui add label` | `label` | Styled |
| [Popover](popover.md) | `dxui add popover` | `popover` | Primitive config + styled parts |
| [Select](select.md) | `dxui add select` | `select` | Primitive config + styled parts |
| [Separator](separator.md) | `dxui add separator` | `separator` | Styled |
| [Skeleton](skeleton.md) | `dxui add skeleton` | `skeleton` | Styled |
| [Switch](switch.md) | `dxui add switch` | `switch` | Controlled styled part |
| [Tabs](tabs.md) | `dxui add tabs` | `tabs` | Controlled styled parts |
| [Textarea](textarea.md) | `dxui add textarea` | `textarea` | Styled |
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
