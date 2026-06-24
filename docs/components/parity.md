# shadcn/ui Parity Matrix

This matrix tracks how `dioxus-ui` maps to the public shadcn/ui component
catalog. It is a planning aid, not a promise that every React component should
be ported one-for-one.

## Implemented

| Group | Components |
| --- | --- |
| Static display | Alert, Avatar, Badge, Card, Separator, Skeleton |
| Form basics | Button, Checkbox, Input, Label, Slider, Switch, Textarea |
| Light interaction | Radio Group, Spinner, Toggle, Toggle Group |
| Disclosure | Accordion, Alert Dialog, Calendar, Date Picker, Drawer, Hover Card, Sheet, Tabs |
| Overlay primitives | Context Menu, Dialog, Dropdown, Menubar, Navigation Menu, Popover, Tooltip |
| Command and search | Command, Combobox, Native Select |
| Selection | Select |
| Feedback and data | Data Table, Progress, Table, Pagination |
| Layout and scroll | Resizable, Scroll Area, Sidebar |

## Planned Static Or Light Interaction

| Group | Components | Notes |
| --- | --- | --- |
| Static display | Aspect Ratio, Breadcrumb, Empty, Field, Item, Kbd, Typography | Mostly styling and composition. |
| Layout and scroll | - | Scroll Area, Resizable, and Sidebar are implemented; runtime measurement and persistence remain app-owned. |

## Planned Complex Interaction

| Group | Components | Notes |
| --- | --- | --- |
| Command and search | - | Current M14 command search components are implemented. |
| Menus | - | Current M13 menu system set is implemented. |
| Date and calendar | - | Calendar and Date Picker are implemented. |
| Overlays | - | Current M12 overlay variant set is implemented. |
| Feedback | Toast, Sonner | Need queue and live-region behavior. |
| Data | Chart | Data Table is implemented; Chart needs a charting decision. |
| Navigation shell | - | Sidebar is implemented; persistence and keyboard shortcuts remain app-owned. |
| Media | Carousel | Needs interaction, gesture, and accessibility decisions. |

## Deferred Or External

| Component | Reason |
| --- | --- |
| Chart | M16 documents the chart strategy; a component is deferred until backend, data API, and accessibility contracts are explicit. |
| Data Table runtime adapters | Data Table state helpers and composition parts are implemented; filtering, async loading, and virtualization remain app-owned. |
| Calendar / Date Picker runtime adapters | Date math primitives are implemented; parsing, localization, and focus adapters remain deferred. |

## Next Milestone Seeds

M8 should not start by implementing a single large component. It should first
define shared primitive behavior for:

- roving focus
- typeahead
- active descendant state
- escape-key dismissal
- pointer outside dismissal
- overlay positioning and collision handling

After Data Table, the safest remaining component order is:

1. Sidebar, Scroll Area, Resizable, and Carousel
2. Toast and Sonner
3. Chart strategy

See [Complex Component Batches](complex-batches.md) for the expanded milestone
plan.
