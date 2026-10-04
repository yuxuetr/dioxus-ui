# RFC 0026: Tabs Manual Activation And Vertical Orientation

- Status: Accepted
- Created: 2026-10-04

## Summary

Give `Tabs` two props:

- `activation`: automatic (the default) or manual. With manual activation,
  arrow keys move focus without selecting, and Enter, Space, or a click
  selects.
- `orientation`: horizontal (the default) or vertical. A vertical list uses
  Up and Down and sits beside the panels.

When focus leaves the list, the Tab stop goes back to the selected trigger.

## Current State

As of M150:

- `TabsList` always sets `data-dxui-roving-activation="focus"`, so every
  arrow, Home, or End key selects the trigger it moves to (RFC 0019). An app
  whose panels are slow to render cannot let the user browse triggers first.
- `TabsList` always renders `aria-orientation="horizontal"` and
  `data-dxui-roving-orientation="horizontal"`, so a list laid out as a column
  still moves with Left and Right and announces itself as horizontal.
- The roving group script picks the Tab stop as the selected item when
  selection follows focus, and otherwise the last focused item. It recomputes
  only after a DOM change while focus is outside the group, so leaving a
  group does not move the Tab stop by itself.
- `docs/components/accessibility.md` lists manual activation and vertical
  tabs as Planned for Tabs.
- WAI-ARIA APG describes both activation modes and vertical tablists, and
  says focus entering the tablist goes to the active tab. Radix Tabs has
  `activationMode` and `orientation` props on its root.

## Decision

### API

```rust
Tabs {
  activation: TabsActivation::Manual,
  orientation: TabsOrientation::Vertical,
  on_value_change: move |value: String| tab.set(value),
  TabsList {
    TabsTrigger { value: "general", active: tab() == "general", "General" }
    TabsTrigger { value: "security", active: tab() == "security", "Security" }
  }
  TabsContent { value: "general", active: tab() == "general", "General settings" }
  TabsContent { value: "security", active: tab() == "security", "Security settings" }
}
```

- `TabsActivation { Automatic, Manual }` and
  `TabsOrientation { Horizontal, Vertical }` are two-variant enums.
  `NavigationOrientation` was considered, but its `Both` value has no
  meaning for a tablist.
- `Tabs` passes both through its context. `TabsList` reads them, and without
  `Tabs` it stays horizontal and automatic.
- `Tabs`, `TabsList`, and `TabsContent` render `data-orientation`.
  `TabsList` renders the matching `aria-orientation`.

### Keys And Selection

| Setting | Behavior |
| --- | --- |
| Horizontal | Left and Right move; Up and Down do nothing |
| Vertical | Up and Down move; Left and Right do nothing |
| Automatic | Moving focus selects the focused trigger |
| Manual | Moving focus does not select; Enter, Space, or a click selects |

Home and End jump in both orientations, and the list wraps as before.

### Tab Stop

The script sets `data-dxui-roving-activation` to `focus` for automatic and
`manual` for manual tabs. Toggle Group sets neither.

- Groups whose items carry a selection (`focus` or `manual`) prefer the
  selected item as the Tab stop. Toggle Group keeps preferring the last
  focused item.
- When focus leaves the group, the script recomputes the Tab stop. For
  manual tabs, Tab from an unselected trigger moves on to the panel, and
  Shift+Tab back into the list lands on the selected trigger.

The browser picks the next focus target before `focusout`, so the recompute
cannot change where Tab goes from the current trigger.

### Layout

The list base class gains `data-[orientation=vertical]` variants that make it
a full-height column. `Tabs` gains a base class that places the list and the
panels side by side when vertical, and `TabsContent` drops its top margin.

## Scope

In scope:

- the `activation` and `orientation` props, their enums, and data attributes
- the Tab stop rule in the roving group script, in the crate and the template
- the Tabs docs page
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Props on `TabsList` alone | Radix and shadcn put them on the root, and the panels need the orientation too | A consumer renders `TabsList` without `Tabs` and needs vertical keys |
| Measuring the vertical layout in the browser | The Web preview serves `preview.css` without running Tailwind, so no utility classes apply there | The preview harness compiles Tailwind |
| Desktop and Mobile self-test scenarios | The roving group script already runs in the WebViews for other groups | The script relies on behavior that differs between WebViews |

## Verification

- Unit tests cover the attribute values for both enums.
- The CLI parity test keeps the template script identical to the crate
  script.
- The Web preview renders a vertical, manually activated Tabs, and
  `npm run verify:runtime-interactions` asserts:
  - `aria-orientation="vertical"` and `data-orientation="vertical"` on the
    root, list, and panels;
  - Up and Down move focus and Left and Right do not;
  - moving focus does not select;
  - Enter, Space, and a click select;
  - after focus leaves from an unselected trigger, the selected trigger is
    the Tab stop.
- Reverse checks: selecting on focus, keeping the last focused trigger as
  the Tab stop, horizontal keys on the vertical list, or a horizontal
  `aria-orientation` each make the verifier fail.
