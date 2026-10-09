# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project uses [Conventional Commits](https://www.conventionalcommits.org/)
for commit messages.

## [Unreleased]

### Added
- Tree component: nested items whose branches open and close, with the
  WAI-ARIA tree keyboard model (arrow keys, Home, End, typeahead, Enter and
  Space to select) and app-owned or tree-owned expansion and selection
  (`dxui add tree`, feature `tree`, [RFC 0082](docs/rfcs/0082-blocks-and-block-driven-components.md)).
- `files` block: a folder tree, breadcrumbs, an upload area that takes
  dropped files, and a file table with a context menu (`dxui add files`).
- `schedule` block: a week of day columns, week navigation, and a new
  event dialog with date and time inputs (`dxui add schedule`).

## [0.6.4] - 2026-10-09

Version 0.6.4 adds five blocks built from the existing components
([RFC 0082](docs/rfcs/0082-blocks-and-block-driven-components.md)). The
crate API does not change; the blocks come with `dxui` 0.6.4.

### Added
- `chat` block: conversations, messages with bubbles and attachments, and
  a composer that takes typed text and dropped files (`dxui add chat`).
- `inbox` block: mail with folders, a searchable message list, and a
  reading pane in resizable panels (`dxui add inbox`).
- `landing` block: a product page with a hero, features, a testimonial,
  an early-access form, and a footer (`dxui add landing`).
- `pricing` block: three plans with a monthly or yearly switch
  (`dxui add pricing`).
- `signup` block: account creation with a password strength meter and
  checked fields (`dxui add signup`).

## [0.6.3] - 2026-10-09

Version 0.6.3 fixes overlay placement, Command, and small buttons, reported
by an app on 0.6.2 ([RFC 0081](docs/rfcs/0081-overlay-and-listbox-fixes.md)).
The API does not change.

### Fixed
- Anchored content (Dropdown, Popover, Select, Combobox, Context Menu, Date
  Picker, and the rest) leaves the page flow before its trigger is measured,
  so a trigger in a flex row with its content beside it no longer gets the
  content placed one content-width away.
- A Command inside a Dialog, Sheet, Popover, tab, or anything else hidden
  with `hidden` reads its keys once shown, and again after a close and a
  reopen; it used to stop on the first frame it was hidden.
- Command highlights the first result when results arrive after the query
  changed, as from a debounced server search.
- `ButtonSize::Sm` is 32 pixels high at the default density again; the
  default density no longer adds a 40 pixel minimum height.

### Documentation
- Navigation Menu contents open after hydration; the page says so and why
  they are not shown by CSS hover before it.

## [0.6.2] - 2026-10-08

Version 0.6.2 hardens the components against values from app data, from an
audit by an app migrating to the crate. The API does not change; copied
components that take link URLs or stack overlays bring two new helpers,
`safe_url.rs` and `layer.rs`.

### Added
- A Security section in the README and Security Rules in
  `docs/component-api.md`: what the components escape and check, and what
  is left to the app.

### Fixed
- Components that take a link's URL (Breadcrumb, Dock, Hover Card, Menu,
  Navigation Menu, Pagination, and Sidebar links) render no `href` for a
  scheme other than `http`, `https`, `mailto`, and `tel`, so a
  `javascript:` URL from app data cannot run when the link is followed.
  Relative URLs and fragments are kept. Copied components bring a
  `safe_url.rs` helper.
- A disabled `NavigationMenuLink` drops its `href`, as disabled Menu,
  Pagination, and Sidebar links already did.
- `ThemeController` and `theme_init_script` apply a stored theme only when
  it is a theme name (letters, digits, `-`, and `_`); any other stored value
  is ignored and the theme stays as the app set it. `theme_init_script`
  writes its storage key as an escaped JavaScript string, so no key can end
  the inline script.
- One Escape closes one layer: Dialog, Sheet, Drawer, and Alert Dialog leave
  Escape to an overlay opened inside them, such as a Popover, Select, or
  another dialog, and close on the next one. Copied components bring a
  `layer.rs` helper.
- The Sidebar shortcut (Ctrl or Command and B) is left to inputs, text areas,
  selects, and editable content, where it means bold in an editor.
- Table and Data Table rule lines use the border token (`border-border`);
  they were drawn in the text color, since Tailwind 4 borders default to
  `currentColor`.
- `dxui` refuses to write through a symbolic link under the app's root,
  such as a linked `src/components`, and writes nothing in that run.
- `dxui init` writes the `@source` line relative to the stylesheet when the
  crate is inside the app's directory (a vendored or path dependency), so it
  holds in Docker and CI; a registry crate keeps the absolute path, and the
  crate README says to run `dxui init` as a build step there.

## [0.6.1] - 2026-10-08

Version 0.6.1 makes the interactive components work under a Content Security
Policy without `'unsafe-eval'`. The API does not change; apps that copy
components add three crates to `Cargo.toml` when they re-copy one with a page
script.

### Fixed
- The interactive components work under a Content Security Policy without
  `'unsafe-eval'` (RFC 0080). Their page scripts ran through
  `document::eval`, which Dioxus Web turns into `new Function`; a page served
  with `script-src 'self' 'wasm-unsafe-eval'` refused it, and the wasm module
  then stopped. They now run as wasm-bindgen snippets on the web, and through
  `document::eval` on Desktop and Mobile as before. `npm run verify:csp` runs
  the interaction checks under that policy in the release gate.

### Changed
- Copied components with a page script bring a `script.rs` helper, which
  needs `serde`, and on `wasm32` `serde_json` and `wasm-bindgen`, in the
  app's `Cargo.toml`; `dxui add` prints the lines the app lacks.
  `dioxus-shadcn` depends on the same crates, which Dioxus Web already
  brings.

### Added
- The four published crates declare `rust-version = "1.88"`, the lowest Rust
  that builds them (Dioxus 0.7's dependencies need it), and CI checks them
  with that toolchain. An older Rust now stops with "requires rustc 1.88"
  instead of a compile error inside a dependency. The release docs state what
  1.x will keep ([Compatibility](docs/release.md#compatibility)).

## [0.6.0] - 2026-10-07

Version 0.6.0 commits to the API apps use (RFC 0079). `dioxus-shadcn` keeps
public the components, their props, the class functions component pages list,
and the helpers the docs use; class constants and internal helpers are
private. The primitives crate keeps its own semver promise and drops what
nothing used. Every public item in the three library crates has a doc
comment, and the release gate now runs `cargo-semver-checks` against the last
release. Components and props do not change, so apps that use components
through their props need no changes.

### Changed
- `dioxus-shadcn` keeps public only what its docs show (RFC 0079): the
  components and their props, the enums props take, the class functions a
  component page lists, the helpers docs text uses, and the primitive types
  a page lists or a public signature needs. Class constants are private,
  except `CHART_COLOR_CLASSES`. Every public item has a doc comment, and the
  crate denies `missing_docs`.
- `dioxus-shadcn-primitives` and `dioxus-shadcn-core` document every public
  item and deny `missing_docs`; the primitives crate keeps its own semver
  promise (RFC 0079).

### Removed
- From `dioxus-shadcn-primitives`, what nothing used: the `dismissal` module
  (`DismissalEvent`, `DismissalDecision`, `DismissBehavior::should_dismiss`
  and `dismissal_decision`), the `typeahead` module, the toast and Sonner
  runtime request helpers (`toast_timer_request`, `sonner_timer_request`,
  `toast_live_region_request`, `sonner_live_region_request`), and
  `slider_snap` and `slider_percent`. `slider_clamp`,
  `chart_domain_normalize`, and `data_table_clamp_page` are private; the
  methods that call them stay.
- Helpers no component used since RFC 0077: `toggle_group_move_value`,
  `toggle_group_item_tabindex`, `toggle_group_focus_state`,
  `slider_percent`, `slider_aria_attributes`,
  `command_active_descendant_state`, and
  `combobox_active_descendant_state`.

### Migration
- A class constant (`BUTTON_BASE_CLASS` and the like): pass `class` to the
  component, call the part's class function where its page lists one, or
  copy the string.
- A class function its page does not list (such as `card_title_class` or
  `select_content_class`): render the part with `class`, or copy the
  component with `dxui add` and use the copy.
- These helpers are private, since the component does their work:
  `aspect_ratio_value`, `calendar_range_attribute`, `carousel_key_step`,
  `chart_area_path`, `chart_bar_rects`, `chart_line_path`,
  `chart_pie_arcs`, `checkbox_requested_state`, `checkbox_state`,
  `countdown_segments`, `diff_position`, `hover_card_align_attribute`,
  `hover_card_side_attribute`, `input_otp_slot_display`, the
  `number_input_*` helpers, `progress_percent`,
  `radial_progress_geometry`, `radial_progress_value`,
  `range_slider_values`, `resizable_handle_key_delta`,
  `resizable_panel_style`, `resizable_separator_orientation`,
  `slider_range_style`, `slider_state`, `slider_thumb_style`,
  `switch_state`, and the `tags_input_*` helpers, with `ChartBarRect` and
  `ChartArc`.
- Helpers no page named are private, since the component sets what they
  compute: `carousel_item_transform`, `carousel_orientation_attribute`,
  `data_table_sort_attribute`, `date_picker_align_attribute`,
  `date_picker_side_attribute`, `message_scroller_intent_attribute`,
  `message_scroller_is_following_intent`, `radio_group_focus_state`,
  `radio_group_orientation_attribute`, `sidebar_side_attribute`,
  `sonner_live_attribute`, `toast_live_attribute`, and
  `toggle_group_orientation_attribute`, with the constants
  `DEFAULT_ASPECT_RATIO` and `CAROUSEL_INDEX_PROPERTY`.
- `message_scroller_class` takes only `class`; drop the intent argument,
  which never changed the classes.
- `dioxus-shadcn` no longer re-exports `carousel_clamp_index`,
  `chart_number_label`, `layout_orientation_attribute`,
  `SliderAriaAttributes`, or Sonner's `sonner_placement_attribute`,
  `sonner_queue_limit`, and `sonner_variant_attribute`; import them from
  `dioxus-shadcn-primitives` (Sonner's under their `toast_` names).
  `chart_domain_normalize` and `data_table_clamp_page` are no longer public;
  call `ChartDomain::normalized` and `DataTablePaginationState::clamped_page`.

## [0.5.0] - 2026-10-07

Version 0.5.0 lets a class passed to a component win (RFC 0076), moves the
state of compound components into their roots (RFC 0077), and takes density
from a `DensityProvider` so every control offers a touch target under
`Touch` (RFC 0078). Most stateful components change: parts lose their
per-part state props, sit inside a root, and stop rendering outside it.
It also carries the crate-mode setup fixes planned for 0.4.3, which was not
published. `cargo-semver-checks` against 0.4.2 finds no breaking change in
`dioxus-shadcn-core` or `dioxus-shadcn-primitives`; every finding in
`dioxus-shadcn` (removed props fields, the removed accordion and toggle
group helpers, `command_item_class`'s parameters, and new props fields) is
covered under Migration.

### Changed
- A class passed to a component now wins as the last class does in
  shadcn/ui: it replaces each component utility whose properties it sets in
  full, under the same variants and importance, so `Button { class: "px-2" }`
  renders `px-2` without the button's `px-4`. Component `hover:` and
  `data-[state=...]:` styles stay, a class that sets only part of a
  component utility (`px-2` over `p-4`) keeps both, and a class the merge
  does not know replaces nothing. `dioxus-shadcn-core` exports
  `merge_classes`, which every class function uses (RFC 0076).
- Select and Tabs own their state (RFC 0077). A new `Select` root holds the
  chosen value, or values with `multiple`, and the open state; `Tabs` holds
  the selected tab. Each takes `default_value` to start the state or `value`
  to control it, and its change callback hears every change either way. The
  parts read the root, link their ids themselves, and render nothing outside
  their root, where Dioxus logs which root they need.
- Dialog, Alert Dialog, Sheet, and Drawer own their open state the same way,
  through new `Dialog`, `AlertDialog`, `Sheet`, and `Drawer` roots with
  `open`, `default_open`, and `on_open_change`. New `DialogTrigger`,
  `AlertDialogTrigger`, `SheetTrigger`, and `DrawerTrigger` parts open them
  and point `aria-controls` at the content; they render an unstyled button
  that takes `class`. `AlertDialogCancel` takes `onclick`, as
  `AlertDialogAction` does.
- Popover gains a `Popover` root and a `PopoverTrigger` that anchors the
  content; Tooltip and Hover Card roots own their open state and take
  `open` and `default_open`; Fab owns its speed dial's open state, is a
  speed dial exactly when it has `FabAction` children, and closes the dial
  when an action is pressed.
- Dropdown and Context Menu gain `Dropdown` and `ContextMenu` roots with a
  `DropdownTrigger` and a `ContextMenuTrigger` area that opens the menu at
  the pointer. `Menubar` owns which menu is open through `value`,
  `default_value`, and `on_value_change`. In every menu, radio groups own
  their value and submenus their open state.
- Combobox gains a `Combobox` root that owns the value, or values with
  `multiple`, and the open state, as `Select` does; typing opens the list.
  Date Picker gains a `DatePicker` root that owns the open state and names
  the trigger. `NavigationMenu` owns which item is open through `value`,
  `default_value`, and `on_value_change`.
- Accordion, Collapsible, and Menu's `MenuGroup` own which items are open.
  `Accordion` takes `value`, `default_value`, and `on_value_change` for the
  one open item, the empty string while all are closed, or with `multiple`,
  `values`, `default_values`, and `on_values_change`. `Collapsible` and
  `MenuGroup` take `open`, `default_open`, and `on_open_change`; the
  Collapsible trigger points `aria-controls` at the content while it shows.
- Radio Group and Toggle Group own their value. `RadioGroup` takes `value`,
  `default_value`, and `on_value_change`; `ToggleGroup` takes the same for
  its one pressed item, the empty string while none is, or with
  `ToggleGroupType::Multiple`, `values`, `default_values`, and
  `on_values_change`.
- Carousel, Command, and Input OTP own their state. `Carousel` takes the
  slide `count` and `index`, `default_index`, and `on_index_change`, and its
  Previous, Next, and indicator parts step and disable themselves.
  `Command` links its input and list and its parts no longer take highlight
  props. `InputOtp` takes the code `length` and `value`, `default_value`,
  and `on_value_change`; its slots show the code by `index`.
- Sidebar gains a `SidebarProvider` root, as in shadcn/ui, that owns the
  collapsed state (`collapsed`, `default_collapsed`, `on_collapsed_change`)
  and, with `off_canvas`, the off-canvas panel (`mobile_open`,
  `on_mobile_open_change`). `Sidebar`, `SidebarRail`, and `SidebarTrigger`
  read it, and the trigger points `aria-controls` at the sidebar.
- Density comes from a new `DensityProvider` root (RFC 0078), read with
  `use_density()`. Under `UiDensity::Touch` every interactive control offers
  a 44 by 44 CSS pixel target: buttons, inputs, triggers, tabs, menu and
  list items, and the like grow to that height, and checkboxes, radio items,
  switches, slider thumbs, carousel indicators, rating stars, and resizable
  handles keep their look with a centered hit area.
  `density_control_class` and `density_hit_area_class` give an app's own
  elements the same classes. The Mobile preview renders under Touch, and its
  self-test fails when a control is under the target; measured at defaults,
  124 of 134 controls were.
### Fixed

- The Message Scroller jump button hides when there is nothing to jump to;
  its `inline-flex` won over `hidden` by stylesheet order. The unread marker
  and the Sidebar rail now set their display only through their state, with
  no visible change. `verify:tailwind-conflicts` skipped single-word
  utilities such as `hidden`.
- The active Menu item takes `text-accent-foreground` as intended; the base
  class's `text-foreground` won by stylesheet order. Alert and Toast drop
  color utilities that never applied for the same reason, which leaves their
  rendering unchanged. `verify:tailwind-conflicts` compiled utilities without
  the token stylesheet, so it could not see conflicts between token colors
  such as `text-foreground` and `text-accent-foreground`.

### Added

- `dxui init` writes the `@source` line crate mode needs, from
  `cargo metadata`, and running it again after a crate upgrade replaces the
  line that names the previous version. Before, the crate README asked for a
  hand-written path, which went stale on every upgrade and left Tailwind
  scanning old source.

### Migration

- An app that passed a class conflicting with a component utility and
  relied on the component's value now gets its own. Overrides written with
  Tailwind's important modifier keep working and no longer need it unless
  they override part of a component utility.
- Select: wrap the parts in `Select`, and move `open`, `on_open_change`, and
  `on_value_change` to it, or drop them to let it hold the state. Remove
  `id` from `SelectTrigger` (name the trigger with `Select { id }` for a
  `Label`), `anchor_id`, `open`, and `multiple` from `SelectContent`, and
  `selected` from `SelectItem`; a multiple Select takes `values` and
  `on_values_change` instead of toggling in `on_value_change`. `SelectValue`
  takes a `placeholder` instead of children; put a label other than the value
  in the `SelectTrigger`.
- Tabs: remove `active` from `TabsTrigger` and `TabsContent`, and pass
  `default_value` or `value` to `Tabs`. Tabs parts outside a `Tabs` no longer
  render.
- Dialog, Alert Dialog, Sheet, Drawer: wrap the parts in the new root and
  move `open` and `on_open_change` to it, or drop them and open it with the
  new trigger part. Remove `open` and `on_open_change` from the overlay,
  content, and close parts (`AlertDialogCancel`, `AlertDialogAction`).
- Popover: wrap the parts in `Popover`, replace the app's trigger button with
  `PopoverTrigger`, and remove `open`, `anchor_id`, and `on_open_change` from
  `PopoverContent`.
- Tooltip, Hover Card: remove `open`, `anchor_id`, and `on_open_change` from
  the content, and the signal that held `open`, unless you control it with
  `open` on the root. Triggers and content outside their root no longer
  render.
- Fab: drop `open` and `on_open_change` unless you control the dial, and the
  `open.set(false)` in each `FabAction` `onclick`. A plain Fab must have no
  children.
- Dropdown: wrap the parts in `Dropdown`, replace the app's trigger button
  with `DropdownTrigger`, and remove `open`, `anchor_id`, and
  `on_open_change` from `DropdownContent`.
- Context Menu: wrap the parts in `ContextMenu`, replace the
  `oncontextmenu` area with `ContextMenuTrigger`, and remove `open`,
  `anchor_point`, and `on_open_change` from `ContextMenuContent`.
- Menubar: remove `id`, `open`, and `on_open_change` from `MenubarTrigger`
  and `open`, `anchor_id`, and `on_open_change` from `MenubarContent`; keep
  the open menu in `Menubar { value, on_value_change }` or let it hold it.
- In Dropdown, Context Menu, and Menubar, give radio items a `value` instead
  of `checked` and move the group's value to `value` and `on_value_change`
  or `default_value`; remove `open` from sub triggers and contents and move
  `on_open_change` (with `open` to control it) to the sub root.
- Combobox: wrap the parts in `Combobox`, move `id` from `ComboboxInput`,
  and `open`, `multiple`, and `on_value_change` from `ComboboxContent`, to
  it (a multiple Combobox takes `values` and `on_values_change` instead of
  toggling); remove `open`, `on_open_change`, and `active_id` from the input,
  `anchor_id` from the content, `active_id` from the list, and `active` and
  `selected` from items, and stop opening the list in `oninput`.
- Date Picker: wrap the parts in `DatePicker`, move the trigger's `id`,
  `open`, and `on_open_change` to it, and remove `open`, `anchor_id`, and
  `on_open_change` from `DatePickerContent`.
- Navigation Menu: remove `open` from triggers, contents, the viewport, and
  the indicator; keep the open item in `NavigationMenu { value,
  on_value_change }` or let it hold it.
- Accordion: remove `open` from triggers and contents and replace
  `on_toggle` with `default_value` or `value` and `on_value_change` on
  `Accordion` (`multiple: true` with the `values` props for several open
  items). `accordion_single_open` and `accordion_multiple_open` are removed.
- Collapsible: move `open` and `on_open_change` from the trigger to
  `Collapsible`, or drop them and let it hold the state; remove `open` from
  the content and `controls` and the content `id`, which the root now links.
- `MenuGroup`: pass `default_open` instead of an `open` the app never
  changes; `open` with `on_open_change` still controls it.
- Radio Group: remove `checked` from items; `RadioGroup { value }` takes a
  plain value or signal to control it, or use `default_value`.
- Toggle Group: remove `pressed` from items and replace `on_toggle` with
  `default_value` or `value` and `on_value_change` (the `values` props for a
  multiple group). `toggle_group_single_selection` and
  `toggle_group_multiple_selection` are removed.
- Carousel: give `Carousel` the slide `count` (and `index` with
  `on_index_change` to control it, or `default_index`), replace `selected`
  with `index` on items and indicators, and remove `on_key_step`, `index`
  and `orientation` from `CarouselContent`, `orientation` from items, and
  `onclick` and the computed `disabled` from the controls and indicators.
- Command: remove `active_id` from `CommandInput` and `CommandList` and
  `active` and `selected` from `CommandItem`; `command_item_class` takes
  only the class. The input and list must be inside `Command`.
- Input OTP: move `value`, `length`, and `on_value_change` from
  `InputOtpHiddenInput` to `InputOtp`, drop `disabled` and `invalid` from
  the input and `value`, `active`, and `invalid` from slots (the root's
  `disabled` and `invalid` reach them), and render one `InputOtpSlot` per
  `index` instead of mapping `otp_slots`.
- Sidebar: wrap the sidebar and its trigger in `SidebarProvider` and move
  `collapsed`, `on_collapsed_change`, `mobile_open`, and
  `on_mobile_open_change` there from `Sidebar`, `SidebarTrigger`, and
  `SidebarRail` (drop them to let it hold the state); replace "pass
  `on_mobile_open_change` for off-canvas" with `off_canvas: true`. The
  trigger's `aria-controls` is no longer needed unless the sidebar has its
  own `id`.
- Button: remove `density` and wrap the app, or the part of it, in
  `DensityProvider { density }`. `button_class` keeps its density argument;
  pass `use_density()` to follow the provider.
- Props structs gain fields for the new root props. Code that builds a
  `*Props` struct by literal, rather than through `rsx!`, adds them.
- Copy mode: re-copy components with `dxui add <name> --overwrite`. `utils`
  now brings two helpers, `class_merge` and `class_merge_table`; the second
  is generated from Tailwind and stores its names reversed so Tailwind
  generates no CSS for them. The components that own state bring
  `root_state`, the overlays also `overlay_root` and `default_attribute`,
  menus `menu_radio`, and Select, Combobox, Accordion, and Toggle Group
  `choice`. Every interactive component brings `density`.

## [0.4.2] - 2026-10-06

### Fixed

- Server-rendered pages keep working after hydration. Generated element ids
  came from process-wide counters, so from the second request on a server
  wrote ids the browser never generated, and the scripts behind keyboard
  navigation, pointer dragging, and overlay placement found no element; Tabs
  ignored arrow keys, for one. Ids are now numbered per virtual DOM (see
  RFC 0075), so each request and the browser hydrating it number them alike.

### Migration

- Copy mode: components that generate ids now import the new `element_id`
  helper. Re-copy them with `dxui add <name> --overwrite` (`dxui diff` lists
  the files that changed); copies made before keep working in apps that
  render only in the browser.

## [0.4.1] - 2026-10-06

A CLI patch: no crate API or template changes.

### Changed

- `dxui diff` without names checks every component and block declared in the
  app's `src/components/ui/mod.rs` and `src/blocks/mod.rs`, with the helpers
  they use, instead of failing with `missing component name`. Modules the app
  wrote itself are skipped. `dxui add` still needs a name.

## [0.4.0] - 2026-10-06

Version 0.4.0 makes copy mode clean: a copied component brings only the
helpers it uses and builds without warnings, and the CLI says what it did
with each file and what changed since an app copied a component. The crates'
APIs are unchanged; `cargo-semver-checks` against 0.3.0 found no breaking
change in `dioxus-shadcn-core`, `dioxus-shadcn-primitives`, or
`dioxus-shadcn`. Apps that copied components with 0.3 should read Migration.

### Added

- `dxui add` takes several names, components and blocks alike, as in
  `dxui add button dialog login`. It checks every name before it writes
  anything, so a misspelled name leaves the app untouched.
- `dxui --version` and `dxui -V` print the CLI version.
- `dxui add` reports each file as written, unchanged, or kept because it
  differs from the template, where it used to print "added" even when it
  kept every file. `dxui diff <name>...` prints a unified diff from the app's
  copies to the templates and exits with status 1 while any copy differs or
  is missing, so CI can check that copies are current.

### Changed

- `dxui add` copies only the helpers a component uses, each in its own file
  named after the crate module it copies, such as `listbox.rs` or
  `modal_focus.rs`, instead of one 1435-line `utils.rs`
  ([RFC 0074](docs/rfcs/0074-helper-templates.md)). `utils.rs` keeps
  `classes` and `UiDensity`. `dxui add button` now copies `button.rs` and a
  27-line `utils.rs`.
- `dxui init` starts `src/components/ui/mod.rs` with
  `#![allow(dead_code, unused_imports)]`, so an app that uses some variants,
  props, and re-exports of a component builds without warnings; a new app
  using Button printed 66 in 0.3.0.

### Fixed

- `dxui add` keeps the lines of `src/components/ui/mod.rs` that are not
  module declarations, such as re-exports, instead of dropping them.

### Migration

- Apps that copied components with dxui 0.3 or earlier keep building: `dxui
  add` keeps their `utils.rs` and writes the new helper files beside it. The
  older components still use the helpers in `utils.rs`, though, and those
  copies number element ids apart from the new files, so an older and a newer
  overlay mounted together can pick the same id. `dxui add` prints a note
  while `utils.rs` holds the old helpers. Re-copy the older components with
  `dxui add <name> --overwrite`, which also replaces `utils.rs`; copy any
  edits you made to them first.
- Apps set up by dxui 0.3 or earlier can add
  `#![allow(dead_code, unused_imports)]` at the top of
  `src/components/ui/mod.rs` to drop the warnings for component API they do
  not use; `dxui init` writes it only for a new `mod.rs`.
- Copied code that imported helpers or overlay types from `utils`, such as
  `components::ui::utils::DismissBehavior`, imports them from their new
  module, such as `components::ui::overlay::DismissBehavior`, once
  `utils.rs` is replaced.

## [0.3.0] - 2026-10-06

Version 0.3.0 moves from components to screens: submenus, scroll lock, an
off-canvas Sidebar, a range slider, a Theme Controller, and blocks that
`dxui add` copies as whole screens. It has 82 components and 3 blocks, fixes
charts that drew upside down, and keeps the source-copy templates in line
with the crate through a parity test. Breaking changes are listed under
Migration.

### Added

- Blocks ([RFC 0073](docs/rfcs/0073-blocks.md)): `dxui add <block>` copies a
  whole screen to `src/blocks/` with the components it uses, and
  `dxui list blocks` lists them. `dashboard` is an app shell with an
  off-canvas sidebar, metrics, a chart, and an orders table; `login` is a
  sign-in page with checked fields; `settings` is a tabbed settings page
  with save and reset.
- Mockup ([RFC 0072](docs/rfcs/0072-menu-and-mockup.md)), ported from
  daisyUI: `MockupBrowser`, `MockupWindow`, `MockupCode` with
  `MockupCodeLine`, and `MockupPhone` frames whose decorations are hidden
  from assistive technology. The library has 82 components.
- Menu ([RFC 0072](docs/rfcs/0072-menu-and-mockup.md)), ported from
  daisyUI: `Menu`, `MenuTitle`, `MenuItem`, and `MenuGroup`, a vertical list
  of links or buttons with `aria-current` and controlled collapsible groups.
- Theme Controller ([RFC 0071](docs/rfcs/0071-theme-controller.md)), the
  component: `ThemeController` applies `Theme::System`, `Light`,
  `Dark`, or a preset to the document root, follows the system scheme by
  default, remembers the choice in `localStorage`, and reports a stored
  theme on mount; `theme_init_script` applies it before first paint. The
  component site uses it and keeps the theme across visits.
- `DropdownCheckboxItem`, `DropdownRadioGroup`, `DropdownRadioItem`, and
  `DropdownShortcut`, as Context Menu and Menubar have, and `inset` on
  `DropdownItem` to line plain items up with them.
- Submenus for Dropdown, Context Menu, and Menubar
  ([RFC 0067](docs/rfcs/0067-menu-submenus.md)): `*Sub`, `*SubTrigger`, and
  `*SubContent`, opened by ArrowRight, Enter, Space, click, or hover and
  closed one level at a time by ArrowLeft or Escape, mirrored in
  right-to-left.
- Scroll lock for Dialog, Alert Dialog, Sheet, and Drawer
  ([RFC 0068](docs/rfcs/0068-modal-scroll-lock.md)): the page stops
  scrolling while one is open, with the scrollbar's width kept as padding,
  and nested modals share the lock.
- Off-canvas Sidebar and shortcut
  ([RFC 0069](docs/rfcs/0069-off-canvas-sidebar.md)): with
  `on_mobile_open_change`, the Sidebar is a modal panel below 768px that
  `SidebarTrigger` opens, and `shortcut` toggles it with Ctrl or Command;
  `on_collapsed_change` on `Sidebar` serves the shortcut on wide viewports.
- `RangeSlider` and `range_slider_values`
  ([RFC 0070](docs/rfcs/0070-range-slider.md)): two thumbs for a low and a
  high value, kept `min_steps_between` steps apart, each a focusable slider
  bounded by the other.
- `pagination_range` and `PaginationRangeItem`: the pages and ellipses to
  render for a current page, with a constant length for long ranges. The
  site's Pagination example uses it; it showed pages 1 to 3 and 10 only.

### Changed

- `FieldDescription` and `FieldError` pass other attributes, such as `id`,
  to their element, so an input's `aria-describedby` can point at them; the
  login block needed it.
- The component site's header theme menu offers System, Light, Dark, and the
  presets, replacing the separate dark toggle, and applies them to the
  document root.
- The component site has a Blocks section: each block renders in a
  full-width preview frame, with its source, add command, and docs.
- The component site renders each component's reference (API, behavior,
  and accessibility notes) from its docs page at build time, with links to
  other components kept on the site, instead of linking to GitHub.
- The component site's code blocks and example sources have copy buttons,
  and its version snippets follow the crate version; the Installation page
  showed a `dioxus-shadcn-0.1.0` path.
- The menu and list script reads only each menu's own items, ignores keys
  from a nested menu, and stops when an ancestor is hidden, for Select,
  Combobox, and Command as well as menus.

### Fixed

- Charts drew upside down: `ChartScale::new` and `chart_scale_value`
  normalized the output range, so a y range from 280 to 32, as the examples
  and docs use for an SVG, lost its direction and larger values went lower.
  The range now keeps its direction. Apps that flipped values themselves to
  work around it must stop. The dashboard block's chart showed it.
- Checked checkbox and radio items in Context Menu and Menubar now show a
  check mark or a dot; they were inset for one but drew nothing, so the
  checked state was only announced, not seen.

- Source-copy templates had drifted from the crate
  ([RFC 0066](docs/rfcs/0066-template-parity.md)); a test now compares them
  item by item. Templates generated by `dxui add` now match crate mode:
  - Select and Combobox show the 0.2.0 check mark on selected options
    instead of the accent background.
  - Calendar's `CalendarDate::new` validates the date and returns `Option`,
    and `CalendarDate::unchecked` exists, as the Calendar docs use it.
  - Variant, size, side, and orientation enums derive `Default`, such as
    `BadgeVariant::default()`.
  - The inlined Resizable, Slider, Toast, Message Scroller, and overlay
    primitives match the crate, including `ResizablePanelState::collapsed`
    and the Slider page step.
  - Sonner's types are the toast types under `pub use` aliases, as in the
    crate.

### Migration

- `ChartScale` keeps the direction of its output range. Charts that passed a
  range from bottom to top, as the docs show, now draw the right way up;
  code that flipped values itself to undo the inversion must stop.
- Apps that copied the Calendar template and call `CalendarDate::new` get an
  `Option` after re-adding it; use `CalendarDate::unchecked` for the old
  behavior. Templates already in an app are not changed.
- A Sidebar given `on_mobile_open_change` renders inside a wrapper element
  (a flex item on wide viewports, the modal panel on phones); selectors
  that expect the `aside` as a direct child of the layout need updating.
- `SidebarProps`, `SidebarTriggerProps`, `DropdownItemProps`,
  `FieldDescriptionProps`, and `FieldErrorProps` have new fields. Code that
  builds them through `rsx!` or their builders is unaffected; a struct
  literal naming every field must add the new ones.

## [0.2.0] - 2026-10-05

Version 0.2.0 adds theme presets, 15 components ported from daisyUI, and the
0.1.0 exclusions apps hit most: multi-select, Navigation Menu submenus, typed
dates, and pie charts. It has 79 components. Breaking changes are listed
under Migration.

### Added

- Theme presets ([RFC 0057](docs/rfcs/0057-theme-presets.md)): 33 daisyUI
  themes mapped to the dioxus-shadcn tokens and scoped by
  `[data-theme="<name>"]`, with foregrounds adjusted to WCAG AA.
  `dxui theme list` prints them and `dxui theme add <name>...` appends them to
  `assets/dioxus-shadcn.css`.
- `Success`, `Warning`, and `Info` variants for Alert and Badge, and the
  `--success-foreground`, `--warning-foreground`, and `--info-foreground`
  tokens ([RFC 0058](docs/rfcs/0058-status-variants.md)).
- Display components ported from daisyUI
  ([RFC 0059](docs/rfcs/0059-display-components.md)): Stat, Timeline, Steps,
  Indicator, Status, Radial Progress, Countdown, and Diff.
- Input components ([RFC 0060](docs/rfcs/0060-input-components.md)): Rating, Number
  Input, Tags Input, File Input, and Swap.
- Mobile navigation ([RFC 0061](docs/rfcs/0061-mobile-navigation.md)): Dock and
  Fab.
- `multiple` on `SelectContent` and `ComboboxContent`
  ([RFC 0062](docs/rfcs/0062-multi-select.md)): a choice keeps the listbox
  open, which is `aria-multiselectable`.
- Navigation Menu submenus
  ([RFC 0063](docs/rfcs/0063-navigation-menu-submenus.md)): a nested
  `NavigationMenu` with `NavigationMenuOrientation::Vertical` shows its panels
  beside its triggers, and each menu's script acts only on its own items.
  `NavigationMenuContent` takes `value` for that layout.
- `DatePickerInput`, `DateOrder`, `parse_date`, and `format_date`
  ([RFC 0064](docs/rfcs/0064-typed-date-input.md)): typed dates in ISO or the
  app's day, month, and year order. The `date-picker` feature now enables
  `calendar`, and `DatePickerTrigger` passes other attributes to its button.
- Pie and donut charts ([RFC 0065](docs/rfcs/0065-pie-and-donut-charts.md)):
  `ChartPieSeries`, `ChartSlice`, `chart_pie_arcs`, and the `Chart1` to
  `Chart5` color tokens for the `--chart-*` palette.

### Changed

- `ButtonVariant::Link` uses `text-foreground decoration-primary` instead of
  `text-primary`, so link buttons stay readable in presets whose primary color
  is light. The default theme looks the same.
- Selected Select and Combobox options show a check mark instead of the
  accent background, which now marks only the highlighted option.
- Checkbox draws its tick and dash as a masked `::before` filled with
  `--primary-foreground` instead of white and dark data URI backgrounds, so
  the marks follow any theme. Checkbox no longer uses the `dark:` variant.

- `ALERT_BASE_CLASS` no longer sets `bg-card`; the `Default` and
  `Destructive` variants do, so status alerts can tint the surface.

### Migration

- `AlertVariant`, `BadgeVariant`, and `ChartColorToken` have new variants;
  exhaustive `match` expressions need an arm or a wildcard.
- A stylesheet from before 0.2.0 lacks the status foreground tokens. Add
  `--success-foreground`, `--warning-foreground`, and `--info-foreground` to
  `:root` and `.dark`, and their `--color-*` lines to `@theme inline`, with
  the values `dxui init` now writes; without them status badges draw text in
  the inherited color.

### Fixed

- Chart series colored `Success`, `Warning`, or the new palette tokens drew
  in the inherited color in crate mode, since Tailwind never saw their
  classes, which the primitives crate returns. `CHART_COLOR_CLASSES` lists
  them in the scanned crate.
- The light theme's `--destructive` token has lightness 0.532 instead of
  0.577, so focused destructive menu items in Dropdown, Context Menu, and
  Menubar reach 4.5:1 on their `bg-destructive/10` highlight (3.99:1 before).
  Apps that ran `dxui init` before 0.2.0 can change the value in
  `assets/dioxus-shadcn.css`.

## [0.1.0] - 2026-10-05

These are the first publish (`0.1.0`) release notes.

### Added

- `dioxus-shadcn-core`: shared conventions, including the `classes` composer,
  `UiDensity`, and registry metadata types used by the CLI.
- `dioxus-shadcn-primitives`: unstyled state and accessibility helpers for
  dialogs, dismissal, overlay placement, roving focus, typeahead, select,
  slider, calendar, data table, Input OTP, Message Scroller, and chart
  measurement.
- `dioxus-shadcn`: Tailwind-styled Dioxus components behind per-component crate
  features, covering 64 components: Accordion, Alert, Alert Dialog, Aspect
  Ratio, Attachment, Avatar, Badge, Breadcrumb, Bubble, Button, Button Group,
  Calendar, Card, Carousel, Chart, Checkbox, Collapsible, Combobox, Command,
  Context Menu, Data Table, Date Picker, Dialog, Direction, Drawer, Dropdown,
  Empty, Field, Hover Card, Input, Input Group, Input OTP, Item, Kbd, Label,
  Marker, Menubar, Message, Message Scroller, Native Select, Navigation Menu,
  Pagination, Popover, Progress, Radio Group, Resizable, Scroll Area, Select,
  Separator, Sheet, Sidebar, Skeleton, Slider, Sonner, Spinner, Switch, Table,
  Tabs, Textarea, Toast, Toggle, Toggle Group, Tooltip, and Typography.
- `dioxus-shadcn-cli`: the `dxui` command with `init`, `add`, and `list`, using
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
- Form control naming: `RadioGroup`, `RadioGroupItem`, `Progress`,
  `SelectTrigger`, and `ComboboxInput` pass through attributes, so a `Label`
  with `for` or `aria-label` names each radio, a progress bar takes a name and
  `aria-valuetext`, and the Select trigger and Combobox input take
  `aria-describedby`.
- Dialog names: Dialog, Alert Dialog, Sheet, Drawer, and Popover content point
  `aria-labelledby` and `aria-describedby` at their mounted title and
  description through generated ids, take a passed `aria-label` instead, and
  pass through attributes. The browser verifier also fails on any id reference
  that matches no element.
- Composite widget names: `TabsList`, `ToggleGroup`, `Menubar`,
  `NavigationMenu`, `CalendarGrid`, and `CalendarCaption` pass through
  attributes, so the widgets take an `aria-label` and a calendar grid can
  point `aria-labelledby` at its caption.
- Checkbox indeterminate state: `Checkbox` gains `indeterminate`, which sets
  the native mixed state and `data-state="indeterminate"`; a change while
  mixed requests `true`, and the component restores the mixed state when the
  app keeps it after a click.
- Vertical sliders: `Slider` gains `orientation` with `SliderOrientation`; a
  vertical slider fills from the bottom, maps the pointer along its height,
  and renders `aria-orientation="vertical"`.
- `npm run verify:desktop-interactions`: an in-app self-test that runs ten
  scenarios in the Desktop preview's WebView and exits with the
  result.
- `npm run verify:mobile-interactions`: the same scenarios in an iOS Simulator
  build of the new `examples/mobile-demo` preview, read from the app console.
- `npm run verify:android-interactions`: the same scenarios in an Android
  emulator build of the same preview, requested through a debug system
  property and read from logcat.
- An opt-in dark theme: `dxui init` writes a `.dark` block into
  `assets/dioxus-shadcn.css` that redefines the semantic color tokens. Adding the
  `dark` class to an ancestor turns it on without changing component
  classes; app palette classes keep their colors under `.dark`.
- `npm run css:preview` regenerates
  `examples/preview-states/assets/preview.generated.css`, compiled Tailwind
  that `PreviewSurface` links for the Web, Desktop, and Mobile previews, and
  `npm run verify:preview-css`, part of the release gate, fails when it is
  stale. Manual previews were unstyled: they linked the uncompiled Tailwind
  input, or no stylesheet on Mobile. The Desktop, iOS, and Android self-tests
  start with a `stylesheet` scenario.
- The Web, Desktop, and Mobile previews have a "Dark theme" toggle in their
  header that adds the `dark` class to the preview root. The browser check
  and a `theme` scenario in the Desktop, iOS, and Android self-tests press
  it; the self-tests now report ten scenarios.
- shadcn/ui semantic color tokens: `dxui init` writes `--background`,
  `--primary`, `--muted-foreground`, and the rest of the shadcn/ui v4 token
  set in `:root` and `.dark`, plus `--destructive-foreground`, `--success`,
  `--warning`, and `--info`, and maps them to Tailwind colors with
  `@theme inline`, so `bg-primary` and similar classes work in app code. The
  `dark` class also drives app `dark:` utilities through `@custom-variant`.
- A component site in `site/` (`dioxus-ui-site`, not published): a Dioxus Web
  app with the catalog sidebar, a page per component with its install
  commands, live examples for all 64 components with a Preview and a Code
  tab, links to the API and accessibility notes, installation and theming
  guides, and a dark theme toggle. Run it
  with `dx serve --package dioxus-ui-site`. `npm run verify:site-css` and
  `npm run verify:site-catalog` keep its compiled stylesheet and catalog data
  fresh in the release gate, and `npm run verify:site` checks every route in
  a browser.
- `npm run verify:tailwind-static` fails when a component or template class
  uses a Tailwind palette color instead of a semantic token, except the
  `bg-black/50` modal overlay.
- Action part callbacks: `ButtonGroupItem`, `InputGroupAction`,
  `AttachmentAction`, `AttachmentTrigger`, `ComboboxTrigger`,
  `MessageScrollerJumpButton`, and `TooltipTrigger` gain `onclick` and pass
  through attributes; before, nothing could run when they were clicked.
  `FieldLabel` gains `r#for`, and `FieldLabel`, `BreadcrumbLink`, and
  `HoverCardTrigger` pass through attributes such as `id`, `title`, and
  `target`.
- An axe-core audit (WCAG 2.1 A and AA and best practices) in
  `npm run verify:site`, on every route in both themes, and in
  `npm run verify:runtime-interactions`, where it checks contrast and while
  each overlay, menu, and popup fixture is open. `axe-core` is a new dev
  dependency.

### Changed

- Template changelog history has been removed from the project changelog.
- The crates publish as `dioxus-shadcn`, `dioxus-shadcn-core`,
  `dioxus-shadcn-primitives`, and `dioxus-shadcn-cli`, since crates.io's
  `dioxus_ui` takes the `dioxus-ui` name; `dxui init` writes
  `assets/dioxus-shadcn.css`. The repository and the `dxui` binary keep their
  names (see RFC 0056).
- `AlertTitle` renders a `div` instead of an `h5`, as in shadcn/ui v4; an
  `h5` skipped heading levels on most pages.
- `CardTitle` renders a `div` instead of an `h3`, as in shadcn/ui v4, so an app
  picks the heading level.
- `ScrollAreaViewport` and `MessageScrollerViewport` are Tab stops with a
  focus ring and pass through attributes, so keyboard users can scroll them.
- The source-copy `utils` helper `default_aria_label` is now
  `default_attribute(attributes, name, value)`, generic over the value.
- `HoverCardContent` renders no `role`, as in the Radix Hover Card; it was an
  unnamed `role="dialog"`.
- Breaking: component classes use the shadcn/ui semantic color tokens
  (`bg-primary`, `text-muted-foreground`, `border-input`, `ring-ring`, and so
  on) instead of fixed Tailwind palette colors, in the crate and the copied
  templates ([RFC 0051](docs/rfcs/0051-semantic-color-tokens.md)). Checked
  states, the primary Button, and focus rings follow `--primary` and `--ring`,
  so they turn from blue to near-black, as in shadcn/ui; Toggle and Toggle
  Group pressed states use `accent`. Toast and Sonner states show in the
  border and the Sonner dot on an opaque popover surface.
  - Migration for crate-mode apps: the stylesheet must define the tokens, or
    the components lose their colors. Run `dxui init` to write
    `assets/dioxus-shadcn.css`, or copy everything after the import from
    `examples/web-demo/assets/preview.css` into your Tailwind input. To keep
    a blue brand, set `--primary` and `--ring` in `:root` and `.dark`.
  - Migration for source-copy apps: copied components keep their palette
    classes until you add them again with `dxui add --overwrite`, which also
    needs the token stylesheet.
- `accordion_trigger_class` and `collapsible_trigger_class` take only the
  user class, and `COLLAPSIBLE_TRIGGER_OPEN_CLASS` and
  `COLLAPSIBLE_TRIGGER_CLOSED_CLASS` are removed: open and closed triggers
  share the foreground color, which is now in the base class.
- The generated stylesheet drops the unused `--dxui-background` and
  `--dxui-foreground` variables; `bg-background` and `text-foreground` now
  read the `--background` and `--foreground` tokens.
- `InputOtpHiddenInput` requires a `length` prop, and its class changes from
  `sr-only` to a transparent overlay over the slots; `InputOtp` gains
  `relative` so the input must be its child.
- Pagination controls without an `href` render `button type="button"`
  instead of `a href=""`.
- Carousel items set the `transform` style and transition it, so slides other
  than the selected one move out of the viewport.
- `ResizableHandle` renders `aria-orientation` for the separator line, so it
  is `vertical` in a horizontal group, where it repeated the group orientation.
- `slider_range_style` and `slider_thumb_style` take a `SliderOrientation`.
- `npm run verify:runtime-interactions` compiles the preview stylesheet with
  the Tailwind Node API and runs every check with compiled Tailwind; the
  repository gains the `tailwindcss`, `@tailwindcss/node`, and
  `@tailwindcss/oxide` dev dependencies.
- Base class constants no longer hold utilities that a state replaces, such
  as `border-zinc-200` in `INPUT_BASE_CLASS` or `w-full` in
  `SLIDER_ROOT_BASE_CLASS`; the class functions add them for the default
  state. Input OTP slots show the invalid border instead of the active one
  when both apply.
- `npm run verify:tailwind-conflicts` joins the release gate, and
  `npm run verify:runtime-interactions` fails on conflicting utilities in
  any rendered class list.

### Fixed

- Stale template repository links are no longer part of the changelog.
- `Label` and `FieldLabel` leave out `for` when none is passed. `Label`
  wrote `for=""`, which points at no control, so a label wrapping its input
  did not name or focus it.
- `MessageScrollerJumpButton` renders `type="button"`; inside a form it
  submitted the form.
- `ButtonGroup` and `ToggleGroup` no longer put `aria-orientation` on
  `role="group"`, which does not support it. `ToggleGroup` reports its
  orientation as `data-orientation`.
- The site and the previews set `lang="en"` on the document.
- Each crate package ships the MIT `LICENSE`; Cargo packages only files inside
  a crate, so the packages had none.
- The Select and Combobox listboxes take their trigger or input's name and
  an id derived from `anchor_id` (`{anchor_id}-content`, `{anchor_id}-list`),
  which the trigger and input point `aria-controls` at; an expanded combobox
  without `aria-controls` is invalid ARIA.
- Disabled `SelectItem` and `DropdownItem` set `aria-disabled`; assistive
  technology did not hear that they were disabled.
- `DatePickerContent` takes its trigger's name through `aria-labelledby`; the
  dialog had no name.
- The Slider thumb follows the value; it was never positioned, so it sat at
  the end of the root for every value.
- A passed `aria-label` on `PaginationPrevious` and `PaginationNext` replaces
  the English default in server-rendered HTML too, where both attributes were
  written and the default won.
- Data-attribute variants match values, such as `data-[disabled=true]:`.
  The bare `data-disabled:` form matched `data-disabled="false"`, so compiled
  Tailwind disabled every enabled option, item, and link, and forms such as
  `data-orientation-vertical:` never matched, so orientation, state, and side
  styles never applied.
- Navigation Menu content opens below its trigger instead of covering it.
- Class functions no longer join a base utility with a state utility for the
  same property. Compiled Tailwind ordered them by name, so invalid fields
  and checked Radio Group items kept their gray border, Toast, Sonner,
  Alert, and Attachment variants kept the default colors, the collapsed
  Sidebar width relied on a data variant, and the vertical Slider stayed full
  width. A vertical Slider root is now `w-5`.
- Checkbox follows its classes: it drops the native appearance, which
  ignored the border and background classes, and draws a tick when checked
  and a dash on the blue fill when mixed, keyed off `data-state` so
  server-rendered HTML shows them.
- Open Select and Combobox lists are at least as wide as their trigger or
  input; they sized to their options, so a full-width trigger opened a narrow
  list. The anchoring script sets `--dxui-anchor-width` on anchored content.
- Pagination content wraps onto centered lines when its links do not fit;
  at phone width the centered row overflowed both edges of its container and
  put Previous out of reach. The shared preview page no longer scrolls
  sideways at 375px in the Web and Mobile previews.

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
