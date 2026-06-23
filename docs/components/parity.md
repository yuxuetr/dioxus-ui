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
| Disclosure | Accordion, Alert Dialog, Sheet, Tabs |
| Overlay primitives | Dialog, Dropdown, Popover, Tooltip |
| Selection | Select |
| Feedback and data | Progress, Table, Pagination |

## Planned Static Or Light Interaction

| Group | Components | Notes |
| --- | --- | --- |
| Static display | Aspect Ratio, Breadcrumb, Empty, Field, Item, Kbd, Typography | Mostly styling and composition. |
| Layout and scroll | Scroll Area, Resizable | Need measurement and platform-specific behavior. |

## Planned Complex Interaction

| Group | Components | Notes |
| --- | --- | --- |
| Command and search | Command, Combobox | Need keyboard navigation, filtering, and active item management. |
| Menus | Context Menu, Menubar, Navigation Menu | Need roving focus, nested menus, dismissal, and positioning. |
| Date and calendar | Calendar, Date Picker, Native Select | Prefer proven date logic instead of hand-rolling calendar rules. |
| Overlays | Drawer, Hover Card | Reuse dialog/popover primitives after focus and positioning mature. |
| Feedback | Toast, Sonner | Need queue and live-region behavior. |
| Data | Data Table, Chart | Data Table needs table composition plus sorting/filtering state; Chart needs a charting decision. |
| Navigation shell | Sidebar | Needs responsive layout, persistence, and keyboard shortcuts. |
| Media | Carousel | Needs interaction, gesture, and accessibility decisions. |

## Deferred Or External

| Component | Reason |
| --- | --- |
| Chart | Should depend on a clear charting backend and data API decision. |
| Data Table | Better built after table, pagination, command, checkbox, and sorting primitives settle. |
| Calendar / Date Picker | Date math and locale behavior should use a proven crate or focused design pass. |

## Next Milestone Seeds

M8 should not start by implementing a single large component. It should first
define shared primitive behavior for:

- roving focus
- typeahead
- active descendant state
- escape-key dismissal
- pointer outside dismissal
- overlay positioning and collision handling

After M10 and M11, the safest remaining component order is:

1. Alert Dialog and Sheet
2. Context Menu and Menubar
3. Command and Combobox
4. Calendar and Date Picker
5. Data Table

See [Complex Component Batches](complex-batches.md) for the expanded milestone
plan.
