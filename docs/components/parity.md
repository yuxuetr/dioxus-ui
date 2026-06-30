# shadcn/ui Parity Matrix

This matrix tracks how `dioxus-ui` maps to the public shadcn/ui component
catalog. It is a planning aid, not a promise that every React component should
be ported one-for-one.

## Implemented

| Group | Components |
| --- | --- |
| Static display | Alert, Aspect Ratio, Avatar, Badge, Breadcrumb, Card, Empty, Field, Item, Kbd, Separator, Skeleton, Typography |
| Form basics | Button, Button Group, Checkbox, Input, Input Group, Input OTP, Label, Slider, Switch, Textarea |
| Light interaction | Radio Group, Spinner, Toggle, Toggle Group |
| Disclosure | Accordion, Alert Dialog, Calendar, Collapsible, Date Picker, Drawer, Hover Card, Sheet, Tabs |
| Overlay primitives | Context Menu, Dialog, Dropdown, Menubar, Navigation Menu, Popover, Tooltip |
| Command and search | Command, Combobox, Native Select |
| Selection | Select |
| Feedback and data | Data Table, Progress, Sonner, Table, Pagination, Toast |
| Layout and scroll | Carousel, Direction, Resizable, Scroll Area, Sidebar |
| Message and AI-style composition | Attachment, Bubble, Marker, Message, Message Scroller |

## Planned Static Or Light Interaction

| Group | Components | Notes |
| --- | --- | --- |
| Low-risk composition | - | Button Group, Input Group, Collapsible, and Direction are implemented. |
| Layout and scroll | - | Carousel, Scroll Area, Resizable, and Sidebar are implemented; runtime measurement, persistence, and gestures remain app-owned. |

## Planned Complex Interaction

| Group | Components | Notes |
| --- | --- | --- |
| Command and search | - | Current M14 command search components are implemented. |
| Menus | - | Current M13 menu system set is implemented. |
| Date and calendar | - | Calendar and Date Picker are implemented. |
| Overlays | - | Current M12 overlay variant set is implemented. |
| Feedback | - | Toast and Sonner are implemented; M23 defines timer and live-region contract types. |
| Data | Chart component | Chart data and accessibility primitives are implemented; M29.1 selects first-party SVG as the first future rendering path, but the public component remains deferred. |
| Form-specific | - | Input OTP is implemented; validation, submission, resend timers, and paste policy remain app-owned. |
| Message and AI-style composition | - | Attachment, Bubble, Message, Marker, and Message Scroller are implemented; actual scroll commands remain app/runtime-owned. |
| Navigation shell | - | Sidebar is implemented; persistence and keyboard shortcuts remain app-owned. |
| Media | - | Carousel is implemented; M24 defines gesture contract types. |

## Deferred Or External

| Component | Reason |
| --- | --- |
| Chart component | M19 implements chart data and accessibility primitives plus recipes; M29.1 selects first-party SVG as the first future rendering path and keeps Chart deferred until measurement, interaction, animation, and fallback-table contracts are explicit. |
| Message Scroller runtime adapters | Component parts and pure helpers are implemented; actual DOM/WebView scroll commands, focus preservation, prepend offset restoration, and browser automation remain gated by runtime verification. |
| Data Table runtime adapters | Data Table state helpers and composition parts are implemented; filtering, async loading, and virtualization remain app-owned. |
| Calendar / Date Picker runtime adapters | Date math primitives are implemented; parsing, localization, and focus adapters remain deferred. |
| Runtime adapters | M21 documents adapter boundaries; M22 implements focus/portal contract types; M23 implements timer/live-region contract types; M24 implements measurement/pointer/gesture contract types. Renderer implementations remain deferred. |

## Next Milestone Seeds

After M34, the safest remaining implementation order is:

1. Chart public component preparation after measurement and fallback-table gates

See [Current shadcn Gap Audit](current-shadcn-gaps.md) and
[Complex Component Batches](complex-batches.md) for the expanded milestone plan.
