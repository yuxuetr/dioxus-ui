# shadcn/ui Parity Matrix

This matrix tracks how `dioxus-ui` maps to the public shadcn/ui component
catalog. It is a planning aid, not a promise that every React component should
be ported one-for-one.

## Implemented

| Group | Components |
| --- | --- |
| Static display | Alert, Avatar, Badge, Card, Separator, Skeleton |
| Form basics | Button, Checkbox, Input, Label, Switch, Textarea |
| Disclosure | Accordion, Tabs |
| Overlay primitives | Dialog, Dropdown, Popover, Tooltip |
| Selection | Select |
| Feedback and data | Progress, Table, Pagination |

## Planned Static Or Light Interaction

| Group | Components | Notes |
| --- | --- | --- |
| Static display | Aspect Ratio, Breadcrumb, Empty, Field, Item, Kbd, Typography | Mostly styling and composition. |
| Form controls | Radio Group, Slider, Toggle, Toggle Group | Need keyboard state and ARIA behavior, but less overlay work. |
| Layout and scroll | Scroll Area, Resizable | Need measurement and platform-specific behavior. |

## Planned Complex Interaction

| Group | Components | Notes |
| --- | --- | --- |
| Command and search | Command, Combobox | Need keyboard navigation, filtering, and active item management. |
| Menus | Context Menu, Menubar, Navigation Menu | Need roving focus, nested menus, dismissal, and positioning. |
| Date and calendar | Calendar, Date Picker, Native Select | Prefer proven date logic instead of hand-rolling calendar rules. |
| Overlays | Alert Dialog, Drawer, Hover Card, Sheet | Reuse dialog/popover primitives after focus and positioning mature. |
| Feedback | Toast, Sonner, Spinner | Toast/Sonner need queue and live-region behavior; Spinner is simple. |
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

After that, the safest component order is:

1. Radio Group and Toggle Group
2. Slider
3. Alert Dialog and Sheet
4. Context Menu and Menubar
5. Command and Combobox
6. Calendar and Date Picker
7. Data Table
