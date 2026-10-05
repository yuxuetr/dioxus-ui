# dioxus-shadcn

shadcn/ui-style components for [Dioxus](https://dioxuslabs.com) 0.7, styled
with Tailwind CSS v4 and the shadcn/ui semantic color tokens. Copy component
source into your app with the `dxui` CLI and edit it freely, or depend on the
crate and enable only the components you use.

| Crate | What it is |
| --- | --- |
| [`dioxus-shadcn`](https://crates.io/crates/dioxus-shadcn) | The styled components, one Cargo feature per component |
| [`dioxus-shadcn-cli`](https://crates.io/crates/dioxus-shadcn-cli) | The `dxui` command, which copies component source into your project |
| [`dioxus-shadcn-primitives`](https://crates.io/crates/dioxus-shadcn-primitives) | Unstyled state and accessibility helpers the components build on |
| [`dioxus-shadcn-core`](https://crates.io/crates/dioxus-shadcn-core) | Shared conventions such as the `classes` composer |

Most apps need only `dioxus-shadcn-cli` or `dioxus-shadcn`.

## Copy the source

```bash
cargo install dioxus-shadcn-cli
dxui init
dxui add button
dxui add dialog
```

`dxui init` writes `assets/dioxus-shadcn.css` and `src/components/ui/mod.rs`.
`dxui add` copies each component, and the `utils.rs` they share, into
`src/components/ui/` and declares its module; it keeps existing files unless
you pass `--overwrite`. `dxui list` prints the 64 components. Copied
components depend only on `dioxus`.

Declare the modules once:

```rust
// src/components/mod.rs
pub mod ui;

// src/main.rs
mod components;
use components::ui::button::{Button, ButtonVariant};
```

## Depend on the crate

```toml
[dependencies]
dioxus-shadcn = { version = "0.1", features = ["button", "dialog"] }
```

```rust
use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonVariant};

fn App() -> Element {
  rsx! {
    Button { variant: ButtonVariant::Outline, onclick: move |_| {}, "Save" }
  }
}
```

Run `dxui init` for the stylesheet. Tailwind generates only the classes it
finds in scanned files, so add an `@source` line for the crate's source to the
stylesheet. Cargo prints where the crate's `Cargo.toml` lives; use its
directory plus `/src`:

```bash
cargo metadata --format-version 1 \
  | jq -r '.packages[] | select(.name == "dioxus-shadcn") | .manifest_path'
```

```css
@import "tailwindcss";
@source "/path/to/dioxus-shadcn-0.1.0/src";
```

## Theme

`assets/dioxus-shadcn.css` is a Tailwind v4 input stylesheet. It defines the
shadcn/ui tokens (`--background`, `--primary`, `--muted`, `--border`, `--ring`,
and the rest) and maps them to Tailwind colors, so `bg-primary` and
`text-muted-foreground` work in your own code too. To rebrand, redefine tokens
in `:root` and `.dark`. Add the `dark` class to a top-level element for the
dark theme.

`dxui theme list` prints 33 presets ported from daisyUI, such as `cupcake`,
`nord`, and `dracula`. `dxui theme add nord dracula` appends them to the
stylesheet; set `data-theme="nord"` on any element to theme its subtree.

## Components

Accordion, Alert, Alert Dialog, Aspect Ratio, Attachment, Avatar, Badge,
Breadcrumb, Bubble, Button, Button Group, Calendar, Card, Carousel, Chart,
Checkbox, Collapsible, Combobox, Command, Context Menu, Data Table, Date
Picker, Dialog, Direction, Drawer, Dropdown, Empty, Field, Hover Card, Input,
Input Group, Input OTP, Item, Kbd, Label, Marker, Menubar, Message, Message
Scroller, Native Select, Navigation Menu, Pagination, Popover, Progress, Radio
Group, Resizable, Scroll Area, Select, Separator, Sheet, Sidebar, Skeleton,
Slider, Sonner, Spinner, Switch, Table, Tabs, Textarea, Toast, Toggle, Toggle
Group, Tooltip, and Typography.

Components are controlled: the app owns state such as `open` or `value` and
receives changes through callbacks such as `on_open_change`. Overlays handle
focus, Escape, outside clicks, and placement, and menus and listboxes handle
the keyboard.

## Status

This is a pre-1.0 release; breaking changes bump the minor version and come
with a migration note in the changelog. The Web renderer is tested in a
browser, including an axe-core accessibility audit. Desktop, iOS, and Android
run a self-test of the main overlays.

- [Component site](https://yuxuetr.github.io/dioxus-ui/)
- [Component docs](https://github.com/yuxuetr/dioxus-ui/tree/main/docs/components)
- [Changelog](https://github.com/yuxuetr/dioxus-ui/blob/main/CHANGELOG.md)
- [Repository](https://github.com/yuxuetr/dioxus-ui)

Licensed under the [MIT License](https://github.com/yuxuetr/dioxus-ui/blob/main/LICENSE).
