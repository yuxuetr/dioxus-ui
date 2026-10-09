# RFC 0081: Overlay and Listbox Fixes

- Status: Accepted
- Created: 2026-10-09

## Summary

Fix four defects that an app on 0.6.2 reported (FB-17 to FB-21) and that the
source confirms: anchored content measured while still in the page flow,
Command inside a hidden container, Command results that arrive late, and
the default density stretching small buttons. Document that Navigation Menu
panels need hydration. All four are fixes of existing behavior; the release
is a patch (0.6.3).

## Current State

Read at `v0.6.2`:

- **FB-19, anchored overlay offset.** No content base class (Dropdown,
  Popover, Select, Combobox, Context Menu, Date Picker, and the rest) sets a
  position, so content opens in normal flow. `place()` in
  `anchored_overlay.rs` reads `anchorRect()` first and only then sets
  `position: fixed`. A trigger in a flex row with its content as a sibling is
  pushed aside by the content while it is measured, and the content is placed
  against the pushed position, then the trigger moves back.
- **FB-17, Command in a Dialog.** Dialog, Sheet, Drawer, Popover, Tabs, and
  Collapsible hide closed content with `hidden` and keep it mounted. Command
  calls `use_listbox(true, …)`, so its script starts on mount, sees the
  hidden ancestor (`closed()`), and returns; `was_open` is already true, so
  the effect never starts it again. If the container is open at mount, the
  first close ends the script the same way. Arrow keys and Enter then do
  nothing and `aria-activedescendant` stays empty.
- **FB-18, late Command results.** The query's `input` sets `resetPending`;
  the next DOM change, usually the input's own re-render, consumes it. Results
  that arrive later from a debounced server request find no highlight and
  nothing restores one. The same happens when the highlighted option is
  removed while the list is empty.
- **FB-21, Sm button height.** `button_class` adds `min-h-10` at the default
  `Comfortable` density whatever the size, so `ButtonSize::Sm` (documented as
  32 pixels, `h-8`) renders 40 pixels high.
- **FB-20, Navigation Menu before hydration.** Panels render `hidden` until
  the app's state opens them, so they cannot open before the Wasm loads.
- **FB-13, site title.** The site renders `document::Title`, which Dioxus Web
  sets through eval, though `site/index.html` already has the title.

## Decision

- **FB-19.** `place()` sets `position: fixed` and `margin: 0` before it reads
  the anchor. It still sets `--dxui-anchor-width` from that reading. The one
  frame between mount and the first placement keeps the content in flow;
  making every content base class `fixed` would remove it but touches ten
  components and their user overrides, so it waits for a report of that
  frame.
- **FB-17.** A Command listbox is always open, so its script ends only when
  the listbox leaves the page, not when an ancestor hides it. While hidden it
  keeps no highlight; when it becomes visible it highlights the initial
  option. Its key listener stays on the input throughout: a hidden input
  cannot take focus, so it receives no keys. Select, Combobox, and menus,
  whose Rust side reopens them, keep ending on hide. No `CommandDialog`
  component: the fix also covers Command in a Sheet, a Popover, or a tab.
- **FB-18.** In Command mode, a DOM change that finds options and no
  highlight highlights the initial option. Combobox, which starts without a
  highlight on purpose, is unaffected.
- **FB-21.** `Comfortable` adds no minimum height: every size already sets
  its height, `Md`, `Lg`, and `Icon` at 40 pixels or more, so only `Sm`
  changes, back to 32 pixels. `Compact` and `Touch` are unchanged.
- **FB-20.** Documentation only. A CSS-driven panel (`group-hover:block`)
  before hydration would fight the state-driven one after it: a hovered panel
  ignores Escape and shows while `aria-expanded` is false. The component page
  says the panels need hydration and that top-level links work without it.
- **FB-13.** The site drops `document::Title`. The cause, `WebDocument::
  set_title` calling eval in `dioxus-web`, belongs upstream; an issue for
  DioxusLabs is drafted for the release owner to post.

Each fix lands in the crate and the CLI template together.

## Validation

- Browser checks, each failing before its fix:
  - a Dropdown trigger in a flex row with its content as a sibling: after
    opening, the trigger has not moved, and the content's right edge sits at
    the trigger's (end alignment);
  - Command in a Dialog: open, press ArrowDown, find `aria-activedescendant`
    set; close, reopen, and find it again;
  - Command with results that arrive after a delay: the first result is
    highlighted without a key press.
- Unit test: `Sm` at `Comfortable` has no `min-h-10`, `Md` keeps its 40
  pixels.
- `verify:csp`, the template parity test, Clippy, the fixture smoke,
  `verify:semver` against `v0.6.2`, and the release gate pass.
