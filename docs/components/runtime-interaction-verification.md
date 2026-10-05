# Runtime Interaction Verification

This document defines the M111 plan for adding the first browser-backed
interaction checks on top of the rendered Web preview.

Status: Fixture targets implemented in M111.2; browser verifier implemented in
M111.3; package alias and documentation wiring implemented in M111.4.

## Problem

M110 proves that every public component preview target exists in a browser DOM,
is visible, contains text, and has a non-empty layout box. That still does not
prove representative component interactions work.

The highest-risk components are runtime-sensitive overlays, disclosure
patterns, selection controls, keyboard-visible states, and components that rely
on focus or ARIA state. Those should get a small, deterministic interaction
fixture before any screenshot or parity claim.

## Decision

M111 should add a focused Web preview interaction layer:

1. Add lightweight interaction fixture targets to `examples/preview-states`.
2. Preserve all existing `data-component-preview`, `data-preview-panel`, and
   rendered DOM verification targets.
3. Add an opt-in Playwright verifier that starts the Web preview and exercises
   the fixture targets.
4. Keep visual diffing, screenshot artifacts, runtime adapter graduation, and
   component API changes out of scope.

This is not the same as the Web runtime adapter harness. The runtime harness
verifies primitive runtime families such as focus, portal, timer, live-region,
measurement, pointer, gesture, and scroll command contracts. M111 verifies that
the rendered component preview can expose representative user-level
interactions in the browser.

## First Interaction Set

The first fixture should cover a small, stable set:

| Interaction | Representative Risk | Expected Browser Assertion |
| --- | --- | --- |
| Disclosure | Accordion, Collapsible, Sidebar-style state | click toggles `aria-expanded`, `data-state`, and visible content |
| Overlay | Dialog, Popover, Dropdown, Select-style open state | trigger opens content, close control hides it, dialog role remains present |
| Selection | Checkbox, Switch, Radio Group, Toggle, Tabs | click or keyboard changes checked/selected/pressed attributes |
| Keyboard | Command, menus, roving-focus components | keyboard activation changes active descendant or active option marker |
| Scroll status | Message Scroller and Scroll Area | jump button or status target exposes deterministic state without mutating app data |

The first implementation can use generic fixture controls rather than every
public component. The goal is to prove interaction verification infrastructure
and representative browser behavior, not to create a full prop matrix.

## Verification Contract

`npm run verify:runtime-interactions` should:

- start the Web preview with `dx serve`
- use Playwright-managed Chromium by default
- support `DIOXUS_UI_BROWSER_EXECUTABLE`
- wait for stable fixture root selectors
- click and keyboard-drive each fixture target
- assert state through DOM attributes, visible text, active element, and
  deterministic status regions
- clean up the preview server before exit

The command should be opt-in and outside `npm run verify` and
`npm run verify:release` until browser availability and fixture behavior are
stable.

The M111 implementation covers five generic fixture targets:

- `disclosure`
- `overlay`
- `selection`
- `keyboard`
- `scroll-status`

It intentionally verifies representative state transitions instead of every
public component prop combination.

For a serial local run of all browser-backed preview checks, use
`npm run verify:browser-local`. Do not parallelize browser preview commands;
each command starts its own `dx serve` process.

## Non-goals

- no screenshots by default
- no visual diffing or shadcn/ui visual parity claims
- no full accessibility certification
- no complete overlay focus trap certification
- no native Desktop WebView automation in this command; the Desktop WebView
  runs `npm run verify:desktop-interactions` (M142, RFC 0017)
- no native Mobile automation in this command; the iOS Simulator runs
  `npm run verify:mobile-interactions` (M143, RFC 0018) and an Android
  emulator runs `npm run verify:android-interactions` (M145, RFC 0020)
- no component API changes
- no generated source-copy template rewrites
- no provider/domain behavior such as uploads, charts backed by external
  libraries, markdown parsing, async data, or network state

## M135 Component Overlay Fixtures

M135 adds fixtures that render real `dioxus-ui` components instead of generic
controls, so the verifier exercises the shipped overlay behavior from
[RFC 0010](../rfcs/0010-overlay-interaction-behavior.md):

| Target | Component | Browser Assertion |
| --- | --- | --- |
| `dialog` | Dialog | opening focuses the first input; Tab and Shift+Tab wrap; Escape, overlay click, and Close each close it and restore focus to the trigger |
| `alert-dialog` | Alert Dialog | opening focuses Cancel; Tab wraps; Escape closes without confirming; Action runs its handler, closes, and restores focus |
| `popover` | Popover | anchored content sits below the trigger inside the viewport; Escape and an outside click close it; near the viewport bottom it flips above the trigger; the trigger still toggles it |
| `tooltip` | Tooltip | hover opens it after the delay and keyboard focus opens it at once; the trigger has `aria-describedby` only while it is open; content sits above the trigger; the pointer can move onto the content without closing it, and leaving closes it; a trigger press closes it and it stays closed while the pointer rests; blur and Escape close it, and an outside press does not |
| `hover-card` | Hover Card | hover opens it after the delay and keyboard focus opens it at once; the trigger never has `aria-describedby`; content sits below the trigger; the pointer can move onto the card, and leaving closes it after the close delay; a trigger press keeps it open, and so does a press on the card's text afterwards; Tab into the card keeps it open and Tab out closes it; Escape and an outside press close it |
| `command` | Command | the input controls the list; the first option starts highlighted; arrows skip the disabled item and stop at the ends; Home and End; typed text, including a space, reaches the input; narrowing and widening the query moves the highlight back to the first match; nothing matching leaves no active descendant and Enter chooses nothing; the pointer highlights; Enter and click choose while focus stays in the input; `CommandStatus` is a polite, atomic status region whose text follows the match count and says "No results", and the same element stays mounted across query changes |
| `toast` | Toast | the viewport is a polite `Notifications` region; the toast closes with reason `timeout` after its countdown, stays open while hovered past its duration, and reports `action` and `close` reasons |
| `sonner` | Sonner | the viewport is a polite region; a mounted toast unmounts with reason `timeout` and reports `close` |
| `select` | Select | the listbox sits below the trigger while focus stays on it; the selected option starts highlighted; arrows skip the disabled option without wrapping; Home, End, and typeahead move the highlight; Enter, Space, and click choose and close; a disabled option cannot be chosen; ArrowDown reopens; Escape and an outside click close it |
| `combobox` | Combobox | typing opens the listbox below the input with no highlight; arrows highlight options; filtering keeps a still-matching highlight and clears a removed one; Enter and click choose, fill the input, and keep focus in it; ArrowDown reopens and Escape closes; `ComboboxStatus` sits outside the popup, stays exposed and mounted while it is closed, and its text follows the match count while open |
| `date-picker` | Date Picker with Calendar | content sits below the trigger; opening focuses the selected day, the only `tabindex="0"` day; arrows, Home, End, Page Up, Page Down, and Shift+Page keys move focus across months with days keyed by date; Tab wraps between the month buttons and the focused day; Enter and click choose a date, close, and return focus to the trigger; Escape returns focus too; an outside click closes it and keeps focus on a clicked control |
| `dropdown` | Dropdown | opening focuses the first item; arrows wrap and skip the disabled item; Home, End, and repeated-letter typeahead move focus; Enter, Space, and click run the item's `onclick`, close, and return focus to the trigger; a disabled item does nothing; Escape and Tab close it |
| `menubar` | Menubar | the triggers are one Tab stop; Left and Right wrap and skip the disabled trigger, Home and End jump; ArrowDown and Enter open a menu on its first item; Left and Right inside an open menu switch to the adjacent menu; with `dir="rtl"` ArrowLeft moves to and opens the next menu and ArrowRight the previous one, while Up and Down inside a menu are unchanged; Escape returns focus to the open menu's trigger; hovering a trigger switches menus only while one is open; click runs an item, closes, and returns focus; clicking the open trigger closes it; Tab closes and leaves the bar |
| `navigation-menu` | Navigation Menu | a click toggles content and a click-closed trigger stays closed under the pointer; Left, Right, Home, and End move between top-level triggers and links, skipping the disabled trigger, with Left and Right swapped under `dir="rtl"`; Enter and Space toggle; ArrowDown opens content on its first link, ArrowDown wraps and skips the disabled link, and Tab continues in document order; Escape returns focus to the trigger; focus leaving, an outside press, and a content link click close it; hover opens after a delay, switches immediately while open, and closes after leaving |
| `tabs` | Tabs | the selected trigger is the only Tab stop and names its panel through `aria-controls` and `aria-labelledby`; Left and Right wrap and skip the disabled trigger, Home and End jump, and each move selects the focused tab; with `dir="rtl"` the next tab sits on the left and ArrowLeft moves to it; removing `dir` restores the keys; Tab moves into the visible panel; a click selects |
| `tabs-vertical` | Tabs, vertical with manual activation | the list has `aria-orientation="vertical"` and the root, list, and panels `data-orientation="vertical"`; Up and Down move, wrap, and leave the selection alone, Left and Right do nothing, Home and End jump; Enter, Space, and click select; Tab from an unselected trigger reaches the selected panel and Shift+Tab returns to the selected trigger |
| `radio-group` | Radio Group | with nothing checked the first enabled item is the Tab stop; all four arrows move, wrap, skip the disabled item, and check the focused item; with `dir="rtl"` Left and Right swap while Up and Down do not; the checked item becomes the Tab stop; a click checks |
| `toggle-group` | Toggle Group | arrows move focus without pressing, ignore Up and Down in a horizontal group, wrap, and skip the disabled item; Left and Right swap under `dir="rtl"`; Enter and click toggle; the last focused item stays the Tab stop after focus leaves |
| `accordion` | Accordion | each trigger sits in a heading and names its region through `aria-controls` and `aria-labelledby`; every enabled trigger is a Tab stop; click, Enter, and Space toggle, and toggling the open item closes it; Up and Down move focus without toggling, wrap, and skip the disabled trigger, Home and End jump, and Left and Right do nothing |
| `context-menu` | Context Menu | a right-click opens the menu at the pointer and a second right-click moves it; the first item takes focus; arrows wrap; Enter toggles the checkbox item and closes; click runs an item; near the viewport bottom the menu flips above the pointer; an outside click closes it |

Since M168 the verifier serves compiled Tailwind, so the Dialog overlay has
its `fixed inset-0` box and the verifier presses it outside the content.
Placement uses inline fixed coordinates and does not depend on Tailwind.

Reverse checks run during M135: removing the Tab wrap listener, ignoring
Escape in Dialog content, disabling the flip, and giving Tooltip the popover
dismissal default each make the verifier fail. M136 adds two more: a countdown
that ignores hover and a countdown that never starts.

M136 adds the Toast and Sonner rows from
[RFC 0011](../rfcs/0011-toast-timer-and-live-region.md).

M137 adds the Select and Combobox rows from
[RFC 0012](../rfcs/0012-listbox-overlay-behavior.md). Visibility comes from the
Rust render while the page scripts attach a frame later, so the verifier waits
for anchored content to become `position: fixed` before pressing keys or
clicking outside. Removing arrow navigation, typeahead, disabled option
skipping, or the filter highlight reset each make the verifier fail.

M138 adds the Date Picker row from
[RFC 0013](../rfcs/0013-date-picker-calendar-keyboard.md). Removing the key
mapping, the focus move after a key, focus entry on the focused day, or the
`tabindex="-1"` filter in Tab wrapping each make the verifier fail. A press on
non-focusable content outside the picker leaves focus on the body, as the
browser does for any outside click, so only the close is asserted there.

M139 adds the Dropdown and Context Menu rows from
[RFC 0014](../rfcs/0014-menu-keyboard-behavior.md). Removing menu wrapping,
item activation on Enter, focus return on close, or the point anchor each make
the verifier fail. The flip check also caught menu items being focused while
the menu was still in normal flow, which scrolled the page away from the
pointer; items are now focused without scrolling.

M140 adds the Menubar row from
[RFC 0015](../rfcs/0015-menubar-keyboard-behavior.md). Removing trigger roving,
Left and Right switching inside a menu, hover switching, or focus return to the
anchor each make the verifier fail. The switching check also caught Chrome
sending `pointerover` when an opening menu shifted the layout under a resting
cursor, which switched back to the menu under the cursor; hover switching now
listens for pointer movement, and reverting to `pointerover` fails the
verifier.

M141 adds the Navigation Menu row from
[RFC 0016](../rfcs/0016-navigation-menu-interaction.md). Removing top-level
arrow movement, content entry, hover opening, the hover open delay, hover
closing, the click-closed guard, outside dismissal, or focus return on Escape
each make the verifier fail.

M144 adds the Tabs, Radio Group, and Toggle Group rows from
[RFC 0019](../rfcs/0019-roving-group-interaction.md). Removing arrow movement,
selection on focus, click reporting, or the first-enabled Tab stop fallback,
making Toggle Group select on focus, or making it prefer the pressed item
over the last focused one each make the verifier fail.

M146 adds the Accordion row from
[RFC 0021](../rfcs/0021-accordion-interaction.md). Removing click reporting,
Up and Down movement, the every-item Tab stop mode, `aria-controls`, or
`aria-labelledby` each make the verifier fail.

M147 extends the Tooltip row from
[RFC 0022](../rfcs/0022-tooltip-hover-and-focus-opening.md). Removing the
hover delay, the content hover grace, keyboard focus opening, press closing,
or `aria-describedby` each make the verifier fail.

M148 adds the Hover Card row from
[RFC 0023](../rfcs/0023-hover-card-hover-and-focus-opening.md), with Tooltip
moved onto the same hover-open script. Removing the open delay, the close
delay, the exemption for focus moving into the card, the exemption for blur
while the pointer is over the parts, or making a trigger press close each
make the verifier fail. Turning on the trigger description fails only when
the content also has an id, since the script links nothing without one; the
reverse check sets both.

M150 extends the Tabs, Radio Group, Toggle Group, Menubar, and Navigation Menu
rows from [RFC 0025](../rfcs/0025-right-to-left-arrow-mirroring.md). The
verifier sets `dir="rtl"` on each fixture, and Radio Group gains a third
enabled item so that wrapping next and previous land on different items.
Removing the swap from the roving group, Menubar, or Navigation Menu script,
or also swapping Up and Down, each make the verifier fail.

M151 adds the vertical Tabs row from
[RFC 0026](../rfcs/0026-tabs-manual-activation-and-vertical-orientation.md).
Selecting on focus in manual mode, removing the Tab stop reset when focus
leaves, preferring the last focused trigger in manual mode, keeping
horizontal keys on the vertical list, or a horizontal `aria-orientation` each
make the verifier fail. Since M168 the verifier also measures that the
vertical triggers stack in one column.

M152 extends the Command and Combobox rows from
[RFC 0027](../rfcs/0027-combobox-and-command-result-announcements.md). The
verifier tags each status element with a script property and checks it is
still there after the query changes. Rendering the region only when it has
text, recreating it by swapping its parent element, removing `role="status"`
or `aria-live`, or moving the Combobox region inside the popup each make the
verifier fail. A changing `key` outside a list, or `if` and `else` branches
that both render `CommandStatus` in the same place, kept the same element in
Dioxus 0.7, so neither counts as a remount.

M153 adds a Switch and Checkbox row from
[RFC 0028](../rfcs/0028-switch-and-checkbox-change-events.md). The verifier
finds each control by the name its `Label` or `aria-label` gives it, then
toggles it with a click, Space, and a label click (and Enter for Switch). A
native checkbox toggles itself even when nobody handles the event, so the
verifier also checks the app state the fixture writes to its article.
Removing either callback, not spreading the attributes, or sending the current
state instead of the requested one each make the verifier fail.

M154 adds a form controls row from
[RFC 0029](../rfcs/0029-button-toggle-input-and-textarea-events.md). The
verifier clicks a Button and presses Enter and Space on it, flips a Toggle by
click and Space, types into a labelled Input and Textarea, and checks that
`type`, `name`, and `rows` passed to the components reach the elements. The
fixture writes each value to its article, so the checks read app state rather
than the DOM alone. Removing any callback, not spreading the attributes, or
sending the current Toggle state instead of the requested one each make the
verifier fail.

M155 adds a Slider row from
[RFC 0030](../rfcs/0030-slider-keyboard-and-pointer-input.md). The verifier
presses every slider key, checks clamping at both ends and that the page did
not scroll, then clicks and drags with the mouse, including past the right
edge while the pointer is captured. A disabled slider must ignore keys and the pointer. Swapping the key directions,
paging by one step, not preventing the default key action, not starting the
pointer script, or removing either disabled guard each make the verifier fail.

M156 adds a Collapsible and Native Select row from
[RFC 0031](../rfcs/0031-collapsible-and-native-select-events.md). The
verifier toggles the trigger by click, Enter, and Space and checks
`aria-expanded` and the content, then chooses Native Select options with
Playwright's `selectOption` and with typeahead on the focused select. It finds
the select through its `Label` and checks attributes passed to each
Collapsible part and to the select. Removing either callback, not spreading
the attributes of any of those four elements, or sending the current open
state instead of the requested one each make the verifier fail.

M157 adds an Input OTP row from
[RFC 0032](../rfcs/0032-input-otp-value-changes.md). The verifier clicks the
slots and expects the overlay input to take focus, types digits, types a
rejected letter and presses Backspace, and inserts `123-4567` the way a paste
or autofill delivers text. It checks both the app code and the native value,
because a filter that only runs in Rust leaves the letter in the input for
Backspace to remove. Removing the callback, not
spreading the attributes of either part, not starting the filter script, or a
filter script that does not cut at the length each make the verifier fail.

M158 adds a Pagination row from
[RFC 0033](../rfcs/0033-pagination-page-changes.md). The verifier changes
pages in a button Pagination by click, Enter, and Space, and checks
`aria-current` and the disabled Previous and Next at the ends. In an anchor
Pagination it checks that an enabled control keeps its `href` and passed
`target`, and that the disabled one has no `href`, cannot take focus, and
ignores a dispatched click. Compiled Tailwind gives it
`pointer-events: none`, which stops a real press, so the verifier checks that
style and dispatches the click to reach the component's own guard.
Removing the callback, always rendering an anchor, keeping `href` on a
disabled anchor, calling `onclick` while disabled, or not spreading the
attributes in either form each make the verifier fail.

M159 adds a Carousel row from
[RFC 0034](../rfcs/0034-carousel-slide-changes.md). The verifier steps a
labelled three-slide Carousel with Next, Previous, an indicator, and
ArrowLeft and ArrowRight from inside the region, and measures each slide's
offset from the 240px viewport to check that the selected slide lines up with
it and its neighbors sit one slide width away. It also checks the disabled
controls at the ends, that ArrowDown does nothing in a horizontal Carousel,
and that passed indicator labels replace the shared default. Since M168 the
slides use their own classes, so neighbors sit one slide width plus the 16px
gap away. Removing the Next or indicator callback, removing
the key step, not translating the slides, or not spreading the attributes of
the root, content, or slides each make the verifier fail. The browser applies
a spread `aria-label` after the explicit one, so keeping the default label
over a passed one only shows in SSR and is caught by a unit test instead.

M161 adds a Resizable row from
[RFC 0036](../rfcs/0036-resizable-handle-input.md). The verifier checks that
the handle between two side-by-side panels is a focusable vertical separator
with `aria-valuenow`, `aria-valuemin`, `aria-valuemax`, and a passed
`aria-controls`, resizes the panels with ArrowRight, ArrowLeft, Home, and End
within the 20% to 80% limits, and drags the handle over the 300px group: 30px
moves it 10%, a drag past the maximum stops at 80%, and moving back puts the
edge under the pointer again. Removing the key
handler, not starting the pointer script, sending per-move deltas instead of
targets, keeping the group orientation in `aria-orientation`, or not
spreading the attributes of the group, a panel, or the handle each make the
verifier fail.

M162 adds a Sidebar row from
[RFC 0037](../rfcs/0037-sidebar-toggle-and-items.md). The verifier toggles the
sidebar with a trigger click and Enter, checks `aria-expanded` and that
`aria-controls` names the `aside` landmark, changes the section with a button
item click and Space while `aria-current` follows, and checks that a link item
keeps its `href`, that a disabled link item has no `href`, cannot take focus,
and ignores a dispatched click, that a disabled button item is natively disabled,
and that an item with neither `href` nor `onclick` stays a `div`. Removing the
trigger or item callback, always rendering a wrapper, keeping `href` or
calling `onclick` on a disabled item, or not spreading the attributes of the
sidebar, the trigger, the group label, or a link item each make the verifier
fail.

M163 adds naming checks from
[RFC 0038](../rfcs/0038-form-control-naming.md). The radio group fixture
names the group with `aria-label`, one item through a `Label` with `for`, and
the others with `aria-label`, and the verifier finds each one with a named
role query. A new Progress fixture checks a progress bar named through
`aria-labelledby` with `aria-valuetext`, and the Select trigger and Combobox
input fixtures check a name and an `aria-describedby` description. Not
spreading the attributes of the radio group, a radio item, Progress, the
Select trigger, or the Combobox input each make the verifier fail.

M164 adds dialog name checks from
[RFC 0039](../rfcs/0039-dialog-names.md). The open Dialog and Popover are
named by their titles and described by their descriptions, the Alert Dialog
takes a passed `aria-label` over its title, and a closed Popover with neither
part renders neither attribute. A page-wide check then fails on any
`aria-labelledby`, `aria-describedby`, or `aria-controls` id that matches no
element, hidden content included. Not rendering the title id, pointing at a
title or description that is not mounted, ignoring a passed `aria-label`, or
not spreading the content attributes each make the verifier fail.

M165 adds composite widget name checks from
[RFC 0040](../rfcs/0040-composite-widget-names.md). The verifier finds the
tab list, toggle group, menu bar, and navigation landmark by role and their
passed `aria-label`, and the Date Picker calendar grid by the caption its
`aria-labelledby` points at. Not spreading the attributes of `TabsList`,
`ToggleGroup`, `Menubar`, `NavigationMenu`, `CalendarGrid`, or
`CalendarCaption` each make the verifier fail.

M166 adds Checkbox indeterminate checks from
[RFC 0041](../rfcs/0041-checkbox-indeterminate-state.md). A "Select all"
checkbox over two items reads the input's `indeterminate` property: mixed
for a partial selection, checked with every item after a click, and mixed
again after one item is unchecked. A checkbox that stays mixed with
`checked` set requests `true` on a click and has its property restored after
the browser clears it. Not setting the property, requesting `!checked` while
mixed, or not re-syncing after a change each make the verifier fail.

M167 adds Slider position and orientation checks from
[RFC 0042](../rfcs/0042-slider-thumb-position-and-vertical-orientation.md).
The verifier measures the volume thumb center at 0 and 100, and a vertical
Balance slider: `aria-orientation="vertical"`, a thumb centered at the value
measured from the bottom, a press near the top that sets 80, and ArrowUp.
Not positioning the thumb, mapping the
vertical pointer along `clientX`, or keeping a horizontal `aria-orientation`
each make the verifier fail.

M168 runs the verifier against compiled Tailwind from
[RFC 0043](../rfcs/0043-compiled-tailwind-browser-checks.md). The verifier
compiles `examples/web-demo/assets/preview.css` with the Tailwind Node API
and answers the page's request for the stylesheet with the result, then
checks that an `sr-only` element is absolutely positioned. The Slider, Input
OTP, Carousel, Resizable, and context menu fixtures no longer carry inline
layout styles. Serving the uncompiled stylesheet, the bare `data-disabled:`
variant, which disables every enabled Select option, or Navigation Menu
content at `top-0`, which covers its trigger, each make the verifier fail.

M169 adds rendered conflict checks from
[RFC 0044](../rfcs/0044-tailwind-utility-conflicts.md). After the first
render and again after the interactions, the verifier reads every class list
on the page and fails when two utilities set the same property under the
same variant. It also checks that the checked Radio Group item has the
border color a lone `border-blue-600` renders (`border-primary` since M177) and an unchecked item the
`border-zinc-300` color, and that the vertical Balance slider is at most
20px wide. Fixture overrides of a property a component sets use the important
modifier, such as `bg-blue-100!`. A fixture override without it, the base
Radio Group border restored, or a full-width vertical Slider each make the
verifier fail.

M170 adds drawn Checkbox checks from
[RFC 0045](../rfcs/0045-drawn-checkbox.md). The verifier checks that a
checked Checkbox has `appearance: none`, the color a lone `bg-blue-600`
renders (`bg-primary` since M177), and an SVG background image, that an unchecked one has no
background image, and that the mixed Select all checkbox has the blue fill
and an SVG mark different from the tick. Keeping the native appearance, or
removing the tick or the dash, each make the verifier fail.

M171 adds list width checks from
[RFC 0046](../rfcs/0046-listbox-width-follows-trigger.md). The verifier
checks that the open Select list is at least as wide as its trigger and the
open Combobox panel at least as wide as its input. Not setting
`--dxui-anchor-width`, or keeping `min-w-32` on either list, each make the
verifier fail.

M172 adds text contrast checks from
[RFC 0047](../rfcs/0047-opt-in-dark-theme.md). The compiled preview
stylesheet carries the opt-in `.dark` block. After the first render, with the
dialog open, and after the interactions, the verifier checks every visible
element that holds text in the light theme and again with `dark` on the root
element, once color transitions settle. Text must reach 4.5:1 against its
composited background, or 3:1 for large text; disabled and faded elements
are skipped. Under `.dark` a lone `bg-white` must render a dark surface. A
pure end-for-end scale inversion, which leaves muted text at 4.12:1, a
stylesheet without the block, or a block without the white remap each make
the verifier fail.

M173 adds a phone-width check from
[RFC 0048](../rfcs/0048-phone-width-preview-layout.md). After the
interactions, the verifier resizes the page to 375px and fails when the page
scrolls sideways or an element extends outside its fixture card without a
clipping ancestor inside the card. Removing `grid-cols-1` from a preview
grid, which makes the page 2379px wide, or the Pagination wrap, which pushes
Previous past the card's left edge, each make the verifier fail.

M175 adds a theme toggle check from
[RFC 0050](../rfcs/0050-preview-theme-toggle.md). After the first render,
the verifier presses the "Dark theme" toggle in the preview header, checks
that the preview root reports `aria-pressed`, `color-scheme: dark`, and a dark
surface, then presses it again and checks the light surface. A toggle that
does not add the `dark` class makes the verifier fail.

## Documentation Alignment

M111 should keep these files aligned:

- `package.json`
- `README.md`
- `docs/README.md`
- `docs/release.md`
- `docs/quality-gates.md`
- `docs/site.md`
- `docs/components/README.md`
- `docs/components/runtime-interaction-verification.md`
- `docs/components/browser-dom-component-verification.md`
- `docs/components/runtime-renderer-verification.md`
- `docs/browser-artifact-policy-metadata.md`

The docs should say clearly that M111 is stronger than DOM existence checks but
weaker than screenshot parity, full accessibility certification, or runtime
adapter graduation.

## Follow-up Milestones

After M111, useful follow-up work is:

1. Expand interaction coverage across more runtime-sensitive components. M135
   covers Dialog, Alert Dialog, Popover, and Tooltip; M136 adds Toast and
   Sonner; M137 adds Select and Combobox; M138 adds Date Picker; M139 adds Dropdown and Context Menu; M140 adds Menubar; M141 adds Navigation Menu; M144 adds Tabs, Radio Group, and Toggle Group; M146 adds Accordion; M147 adds Tooltip hover and focus opening; M148 adds Hover Card; M149 adds Command; M150 adds right-to-left arrow mirroring; M151 adds vertical, manually activated Tabs; M152 adds result announcements to Command and Combobox; M153 adds Switch and Checkbox change events; M154 adds Button, Toggle, Input, and Textarea events; M155 adds Slider keyboard and pointer input; M156 adds Collapsible and Native Select events; M157 adds Input OTP value changes; M158 adds Pagination page changes; M159 adds Carousel slide changes; M161 adds Resizable handle input; M162 adds Sidebar toggle and items; M163 adds form control naming; M164 adds dialog names; M165 adds composite widget names; M166 adds the Checkbox indeterminate state; M167 adds Slider thumb position and vertical orientation; M172 adds text contrast in the light and dark themes; M173 adds the phone-width layout; M175 adds the preview theme toggle.
2. Add targeted screenshot smoke for a small set of stable panels.
3. Decide whether any browser command should move into CI after local
   reliability is proven.

M177 moves the components onto the semantic color tokens from
[RFC 0051](../rfcs/0051-semantic-color-tokens.md). The Radio Group and
Checkbox color checks compare against `border-primary`, `border-input`, and
`bg-primary`, and the verifier checks that the Checkbox tick switches to the
dark stroke under `.dark`, because a data-URI mark cannot read
`--primary-foreground`. Removing the dark tick makes the verifier fail. The
contrast checks in both themes now measure token colors.
