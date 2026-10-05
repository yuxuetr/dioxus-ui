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
| Stat | Uses a definition list so each value is read with its title. | Implemented |
| Typography | Uses native text elements; consuming apps own heading hierarchy and landmark placement. | Implemented |

## Form Basics

| Component | Contract | Status |
| --- | --- | --- |
| Button | Supports disabled state and native button semantics; `onclick` fires on click, Enter, and Space, and `type` and `aria-*` pass through; browser-verified on Web. | Implemented |
| Button Group | Uses grouped native button composition; apps own labels, pressed state, and toolbar semantics. | Implemented |
| Checkbox | Uses native checkbox input state; `on_checked_change` reports click, Space, and label changes, and `id`, `name`, and `aria-*` pass through; browser-verified on Web. | Implemented |
| Checkbox | `indeterminate` sets the native mixed state, a change while mixed requests checked, and the state is restored when the app keeps it; browser-verified on Web. | Implemented |
| Checkbox | The mixed state is set after hydration, not in server-rendered HTML. | Planned |
| Input | Supports `aria-invalid` for invalid state; `on_value_change` reports typed text, and `id`, `type`, and `aria-*` pass through so a `Label` can name it; browser-verified on Web. | Implemented |
| Input Group | Preserves native input semantics while apps own labels, descriptions, and decorative addon handling. | Implemented |
| Input OTP | Provides visual presentation slots over one transparent native input; a press on the slots focuses it, `on_value_change` reports the code cleaned to the input mode and length, and `aria-labelledby` passes through; apps own labels and descriptions; browser-verified on Web. | Implemented |
| Label | Can be associated with a form control by the app; `id` and other attributes pass through for `aria-labelledby`. | Implemented |
| Switch | `role="switch"` with `aria-checked` and `data-state`; `on_checked_change` reports click, Space, Enter, and label changes, and `id` and `aria-*` pass through for a name; browser-verified on Web. | Implemented |
| Switch | A hidden input for native form submission is not implemented. | Planned |
| Textarea | Supports `aria-invalid` for invalid state; `on_value_change` reports typed text, and `id` and `aria-*` pass through so a `Label` can name it; browser-verified on Web. | Implemented |

## Disclosure And Selection

| Component | Contract | Status |
| --- | --- | --- |
| Accordion | Triggers are buttons with `aria-expanded` inside `h3` headings; under `Accordion`, triggers link to `role="region"` content with `aria-controls` and `aria-labelledby`; Up, Down, Home, and End move focus past disabled triggers and every trigger stays a Tab stop; browser-verified on Web. | Implemented |
| Accordion | An expanded item that cannot collapse, other heading levels, and horizontal accordions are not implemented. | Planned |
| Calendar | Exposes grid, row, columnheader, gridcell, selected, disabled, today, and range state attributes; the grid takes `aria-labelledby` pointing at a caption `id`; browser-verified on Web. | Implemented |
| Calendar | Keyboard-managed days use roving tabindex, map arrow, Page, Home, and End keys to moves, and follow the focused date with DOM focus; browser-verified on Web inside Date Picker. | Implemented |
| Calendar | Right-to-left arrow mirroring and disabled date skipping remain app-owned. | Planned |
| Collapsible | Uses native trigger button with expanded state and optional trigger/content association; `on_open_change` sends the requested state on click, Enter, and Space, and every part passes attributes through; browser-verified on Web. | Implemented |
| Command | Uses combobox, listbox, option, and active descendant semantics. | Implemented |
| Command | Under `Command`, focus stays in the input, Up, Down, Home, and End move the highlight, a query change returns it to the first match, and Enter or a click chooses; the input controls the list; browser-verified on Web. | Implemented |
| Command | `CommandStatus` is a polite status region for result counts; the app gives the wording; browser-verified on Web. | Implemented |
| Command | Fuzzy ranking, looping, and Ctrl key bindings are not implemented. | Planned |
| Date Picker | Uses button trigger semantics, dialog content semantics, expanded state, invalid state, and placement data attributes. | Implemented |
| Date Picker | Content enters focus on the focused Calendar day, wraps Tab, closes on Escape or outside interaction, and returns focus to the trigger; browser-verified on Web. | Implemented |
| Date Picker | Typed date parsing remains app-owned. | Planned |
| Combobox | Uses combobox, listbox, option, and active descendant semantics. | Implemented |
| Combobox | Input keeps focus with `aria-activedescendant`; arrows highlight options, Enter or click chooses, and a filtered-out highlight clears; browser-verified on Web. | Implemented |
| Combobox | `ComboboxStatus` is a polite status region for result counts, placed outside the popup; the app gives the wording; browser-verified on Web. | Implemented |
| Combobox | Async loading remains app-owned. | Planned |
| Native Select | Uses native select, optgroup, and option elements with platform keyboard and form behavior; `on_value_change` reports the chosen value, and `id`, `name`, and `aria-*` pass through so a `Label` can name it; browser-verified on Web. | Implemented |
| Radio Group | Uses radiogroup/radio roles and checked state; one Tab stop on the checked or first enabled item, arrows move focus past disabled items and check the focused item, with Left and Right swapped in right-to-left layouts; the group and items take `aria-label`, `aria-labelledby`, or an `id` for a `Label`; browser-verified on Web. | Implemented |
| Tabs | Uses tablist, tab, and tabpanel roles with `aria-controls` and `aria-labelledby` under `Tabs`; one Tab stop on the selected trigger, Left, Right, Home, and End move focus past disabled triggers and select the focused tab, with Left and Right swapped in right-to-left layouts; manual activation selects only on Enter, Space, or click, and vertical tabs use Up and Down with `aria-orientation="vertical"`; leaving the list makes the selected trigger the Tab stop again; the tab list takes a passed `aria-label`; browser-verified on Web. | Implemented |
| Select | Uses select-only combobox semantics with listbox content; trigger keeps focus with `aria-activedescendant`; arrows, Home, End, and typeahead move the highlight; Enter, Space, or click chooses; browser-verified on Web. | Implemented |

## Light Interaction

| Component | Contract | Status |
| --- | --- | --- |
| Slider | Uses slider role, horizontal or vertical orientation, and value attributes, with the thumb centered on the value; Arrow, Page, Home, and End keys and pointer press and drag change the value without scrolling the page, and `aria-*` passes through for a name; browser-verified on Web. | Implemented |
| Slider | Right-to-left and multi-thumb sliders are not implemented. | Planned |
| Spinner | Uses status semantics and an accessible label. | Implemented |
| Toggle | Uses button semantics with `aria-pressed`; `on_pressed_change` sends the requested state on click, Enter, and Space, and `aria-*` passes through; browser-verified on Web. | Implemented |
| Toggle Group | Uses grouped toggle buttons with `aria-pressed`; one Tab stop on the last focused item, arrows move focus past disabled items without pressing, with Left and Right swapped in right-to-left layouts; the group takes a passed `aria-label`; browser-verified on Web. | Implemented |

## Message Components

| Component | Contract | Status |
| --- | --- | --- |
| Attachment | Icon-only actions need labels; error state includes visible text, not only color. | Implemented |
| Bubble | Variant meaning must be supported by text, alignment, or surrounding context. | Implemented |
| Marker | Progress or streaming announcements remain app-owned; static marker semantics are implemented. | Implemented |
| Message | Footer icon-only actions need labels; assistant/tool semantics remain app-owned. | Implemented |
| Message Scroller | Controlled parts avoid noisy announcements by default; focus preservation and scroll behavior remain runtime-gated. | Implemented |

## Overlays

| Component | Contract | Status |
| --- | --- | --- |
| Alert Dialog | Uses alertdialog role and modal state for confirmation flows, named and described by its title and description or a passed `aria-label`; browser-verified on Web. | Implemented |
| Alert Dialog | Focuses the first focusable element, wraps Tab, restores focus on close, and closes on Escape; browser-verified on Web. | Implemented |
| Context Menu | Exposes menu, menuitem, menuitemcheckbox, and menuitemradio roles. | Implemented |
| Context Menu | Opens at the pointer, focuses the first item, moves DOM focus with wrapping arrows, Home, End, and typeahead, activates items, and returns focus; browser-verified on Web. | Implemented |
| Context Menu | Nested submenus are not implemented. | Planned |
| Dialog | Exposes dialog role and modal state, named and described by its title and description or a passed `aria-label`; browser-verified on Web. | Implemented |
| Dialog | Focuses the first focusable element, wraps Tab, restores focus on close, and closes on Escape or configured overlay click; browser-verified on Web. | Implemented |
| Drawer | Uses dialog role and modal state for bottom-panel flows, named and described like Dialog. | Implemented |
| Drawer | Shares the Dialog focus scope and dismissal. | Implemented |
| Drawer | Needs gesture and drag-to-dismiss verification. | Planned |
| Dropdown | Anchored placement and Escape or outside dismissal. | Implemented |
| Dropdown | Focuses the first item, moves DOM focus with wrapping arrows, Home, End, and typeahead, activates items, and returns focus; browser-verified on Web. | Implemented |
| Hover Card | Provides controlled rich preview content with placement metadata; under `HoverCard`, hover opens after a delay, keyboard focus opens at once, the pointer and focus can move into the card, and it closes after a close delay; browser-verified on Web. | Implemented |
| Hover Card | Touch opening and a mobile fallback are not implemented; use Popover or Sheet on mobile. | Planned |
| Menubar | Exposes menubar, menu, and menu item roles; the menu bar takes a passed `aria-label`; browser-verified on Web. | Implemented |
| Menubar | Keeps the triggers one Tab stop with Left, Right, Home, and End movement, opens menus that behave like Dropdown, switches menus with Left, Right, or hover, and returns focus to the open menu's trigger, with Left and Right swapped in right-to-left layouts; browser-verified on Web. | Implemented |
| Menubar | Nested submenus are not implemented. | Planned |
| Navigation Menu | Uses navigation structure and link semantics for destinations; the landmark takes a passed `aria-label`; browser-verified on Web. | Implemented |
| Navigation Menu | Follows the disclosure navigation pattern: triggers toggle content on click, keys, or hover, arrows move between top-level items and content links, and Escape returns focus to the trigger, with Left and Right swapped in right-to-left layouts; browser-verified on Web. | Implemented |
| Navigation Menu | Viewport size measurement and submenus are not implemented. | Planned |
| Popover | Anchored placement with flip and shift, Escape or outside dismissal, and a name from its title or a passed `aria-label`; browser-verified on Web. | Implemented |
| Sheet | Uses dialog role and modal state for side-panel flows, named and described like Dialog. | Implemented |
| Sheet | Shares the Dialog focus scope and dismissal. | Implemented |
| Tooltip | Anchored placement and Escape dismissal; under `Tooltip`, hover opens after a delay, keyboard focus opens at once, the pointer can move onto the content, and the trigger has `aria-describedby` while open; browser-verified on Web. | Implemented |
| Tooltip | Skipping the delay between adjacent tooltips and touch long press are not implemented. | Planned |

## Data And Navigation

| Component | Contract | Status |
| --- | --- | --- |
| Pagination | Uses navigation region and current-page state; controls without an `href` are buttons that report `onclick`, and a disabled control cannot take focus or be activated; browser-verified on Web. | Implemented |
| Progress | Uses progressbar value attributes and takes a name and `aria-valuetext` through passed attributes; browser-verified on Web. | Implemented |
| Table | Uses semantic table elements. | Implemented |
| Data Table | Exposes sorted header state, selected row state, hidden cells, loading status, and empty state composition. | Implemented |
| Data Table | Needs app-level filtering, async loading, keyboard shortcuts, and virtualization verification. | Planned |
| Direction | Sets native `dir` semantics for scoped LTR/RTL content. | Implemented |
| Chart primitives | Expose summary text, color-independent series labels, value labels, and fallback-row metadata. | Implemented |
| Chart component | Provides SVG `role="img"` composition, title and description references, text legend hooks, and fallback table rendering for line, bar, and area charts. | Implemented |
| Chart runtime | Cursor exploration, hit testing, keyboard data navigation, animation timing, and external backend accessibility remain app-owned or deferred. | Planned |
| Toast | Exposes status semantics, variant urgency, close/action native controls, and queue state helpers. | Implemented |
| Toast | Persistent polite viewport region, countdown that pauses on hover and focus, and dismiss reasons; browser-verified on Web. | Implemented |
| Toast | Needs app-level announcement wording, portal mounting, and focus policy verification. | Planned |
| Sonner | Exposes status semantics, decorative variant icons, close/action native controls, and queue state helpers. | Implemented |
| Sonner | Persistent polite viewport region, countdown that pauses on hover and focus, and dismiss reasons; browser-verified on Web. | Implemented |
| Sonner | Needs app-level promise orchestration, announcement wording, portal mounting, and focus policy verification. | Planned |
| Carousel | Exposes carousel region, slide group semantics, selected indicator state, and disabled native controls; the content shows the selected index, controls report `onclick`, arrow keys report `on_key_step`, and passed labels name each control; browser-verified on Web. | Implemented |
| Carousel | Needs off-screen slide hiding, gesture behavior, and autoplay verification. | Planned |
| Scroll Area | Keeps native scrolling behavior and exposes presentational scrollbar hooks. | Implemented |
| Resizable | Handles are focusable window splitters with `aria-valuenow` and a separator-line `aria-orientation`; arrow, Home, and End keys and pointer drags report `on_resize`; browser-verified on Web. | Implemented |
| Resizable | Needs right-to-left groups and keyboard collapse. | Planned |
| Sidebar | Exposes collapsed, side, and trigger expansion; the trigger reports `on_collapsed_change`, link and button items mark the current page with `aria-current`, and a disabled item cannot take focus or be activated; browser-verified on Web. | Implemented |
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

M22 implements focus and portal contract types only. M135 implements modal
focus and anchored placement in the styled components themselves (see
[RFC 0010](../rfcs/0010-overlay-interaction-behavior.md)); DOM portal mounting
remains out of scope.
M23 implements timer and live-region contract types only. M136 implements the
Toast and Sonner countdown and persistent viewport live regions in the styled
components (see [RFC 0011](../rfcs/0011-toast-timer-and-live-region.md)).
Screen reader announcements themselves are not automated.
M137 implements Select and Combobox listbox behavior (see
[RFC 0012](../rfcs/0012-listbox-overlay-behavior.md)). M138 implements Calendar
keyboard navigation and Date Picker focus behavior (see
[RFC 0013](../rfcs/0013-date-picker-calendar-keyboard.md)). M139 implements
Dropdown and Context Menu keyboard behavior (see
[RFC 0014](../rfcs/0014-menu-keyboard-behavior.md)). M140 implements Menubar
keyboard behavior (see [RFC 0015](../rfcs/0015-menubar-keyboard-behavior.md)).
M141 implements Navigation Menu interaction behavior (see
[RFC 0016](../rfcs/0016-navigation-menu-interaction.md)).
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
