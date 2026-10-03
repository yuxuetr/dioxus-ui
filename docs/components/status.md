# Component Status

This page is generated from the docs catalog builder. Update it by running:

```bash
node scripts/docs-component-status.mjs
```

It reflects the local implementation surface: registry entries, source-copy
templates, crate features, styled crate modules, and component docs pages.
It does not refresh live upstream shadcn/ui parity and does not claim visual
parity.

## Summary

- Public components: 64
- Registry entries: 65
- Source-copy helpers: utils
- Templates: 65
- Crate modules: 64
- Crate features: 64
- Component docs pages: 64
- Complete local wiring: 64
- Incomplete local wiring: 0

## Category Counts

| Category | Components |
| --- | ---: |
| Actions | 6 |
| Forms | 14 |
| Overlays | 11 |
| Navigation | 5 |
| Layout | 10 |
| Data Display | 8 |
| Feedback | 5 |
| Messaging | 5 |

## Coverage Matrix

| Component | Category | Docs | Template | Target | Feature | Module | Source Preview | Complete |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Button | Actions | yes | yes | yes | yes | yes | yes | yes |
| Button Group | Actions | yes | yes | yes | yes | yes | yes | yes |
| Command | Actions | yes | yes | yes | yes | yes | yes | yes |
| Kbd | Actions | yes | yes | yes | yes | yes | yes | yes |
| Toggle | Actions | yes | yes | yes | yes | yes | yes | yes |
| Toggle Group | Actions | yes | yes | yes | yes | yes | yes | yes |
| Avatar | Data Display | yes | yes | yes | yes | yes | yes | yes |
| Badge | Data Display | yes | yes | yes | yes | yes | yes | yes |
| Chart | Data Display | yes | yes | yes | yes | yes | yes | yes |
| Data Table | Data Display | yes | yes | yes | yes | yes | yes | yes |
| Empty | Data Display | yes | yes | yes | yes | yes | yes | yes |
| Progress | Data Display | yes | yes | yes | yes | yes | yes | yes |
| Table | Data Display | yes | yes | yes | yes | yes | yes | yes |
| Typography | Data Display | yes | yes | yes | yes | yes | yes | yes |
| Alert | Feedback | yes | yes | yes | yes | yes | yes | yes |
| Skeleton | Feedback | yes | yes | yes | yes | yes | yes | yes |
| Sonner | Feedback | yes | yes | yes | yes | yes | yes | yes |
| Spinner | Feedback | yes | yes | yes | yes | yes | yes | yes |
| Toast | Feedback | yes | yes | yes | yes | yes | yes | yes |
| Calendar | Forms | yes | yes | yes | yes | yes | yes | yes |
| Checkbox | Forms | yes | yes | yes | yes | yes | yes | yes |
| Date Picker | Forms | yes | yes | yes | yes | yes | yes | yes |
| Field | Forms | yes | yes | yes | yes | yes | yes | yes |
| Input | Forms | yes | yes | yes | yes | yes | yes | yes |
| Input Group | Forms | yes | yes | yes | yes | yes | yes | yes |
| Input Otp | Forms | yes | yes | yes | yes | yes | yes | yes |
| Label | Forms | yes | yes | yes | yes | yes | yes | yes |
| Native Select | Forms | yes | yes | yes | yes | yes | yes | yes |
| Radio Group | Forms | yes | yes | yes | yes | yes | yes | yes |
| Select | Forms | yes | yes | yes | yes | yes | yes | yes |
| Slider | Forms | yes | yes | yes | yes | yes | yes | yes |
| Switch | Forms | yes | yes | yes | yes | yes | yes | yes |
| Textarea | Forms | yes | yes | yes | yes | yes | yes | yes |
| Accordion | Layout | yes | yes | yes | yes | yes | yes | yes |
| Aspect Ratio | Layout | yes | yes | yes | yes | yes | yes | yes |
| Card | Layout | yes | yes | yes | yes | yes | yes | yes |
| Carousel | Layout | yes | yes | yes | yes | yes | yes | yes |
| Collapsible | Layout | yes | yes | yes | yes | yes | yes | yes |
| Direction | Layout | yes | yes | yes | yes | yes | yes | yes |
| Item | Layout | yes | yes | yes | yes | yes | yes | yes |
| Resizable | Layout | yes | yes | yes | yes | yes | yes | yes |
| Scroll Area | Layout | yes | yes | yes | yes | yes | yes | yes |
| Separator | Layout | yes | yes | yes | yes | yes | yes | yes |
| Attachment | Messaging | yes | yes | yes | yes | yes | yes | yes |
| Bubble | Messaging | yes | yes | yes | yes | yes | yes | yes |
| Marker | Messaging | yes | yes | yes | yes | yes | yes | yes |
| Message | Messaging | yes | yes | yes | yes | yes | yes | yes |
| Message Scroller | Messaging | yes | yes | yes | yes | yes | yes | yes |
| Breadcrumb | Navigation | yes | yes | yes | yes | yes | yes | yes |
| Navigation Menu | Navigation | yes | yes | yes | yes | yes | yes | yes |
| Pagination | Navigation | yes | yes | yes | yes | yes | yes | yes |
| Sidebar | Navigation | yes | yes | yes | yes | yes | yes | yes |
| Tabs | Navigation | yes | yes | yes | yes | yes | yes | yes |
| Alert Dialog | Overlays | yes | yes | yes | yes | yes | yes | yes |
| Combobox | Overlays | yes | yes | yes | yes | yes | yes | yes |
| Context Menu | Overlays | yes | yes | yes | yes | yes | yes | yes |
| Dialog | Overlays | yes | yes | yes | yes | yes | yes | yes |
| Drawer | Overlays | yes | yes | yes | yes | yes | yes | yes |
| Dropdown | Overlays | yes | yes | yes | yes | yes | yes | yes |
| Hover Card | Overlays | yes | yes | yes | yes | yes | yes | yes |
| Menubar | Overlays | yes | yes | yes | yes | yes | yes | yes |
| Popover | Overlays | yes | yes | yes | yes | yes | yes | yes |
| Sheet | Overlays | yes | yes | yes | yes | yes | yes | yes |
| Tooltip | Overlays | yes | yes | yes | yes | yes | yes | yes |

## Public Component Details

| Component | Category | Docs | CLI | Feature | Template | Target |
| --- | --- | --- | --- | --- | --- | --- |
| Button | Actions | [docs](button.md) | `dxui add button` | `button` | `crates/dioxus-ui-cli/templates/button.rs` | `src/components/ui/button.rs` |
| Button Group | Actions | [docs](button-group.md) | `dxui add button-group` | `button-group` | `crates/dioxus-ui-cli/templates/button_group.rs` | `src/components/ui/button_group.rs` |
| Command | Actions | [docs](command.md) | `dxui add command` | `command` | `crates/dioxus-ui-cli/templates/command.rs` | `src/components/ui/command.rs` |
| Kbd | Actions | [docs](kbd.md) | `dxui add kbd` | `kbd` | `crates/dioxus-ui-cli/templates/kbd.rs` | `src/components/ui/kbd.rs` |
| Toggle | Actions | [docs](toggle.md) | `dxui add toggle` | `toggle` | `crates/dioxus-ui-cli/templates/toggle.rs` | `src/components/ui/toggle.rs` |
| Toggle Group | Actions | [docs](toggle-group.md) | `dxui add toggle-group` | `toggle-group` | `crates/dioxus-ui-cli/templates/toggle_group.rs` | `src/components/ui/toggle_group.rs` |
| Avatar | Data Display | [docs](avatar.md) | `dxui add avatar` | `avatar` | `crates/dioxus-ui-cli/templates/avatar.rs` | `src/components/ui/avatar.rs` |
| Badge | Data Display | [docs](badge.md) | `dxui add badge` | `badge` | `crates/dioxus-ui-cli/templates/badge.rs` | `src/components/ui/badge.rs` |
| Chart | Data Display | [docs](chart.md) | `dxui add chart` | `chart` | `crates/dioxus-ui-cli/templates/chart.rs` | `src/components/ui/chart.rs` |
| Data Table | Data Display | [docs](data-table.md) | `dxui add data-table` | `data-table` | `crates/dioxus-ui-cli/templates/data_table.rs` | `src/components/ui/data_table.rs` |
| Empty | Data Display | [docs](empty.md) | `dxui add empty` | `empty` | `crates/dioxus-ui-cli/templates/empty.rs` | `src/components/ui/empty.rs` |
| Progress | Data Display | [docs](progress.md) | `dxui add progress` | `progress` | `crates/dioxus-ui-cli/templates/progress.rs` | `src/components/ui/progress.rs` |
| Table | Data Display | [docs](table.md) | `dxui add table` | `table` | `crates/dioxus-ui-cli/templates/table.rs` | `src/components/ui/table.rs` |
| Typography | Data Display | [docs](typography.md) | `dxui add typography` | `typography` | `crates/dioxus-ui-cli/templates/typography.rs` | `src/components/ui/typography.rs` |
| Alert | Feedback | [docs](alert.md) | `dxui add alert` | `alert` | `crates/dioxus-ui-cli/templates/alert.rs` | `src/components/ui/alert.rs` |
| Skeleton | Feedback | [docs](skeleton.md) | `dxui add skeleton` | `skeleton` | `crates/dioxus-ui-cli/templates/skeleton.rs` | `src/components/ui/skeleton.rs` |
| Sonner | Feedback | [docs](sonner.md) | `dxui add sonner` | `sonner` | `crates/dioxus-ui-cli/templates/sonner.rs` | `src/components/ui/sonner.rs` |
| Spinner | Feedback | [docs](spinner.md) | `dxui add spinner` | `spinner` | `crates/dioxus-ui-cli/templates/spinner.rs` | `src/components/ui/spinner.rs` |
| Toast | Feedback | [docs](toast.md) | `dxui add toast` | `toast` | `crates/dioxus-ui-cli/templates/toast.rs` | `src/components/ui/toast.rs` |
| Calendar | Forms | [docs](calendar.md) | `dxui add calendar` | `calendar` | `crates/dioxus-ui-cli/templates/calendar.rs` | `src/components/ui/calendar.rs` |
| Checkbox | Forms | [docs](checkbox.md) | `dxui add checkbox` | `checkbox` | `crates/dioxus-ui-cli/templates/checkbox.rs` | `src/components/ui/checkbox.rs` |
| Date Picker | Forms | [docs](date-picker.md) | `dxui add date-picker` | `date-picker` | `crates/dioxus-ui-cli/templates/date_picker.rs` | `src/components/ui/date_picker.rs` |
| Field | Forms | [docs](field.md) | `dxui add field` | `field` | `crates/dioxus-ui-cli/templates/field.rs` | `src/components/ui/field.rs` |
| Input | Forms | [docs](input.md) | `dxui add input` | `input` | `crates/dioxus-ui-cli/templates/input.rs` | `src/components/ui/input.rs` |
| Input Group | Forms | [docs](input-group.md) | `dxui add input-group` | `input-group` | `crates/dioxus-ui-cli/templates/input_group.rs` | `src/components/ui/input_group.rs` |
| Input Otp | Forms | [docs](input-otp.md) | `dxui add input-otp` | `input-otp` | `crates/dioxus-ui-cli/templates/input_otp.rs` | `src/components/ui/input_otp.rs` |
| Label | Forms | [docs](label.md) | `dxui add label` | `label` | `crates/dioxus-ui-cli/templates/label.rs` | `src/components/ui/label.rs` |
| Native Select | Forms | [docs](native-select.md) | `dxui add native-select` | `native-select` | `crates/dioxus-ui-cli/templates/native_select.rs` | `src/components/ui/native_select.rs` |
| Radio Group | Forms | [docs](radio-group.md) | `dxui add radio-group` | `radio-group` | `crates/dioxus-ui-cli/templates/radio_group.rs` | `src/components/ui/radio_group.rs` |
| Select | Forms | [docs](select.md) | `dxui add select` | `select` | `crates/dioxus-ui-cli/templates/select.rs` | `src/components/ui/select.rs` |
| Slider | Forms | [docs](slider.md) | `dxui add slider` | `slider` | `crates/dioxus-ui-cli/templates/slider.rs` | `src/components/ui/slider.rs` |
| Switch | Forms | [docs](switch.md) | `dxui add switch` | `switch` | `crates/dioxus-ui-cli/templates/switch.rs` | `src/components/ui/switch.rs` |
| Textarea | Forms | [docs](textarea.md) | `dxui add textarea` | `textarea` | `crates/dioxus-ui-cli/templates/textarea.rs` | `src/components/ui/textarea.rs` |
| Accordion | Layout | [docs](accordion.md) | `dxui add accordion` | `accordion` | `crates/dioxus-ui-cli/templates/accordion.rs` | `src/components/ui/accordion.rs` |
| Aspect Ratio | Layout | [docs](aspect-ratio.md) | `dxui add aspect-ratio` | `aspect-ratio` | `crates/dioxus-ui-cli/templates/aspect_ratio.rs` | `src/components/ui/aspect_ratio.rs` |
| Card | Layout | [docs](card.md) | `dxui add card` | `card` | `crates/dioxus-ui-cli/templates/card.rs` | `src/components/ui/card.rs` |
| Carousel | Layout | [docs](carousel.md) | `dxui add carousel` | `carousel` | `crates/dioxus-ui-cli/templates/carousel.rs` | `src/components/ui/carousel.rs` |
| Collapsible | Layout | [docs](collapsible.md) | `dxui add collapsible` | `collapsible` | `crates/dioxus-ui-cli/templates/collapsible.rs` | `src/components/ui/collapsible.rs` |
| Direction | Layout | [docs](direction.md) | `dxui add direction` | `direction` | `crates/dioxus-ui-cli/templates/direction.rs` | `src/components/ui/direction.rs` |
| Item | Layout | [docs](item.md) | `dxui add item` | `item` | `crates/dioxus-ui-cli/templates/item.rs` | `src/components/ui/item.rs` |
| Resizable | Layout | [docs](resizable.md) | `dxui add resizable` | `resizable` | `crates/dioxus-ui-cli/templates/resizable.rs` | `src/components/ui/resizable.rs` |
| Scroll Area | Layout | [docs](scroll-area.md) | `dxui add scroll-area` | `scroll-area` | `crates/dioxus-ui-cli/templates/scroll_area.rs` | `src/components/ui/scroll_area.rs` |
| Separator | Layout | [docs](separator.md) | `dxui add separator` | `separator` | `crates/dioxus-ui-cli/templates/separator.rs` | `src/components/ui/separator.rs` |
| Attachment | Messaging | [docs](attachment.md) | `dxui add attachment` | `attachment` | `crates/dioxus-ui-cli/templates/attachment.rs` | `src/components/ui/attachment.rs` |
| Bubble | Messaging | [docs](bubble.md) | `dxui add bubble` | `bubble` | `crates/dioxus-ui-cli/templates/bubble.rs` | `src/components/ui/bubble.rs` |
| Marker | Messaging | [docs](marker.md) | `dxui add marker` | `marker` | `crates/dioxus-ui-cli/templates/marker.rs` | `src/components/ui/marker.rs` |
| Message | Messaging | [docs](message.md) | `dxui add message` | `message` | `crates/dioxus-ui-cli/templates/message.rs` | `src/components/ui/message.rs` |
| Message Scroller | Messaging | [docs](message-scroller.md) | `dxui add message-scroller` | `message-scroller` | `crates/dioxus-ui-cli/templates/message_scroller.rs` | `src/components/ui/message_scroller.rs` |
| Breadcrumb | Navigation | [docs](breadcrumb.md) | `dxui add breadcrumb` | `breadcrumb` | `crates/dioxus-ui-cli/templates/breadcrumb.rs` | `src/components/ui/breadcrumb.rs` |
| Navigation Menu | Navigation | [docs](navigation-menu.md) | `dxui add navigation-menu` | `navigation-menu` | `crates/dioxus-ui-cli/templates/navigation_menu.rs` | `src/components/ui/navigation_menu.rs` |
| Pagination | Navigation | [docs](pagination.md) | `dxui add pagination` | `pagination` | `crates/dioxus-ui-cli/templates/pagination.rs` | `src/components/ui/pagination.rs` |
| Sidebar | Navigation | [docs](sidebar.md) | `dxui add sidebar` | `sidebar` | `crates/dioxus-ui-cli/templates/sidebar.rs` | `src/components/ui/sidebar.rs` |
| Tabs | Navigation | [docs](tabs.md) | `dxui add tabs` | `tabs` | `crates/dioxus-ui-cli/templates/tabs.rs` | `src/components/ui/tabs.rs` |
| Alert Dialog | Overlays | [docs](alert-dialog.md) | `dxui add alert-dialog` | `alert-dialog` | `crates/dioxus-ui-cli/templates/alert_dialog.rs` | `src/components/ui/alert_dialog.rs` |
| Combobox | Overlays | [docs](combobox.md) | `dxui add combobox` | `combobox` | `crates/dioxus-ui-cli/templates/combobox.rs` | `src/components/ui/combobox.rs` |
| Context Menu | Overlays | [docs](context-menu.md) | `dxui add context-menu` | `context-menu` | `crates/dioxus-ui-cli/templates/context_menu.rs` | `src/components/ui/context_menu.rs` |
| Dialog | Overlays | [docs](dialog.md) | `dxui add dialog` | `dialog` | `crates/dioxus-ui-cli/templates/dialog.rs` | `src/components/ui/dialog.rs` |
| Drawer | Overlays | [docs](drawer.md) | `dxui add drawer` | `drawer` | `crates/dioxus-ui-cli/templates/drawer.rs` | `src/components/ui/drawer.rs` |
| Dropdown | Overlays | [docs](dropdown.md) | `dxui add dropdown` | `dropdown` | `crates/dioxus-ui-cli/templates/dropdown.rs` | `src/components/ui/dropdown.rs` |
| Hover Card | Overlays | [docs](hover-card.md) | `dxui add hover-card` | `hover-card` | `crates/dioxus-ui-cli/templates/hover_card.rs` | `src/components/ui/hover_card.rs` |
| Menubar | Overlays | [docs](menubar.md) | `dxui add menubar` | `menubar` | `crates/dioxus-ui-cli/templates/menubar.rs` | `src/components/ui/menubar.rs` |
| Popover | Overlays | [docs](popover.md) | `dxui add popover` | `popover` | `crates/dioxus-ui-cli/templates/popover.rs` | `src/components/ui/popover.rs` |
| Sheet | Overlays | [docs](sheet.md) | `dxui add sheet` | `sheet` | `crates/dioxus-ui-cli/templates/sheet.rs` | `src/components/ui/sheet.rs` |
| Tooltip | Overlays | [docs](tooltip.md) | `dxui add tooltip` | `tooltip` | `crates/dioxus-ui-cli/templates/tooltip.rs` | `src/components/ui/tooltip.rs` |
