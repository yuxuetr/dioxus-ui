# shadcn/ui Parity Matrix

This matrix tracks how `dioxus-ui` maps to the public shadcn/ui component
catalog. It is a planning aid, not a promise that every React component should
be ported one-for-one.

## Implemented

| Group | Components |
| --- | --- |
| Static display | Alert, Aspect Ratio, Avatar, Badge, Breadcrumb, Card, Empty, Kbd, Separator, Skeleton, Typography |
| Form basics | Button, Checkbox, Input, Label, Slider, Switch, Textarea |
| Light interaction | Radio Group, Spinner, Toggle, Toggle Group |
| Disclosure | Accordion, Alert Dialog, Calendar, Date Picker, Drawer, Hover Card, Sheet, Tabs |
| Overlay primitives | Context Menu, Dialog, Dropdown, Menubar, Navigation Menu, Popover, Tooltip |
| Command and search | Command, Combobox, Native Select |
| Selection | Select |
| Feedback and data | Data Table, Progress, Sonner, Table, Pagination, Toast |
| Layout and scroll | Carousel, Resizable, Scroll Area, Sidebar |

## Planned Static Or Light Interaction

| Group | Components | Notes |
| --- | --- | --- |
| Static display | Field, Item | Mostly styling and composition. |
| Layout and scroll | - | Carousel, Scroll Area, Resizable, and Sidebar are implemented; runtime measurement, persistence, and gestures remain app-owned. |

## Planned Complex Interaction

| Group | Components | Notes |
| --- | --- | --- |
| Command and search | - | Current M14 command search components are implemented. |
| Menus | - | Current M13 menu system set is implemented. |
| Date and calendar | - | Calendar and Date Picker are implemented. |
| Overlays | - | Current M12 overlay variant set is implemented. |
| Feedback | - | Toast and Sonner are implemented; runtime timers and live regions remain app-owned. |
| Data | Chart component | Chart data and accessibility primitives are implemented; public rendering component remains deferred. |
| Navigation shell | - | Sidebar is implemented; persistence and keyboard shortcuts remain app-owned. |
| Media | - | Carousel is implemented; gestures, autoplay, and live announcements remain app-owned. |

## Deferred Or External

| Component | Reason |
| --- | --- |
| Chart component | M19 implements chart data and accessibility primitives plus recipes; a component remains deferred until backend, measurement, interaction, and fallback-table contracts are explicit. |
| Data Table runtime adapters | Data Table state helpers and composition parts are implemented; filtering, async loading, and virtualization remain app-owned. |
| Calendar / Date Picker runtime adapters | Date math primitives are implemented; parsing, localization, and focus adapters remain deferred. |

## Next Milestone Seeds

After M19, the safest remaining component order is:

1. Static composition gaps such as Field and Item
2. Runtime adapters for timers, portals, focus, gestures, and live announcements
3. Chart rendering backend evaluation after adapter requirements are concrete

See [Complex Component Batches](complex-batches.md) for the expanded milestone
plan.
