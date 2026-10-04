# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project uses [Conventional Commits](https://www.conventionalcommits.org/)
for commit messages.

## [Unreleased]

The Unreleased section carries the first publish (`0.1.0`) release notes. The
release owner renames it to the released version at publish time.

### Added

- `dioxus-ui-core`: shared conventions, including the `classes` composer,
  `UiDensity`, and registry metadata types used by the CLI.
- `dioxus-ui-primitives`: unstyled state and accessibility helpers for
  dialogs, dismissal, overlay placement, roving focus, typeahead, select,
  slider, calendar, data table, Input OTP, Message Scroller, and chart
  measurement.
- `dioxus-ui`: Tailwind-styled Dioxus components behind per-component crate
  features, covering 64 components: Accordion, Alert, Alert Dialog, Aspect
  Ratio, Attachment, Avatar, Badge, Breadcrumb, Bubble, Button, Button Group,
  Calendar, Card, Carousel, Chart, Checkbox, Collapsible, Combobox, Command,
  Context Menu, Data Table, Date Picker, Dialog, Direction, Drawer, Dropdown,
  Empty, Field, Hover Card, Input, Input Group, Input OTP, Item, Kbd, Label,
  Marker, Menubar, Message, Message Scroller, Native Select, Navigation Menu,
  Pagination, Popover, Progress, Radio Group, Resizable, Scroll Area, Select,
  Separator, Sheet, Sidebar, Skeleton, Slider, Sonner, Spinner, Switch, Table,
  Tabs, Textarea, Toast, Toggle, Toggle Group, Tooltip, and Typography.
- `dioxus-ui-cli`: the `dxui` command with `init`, `add`, and `list`, using
  registry and template assets embedded at compile time for source-copy
  installs.
- Component registry entries, source-copy templates, and docs pages for every
  public component.
- Overlay interaction behavior: Dialog, Alert Dialog, Sheet, and Drawer close
  on Escape and (when configured) overlay click through `on_open_change`, focus
  their first focusable element on open, wrap Tab, and restore focus on close.
  Popover, Dropdown, Hover Card, and Tooltip accept `anchor_id` for fixed
  anchored placement with flip and shift, and close on Escape or outside
  interaction per `dismiss`. Source-copy templates carry the same behavior in
  `utils.rs`.

### Changed

- Template changelog history has been removed from the project changelog.

### Fixed

- Stale template repository links are no longer part of the changelog.

### Excluded From First Publish

- Chart external backends (Plotters, Charming), cursor exploration, hit
  testing, animation runtime, dense-data rendering, and chart families beyond
  line, bar, and area.
- Attachment upload transport, drag-drop, and object URL lifecycle.
- Message markdown parsing, syntax highlighting, citation resolution,
  streaming, and provider integration.
- DOM/WebView scroll commands for Message Scroller.
- Verified Mobile runtime behavior; Desktop and Mobile coverage remains
  checklist or fixture based.
- Overlay behavior for Select, Combobox, Date Picker, Navigation Menu, Context
  Menu, and Menubar, DOM portal mounting, and scroll lock.

### Known Warnings

- APIs are pre-`1.0`. Patch releases (`0.1.x`) do not break crate-mode APIs;
  breaking changes ship only in a minor bump with a migration note in this
  file. Source-copy templates are owned by the consuming app after generation.
- `block` `0.1.6` reports a Rust future-incompatibility warning during
  `cargo check --workspace --all-features` and
  `cargo test --workspace --all-features`.
- crates.io name availability and ownership have not been confirmed; the
  crates are not published until that evidence exists.

## Notes

The changelog metadata gate validates ownership and structure only. It does not
generate release notes, derive changes from Git history, create tags, or publish
releases. First publish release notes above were written from the component
catalog and maintainer decisions, not derived from Git history.
