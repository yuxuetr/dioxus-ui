# RFC 0078: Touch Density

- Status: Accepted
- Created: 2026-10-07

## Summary

Density comes from a root, `DensityProvider`, and the interactive components
read it. Under `UiDensity::Touch` every interactive control offers at least a
44 by 44 CSS pixel target: controls that hold text grow to that height, and
controls drawn smaller keep their look and gain a centered hit area. Button's
`density` prop goes away.

## Current State

`UiDensity` (Compact, Comfortable, Touch) is public, but only `Button` takes
it, as a per-component prop, and nothing else changes with it. The design
names density the platform switch without a way to set it for a page.

Measured on 2026-10-07 in the Mobile self-test (RFC 0018) on an iPhone 17 Pro
simulator, iOS 26.3: the rendered size of every visible interactive control
in the interaction fixtures, before any scenario ran, so at defaults. 124 of
134 controls are under 44 by 44 CSS pixels, the target size of WCAG 2.5.5
and Apple's Human Interface Guidelines. Per fixture and control (sizes are
the smallest width and height among them):

| Fixture | Control | Count | Min width | Min height | Under 44x44 |
| --- | --- | --- | --- | --- | --- |
| accordion | button | 4 | 320 | 52 | 0 |
| action-parts | button | 7 | 56 | 20 | 7 |
| action-parts | combobox | 1 | 192 | 40 | 1 |
| action-parts | input | 2 | 163 | 40 | 2 |
| alert-dialog | button | 1 | 117 | 32 | 1 |
| carousel | button | 5 | 8 | 8 | 5 |
| collapsible-select | button | 1 | 320 | 20 | 1 |
| collapsible-select | select | 1 | 320 | 40 | 1 |
| combobox | combobox | 1 | 320 | 40 | 1 |
| command | combobox | 1 | 318 | 44 | 0 |
| command | option | 6 | 310 | 32 | 6 |
| date-input | input | 1 | 320 | 40 | 1 |
| date-picker | button | 1 | 320 | 40 | 1 |
| dialog | button | 1 | 103 | 32 | 1 |
| diff | input | 1 | 320 | 96 | 0 |
| disclosure | button | 1 | 140 | 32 | 1 |
| dropdown | button | 1 | 74 | 32 | 1 |
| dropdown-options | button | 1 | 56 | 32 | 1 |
| dropdown-submenu | button | 1 | 47 | 32 | 1 |
| fab | button | 1 | 56 | 56 | 0 |
| file-input | input | 1 | 320 | 40 | 1 |
| form-controls | button | 3 | 33 | 40 | 3 |
| form-controls | input | 1 | 320 | 40 | 1 |
| form-controls | textarea | 1 | 320 | 96 | 0 |
| hover-card | a | 1 | 56 | 20 | 1 |
| input-otp | input | 1 | 320 | 40 | 1 |
| keyboard | listbox | 1 | 320 | 50 | 0 |
| keyboard | option | 2 | 81 | 32 | 2 |
| menu | button | 3 | 224 | 36 | 3 |
| menubar | menuitem | 4 | 47 | 32 | 4 |
| multi-combobox | combobox | 1 | 320 | 40 | 1 |
| multi-select | combobox | 1 | 320 | 40 | 1 |
| navigation-menu | a | 1 | 53 | 38 | 1 |
| navigation-menu | button | 3 | 66 | 40 | 3 |
| navigation-submenu | button | 1 | 94 | 40 | 1 |
| number-input | button | 2 | 40 | 38 | 2 |
| number-input | spinbutton | 1 | 78 | 38 | 1 |
| overlay | button | 1 | 111 | 32 | 1 |
| pagination | a | 2 | 40 | 40 | 2 |
| pagination | button | 7 | 40 | 40 | 7 |
| popover | button | 1 | 126 | 32 | 1 |
| radio-group | radio | 4 | 16 | 16 | 4 |
| range-slider | slider | 2 | 20 | 20 | 2 |
| rating | input | 5 | 24 | 24 | 5 |
| resizable | separator | 1 | 1 | 80 | 1 |
| scroll-status | button | 1 | 118 | 32 | 1 |
| select | combobox | 1 | 320 | 40 | 1 |
| selection | radio | 2 | 54 | 32 | 2 |
| sidebar | a | 1 | 239 | 36 | 1 |
| sidebar | button | 4 | 36 | 36 | 4 |
| sidebar-mobile | button | 1 | 36 | 36 | 1 |
| slider | slider | 3 | 20 | 8 | 3 |
| sonner | button | 1 | 109 | 32 | 1 |
| swap | button | 1 | 36 | 36 | 1 |
| switch | input | 6 | 16 | 16 | 6 |
| switch | switch | 2 | 44 | 24 | 2 |
| tabs | tab | 4 | 60 | 32 | 4 |
| tabs | tabpanel | 1 | 320 | 24 | 1 |
| tabs-vertical | tab | 3 | 108 | 32 | 3 |
| tabs-vertical | tabpanel | 1 | 119 | 104 | 0 |
| tags-input | input | 1 | 302 | 24 | 1 |
| theme-controller | button | 5 | 55 | 32 | 5 |
| toast | button | 1 | 98 | 32 | 1 |
| toggle-group | button | 4 | 54 | 36 | 4 |
| tooltip | button | 1 | 132 | 32 | 1 |

The largest gaps are the controls drawn small: carousel indicators (8 by 8),
radio items and checkboxes (16 by 16), slider thumbs (20 by 20), rating stars
(24 by 24), the resizable handle (1 pixel wide), and the switch (44 by 24).
Controls that hold text sit at 32 to 40 pixels high.

## Decision

TODOs M211.1 set the rule: if any control is under 44 by 44, density comes
from a root context and the interactive components take it.

- `DensityProvider { density, children }` provides a `UiDensity`;
  `use_density()` reads it and returns `Comfortable` without a provider. A
  provider is optional, unlike an RFC 0077 root: a component without one
  renders at today's sizes.
- `UiDensity::control_class()` is the Touch floor for controls that hold
  text, `min-h-11`, plus `min-w-11` so icon buttons stay square. Buttons,
  inputs, triggers, tabs, toggles, menu, command, and select items,
  pagination links, and the like append it; they grow as iOS controls do.
- `UiDensity::hit_area_class()` is the Touch hit area for controls drawn
  smaller than a target: an `after:` pseudo-element, 44 by 44 and centered,
  that takes the press without changing the look. Checkbox, radio items, the
  switch, slider thumbs, carousel indicators, rating stars, and the
  resizable handle append it.
- Compact keeps Button's `min-h-8`; other components have no smaller size
  yet, since shadcn/ui's defaults are already compact. It waits for an issue
  that asks for denser controls.
- `Button` loses its `density` prop and reads the provider. `button_class`
  keeps its density argument for elements styled as buttons, such as the
  RFC 0077 triggers: `button_class(variant, size, use_density(), "")`.
- Where a component's own box is not the target, the parts inside take the
  floor: the Number Input buttons and field, the Tags Input field (its remove
  buttons get a hit area), and the root of a Slider, which a press anywhere
  on moves. A Native Select takes `h-11` instead, since WebKit keeps a native
  select's height over a `min-height`. The Resizable handle widens its
  existing `after:` strip to 44 pixels across the line, and Rating wraps each
  star in a `label`, so its hit area presses the radio.
- An element the app renders, such as the input inside an `InputGroup`,
  takes `density_control_class(use_density())` in its own class.
- The Mobile preview renders under `Touch`, and the Mobile self-test fails
  when any measured control's target, its box or its hit area (or its
  label's), is under 44 by 44. A link inside a line of text is exempt, as in
  WCAG 2.5.8, and a tab panel is not a press target.

## Alternatives

| Option | Why not |
| --- | --- |
| Remove `UiDensity` | The measurement shows the defaults miss the touch target on a phone |
| A `density` prop on every interactive component | Every app repeats it on every control, the wiring RFC 0077 removed for state |
| Grow every control to 44 by 44 under Touch | A 44 pixel checkbox or radio no longer looks like one; a hit area keeps the look |
| Pick density from the viewport automatically | A narrow desktop window is not a touch screen; the app knows its platform |

## Impact

- Breaking for apps that pass `density` to `Button`: wrap the app, or the
  part of it, in `DensityProvider` instead.
- Copy mode gains a `density` helper that the interactive components import.

Hit areas of neighbors can overlap, for example the stars of a Rating or a
column of radio items with a small gap; a press in the overlap goes to the
later control. A pseudo-element on a native `input`, which Checkbox and
Rating's radios are, renders in WebKit and Blink, the engines of the iOS and
Android WebViews, but not in Firefox.

## Validation

M211.1 is done when this record holds the table above, the components named
here append the density classes in crate and templates with tests, and the
Mobile self-test under Touch reports no control under 44 by 44.

Measured on 2026-10-07 on the same simulator after the change: 0 of 131
controls under 44 by 44 under `Touch` (one fewer control, since the hover
card link now sits in a line of text). The gate went red while components
were still missing their class: 52, then 17, then 1 control under the
target.
