# Accessibility Contract Checklist

This checklist tracks expected accessibility contracts by component group.
Statuses:

- Implemented: current crate and generated templates expose the contract.
- Planned: required before the component should be considered stable.
- Deferred: intentionally postponed until a primitive or platform decision is
  made.

## Static Display

| Component | Contract | Status |
| --- | --- | --- |
| Alert | Uses alert semantics for urgent messages. | Implemented |
| Aspect Ratio | Adds no implicit media semantics; consuming apps provide labels, alt text, captions, or descriptions for slotted content. | Implemented |
| Avatar | Image `alt` text is provided by the consuming app. | Implemented |
| Badge | Text must communicate state, not color alone. | Planned |
| Breadcrumb | Uses navigation and ordered-list semantics, with current page state on links or page text. | Implemented |
| Card | Does not add implicit landmark or interactive semantics. | Implemented |
| Empty | Does not add implicit alert or status semantics; text and actions remain app-owned. | Implemented |
| Field | Exposes invalid and disabled state attributes while apps own control association and validation semantics. | Implemented |
| Item | Exposes selected and disabled state attributes while apps own collection roles and actions. | Implemented |
| Kbd | Uses native `kbd` semantics for keyboard hints. | Implemented |
| Separator | Supports decorative and semantic separator usage. | Implemented |
| Skeleton | Hidden from assistive technology by default. | Implemented |
| Typography | Uses native text elements; consuming apps own heading hierarchy and landmark placement. | Implemented |

## Form Basics

| Component | Contract | Status |
| --- | --- | --- |
| Button | Supports disabled state and native button semantics. | Implemented |
| Button Group | Uses grouped native button composition; apps own labels, pressed state, and toolbar semantics. | Implemented |
| Checkbox | Uses native checkbox input state. | Implemented |
| Input | Supports `aria-invalid` for invalid state. | Implemented |
| Input Group | Preserves native input semantics while apps own labels, descriptions, and decorative addon handling. | Implemented |
| Input OTP | Provides visual presentation slots plus a native input strategy; apps own labels, descriptions, paste policy, and keyboard handlers. | Implemented |
| Label | Can be associated with a form control by the app. | Implemented |
| Switch | Needs explicit switch semantics before stability. | Planned |
| Textarea | Supports `aria-invalid` for invalid state. | Implemented |

## Disclosure And Selection

| Component | Contract | Status |
| --- | --- | --- |
| Accordion | Needs trigger/content ARIA relationships. | Planned |
| Calendar | Exposes grid, row, columnheader, gridcell, selected, disabled, today, and range state attributes. | Implemented |
| Calendar | Needs keyboard event and DOM focus integration verification. | Planned |
| Collapsible | Uses native trigger button with expanded state and optional trigger/content association. | Implemented |
| Command | Uses combobox, listbox, option, and active descendant semantics. | Implemented |
| Command | Needs keyboard event and filtering integration verification. | Planned |
| Date Picker | Uses button trigger semantics, dialog content semantics, expanded state, invalid state, and placement data attributes. | Implemented |
| Date Picker | Needs focus entry, focus return, typed parsing, and Calendar keyboard integration verification. | Planned |
| Combobox | Uses combobox, listbox, option, and active descendant semantics. | Implemented |
| Combobox | Needs filtering, async loading, and keyboard event integration verification. | Planned |
| Native Select | Uses native select, optgroup, and option elements with platform keyboard and form behavior. | Implemented |
| Radio Group | Uses radiogroup/radio roles, checked state, and roving focus helpers. | Implemented |
| Tabs | Needs tablist, tab, and tabpanel roles. | Planned |
| Select | Needs listbox semantics and keyboard navigation. | Planned |

## Light Interaction

| Component | Contract | Status |
| --- | --- | --- |
| Slider | Uses slider role, horizontal orientation, and value attributes. | Implemented |
| Spinner | Uses status semantics and an accessible label. | Implemented |
| Toggle | Uses button semantics with `aria-pressed`. | Implemented |
| Toggle Group | Uses grouped toggle buttons with roving focus helpers. | Implemented |

## Message Components

| Component | Contract | Status |
| --- | --- | --- |
| Attachment | Icon-only actions need labels; error state includes visible text, not only color. | Implemented |
| Bubble | Variant meaning must be supported by text, alignment, or surrounding context. | Implemented |
| Marker | Progress or streaming announcements remain app-owned; static marker semantics are implemented. | Implemented |
| Message | Footer icon-only actions need labels; assistant/tool semantics remain app-owned. | Implemented |
| Message Scroller | Must preserve focus, avoid noisy announcements, and verify scroll behavior per runtime. | Planned |

## Overlays

| Component | Contract | Status |
| --- | --- | --- |
| Alert Dialog | Uses alertdialog role and modal state for confirmation flows. | Implemented |
| Alert Dialog | Needs focus trap and focus return verification. | Planned |
| Context Menu | Exposes menu, menuitem, menuitemcheckbox, and menuitemradio roles. | Implemented |
| Context Menu | Needs roving focus, typeahead, anchoring, and nested submenu verification. | Planned |
| Dialog | Exposes dialog role and modal state. | Implemented |
| Dialog | Needs focus trap and focus return verification. | Planned |
| Drawer | Uses dialog role and modal state for bottom-panel flows. | Implemented |
| Drawer | Needs focus trap, focus return, and gesture verification. | Planned |
| Dropdown | Needs menu roles, roving focus, and typeahead. | Planned |
| Hover Card | Provides controlled rich preview content with placement metadata. | Implemented |
| Hover Card | Needs hover/focus timing and mobile fallback verification. | Planned |
| Menubar | Exposes menubar, menu, and menu item roles. | Implemented |
| Menubar | Needs roving focus, typeahead, and nested submenu verification. | Planned |
| Navigation Menu | Uses navigation structure and link semantics for destinations. | Implemented |
| Navigation Menu | Needs trigger roving focus and viewport measurement verification. | Planned |
| Popover | Needs dismissal and focus behavior verification. | Planned |
| Sheet | Uses dialog role and modal state for side-panel flows. | Implemented |
| Sheet | Needs focus trap and focus return verification. | Planned |
| Tooltip | Should be discoverable by hover and focus. | Planned |

## Data And Navigation

| Component | Contract | Status |
| --- | --- | --- |
| Pagination | Uses navigation region and current-page state. | Implemented |
| Progress | Uses progressbar value attributes. | Implemented |
| Table | Uses semantic table elements. | Implemented |
| Data Table | Exposes sorted header state, selected row state, hidden cells, loading status, and empty state composition. | Implemented |
| Data Table | Needs app-level filtering, async loading, keyboard shortcuts, and virtualization verification. | Planned |
| Direction | Sets native `dir` semantics for scoped LTR/RTL content. | Implemented |
| Chart primitives | Expose summary text, color-independent series labels, value labels, and fallback-row metadata. | Implemented |
| Chart component | Public rendering component remains deferred until SVG backend, measurement, keyboard, tooltip, animation, and fallback-table contracts are proven. | Deferred |
| Toast | Exposes status semantics, variant urgency, close/action native controls, and queue state helpers. | Implemented |
| Toast | Needs app-level live-region wording, timer scheduling, portal mounting, and focus policy verification. | Planned |
| Sonner | Exposes status semantics, decorative variant icons, close/action native controls, and queue state helpers. | Implemented |
| Sonner | Needs app-level promise orchestration, live-region wording, timer scheduling, portal mounting, and focus policy verification. | Planned |
| Carousel | Exposes carousel region, slide group semantics, selected indicator state, and disabled native controls. | Implemented |
| Carousel | Needs app-level keyboard shortcuts, live announcements, gesture behavior, and autoplay verification. | Planned |
| Scroll Area | Keeps native scrolling behavior and exposes presentational scrollbar hooks. | Implemented |
| Resizable | Exposes separator handles with orientation and disabled state. | Implemented |
| Resizable | Needs app-level keyboard resizing, pointer dragging, and measurement verification. | Planned |
| Sidebar | Exposes collapsed, side, active item, disabled item, trigger expansion, and native navigation composition hooks. | Implemented |
| Sidebar | Needs app-level persistence, responsive breakpoint behavior, and keyboard shortcut verification. | Planned |

## Complex Component Gates

Runtime adapter planning is tracked in:

- [Runtime Adapter Plan](runtime-adapters.md)
- [Focus And Portal Adapter Plan](focus-portal-adapters.md)
- [Focus And Portal Contract Implementation Plan](focus-portal-contracts.md)
- [Timer And Live Region Adapter Plan](timer-live-region-adapters.md)
- [Timer And Live Region Contract Implementation Plan](timer-live-region-contracts.md)
- [Measurement, Pointer, And Gesture Adapter Plan](measurement-pointer-gesture-adapters.md)
- [Measurement Pointer Gesture Contract Implementation Plan](measurement-pointer-gesture-contracts.md)
- [Runtime Renderer Verification Matrix](runtime-renderer-verification.md)
- [Desktop And Mobile Runtime Verification Strategy](runtime-desktop-mobile-verification.md)
- [Runtime Implementation Milestone Seeds](runtime-implementation-milestones.md)
- [Web Runtime Adapter Module Boundaries](runtime-web-adapter-boundaries.md)
- [Input OTP API Plan](input-otp-plan.md)
- [Message and AI-style API Plan](message-ai-plan.md)

M22 implements focus and portal contract types only. Components still require
renderer-level verification before planned focus trap, focus return, and portal
contracts can be marked stable.
M23 implements timer and live-region contract types only. Toast and Sonner still
require renderer-level timer and live-region verification before announcement
runtime behavior can be marked stable.
M24 implements measurement, pointer, and gesture contract types only. Popover,
Tooltip, Dropdown, Select, menu overlays, Resizable, Carousel, and Chart still
require renderer-level measurement, pointer, and gesture verification before
runtime behavior can be marked stable.
M25 defines the renderer verification path and implementation order. Runtime
accessibility behavior should not move from planned to stable until the relevant
Web, Desktop, or Mobile verification milestone is implemented.
M26 adds Web fixture status output for accessibility-relevant runtime paths, but
browser automation and manual assistive checks are still required before stable
runtime accessibility claims.
M27 starts with opt-in Web adapter module boundaries. Runtime adapters remain
experimental until browser assertions cover the relevant behavior.

Before complex interaction components are marked stable:

- Keyboard behavior must be documented.
- Role and ARIA attributes must be listed in component docs.
- Focus entry, movement, and return behavior must be tested or explicitly
  deferred.
- Web and Desktop behavior must be checked separately when portals or DOM focus
  are involved.
- Mobile behavior must not rely on hover-only interaction.
