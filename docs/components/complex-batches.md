# Complex Component Batches

This plan ranks the remaining shadcn-style components by dependency order and
implementation risk.

Accessibility expectations for these batches are tracked in the
[accessibility contract checklist](accessibility.md).

## Batch 1: Light Interaction Controls

Components:

- Radio Group
- Toggle
- Toggle Group
- Slider
- Spinner

Dependencies:

- keyboard navigation primitives
- controlled state APIs
- ARIA role and value mapping

Rationale:

These components exercise roving focus and value semantics without requiring
portal positioning.

## Batch 2: Overlay Variants

Components:

- Alert Dialog
- Sheet
- Drawer
- Hover Card

Dependencies:

- focus and portal primitives
- overlay positioning plan
- escape and outside dismissal behavior

Rationale:

These can reuse Dialog, Popover, and Tooltip foundations after portal behavior
is more mature.

## Batch 3: Menu Systems

Components:

- Context Menu
- Menubar
- Navigation Menu

Dependencies:

- roving focus
- typeahead
- nested overlay positioning
- pointer and keyboard dismissal

Rationale:

Menu components combine focus, positioning, nesting, and selection. They should
not be started before the primitives are tested on Web and Desktop.

## Batch 4: Command and Choice

Components:

- Command
- Combobox
- Native Select

Dependencies:

- active descendant state
- typeahead and filtering
- selection state
- listbox semantics

Rationale:

Command and Combobox should share the same active item and filtering model.
Native Select can be simpler but should be documented separately from custom
Select.

## Batch 5: Date and Calendar

Components:

- Calendar
- Date Picker

Dependencies:

- date math and locale strategy
- keyboard grid navigation
- range and single selection model
- popover or sheet presentation

Rationale:

Date logic should use a proven crate or a dedicated design pass. Hand-rolled
calendar behavior is high risk.

## Batch 6: Data and Visualization

Components:

- Data Table
- Chart

Dependencies:

- Table
- Pagination
- Checkbox
- Command or filtering primitive
- sorting and column state model
- charting backend decision

Rationale:

Data Table is a composition layer over many existing parts. Chart should wait
until the project chooses a rendering and data API strategy.

## Batch 7: Layout Shells and Media

Components:

- Sidebar
- Carousel
- Scroll Area
- Resizable

Dependencies:

- responsive layout policy
- gesture and pointer behavior
- persistence or controlled state APIs
- measurement helpers

Rationale:

These components are less about static styling and more about layout behavior,
measurement, and cross-platform ergonomics.

## Recommended Next Milestones

```text
M9  Interaction primitives implementation
M10 Radio Group, Toggle, Toggle Group, Slider, Spinner
M11 Alert Dialog, Sheet, Drawer, Hover Card
M12 Context Menu, Menubar, Navigation Menu
M13 Command, Combobox, Native Select
M14 Calendar and Date Picker
M15 Data Table and Chart strategy
M16 Sidebar, Carousel, Scroll Area, Resizable
```

## Third-Party Logic Candidates

| Area | Prefer |
| --- | --- |
| Calendar/date math | proven Rust date/time crate |
| Chart rendering | explicit chart backend decision |
| Virtualized data tables | dedicated virtualization strategy |
| Gesture-heavy carousel | proven interaction logic or a focused primitive |

The default should remain first-party primitives for focus, dismissal, class
composition, and registry integration.
