# RFC 0021: Accordion Interaction

- Status: Accepted
- Created: 2026-10-04

## Summary

Give Accordion a root that reports toggled items, link each trigger to its
content by id, wrap triggers in headings, and move focus between triggers
with Up, Down, Home, and End. Reuse the roving group script (RFC 0019) with
a mode that keeps every trigger in the Tab order.

## Current State

As of M145:

- `AccordionItem` takes only `class`. Items have no value, so nothing can
  report which item was toggled.
- `AccordionTrigger` is a `button` with `aria-expanded` and no click
  reporting. An app has to put its own `onclick` on each trigger.
- No trigger handles arrow keys.
- Triggers are not wrapped in heading elements, so headings navigation does
  not reach them. The WAI-ARIA accordion pattern puts each trigger inside a
  heading.
- Triggers have no `aria-controls`, and content has no id, role, or
  `aria-labelledby`. `docs/components/accessibility.md` lists Accordion as
  "Needs trigger/content ARIA relationships. Planned".
- RFC 0019 deferred Accordion because its triggers are each a Tab stop,
  unlike the one Tab stop groups the roving group script serves.

## Decision

### State

State stays controlled. The app keeps the open values and passes `open` to
each trigger and content part, as today.

- `Accordion` reports `on_toggle(String)` with the value of the item the user
  toggled. This matches `ToggleGroup`, because the root does not hold the
  open values.
- `accordion_single_open(current, toggled)` returns the next open value for a
  single-open accordion. Toggling the open item closes it.
- `accordion_multiple_open(current, toggled)` adds or removes the toggled
  value.

### Components

- `Accordion` is a new root `div`. It provides a base id through context and
  runs the roving group script with:
  - vertical orientation;
  - looping on;
  - selection not following focus;
  - every enabled trigger kept as a Tab stop.
- `AccordionItem` gains a required `value: String` and provides it to its
  trigger and content through context. This is a breaking change before 1.0;
  `TabsTrigger` already requires `value` the same way.
- `AccordionTrigger` renders an `h3` with class `flex` around the button, as
  shadcn/ui does. Inside `Accordion` and `AccordionItem`, the button renders
  `id`, `aria-controls`, `data-value`, `data-state`, and the roving item
  marker.
- `AccordionContent` renders `role="region"`. Inside `Accordion` and
  `AccordionItem`, it also renders `id` and `aria-labelledby`.
- Without `Accordion`, the parts render no ids and report nothing.

The id builder that `Tabs` uses moves into the shared roving group module and
the template `utils.rs`, so Tabs and Accordion build ids the same way.

### Roving Group Script

The root gains one attribute: `data-dxui-roving-tab-stops="all"`. With it:

- the script does not set `tabindex` on items, so every enabled button stays
  in the Tab order;
- arrow, Home, and End movement and click reporting work as before.

Keyboard:

| Key | Behavior |
| --- | --- |
| ArrowDown | Focus the next enabled trigger, wrapping to the first |
| ArrowUp | Focus the previous enabled trigger, wrapping to the last |
| Home, End | Focus the first or last enabled trigger |
| Enter, Space | Toggle the item through the native button click |
| Tab | Move to the next focusable element, including every trigger |

- Arrows only move focus; they do not toggle.
- A click on an enabled trigger reports its value.
- Disabled triggers are skipped by arrows and Tab.
- Triggers inside a nested accordion belong to the nested root, as with
  nested roving groups.

## Scope

In scope:

- the `Accordion` root, `AccordionItem` value, and toggle reporting
- heading-wrapped triggers and id links between triggers and regions
- Up, Down, Home, and End movement that keeps every trigger in the Tab order
- single and multiple open-value helpers
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| An expanded item that cannot collapse (`aria-disabled` on its trigger) | The app decides whether toggling the open item closes it; the helpers close it | A consumer needs a single-open accordion that always keeps one item open |
| Heading level other than `h3` | shadcn/ui uses `h3`; the trigger's `class` styles the button, not the heading | A consumer reports a heading outline that needs another level |
| Horizontal accordions | shadcn/ui accordions are vertical | A horizontal accordion is requested |
| Open and close animation | Content uses `hidden`, which cannot animate height | Animated disclosure is designed for Collapsible and Accordion together |
| Desktop and Mobile self-test scenarios | The script is the RFC 0019 script, which uses only focus and click and already runs in the WebViews | The script relies on behavior that differs between WebViews |

## Verification

- The CLI parity test keeps the template script identical to the crate
  script.
- Unit tests cover the open-value helpers and the shared id builder.
- The Web preview renders a real Accordion, and
  `npm run verify:runtime-interactions` asserts:
  - trigger and region id links, with each region named by its trigger;
  - clicks and Enter toggling, and closing the open item;
  - arrow movement that skips the disabled trigger and wraps;
  - Home and End;
  - arrows moving focus without toggling;
  - Tab reaching every enabled trigger.
