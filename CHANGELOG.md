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
- Toast and Sonner dismissal: roots count down `duration_ms` (default `5000`,
  `0` disables) while the pointer is outside and focus is not inside, then call
  `on_dismiss(Timeout)`; action and close buttons call `on_dismiss` with
  `Action` and `Close`. Viewports are persistent polite `Notifications` live
  regions.
- Select and Combobox listbox behavior: the Select trigger and Combobox input
  open anchored listbox content through `on_open_change` and keep focus while
  `aria-activedescendant` tracks the highlighted option. Arrows skip disabled
  options, Select adds Home, End, Space, and typeahead, and choosing an option
  calls `on_value_change` before requesting close. `ComboboxInput` gains
  `oninput`, `placeholder`, and an `open` prop for `aria-expanded`.
- Calendar and Date Picker keyboard behavior: keyboard-managed Calendar days
  use roving tabindex, report arrow, Page Up, Page Down, Home, and End moves
  through `on_key_move`, follow the focused date with DOM focus, and report
  clicks through `on_select`. Date Picker content anchors to its trigger,
  enters focus on the focused day, wraps Tab, closes on Escape or outside
  interaction, and returns focus to the trigger.
- Dropdown and Context Menu keyboard behavior: opening focuses the first
  item, arrows wrap, Home, End, and typeahead jump, and Enter, Space, or click
  runs the item's new `onclick`, closes the menu, and returns focus. Context
  Menu content opens at `anchor_point`, usually the pointer position, with
  flip and shift.
- Menubar keyboard behavior: the triggers form one Tab stop with Left, Right,
  Home, and End movement; ArrowDown, Enter, Space, or click opens a menu that
  behaves like Dropdown; Left and Right inside an open menu, or hovering
  another trigger, switch menus through the new `Menubar` `on_value_change`;
  closing returns focus to the open menu's trigger. Menu items gain `onclick`.
- Navigation Menu interaction: a click, Enter, or Space toggles content, the
  mouse resting on a trigger opens it after a delay and leaving closes it,
  arrows move between top-level items and content links, and Escape, outside
  presses, focus leaving, or content link clicks close it. Requests reach the
  app through the new `NavigationMenu` `on_value_change`, and
  `NavigationMenuItem` gains `value`.
- Tabs, Radio Group, and Toggle Group keyboard behavior: each group is one
  Tab stop, and arrows, Home, and End move focus past disabled items. Tabs
  and Radio Group select the focused item and report clicks through the new
  `on_value_change`; Toggle Group reports clicks through the new `on_toggle`.
  The new `Tabs` root links triggers and panels with `aria-controls` and
  `aria-labelledby`. Items no longer render `tabindex`; the shared group
  script owns the Tab stop.
- Accordion interaction: the new `Accordion` root reports toggled items
  through `on_toggle`, and Up, Down, Home, and End move focus between enabled
  triggers while every trigger stays in the Tab order. Triggers render inside
  an `h3` with `aria-controls`, and content renders as a region with
  `aria-labelledby`. `accordion_single_open` and `accordion_multiple_open`
  compute the next open values. Breaking: `AccordionItem` now requires
  `value`.
- Tooltip hover and focus opening: the new `Tooltip` root and
  `TooltipTrigger` open the tooltip after a hover delay (`delay_ms`, default
  700 ms) or at once on keyboard focus, keep it open while the pointer moves
  onto the content, and close it on pointer leave, blur, and trigger presses.
  Requests reach the app through `Tooltip` `on_open_change`, and the trigger
  has `aria-describedby` while the tooltip is open.
- Hover Card hover and focus opening: the new `HoverCard` root and link
  `HoverCardTrigger` open the card after `open_delay_ms` (default 700 ms) or
  at once on keyboard focus, keep it open while the pointer or focus is on the
  trigger or the card, and close it `close_delay_ms` (default 300 ms) after
  the pointer leaves. Trigger presses keep it open. Tooltip now runs on the
  same shared hover-open script; Tab away while the pointer rests on a
  tooltip trigger leaves closing to pointer leave.
- Command keyboard and filtering: the new `Command` `on_select` reports the
  chosen item. Focus stays in `CommandInput`, which now takes `placeholder`
  and `oninput` and controls the list; the first option starts highlighted,
  Up, Down, Home, and End move the highlight, a query change moves it back to
  the first match, and Enter or a click chooses. `CommandItem` takes an
  optional `value`, and `command_matches` filters labels.
- Right-to-left arrow mirroring: Tabs, Radio Group, Toggle Group, Menubar, and
  Navigation Menu read the computed text direction on each key press, so
  inside `Direction` or any `dir="rtl"` ancestor ArrowLeft moves to the next
  item and ArrowRight to the previous one. Up, Down, Home, and End are
  unchanged.
- Tabs manual activation and vertical orientation: `Tabs` takes `activation`
  (`TabsActivation::Automatic` or `Manual`) and `orientation`
  (`TabsOrientation::Horizontal` or `Vertical`). Manual tabs move focus
  without selecting; vertical tabs move with Up and Down and render
  `aria-orientation="vertical"` and `data-orientation`. When focus leaves the
  list, the selected trigger becomes the Tab stop again.
- Combobox and Command result announcements: `ComboboxStatus` and
  `CommandStatus` render a visually hidden polite status region. Keep it
  mounted and change its text, such as "3 results", to announce filtered
  counts; the wording stays with the app.
- Switch and Checkbox change events: `on_checked_change` receives the
  requested state, both components pass through `id`, `name`, and `aria-*`
  attributes so a `Label` can name them, and Switch renders `data-state`.
- Button, Toggle, Input, and Textarea events: Button gains `onclick`, Toggle
  gains `on_pressed_change` with the requested state, and Input and Textarea
  gain `on_value_change`; all four pass through attributes such as `id`,
  `name`, `type`, and `aria-*`.
- Slider keyboard and pointer input: Arrow, Page Up, Page Down, Home, and
  End keys and pointer presses and drags change the value through
  `on_value_change`, and attributes such as `aria-labelledby` pass through.
  `Label` passes through attributes such as `id`.
- Collapsible and Native Select events: `CollapsibleTrigger` gains
  `on_open_change` with the requested state and `NativeSelect` gains
  `on_value_change`; both, and the other Collapsible parts, pass through
  attributes such as `id`, `name`, and `aria-*`.
- Input OTP value changes: `InputOtpHiddenInput` gains `on_value_change` with
  the code cleaned by the new `input_otp_sanitize`, a page script keeps
  rejected characters out of the native value, and both Input OTP parts pass
  through attributes such as `aria-labelledby`.
- Pagination page changes: `PaginationLink`, `PaginationPrevious`, and
  `PaginationNext` gain `onclick`, render a button when `href` is empty, and
  pass through attributes such as `title` and `target`; a disabled anchor
  drops its `href`.
- Carousel slide changes: `CarouselContent` takes `index` and its items
  translate to show the selected slide, `CarouselPrevious`, `CarouselNext`, and
  `CarouselIndicator` gain `onclick`, the `Carousel` root reports arrow keys
  through `on_key_step` with `CarouselStep`, and every part passes through
  attributes; a passed `aria-label` replaces the English default on the
  controls.
- Resizable handle input: `ResizableHandle` takes `value`, `min`, `max`, and
  `step`, becomes a Tab stop with `aria-valuenow`, and reports arrow, Home,
  and End keys and pointer drags as a percent delta through `on_resize`, which
  `resizable_resize_pair` applies. The group, panels, and handle pass through
  attributes, and source-copy templates gain `ResizablePanelState` and
  `resizable_resize_pair`.
- Sidebar toggle and items: `SidebarTrigger` gains `on_collapsed_change` with
  the requested state, `SidebarItem` renders a link with an `href` or a button
  with an `onclick` that marks the current page with `aria-current`, a
  disabled item cannot take focus or call `onclick`, and the Sidebar parts
  pass through attributes such as `id`, `aria-label`, and `aria-controls`.
- `npm run verify:desktop-interactions`: an in-app self-test that runs eight
  interaction scenarios in the Desktop preview's WebView and exits with the
  result.
- `npm run verify:mobile-interactions`: the same scenarios in an iOS Simulator
  build of the new `examples/mobile-demo` preview, read from the app console.
- `npm run verify:android-interactions`: the same scenarios in an Android
  emulator build of the same preview, requested through a debug system
  property and read from logcat.

### Changed

- Template changelog history has been removed from the project changelog.
- `InputOtpHiddenInput` requires a `length` prop, and its class changes from
  `sr-only` to a transparent overlay over the slots; `InputOtp` gains
  `relative` so the input must be its child.
- Pagination controls without an `href` render `button type="button"`
  instead of `a href=""`.
- Carousel items set the `transform` style and transition it, so slides other
  than the selected one move out of the viewport.
- `ResizableHandle` renders `aria-orientation` for the separator line, so it
  is `vertical` in a horizontal group, where it repeated the group orientation.

### Fixed

- Stale template repository links are no longer part of the changelog.
- A passed `aria-label` on `PaginationPrevious` and `PaginationNext` replaces
  the English default in server-rendered HTML too, where both attributes were
  written and the default won.

### Excluded From First Publish

- Chart external backends (Plotters, Charming), cursor exploration, hit
  testing, animation runtime, dense-data rendering, and chart families beyond
  line, bar, and area.
- Attachment upload transport, drag-drop, and object URL lifecycle.
- Message markdown parsing, syntax highlighting, citation resolution,
  streaming, and provider integration.
- DOM/WebView scroll commands for Message Scroller.
- Physical device automation, touch gesture verification, and Desktop
  automation outside macOS; Mobile touch, viewport, and assistive checks remain
  checklist based.
- Navigation Menu viewport size measurement, submenus, multi-select,
  typed date parsing, DOM portal mounting, and scroll lock.

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
