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

Each library crate follows semver on its whole public API, and every public
item has a doc comment. `dioxus-shadcn-primitives` makes the same promise as
`dioxus-shadcn`, so an app may depend on it directly, for example to
implement the runtime adapter traits; `dioxus-shadcn` re-exports the
primitive types its components take ([RFC 0079](https://github.com/yuxuetr/dioxus-ui/blob/main/docs/rfcs/0079-public-surface.md)).

## Copy the source

```bash
cargo install dioxus-shadcn-cli
dxui init
dxui add button dialog
```

`dxui init` writes `assets/dioxus-shadcn.css` and `src/components/ui/mod.rs`,
which starts with `#![allow(dead_code, unused_imports)]`: an app uses a few
variants and props of each component, and without it a binary crate warns
about the rest.
`dxui add` copies each component, and the helper files it uses, into
`src/components/ui/` and declares its module; it keeps existing files unless
you pass `--overwrite`, and says which files it kept. `dxui diff [<name>...]`
shows how your copies, all of them unless you name some, differ from the
templates of the installed CLI.
`dxui list` prints the 82 components. Copied
components depend only on `dioxus`.

`dxui add` also takes a block, a whole screen such as `login`: it copies the
block to `src/blocks/` with the components it uses. `dxui list blocks`
prints the blocks.

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
dioxus-shadcn = { version = "0.6", features = ["button", "dialog"] }
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

Run `dxui init` in the app's directory for the stylesheet. Tailwind
generates only the classes it finds in scanned files, so `dxui init` also
writes an `@source` line for the crate's source, after the import, from
`cargo metadata`:

```css
@import "tailwindcss";
@source "/home/me/.cargo/registry/src/index.crates.io-…/dioxus-shadcn-0.6.3/src";
```

The path names one version, so run `dxui init` again after upgrading the
crate: it replaces the line that names the previous version and says so.
Without it Tailwind keeps scanning the old source and misses classes the new
version added. `dxui init` manages the line only in
`assets/dioxus-shadcn.css`; an app that compiles another stylesheet copies the
line from there.

A crate from Cargo's registry lives in a different place on each machine,
so the path does not carry over to a Docker image or a CI runner: run
`dxui init` there before Tailwind, as a build step, and it rewrites the line
for that machine. A crate inside the app's directory, such as one
`cargo vendor` placed in `vendor/` or a path dependency, gets a path relative
to the stylesheet (`@source "../vendor/dioxus-shadcn/src";`), which holds
wherever the app is checked out.

`dxui` writes only inside the app: it refuses to write through a symbolic
link under `--root`, such as a linked `src/components`, and names it.

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
Checkbox, Collapsible, Combobox, Command, Context Menu, Countdown, Data Table,
Date Picker, Dialog, Diff, Direction, Dock, Drawer, Dropdown, Empty, Fab,
Field, File Input, Hover Card, Indicator, Input, Input Group, Input OTP, Item,
Kbd, Label, Marker, Menubar, Message, Message Scroller, Native Select,
Navigation Menu, Number Input, Pagination, Popover, Progress, Radial Progress,
Radio Group, Rating, Resizable, Scroll Area, Select, Separator, Sheet,
Sidebar, Skeleton, Slider, Sonner, Spinner, Stat, Status, Steps, Swap, Switch,
Table, Tabs, Tags Input, Textarea, Timeline, Toast, Toggle, Toggle Group,
Tooltip, and Typography.

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
