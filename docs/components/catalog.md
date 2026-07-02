# Component Catalog

This page is generated from the docs catalog builder. Update it by running:

```bash
node scripts/docs-catalog-markdown.mjs
```

The catalog is derived from registry entries, templates, component docs,
crate features, and crate modules. It intentionally does not include visual
preview routes, screenshot artifacts, or generated JSON metadata.

Public components: 64

## Groups

### Actions

- [Button](button.md): Button component with variants, sizes, and density-aware spacing.
- [Button Group](button-group.md): Button Group component for grouped command buttons.
- [Command](command.md): Controlled command palette parts with active descendant semantics.
- [Kbd](kbd.md): Styled keyboard shortcut hint.
- [Toggle](toggle.md): Toggle component for controlled pressed button states.
- [Toggle Group](toggle-group.md): Toggle Group component for grouped single or multiple pressed states.

### Forms

- [Calendar](calendar.md): Calendar components backed by pure date grid primitives.
- [Checkbox](checkbox.md): Checkbox component with checked and disabled states.
- [Date Picker](date-picker.md): Date Picker trigger, value, and popover content parts for composing Calendar.
- [Field](field.md): Form field layout composition parts.
- [Input](input.md): Input component with disabled and invalid states.
- [Input Group](input-group.md): Input Group component for addons, controls, and actions around inputs.
- [Input Otp](input-otp.md): Input OTP component with controlled visual slots and native input support.
- [Label](label.md): Label component for associating text with form controls.
- [Native Select](native-select.md): Styled native select, optgroup, and option components.
- [Radio Group](radio-group.md): Radio Group component for single-choice grouped selection.
- [Select](select.md): Select components backed by primitive configuration types.
- [Slider](slider.md): Slider component for controlled horizontal numeric values.
- [Switch](switch.md): Switch component with checked and disabled states.
- [Textarea](textarea.md): Textarea component with disabled and invalid states.

### Overlays

- [Alert Dialog](alert-dialog.md): Alert dialog confirmation components backed by dialog primitive configuration.
- [Combobox](combobox.md): Controlled searchable selection parts backed by popover primitive configuration.
- [Context Menu](context-menu.md): Controlled context menu parts backed by dropdown primitive configuration.
- [Dialog](dialog.md): Dialog overlay components backed by primitive configuration types.
- [Drawer](drawer.md): Mobile-oriented bottom drawer components backed by dialog primitive configuration.
- [Dropdown](dropdown.md): Dropdown menu components backed by primitive configuration types.
- [Hover Card](hover-card.md): Controlled rich preview content backed by popover primitive configuration.
- [Menubar](menubar.md): Controlled menubar parts backed by dropdown primitive configuration.
- [Popover](popover.md): Popover content components backed by primitive configuration types.
- [Sheet](sheet.md): Side sheet overlay components backed by dialog primitive configuration.
- [Tooltip](tooltip.md): Tooltip content component backed by primitive configuration types.

### Navigation

- [Breadcrumb](breadcrumb.md): Semantic breadcrumb navigation composition parts.
- [Navigation Menu](navigation-menu.md): Controlled navigation menu parts with navigation semantics.
- [Pagination](pagination.md): Pagination component with link, item, and ellipsis parts.
- [Sidebar](sidebar.md): Controlled sidebar shell and navigation composition parts.
- [Tabs](tabs.md): Tabs components with controlled active state.

### Layout

- [Accordion](accordion.md): Accordion components with controlled open state.
- [Aspect Ratio](aspect-ratio.md): Fixed-ratio media and content slot.
- [Card](card.md): Card component with header, content, and footer parts.
- [Carousel](carousel.md): Controlled carousel composition parts and index helpers.
- [Collapsible](collapsible.md): Collapsible component for controlled disclosure content.
- [Direction](direction.md): Direction component for scoped native ltr/rtl text direction.
- [Item](item.md): Generic list item composition parts.
- [Resizable](resizable.md): Controlled resizable panel group, panel, and handle parts.
- [Scroll Area](scroll-area.md): Native scroll area wrapper with styled viewport and scrollbar parts.
- [Separator](separator.md): Separator component for visual or semantic content division.

### Data Display

- [Avatar](avatar.md): Avatar component with image and fallback parts.
- [Badge](badge.md): Badge component with static status variants.
- [Chart](chart.md): Source-copy friendly SVG chart composition parts.
- [Data Table](data-table.md): Controlled Data Table composition parts and state helpers.
- [Empty](empty.md): Empty-state layout composition parts.
- [Progress](progress.md): Progress component with accessible value semantics.
- [Table](table.md): Table component with styled table parts.
- [Typography](typography.md): Styled semantic typography parts.

### Feedback

- [Alert](alert.md): Alert component with title and description parts.
- [Skeleton](skeleton.md): Skeleton component for loading placeholders.
- [Sonner](sonner.md): Opinionated toast notification parts and queue helpers.
- [Spinner](spinner.md): Spinner component for loading status feedback.
- [Toast](toast.md): Controlled toast notification parts and queue helpers.

### Messaging

- [Attachment](attachment.md): Attachment component for provider-neutral file preview rows and actions.
- [Bubble](bubble.md): Bubble component for provider-neutral message surfaces and reactions.
- [Marker](marker.md): Marker component for inline status, bordered rows, and labeled separators.
- [Message](message.md): Message component for provider-neutral chat row layout.
- [Message Scroller](message-scroller.md): Controlled message scroller composition parts with pure scroll intent helpers.

## Full Index

| Component | Description | CLI | Feature | Template | Source Target |
| --- | --- | --- | --- | --- | --- |
| [Accordion](accordion.md) | Accordion components with controlled open state. | `dxui add accordion` | `accordion` | `templates/accordion.rs` | `src/components/ui/accordion.rs` |
| [Alert](alert.md) | Alert component with title and description parts. | `dxui add alert` | `alert` | `templates/alert.rs` | `src/components/ui/alert.rs` |
| [Alert Dialog](alert-dialog.md) | Alert dialog confirmation components backed by dialog primitive configuration. | `dxui add alert-dialog` | `alert-dialog` | `templates/alert_dialog.rs` | `src/components/ui/alert_dialog.rs` |
| [Aspect Ratio](aspect-ratio.md) | Fixed-ratio media and content slot. | `dxui add aspect-ratio` | `aspect-ratio` | `templates/aspect_ratio.rs` | `src/components/ui/aspect_ratio.rs` |
| [Attachment](attachment.md) | Attachment component for provider-neutral file preview rows and actions. | `dxui add attachment` | `attachment` | `templates/attachment.rs` | `src/components/ui/attachment.rs` |
| [Avatar](avatar.md) | Avatar component with image and fallback parts. | `dxui add avatar` | `avatar` | `templates/avatar.rs` | `src/components/ui/avatar.rs` |
| [Badge](badge.md) | Badge component with static status variants. | `dxui add badge` | `badge` | `templates/badge.rs` | `src/components/ui/badge.rs` |
| [Breadcrumb](breadcrumb.md) | Semantic breadcrumb navigation composition parts. | `dxui add breadcrumb` | `breadcrumb` | `templates/breadcrumb.rs` | `src/components/ui/breadcrumb.rs` |
| [Bubble](bubble.md) | Bubble component for provider-neutral message surfaces and reactions. | `dxui add bubble` | `bubble` | `templates/bubble.rs` | `src/components/ui/bubble.rs` |
| [Button](button.md) | Button component with variants, sizes, and density-aware spacing. | `dxui add button` | `button` | `templates/button.rs` | `src/components/ui/button.rs` |
| [Button Group](button-group.md) | Button Group component for grouped command buttons. | `dxui add button-group` | `button-group` | `templates/button_group.rs` | `src/components/ui/button_group.rs` |
| [Calendar](calendar.md) | Calendar components backed by pure date grid primitives. | `dxui add calendar` | `calendar` | `templates/calendar.rs` | `src/components/ui/calendar.rs` |
| [Card](card.md) | Card component with header, content, and footer parts. | `dxui add card` | `card` | `templates/card.rs` | `src/components/ui/card.rs` |
| [Carousel](carousel.md) | Controlled carousel composition parts and index helpers. | `dxui add carousel` | `carousel` | `templates/carousel.rs` | `src/components/ui/carousel.rs` |
| [Chart](chart.md) | Source-copy friendly SVG chart composition parts. | `dxui add chart` | `chart` | `templates/chart.rs` | `src/components/ui/chart.rs` |
| [Checkbox](checkbox.md) | Checkbox component with checked and disabled states. | `dxui add checkbox` | `checkbox` | `templates/checkbox.rs` | `src/components/ui/checkbox.rs` |
| [Collapsible](collapsible.md) | Collapsible component for controlled disclosure content. | `dxui add collapsible` | `collapsible` | `templates/collapsible.rs` | `src/components/ui/collapsible.rs` |
| [Combobox](combobox.md) | Controlled searchable selection parts backed by popover primitive configuration. | `dxui add combobox` | `combobox` | `templates/combobox.rs` | `src/components/ui/combobox.rs` |
| [Command](command.md) | Controlled command palette parts with active descendant semantics. | `dxui add command` | `command` | `templates/command.rs` | `src/components/ui/command.rs` |
| [Context Menu](context-menu.md) | Controlled context menu parts backed by dropdown primitive configuration. | `dxui add context-menu` | `context-menu` | `templates/context_menu.rs` | `src/components/ui/context_menu.rs` |
| [Data Table](data-table.md) | Controlled Data Table composition parts and state helpers. | `dxui add data-table` | `data-table` | `templates/data_table.rs` | `src/components/ui/data_table.rs` |
| [Date Picker](date-picker.md) | Date Picker trigger, value, and popover content parts for composing Calendar. | `dxui add date-picker` | `date-picker` | `templates/date_picker.rs` | `src/components/ui/date_picker.rs` |
| [Dialog](dialog.md) | Dialog overlay components backed by primitive configuration types. | `dxui add dialog` | `dialog` | `templates/dialog.rs` | `src/components/ui/dialog.rs` |
| [Direction](direction.md) | Direction component for scoped native ltr/rtl text direction. | `dxui add direction` | `direction` | `templates/direction.rs` | `src/components/ui/direction.rs` |
| [Drawer](drawer.md) | Mobile-oriented bottom drawer components backed by dialog primitive configuration. | `dxui add drawer` | `drawer` | `templates/drawer.rs` | `src/components/ui/drawer.rs` |
| [Dropdown](dropdown.md) | Dropdown menu components backed by primitive configuration types. | `dxui add dropdown` | `dropdown` | `templates/dropdown.rs` | `src/components/ui/dropdown.rs` |
| [Empty](empty.md) | Empty-state layout composition parts. | `dxui add empty` | `empty` | `templates/empty.rs` | `src/components/ui/empty.rs` |
| [Field](field.md) | Form field layout composition parts. | `dxui add field` | `field` | `templates/field.rs` | `src/components/ui/field.rs` |
| [Hover Card](hover-card.md) | Controlled rich preview content backed by popover primitive configuration. | `dxui add hover-card` | `hover-card` | `templates/hover_card.rs` | `src/components/ui/hover_card.rs` |
| [Input](input.md) | Input component with disabled and invalid states. | `dxui add input` | `input` | `templates/input.rs` | `src/components/ui/input.rs` |
| [Input Group](input-group.md) | Input Group component for addons, controls, and actions around inputs. | `dxui add input-group` | `input-group` | `templates/input_group.rs` | `src/components/ui/input_group.rs` |
| [Input Otp](input-otp.md) | Input OTP component with controlled visual slots and native input support. | `dxui add input-otp` | `input-otp` | `templates/input_otp.rs` | `src/components/ui/input_otp.rs` |
| [Item](item.md) | Generic list item composition parts. | `dxui add item` | `item` | `templates/item.rs` | `src/components/ui/item.rs` |
| [Kbd](kbd.md) | Styled keyboard shortcut hint. | `dxui add kbd` | `kbd` | `templates/kbd.rs` | `src/components/ui/kbd.rs` |
| [Label](label.md) | Label component for associating text with form controls. | `dxui add label` | `label` | `templates/label.rs` | `src/components/ui/label.rs` |
| [Marker](marker.md) | Marker component for inline status, bordered rows, and labeled separators. | `dxui add marker` | `marker` | `templates/marker.rs` | `src/components/ui/marker.rs` |
| [Menubar](menubar.md) | Controlled menubar parts backed by dropdown primitive configuration. | `dxui add menubar` | `menubar` | `templates/menubar.rs` | `src/components/ui/menubar.rs` |
| [Message](message.md) | Message component for provider-neutral chat row layout. | `dxui add message` | `message` | `templates/message.rs` | `src/components/ui/message.rs` |
| [Message Scroller](message-scroller.md) | Controlled message scroller composition parts with pure scroll intent helpers. | `dxui add message-scroller` | `message-scroller` | `templates/message_scroller.rs` | `src/components/ui/message_scroller.rs` |
| [Native Select](native-select.md) | Styled native select, optgroup, and option components. | `dxui add native-select` | `native-select` | `templates/native_select.rs` | `src/components/ui/native_select.rs` |
| [Navigation Menu](navigation-menu.md) | Controlled navigation menu parts with navigation semantics. | `dxui add navigation-menu` | `navigation-menu` | `templates/navigation_menu.rs` | `src/components/ui/navigation_menu.rs` |
| [Pagination](pagination.md) | Pagination component with link, item, and ellipsis parts. | `dxui add pagination` | `pagination` | `templates/pagination.rs` | `src/components/ui/pagination.rs` |
| [Popover](popover.md) | Popover content components backed by primitive configuration types. | `dxui add popover` | `popover` | `templates/popover.rs` | `src/components/ui/popover.rs` |
| [Progress](progress.md) | Progress component with accessible value semantics. | `dxui add progress` | `progress` | `templates/progress.rs` | `src/components/ui/progress.rs` |
| [Radio Group](radio-group.md) | Radio Group component for single-choice grouped selection. | `dxui add radio-group` | `radio-group` | `templates/radio_group.rs` | `src/components/ui/radio_group.rs` |
| [Resizable](resizable.md) | Controlled resizable panel group, panel, and handle parts. | `dxui add resizable` | `resizable` | `templates/resizable.rs` | `src/components/ui/resizable.rs` |
| [Scroll Area](scroll-area.md) | Native scroll area wrapper with styled viewport and scrollbar parts. | `dxui add scroll-area` | `scroll-area` | `templates/scroll_area.rs` | `src/components/ui/scroll_area.rs` |
| [Select](select.md) | Select components backed by primitive configuration types. | `dxui add select` | `select` | `templates/select.rs` | `src/components/ui/select.rs` |
| [Separator](separator.md) | Separator component for visual or semantic content division. | `dxui add separator` | `separator` | `templates/separator.rs` | `src/components/ui/separator.rs` |
| [Sheet](sheet.md) | Side sheet overlay components backed by dialog primitive configuration. | `dxui add sheet` | `sheet` | `templates/sheet.rs` | `src/components/ui/sheet.rs` |
| [Sidebar](sidebar.md) | Controlled sidebar shell and navigation composition parts. | `dxui add sidebar` | `sidebar` | `templates/sidebar.rs` | `src/components/ui/sidebar.rs` |
| [Skeleton](skeleton.md) | Skeleton component for loading placeholders. | `dxui add skeleton` | `skeleton` | `templates/skeleton.rs` | `src/components/ui/skeleton.rs` |
| [Slider](slider.md) | Slider component for controlled horizontal numeric values. | `dxui add slider` | `slider` | `templates/slider.rs` | `src/components/ui/slider.rs` |
| [Sonner](sonner.md) | Opinionated toast notification parts and queue helpers. | `dxui add sonner` | `sonner` | `templates/sonner.rs` | `src/components/ui/sonner.rs` |
| [Spinner](spinner.md) | Spinner component for loading status feedback. | `dxui add spinner` | `spinner` | `templates/spinner.rs` | `src/components/ui/spinner.rs` |
| [Switch](switch.md) | Switch component with checked and disabled states. | `dxui add switch` | `switch` | `templates/switch.rs` | `src/components/ui/switch.rs` |
| [Table](table.md) | Table component with styled table parts. | `dxui add table` | `table` | `templates/table.rs` | `src/components/ui/table.rs` |
| [Tabs](tabs.md) | Tabs components with controlled active state. | `dxui add tabs` | `tabs` | `templates/tabs.rs` | `src/components/ui/tabs.rs` |
| [Textarea](textarea.md) | Textarea component with disabled and invalid states. | `dxui add textarea` | `textarea` | `templates/textarea.rs` | `src/components/ui/textarea.rs` |
| [Toast](toast.md) | Controlled toast notification parts and queue helpers. | `dxui add toast` | `toast` | `templates/toast.rs` | `src/components/ui/toast.rs` |
| [Toggle](toggle.md) | Toggle component for controlled pressed button states. | `dxui add toggle` | `toggle` | `templates/toggle.rs` | `src/components/ui/toggle.rs` |
| [Toggle Group](toggle-group.md) | Toggle Group component for grouped single or multiple pressed states. | `dxui add toggle-group` | `toggle-group` | `templates/toggle_group.rs` | `src/components/ui/toggle_group.rs` |
| [Tooltip](tooltip.md) | Tooltip content component backed by primitive configuration types. | `dxui add tooltip` | `tooltip` | `templates/tooltip.rs` | `src/components/ui/tooltip.rs` |
| [Typography](typography.md) | Styled semantic typography parts. | `dxui add typography` | `typography` | `templates/typography.rs` | `src/components/ui/typography.rs` |
