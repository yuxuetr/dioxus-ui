# RFC 0044: Tailwind Utility Conflicts

- Status: Accepted
- Created: 2026-10-05

## Summary

Stop class functions from joining utilities that set the same CSS property.
Each contested utility moves out of the base class into every branch of the
state that replaces it. A static check covers the class functions, a browser
check covers every rendered class list, and the docs name the important
modifier for user overrides.

## Current State

As of M168:

- Class functions join an always-applied base class with state classes.
  Many states set a property the base already sets: the invalid
  `border-red-500` of Input, Textarea, Native Select, Select, Combobox, Date
  Picker, Input Group, and Input OTP over a base `border-zinc-200`; the
  checked `border-blue-600` of Checkbox and Radio Group over
  `border-zinc-300`; Toast, Sonner, Alert, and Attachment variant colors over
  base colors; the open Navigation Menu trigger, the selected Calendar day
  hover, the destructive menu item focus, the sorted table header, the
  collapsed Sidebar width, and the vertical Toggle Group alignment.
- Tailwind orders utilities in the stylesheet by property and name, not by
  their position in the class list. With compiled CSS a later
  `border-zinc-200` beats `border-red-500` and `border-blue-600`, so invalid
  fields and checked radios keep their gray border, and Toast and Sonner
  variants keep the default text color.
- The vertical Slider adds `w-auto` and `w-2` to a root and track whose base
  classes have `w-full`; `w-full` wins, and the vertical slider renders as a
  wide blob.
- The docs say user classes are appended last, which reads as if they win.
  They lose the same way when they set a property the base class sets.

## Decision

- A class function never joins two utilities that set the same property
  under the same variant. The contested utility moves out of the base class
  into the default branch, such as `border-zinc-200 focus-visible:ring-blue-600`
  for a valid field. The rendered attributes do not change.
- Input OTP slots pick one border: invalid, then active, then the default.
- The Slider base classes drop their size utilities; the orientation adds
  them, and a vertical root is `h-full w-5 flex-col`, the thumb's width.
- `scripts/preview-tailwind.mjs` gains `utilityConflicts`, which compiles each
  utility on its own with the Tailwind Node API and reports pairs that set
  the same property, custom properties included, under the same variant.
  Declarations that read a `--tw-*` variable compose by design and do not
  count, nor does an important utility paired with a plain one.
- `npm run verify:tailwind-conflicts`, part of the release gate, pairs each
  base class in a class function with every class the function can add:
  other constants, literals, and the literals and constants of the helpers
  and methods it calls, with methods resolved through the parameter type.
- `npm run verify:runtime-interactions` fails when any rendered class list
  holds a conflict, once after the first render and once after the
  interactions, which opens menus and dialogs and changes states.
- `docs/component-api.md` says that an appended class does not win by
  position and names the important modifier, such as `bg-blue-100!`, for
  overriding a property the component sets. The preview fixtures use it.

## Scope

In scope:

- the class fixes in the crate and the templates
- the static and rendered conflict checks
- the fixture overrides, and measured checks of the Radio Group checked
  border and the vertical Slider width
- the `class` rules in the component API docs

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Merging user classes into the component classes | Needs a Tailwind-aware merge table in Rust | Consumers report that the important modifier is not enough |
| The native Checkbox appearance | `Checkbox` renders `<input type="checkbox">` with native appearance, so Chrome ignores its border and background classes; a custom look needs `appearance-none` and a drawn check | A consumer needs the Checkbox to follow the theme |
| Conflicts between two state classes, such as an outside-month day that is also selected | Each pair is a design choice about which state shows | A combined state renders wrong in a reported case |

## Verification

- `npm run verify:tailwind-conflicts` passes on the fixed classes and fails
  when the base invalid border returns to `INPUT_BASE_CLASS`.
- Unit tests keep the class functions' state classes, and the Input OTP test
  checks that invalid replaces active.
- `npm run verify:runtime-interactions` finds no conflict in any rendered
  class list, checks that the checked Radio Group item has the
  `border-blue-600` color and an unchecked one the `border-zinc-300` color,
  and that the vertical Slider is at most 20px wide.
- Reverse checks: the base Radio border restored, a fixture override without
  the important modifier, or a full-width vertical Slider each make the
  verifier fail.
