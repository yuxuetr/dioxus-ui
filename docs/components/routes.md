# Docs Route Manifest

This page is generated from the docs catalog builder. Update it by running:

```bash
node scripts/docs-route-manifest-markdown.mjs
```

The manifest defines static route metadata for a future Dioxus docs runtime.
It does not create rendered routes, router code, screenshots, or generated JSON.
For source preview route metadata, see the
[Source Preview Manifest](source-preview.md).

Component routes: 64
Category routes: 8

## Top-level Routes

| Route | Purpose |
| --- | --- |
| `/components` | Component catalog index |
| `/components/{slug}` | Component detail page |
| `/components#category-{category}` | Grouped catalog anchor |
| `/components/{slug}/source` | Future generated source preview |

## Category Routes

| Category | Route |
| --- | --- |
| Actions | /components#category-actions |
| Forms | /components#category-forms |
| Overlays | /components#category-overlays |
| Navigation | /components#category-navigation |
| Layout | /components#category-layout |
| Data Display | /components#category-data-display |
| Feedback | /components#category-feedback |
| Messaging | /components#category-messaging |

## Component Routes

| Component | Route | Markdown | Category Route | Source Route |
| --- | --- | --- | --- | --- |
| [Accordion](accordion.md) | /components/accordion | docs/components/accordion.md | /components#category-layout | /components/accordion/source |
| [Alert](alert.md) | /components/alert | docs/components/alert.md | /components#category-feedback | /components/alert/source |
| [Alert Dialog](alert-dialog.md) | /components/alert-dialog | docs/components/alert-dialog.md | /components#category-overlays | /components/alert-dialog/source |
| [Aspect Ratio](aspect-ratio.md) | /components/aspect-ratio | docs/components/aspect-ratio.md | /components#category-layout | /components/aspect-ratio/source |
| [Attachment](attachment.md) | /components/attachment | docs/components/attachment.md | /components#category-messaging | /components/attachment/source |
| [Avatar](avatar.md) | /components/avatar | docs/components/avatar.md | /components#category-data-display | /components/avatar/source |
| [Badge](badge.md) | /components/badge | docs/components/badge.md | /components#category-data-display | /components/badge/source |
| [Breadcrumb](breadcrumb.md) | /components/breadcrumb | docs/components/breadcrumb.md | /components#category-navigation | /components/breadcrumb/source |
| [Bubble](bubble.md) | /components/bubble | docs/components/bubble.md | /components#category-messaging | /components/bubble/source |
| [Button](button.md) | /components/button | docs/components/button.md | /components#category-actions | /components/button/source |
| [Button Group](button-group.md) | /components/button-group | docs/components/button-group.md | /components#category-actions | /components/button-group/source |
| [Calendar](calendar.md) | /components/calendar | docs/components/calendar.md | /components#category-forms | /components/calendar/source |
| [Card](card.md) | /components/card | docs/components/card.md | /components#category-layout | /components/card/source |
| [Carousel](carousel.md) | /components/carousel | docs/components/carousel.md | /components#category-layout | /components/carousel/source |
| [Chart](chart.md) | /components/chart | docs/components/chart.md | /components#category-data-display | /components/chart/source |
| [Checkbox](checkbox.md) | /components/checkbox | docs/components/checkbox.md | /components#category-forms | /components/checkbox/source |
| [Collapsible](collapsible.md) | /components/collapsible | docs/components/collapsible.md | /components#category-layout | /components/collapsible/source |
| [Combobox](combobox.md) | /components/combobox | docs/components/combobox.md | /components#category-overlays | /components/combobox/source |
| [Command](command.md) | /components/command | docs/components/command.md | /components#category-actions | /components/command/source |
| [Context Menu](context-menu.md) | /components/context-menu | docs/components/context-menu.md | /components#category-overlays | /components/context-menu/source |
| [Data Table](data-table.md) | /components/data-table | docs/components/data-table.md | /components#category-data-display | /components/data-table/source |
| [Date Picker](date-picker.md) | /components/date-picker | docs/components/date-picker.md | /components#category-forms | /components/date-picker/source |
| [Dialog](dialog.md) | /components/dialog | docs/components/dialog.md | /components#category-overlays | /components/dialog/source |
| [Direction](direction.md) | /components/direction | docs/components/direction.md | /components#category-layout | /components/direction/source |
| [Drawer](drawer.md) | /components/drawer | docs/components/drawer.md | /components#category-overlays | /components/drawer/source |
| [Dropdown](dropdown.md) | /components/dropdown | docs/components/dropdown.md | /components#category-overlays | /components/dropdown/source |
| [Empty](empty.md) | /components/empty | docs/components/empty.md | /components#category-data-display | /components/empty/source |
| [Field](field.md) | /components/field | docs/components/field.md | /components#category-forms | /components/field/source |
| [Hover Card](hover-card.md) | /components/hover-card | docs/components/hover-card.md | /components#category-overlays | /components/hover-card/source |
| [Input](input.md) | /components/input | docs/components/input.md | /components#category-forms | /components/input/source |
| [Input Group](input-group.md) | /components/input-group | docs/components/input-group.md | /components#category-forms | /components/input-group/source |
| [Input Otp](input-otp.md) | /components/input-otp | docs/components/input-otp.md | /components#category-forms | /components/input-otp/source |
| [Item](item.md) | /components/item | docs/components/item.md | /components#category-layout | /components/item/source |
| [Kbd](kbd.md) | /components/kbd | docs/components/kbd.md | /components#category-actions | /components/kbd/source |
| [Label](label.md) | /components/label | docs/components/label.md | /components#category-forms | /components/label/source |
| [Marker](marker.md) | /components/marker | docs/components/marker.md | /components#category-messaging | /components/marker/source |
| [Menubar](menubar.md) | /components/menubar | docs/components/menubar.md | /components#category-overlays | /components/menubar/source |
| [Message](message.md) | /components/message | docs/components/message.md | /components#category-messaging | /components/message/source |
| [Message Scroller](message-scroller.md) | /components/message-scroller | docs/components/message-scroller.md | /components#category-messaging | /components/message-scroller/source |
| [Native Select](native-select.md) | /components/native-select | docs/components/native-select.md | /components#category-forms | /components/native-select/source |
| [Navigation Menu](navigation-menu.md) | /components/navigation-menu | docs/components/navigation-menu.md | /components#category-navigation | /components/navigation-menu/source |
| [Pagination](pagination.md) | /components/pagination | docs/components/pagination.md | /components#category-navigation | /components/pagination/source |
| [Popover](popover.md) | /components/popover | docs/components/popover.md | /components#category-overlays | /components/popover/source |
| [Progress](progress.md) | /components/progress | docs/components/progress.md | /components#category-data-display | /components/progress/source |
| [Radio Group](radio-group.md) | /components/radio-group | docs/components/radio-group.md | /components#category-forms | /components/radio-group/source |
| [Resizable](resizable.md) | /components/resizable | docs/components/resizable.md | /components#category-layout | /components/resizable/source |
| [Scroll Area](scroll-area.md) | /components/scroll-area | docs/components/scroll-area.md | /components#category-layout | /components/scroll-area/source |
| [Select](select.md) | /components/select | docs/components/select.md | /components#category-forms | /components/select/source |
| [Separator](separator.md) | /components/separator | docs/components/separator.md | /components#category-layout | /components/separator/source |
| [Sheet](sheet.md) | /components/sheet | docs/components/sheet.md | /components#category-overlays | /components/sheet/source |
| [Sidebar](sidebar.md) | /components/sidebar | docs/components/sidebar.md | /components#category-navigation | /components/sidebar/source |
| [Skeleton](skeleton.md) | /components/skeleton | docs/components/skeleton.md | /components#category-feedback | /components/skeleton/source |
| [Slider](slider.md) | /components/slider | docs/components/slider.md | /components#category-forms | /components/slider/source |
| [Sonner](sonner.md) | /components/sonner | docs/components/sonner.md | /components#category-feedback | /components/sonner/source |
| [Spinner](spinner.md) | /components/spinner | docs/components/spinner.md | /components#category-feedback | /components/spinner/source |
| [Switch](switch.md) | /components/switch | docs/components/switch.md | /components#category-forms | /components/switch/source |
| [Table](table.md) | /components/table | docs/components/table.md | /components#category-data-display | /components/table/source |
| [Tabs](tabs.md) | /components/tabs | docs/components/tabs.md | /components#category-navigation | /components/tabs/source |
| [Textarea](textarea.md) | /components/textarea | docs/components/textarea.md | /components#category-forms | /components/textarea/source |
| [Toast](toast.md) | /components/toast | docs/components/toast.md | /components#category-feedback | /components/toast/source |
| [Toggle](toggle.md) | /components/toggle | docs/components/toggle.md | /components#category-actions | /components/toggle/source |
| [Toggle Group](toggle-group.md) | /components/toggle-group | docs/components/toggle-group.md | /components#category-actions | /components/toggle-group/source |
| [Tooltip](tooltip.md) | /components/tooltip | docs/components/tooltip.md | /components#category-overlays | /components/tooltip/source |
| [Typography](typography.md) | /components/typography | docs/components/typography.md | /components#category-data-display | /components/typography/source |
