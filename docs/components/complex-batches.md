# Complex Component Batches

This plan ranks the remaining shadcn-style components by dependency order and
implementation risk.

Accessibility expectations for these batches are tracked in the
[accessibility contract checklist](accessibility.md).

## Batch 1: Light Interaction Controls

Status: Implemented in M11.

Components:

- Radio Group
- Toggle
- Toggle Group
- Slider
- Spinner

Implementation specification:

- [Interaction Batch 1 Specification](interaction-batch-1.md)

Dependencies:

- keyboard navigation primitives
- controlled state APIs
- ARIA role and value mapping

Rationale:

These components exercise roving focus and value semantics without requiring
portal positioning.

## Batch 2: Overlay Variants

Status: Implemented in M12.

Components:

- Alert Dialog
- Sheet
- Drawer
- Hover Card

Implementation specification:

- [Overlay Variant API Plan](overlay-variants.md)

Dependencies:

- focus and portal primitives
- overlay positioning plan
- escape and outside dismissal behavior

Rationale:

These can reuse Dialog, Popover, and Tooltip foundations after portal behavior
is more mature.

M12 shipped controlled styled parts for all four overlay variants. Runtime focus
trapping, portal mounting, hover timing, gestures, and transition orchestration
remain tracked as overlay runtime adapter work rather than component API work.

## Batch 3: Menu Systems

Status: Implemented in M13.

Components:

- Context Menu
- Menubar
- Navigation Menu

Implementation specification:

- [Menu System API Plan](menu-systems.md)

Dependencies:

- roving focus
- typeahead
- nested overlay positioning
- pointer and keyboard dismissal

Rationale:

Menu components combine focus, positioning, nesting, and selection. They should
not be started before the primitives are tested on Web and Desktop.

M13 shipped controlled styled parts for all three menu systems. Runtime roving
focus commands, typeahead event wiring, context-trigger anchoring, nested
submenu handoff, and viewport measurement remain tracked as adapter work.

## Batch 4: Command and Choice

Status: Implemented in M14.

Components:

- Command
- Combobox
- Native Select

Implementation specification:

- [Command and Choice API Plan](command-choice.md)

Dependencies:

- active descendant state
- typeahead and filtering
- selection state
- listbox semantics

Rationale:

Command and Combobox should share the same active item and filtering model.
Native Select can be simpler but should be documented separately from custom
Select.

M14 shipped controlled styled parts for Command, Combobox, and Native Select.
Runtime filtering, keyboard event wiring, DOM focus commands, async loading,
portal mounting, and virtualization remain app-owned or deferred adapter work.

## Batch 5: Date and Calendar

Status: Implemented in M15.

Components:

- Calendar
- Date Picker

Implementation specification:

- [Calendar and Date Picker API Plan](calendar-date.md)

Dependencies:

- date math and locale strategy
- keyboard grid navigation
- range and single selection model
- popover or sheet presentation

Rationale:

Date logic should use a proven crate or a dedicated design pass. Hand-rolled
calendar behavior is high risk.

M15 shipped pure first-party Calendar date primitives, controlled Calendar
styled parts, and controlled Date Picker composition parts. Typed parsing,
locale formatting, time zone conversion, non-Gregorian calendars, DOM focus
commands, and portal mounting remain adapter or application work.

## Batch 6: Data and Visualization

Status: Implemented in M16 for Data Table; Chart strategy documented and
component deferred.

Components:

- Data Table
- Chart

Implementation specification:

- [Data Table and Chart Strategy](data-visualization.md)

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

M16 shipped pure Data Table state primitives and controlled Data Table
composition parts. Chart remains intentionally deferred as a component; the
backend, data API, accessibility, source-copy dependency, and platform support
policy are documented in the Chart strategy.

## Batch 7: Layout Shells and Media

Status: Implemented in M17.

Components:

- Sidebar
- Carousel
- Scroll Area
- Resizable

Implementation specification:

- [Layout Shells and Media API Plan](layout-media.md)

Dependencies:

- responsive layout policy
- gesture and pointer behavior
- persistence or controlled state APIs
- measurement helpers

Rationale:

These components are less about static styling and more about layout behavior,
measurement, and cross-platform ergonomics.

M17 shipped pure layout/media state primitives and controlled composition parts
for Sidebar, Carousel, Scroll Area, and Resizable. Runtime measurement, pointer
dragging, persistence, responsive breakpoint orchestration, carousel gestures,
autoplay, and live announcements remain app-owned or deferred adapter work.

## Batch 8: Feedback Notifications

Status: Implemented in M18.

Components:

- Toast
- Sonner

Implementation specification:

- [Feedback Notifications API Plan](feedback-notifications.md)

Dependencies:

- queue state primitives
- placement policy
- dismissal reasons
- live-region accessibility contract
- timer ownership policy

Rationale:

Feedback notifications combine transient state, urgency, announcement timing,
actions, and dismissal behavior. They should start with deterministic queue
helpers and controlled styled parts before adding timer, portal, or async
runtime adapters.

M18 shipped pure feedback queue primitives and controlled composition parts for
Toast and Sonner. Runtime timers, portal mounting, promise orchestration,
live-region announcement wording, escape-key behavior, and focus policy remain
app-owned or deferred adapter work.

## Batch 9: Chart Follow-through

Status: Implemented in M19.

Components:

- Chart data primitives
- Chart accessibility helpers
- Docs-only chart recipes

Implementation specification:

- [Chart Follow-through API Plan](chart-follow-through.md)

Dependencies:

- data domain helpers
- scale helpers
- color token policy
- accessible summary and fallback-row helpers
- backend dependency policy

Rationale:

Chart remains too broad for a public component without a backend decision, but
the project can still make progress by defining data and accessibility
primitives that future rendering adapters can share.

M19 shipped pure chart data primitives, scale helpers, color token helpers,
summary helpers, fallback-row helpers, and docs-only recipes for line, bar, and
area charts. A public Chart component, registry entry, crate feature, rendering
backend, tooltip runtime, and chart animation runtime remain deferred.

## Batch 10: Static Composition Gaps

Status: Implemented in M20.

Components:

- Aspect Ratio
- Kbd
- Typography
- Breadcrumb
- Empty
- Field
- Item

Implementation specification:

- [Static Composition API Plan](static-composition.md)

Dependencies:

- class composition helpers
- semantic HTML mapping
- source-copy template policy
- accessibility checklist updates

Rationale:

These components are mostly styled wrappers and semantic composition parts.
They close visible catalog gaps without requiring runtime adapters, routing,
validation engines, or rendering backends.

M20 shipped all seven components in crate mode and source-copy mode. Runtime
routing, validation, collection semantics, icon rendering, and media loading
remain app-owned.

## Batch 11: Runtime Adapter Planning

Status: Implemented in M21.

Adapter families:

- focus commands and focus traps
- portal mounting
- timer scheduling
- live-region announcements
- measurement
- pointer dragging
- gestures

Implementation specifications:

- [Runtime Adapter Plan](runtime-adapters.md)
- [Focus And Portal Adapter Plan](focus-portal-adapters.md)
- [Timer And Live Region Adapter Plan](timer-live-region-adapters.md)
- [Measurement, Pointer, And Gesture Adapter Plan](measurement-pointer-gesture-adapters.md)

Dependencies:

- existing pure primitives
- controlled styled component APIs
- Web/Desktop/Mobile verification strategy
- source-copy opt-in policy

Rationale:

M10-M20 intentionally left runtime commands out of primitives and styled
components. M21 defines the adapter boundaries needed to add richer behavior
without making source-copy components renderer-specific by default.

M21 recommended implementing focus and portal adapter contracts first. M22
shipped the primitive-layer contract surface and pure Dialog/Popover mapping
tests. M23 shipped timer and live-region contracts plus Toast/Sonner mapping
helpers. M24 shipped measurement, pointer, and gesture contracts plus Carousel
gesture mapping helpers. M25 defines the renderer verification matrix, Web
harness plan, Desktop/Mobile strategy, and implementation milestone seeds.
M26 adds a compile-checked Web runtime verification fixture with visible
fallback output for all runtime contract families. M27 adds fixture-local
experimental Web adapters for feedback, overlay, measurement, pointer, and
gesture families. M28 adds Desktop smoke verification and a Mobile checklist.
M29 evaluates chart rendering backends.
Renderer-specific default runtime commands remain future work.

## Batch 12: Current shadcn Gap Planning

Status: Planned in M30.

Components:

- Button Group
- Input Group
- Collapsible
- Direction
- Input OTP
- Attachment
- Bubble
- Message
- Marker
- Message Scroller
- Chart

Implementation specification:

- [Current shadcn Gap Audit](current-shadcn-gaps.md)
- [Low-risk Composition Gap API Plan](low-risk-composition-gaps.md)
- [Input OTP API Plan](input-otp-plan.md)
- [Message and AI-style API Plan](message-ai-plan.md)

Dependencies:

- current upstream component audit
- local registry/template/docs parity check
- source-copy policy
- runtime verification status

Rationale:

The upstream shadcn catalog has added low-risk composition components,
form-specific components, and message/AI-style components. M30 classifies these
before implementation so runtime-heavy scrolling, upload, markdown, and chart
behavior stay app-owned until explicitly designed.

## Batch 13: Low-risk Composition Gaps

Status: Implemented in M31.

Components:

- Button Group
- Input Group
- Collapsible
- Direction

Implementation specification:

- [Low-risk Composition Gap API Plan](low-risk-composition-gaps.md)

Dependencies:

- existing Button and Input components
- disclosure and controlled state conventions
- RTL/LTR attribute and class policy
- generated source-copy smoke checks

Rationale:

These are the safest current upstream gaps. They mostly compose existing parts
and were implemented before message and chart work.

## Batch 14: Form-specific Gap

Status: Planned in M32.

Components:

- Input OTP

Implementation specification:

- [Input OTP API Plan](input-otp-plan.md)

Dependencies:

- input semantics
- keyboard and paste handling
- mobile keyboard constraints
- screen-reader labeling strategy

Rationale:

Input OTP is small but easy to get wrong. It should get a focused primitive or
helper pass before the styled component lands.

## Batch 15: Message and Attachment Components

Status: Planned in M33.

Components:

- Attachment
- Bubble
- Message
- Marker

Implementation specification:

- [Message and AI-style API Plan](message-ai-plan.md)

Dependencies:

- static composition conventions
- Avatar, Badge, Button, Tooltip, and Popover composition
- accessibility checklist updates
- source-copy examples

Rationale:

These components should remain provider-neutral. Upload transport, markdown,
syntax highlighting, citation resolution, streaming, and model/provider behavior
stay app-owned.

## Batch 16: Message Scroller and Runtime Follow-through

Status: Planned in M34.

Components:

- Message Scroller

Implementation specification:

- [Message and AI-style API Plan](message-ai-plan.md)

Dependencies:

- scroll intent state helpers
- runtime measurement and scroll command boundaries
- Web browser assertions
- Desktop/Mobile deferred behavior notes

Rationale:

Message Scroller depends on runtime behavior: sticky bottom, unread marker,
streaming append, user-scrolled-away state, and scroll commands. It should come
after static message components.

## Batch 17: Chart Public Component Preparation

Status: Planned in M35.

Components:

- Chart

Dependencies:

- M29 backend decision
- chart data and accessibility primitives
- measurement verification
- fallback table policy
- tooltip and reduced-motion strategy

Rationale:

M29 selects first-party SVG as the preferred first rendering path, but a public
Chart component remains gated by measurement, accessibility, fallback table,
tooltip, animation, and source-copy policy.

## Recommended Next Milestones

```text
M10-M20 completed component and primitive batches
M21 completed runtime adapter planning
M22 completed focus and portal adapter contracts with Dialog/Popover mapping tests
M23 completed timer/live-region adapter contracts for Toast and Sonner
M24 completed measurement/pointer/gesture adapter contracts
M25 completed renderer runtime verification planning for Web, Desktop, and Mobile
M26 completed Web runtime verification fixture scaffold and command documentation
M27 completed fixture-local experimental Web runtime adapters
M28 completed Desktop smoke fixture and Mobile verification checklist
M29 completed chart rendering backend evaluation
Next: verify current shadcn gaps and plan missing API surfaces
Next: implement Input OTP
Next: implement message and attachment components
Next: implement Message Scroller after runtime boundary planning
Next: prepare first-party SVG Chart only after gates pass
```

## Third-Party Logic Candidates

| Area | Prefer |
| --- | --- |
| Calendar/date math | proven Rust date/time crate |
| Chart rendering | explicit chart backend decision |
| Message virtualization | dedicated virtualization strategy |
| Markdown or rich message content | app-owned parser/renderer or explicit optional adapter |
| Virtualized data tables | dedicated virtualization strategy |
| Gesture-heavy carousel | proven interaction logic or a focused primitive |

The default should remain first-party primitives for focus, dismissal, class
composition, and registry integration.
