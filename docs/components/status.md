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

## Public Components

| Component | Category | Docs | CLI | Feature | Template | Target |
| --- | --- | --- | --- | --- | --- | --- |
| Button | Actions | [docs](button.md) | `dxui add button` | `button` | `templates/button.rs` | `src/components/ui/button.rs` |
| Button Group | Actions | [docs](button-group.md) | `dxui add button-group` | `button-group` | `templates/button_group.rs` | `src/components/ui/button_group.rs` |
| Command | Actions | [docs](command.md) | `dxui add command` | `command` | `templates/command.rs` | `src/components/ui/command.rs` |
| Kbd | Actions | [docs](kbd.md) | `dxui add kbd` | `kbd` | `templates/kbd.rs` | `src/components/ui/kbd.rs` |
| Toggle | Actions | [docs](toggle.md) | `dxui add toggle` | `toggle` | `templates/toggle.rs` | `src/components/ui/toggle.rs` |
| Toggle Group | Actions | [docs](toggle-group.md) | `dxui add toggle-group` | `toggle-group` | `templates/toggle_group.rs` | `src/components/ui/toggle_group.rs` |
| Avatar | Data Display | [docs](avatar.md) | `dxui add avatar` | `avatar` | `templates/avatar.rs` | `src/components/ui/avatar.rs` |
| Badge | Data Display | [docs](badge.md) | `dxui add badge` | `badge` | `templates/badge.rs` | `src/components/ui/badge.rs` |
| Chart | Data Display | [docs](chart.md) | `dxui add chart` | `chart` | `templates/chart.rs` | `src/components/ui/chart.rs` |
| Data Table | Data Display | [docs](data-table.md) | `dxui add data-table` | `data-table` | `templates/data_table.rs` | `src/components/ui/data_table.rs` |
| Empty | Data Display | [docs](empty.md) | `dxui add empty` | `empty` | `templates/empty.rs` | `src/components/ui/empty.rs` |
| Progress | Data Display | [docs](progress.md) | `dxui add progress` | `progress` | `templates/progress.rs` | `src/components/ui/progress.rs` |
| Table | Data Display | [docs](table.md) | `dxui add table` | `table` | `templates/table.rs` | `src/components/ui/table.rs` |
| Typography | Data Display | [docs](typography.md) | `dxui add typography` | `typography` | `templates/typography.rs` | `src/components/ui/typography.rs` |
| Alert | Feedback | [docs](alert.md) | `dxui add alert` | `alert` | `templates/alert.rs` | `src/components/ui/alert.rs` |
| Skeleton | Feedback | [docs](skeleton.md) | `dxui add skeleton` | `skeleton` | `templates/skeleton.rs` | `src/components/ui/skeleton.rs` |
| Sonner | Feedback | [docs](sonner.md) | `dxui add sonner` | `sonner` | `templates/sonner.rs` | `src/components/ui/sonner.rs` |
| Spinner | Feedback | [docs](spinner.md) | `dxui add spinner` | `spinner` | `templates/spinner.rs` | `src/components/ui/spinner.rs` |
| Toast | Feedback | [docs](toast.md) | `dxui add toast` | `toast` | `templates/toast.rs` | `src/components/ui/toast.rs` |
| Calendar | Forms | [docs](calendar.md) | `dxui add calendar` | `calendar` | `templates/calendar.rs` | `src/components/ui/calendar.rs` |
| Checkbox | Forms | [docs](checkbox.md) | `dxui add checkbox` | `checkbox` | `templates/checkbox.rs` | `src/components/ui/checkbox.rs` |
| Date Picker | Forms | [docs](date-picker.md) | `dxui add date-picker` | `date-picker` | `templates/date_picker.rs` | `src/components/ui/date_picker.rs` |
| Field | Forms | [docs](field.md) | `dxui add field` | `field` | `templates/field.rs` | `src/components/ui/field.rs` |
| Input | Forms | [docs](input.md) | `dxui add input` | `input` | `templates/input.rs` | `src/components/ui/input.rs` |
| Input Group | Forms | [docs](input-group.md) | `dxui add input-group` | `input-group` | `templates/input_group.rs` | `src/components/ui/input_group.rs` |
| Input Otp | Forms | [docs](input-otp.md) | `dxui add input-otp` | `input-otp` | `templates/input_otp.rs` | `src/components/ui/input_otp.rs` |
| Label | Forms | [docs](label.md) | `dxui add label` | `label` | `templates/label.rs` | `src/components/ui/label.rs` |
| Native Select | Forms | [docs](native-select.md) | `dxui add native-select` | `native-select` | `templates/native_select.rs` | `src/components/ui/native_select.rs` |
| Radio Group | Forms | [docs](radio-group.md) | `dxui add radio-group` | `radio-group` | `templates/radio_group.rs` | `src/components/ui/radio_group.rs` |
| Select | Forms | [docs](select.md) | `dxui add select` | `select` | `templates/select.rs` | `src/components/ui/select.rs` |
| Slider | Forms | [docs](slider.md) | `dxui add slider` | `slider` | `templates/slider.rs` | `src/components/ui/slider.rs` |
| Switch | Forms | [docs](switch.md) | `dxui add switch` | `switch` | `templates/switch.rs` | `src/components/ui/switch.rs` |
| Textarea | Forms | [docs](textarea.md) | `dxui add textarea` | `textarea` | `templates/textarea.rs` | `src/components/ui/textarea.rs` |
| Accordion | Layout | [docs](accordion.md) | `dxui add accordion` | `accordion` | `templates/accordion.rs` | `src/components/ui/accordion.rs` |
| Aspect Ratio | Layout | [docs](aspect-ratio.md) | `dxui add aspect-ratio` | `aspect-ratio` | `templates/aspect_ratio.rs` | `src/components/ui/aspect_ratio.rs` |
| Card | Layout | [docs](card.md) | `dxui add card` | `card` | `templates/card.rs` | `src/components/ui/card.rs` |
| Carousel | Layout | [docs](carousel.md) | `dxui add carousel` | `carousel` | `templates/carousel.rs` | `src/components/ui/carousel.rs` |
| Collapsible | Layout | [docs](collapsible.md) | `dxui add collapsible` | `collapsible` | `templates/collapsible.rs` | `src/components/ui/collapsible.rs` |
| Direction | Layout | [docs](direction.md) | `dxui add direction` | `direction` | `templates/direction.rs` | `src/components/ui/direction.rs` |
| Item | Layout | [docs](item.md) | `dxui add item` | `item` | `templates/item.rs` | `src/components/ui/item.rs` |
| Resizable | Layout | [docs](resizable.md) | `dxui add resizable` | `resizable` | `templates/resizable.rs` | `src/components/ui/resizable.rs` |
| Scroll Area | Layout | [docs](scroll-area.md) | `dxui add scroll-area` | `scroll-area` | `templates/scroll_area.rs` | `src/components/ui/scroll_area.rs` |
| Separator | Layout | [docs](separator.md) | `dxui add separator` | `separator` | `templates/separator.rs` | `src/components/ui/separator.rs` |
| Attachment | Messaging | [docs](attachment.md) | `dxui add attachment` | `attachment` | `templates/attachment.rs` | `src/components/ui/attachment.rs` |
| Bubble | Messaging | [docs](bubble.md) | `dxui add bubble` | `bubble` | `templates/bubble.rs` | `src/components/ui/bubble.rs` |
| Marker | Messaging | [docs](marker.md) | `dxui add marker` | `marker` | `templates/marker.rs` | `src/components/ui/marker.rs` |
| Message | Messaging | [docs](message.md) | `dxui add message` | `message` | `templates/message.rs` | `src/components/ui/message.rs` |
| Message Scroller | Messaging | [docs](message-scroller.md) | `dxui add message-scroller` | `message-scroller` | `templates/message_scroller.rs` | `src/components/ui/message_scroller.rs` |
| Breadcrumb | Navigation | [docs](breadcrumb.md) | `dxui add breadcrumb` | `breadcrumb` | `templates/breadcrumb.rs` | `src/components/ui/breadcrumb.rs` |
| Navigation Menu | Navigation | [docs](navigation-menu.md) | `dxui add navigation-menu` | `navigation-menu` | `templates/navigation_menu.rs` | `src/components/ui/navigation_menu.rs` |
| Pagination | Navigation | [docs](pagination.md) | `dxui add pagination` | `pagination` | `templates/pagination.rs` | `src/components/ui/pagination.rs` |
| Sidebar | Navigation | [docs](sidebar.md) | `dxui add sidebar` | `sidebar` | `templates/sidebar.rs` | `src/components/ui/sidebar.rs` |
| Tabs | Navigation | [docs](tabs.md) | `dxui add tabs` | `tabs` | `templates/tabs.rs` | `src/components/ui/tabs.rs` |
| Alert Dialog | Overlays | [docs](alert-dialog.md) | `dxui add alert-dialog` | `alert-dialog` | `templates/alert_dialog.rs` | `src/components/ui/alert_dialog.rs` |
| Combobox | Overlays | [docs](combobox.md) | `dxui add combobox` | `combobox` | `templates/combobox.rs` | `src/components/ui/combobox.rs` |
| Context Menu | Overlays | [docs](context-menu.md) | `dxui add context-menu` | `context-menu` | `templates/context_menu.rs` | `src/components/ui/context_menu.rs` |
| Dialog | Overlays | [docs](dialog.md) | `dxui add dialog` | `dialog` | `templates/dialog.rs` | `src/components/ui/dialog.rs` |
| Drawer | Overlays | [docs](drawer.md) | `dxui add drawer` | `drawer` | `templates/drawer.rs` | `src/components/ui/drawer.rs` |
| Dropdown | Overlays | [docs](dropdown.md) | `dxui add dropdown` | `dropdown` | `templates/dropdown.rs` | `src/components/ui/dropdown.rs` |
| Hover Card | Overlays | [docs](hover-card.md) | `dxui add hover-card` | `hover-card` | `templates/hover_card.rs` | `src/components/ui/hover_card.rs` |
| Menubar | Overlays | [docs](menubar.md) | `dxui add menubar` | `menubar` | `templates/menubar.rs` | `src/components/ui/menubar.rs` |
| Popover | Overlays | [docs](popover.md) | `dxui add popover` | `popover` | `templates/popover.rs` | `src/components/ui/popover.rs` |
| Sheet | Overlays | [docs](sheet.md) | `dxui add sheet` | `sheet` | `templates/sheet.rs` | `src/components/ui/sheet.rs` |
| Tooltip | Overlays | [docs](tooltip.md) | `dxui add tooltip` | `tooltip` | `templates/tooltip.rs` | `src/components/ui/tooltip.rs` |
