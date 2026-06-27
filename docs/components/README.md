# Components

This page is the documentation-site seed for `dioxus-ui` components.

Current preview mode is command-line smoke output from the example crates. A
future docs site should replace this with visual Dioxus Web/Desktop previews.

For shadcn/ui coverage planning, see the [parity matrix](parity.md),
[complex component batches](complex-batches.md), and
[interaction batch 1 specification](interaction-batch-1.md). For the next
overlay milestone, see the [overlay variant API plan](overlay-variants.md).
For the menu milestone, see the [menu system API plan](menu-systems.md).
For the command and choice milestone, see the
[command and choice API plan](command-choice.md).
For the calendar milestone, see the
[calendar and date picker API plan](calendar-date.md).
For the data milestone, see the
[data table and chart strategy](data-visualization.md) and
[chart strategy](chart-strategy.md). For chart follow-through, see the
[chart follow-through API plan](chart-follow-through.md) and
[chart recipes](chart-recipes.md).
For the layout and media milestone, see the
[layout shells and media API plan](layout-media.md).
For the feedback notification milestone, see the
[feedback notifications API plan](feedback-notifications.md).
For the static composition milestone, see the
[static composition API plan](static-composition.md). For the runtime adapter
planning milestone, see the [runtime adapter plan](runtime-adapters.md) and
[focus and portal adapter plan](focus-portal-adapters.md). For feedback runtime
details, see the
[timer and live-region adapter plan](timer-live-region-adapters.md). For
measurement and gesture runtime details, see the
[measurement, pointer, and gesture adapter plan](measurement-pointer-gesture-adapters.md).

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
| [Aspect Ratio](aspect-ratio.md) | `dxui add aspect-ratio` | `aspect-ratio` | Fixed-ratio content slot |
| [Avatar](avatar.md) | `dxui add avatar` | `avatar` | Styled parts |
| [Badge](badge.md) | `dxui add badge` | `badge` | Styled |
| [Breadcrumb](breadcrumb.md) | `dxui add breadcrumb` | `breadcrumb` | Navigation composition parts |
| [Button](button.md) | `dxui add button` | `button` | Styled |
| [Calendar](calendar.md) | `dxui add calendar` | `calendar` | Date grid styled parts |
| [Carousel](carousel.md) | `dxui add carousel` | `carousel` | Controlled slide composition parts |
| [Card](card.md) | `dxui add card` | `card` | Styled parts |
| [Checkbox](checkbox.md) | `dxui add checkbox` | `checkbox` | Controlled styled part |
| [Command](command.md) | `dxui add command` | `command` | Active descendant command parts |
| [Combobox](combobox.md) | `dxui add combobox` | `combobox` | Popover-backed searchable choice parts |
| [Context Menu](context-menu.md) | `dxui add context-menu` | `context-menu` | Dropdown-backed menu parts |
| [Data Table](data-table.md) | `dxui add data-table` | `data-table` | Controlled data table parts |
| [Date Picker](date-picker.md) | `dxui add date-picker` | `date-picker` | Calendar popover composition parts |
| [Dialog](dialog.md) | `dxui add dialog` | `dialog` | Primitive config + styled parts |
| [Drawer](drawer.md) | `dxui add drawer` | `drawer` | Dialog-backed bottom drawer |
| [Dropdown](dropdown.md) | `dxui add dropdown` | `dropdown` | Primitive config + styled parts |
| [Empty](empty.md) | `dxui add empty` | `empty` | Empty-state composition parts |
| [Field](field.md) | `dxui add field` | `field` | Form field composition parts |
| [Hover Card](hover-card.md) | `dxui add hover-card` | `hover-card` | Popover-backed preview content |
| [Input](input.md) | `dxui add input` | `input` | Styled |
| [Item](item.md) | `dxui add item` | `item` | Generic item composition parts |
| [Kbd](kbd.md) | `dxui add kbd` | `kbd` | Styled keyboard hint |
| [Label](label.md) | `dxui add label` | `label` | Styled |
| [Menubar](menubar.md) | `dxui add menubar` | `menubar` | Dropdown-backed menubar parts |
| [Native Select](native-select.md) | `dxui add native-select` | `native-select` | Styled native form select |
| [Navigation Menu](navigation-menu.md) | `dxui add navigation-menu` | `navigation-menu` | Navigation-oriented disclosure parts |
| [Pagination](pagination.md) | `dxui add pagination` | `pagination` | Styled parts |
| [Popover](popover.md) | `dxui add popover` | `popover` | Primitive config + styled parts |
| [Progress](progress.md) | `dxui add progress` | `progress` | Styled |
| [Radio Group](radio-group.md) | `dxui add radio-group` | `radio-group` | Primitive-backed styled parts |
| [Resizable](resizable.md) | `dxui add resizable` | `resizable` | Controlled resizable panel parts |
| [Select](select.md) | `dxui add select` | `select` | Primitive config + styled parts |
| [Scroll Area](scroll-area.md) | `dxui add scroll-area` | `scroll-area` | Native scroll styled parts |
| [Separator](separator.md) | `dxui add separator` | `separator` | Styled |
| [Sheet](sheet.md) | `dxui add sheet` | `sheet` | Dialog-backed side panel |
| [Sidebar](sidebar.md) | `dxui add sidebar` | `sidebar` | Controlled navigation shell parts |
| [Skeleton](skeleton.md) | `dxui add skeleton` | `skeleton` | Styled |
| [Slider](slider.md) | `dxui add slider` | `slider` | Primitive-backed styled part |
| [Sonner](sonner.md) | `dxui add sonner` | `sonner` | Opinionated notification parts |
| [Spinner](spinner.md) | `dxui add spinner` | `spinner` | Styled |
| [Switch](switch.md) | `dxui add switch` | `switch` | Controlled styled part |
| [Table](table.md) | `dxui add table` | `table` | Styled parts |
| [Tabs](tabs.md) | `dxui add tabs` | `tabs` | Controlled styled parts |
| [Textarea](textarea.md) | `dxui add textarea` | `textarea` | Styled |
| [Toggle](toggle.md) | `dxui add toggle` | `toggle` | Controlled styled part |
| [Toggle Group](toggle-group.md) | `dxui add toggle-group` | `toggle-group` | Primitive-backed styled parts |
| [Toast](toast.md) | `dxui add toast` | `toast` | Controlled notification parts |
| [Tooltip](tooltip.md) | `dxui add tooltip` | `tooltip` | Primitive config + styled part |
| [Typography](typography.md) | `dxui add typography` | `typography` | Styled semantic text parts |

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
