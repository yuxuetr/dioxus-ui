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
| Avatar | Image `alt` text is provided by the consuming app. | Implemented |
| Badge | Text must communicate state, not color alone. | Planned |
| Card | Does not add implicit landmark or interactive semantics. | Implemented |
| Separator | Supports decorative and semantic separator usage. | Implemented |
| Skeleton | Hidden from assistive technology by default. | Implemented |

## Form Basics

| Component | Contract | Status |
| --- | --- | --- |
| Button | Supports disabled state and native button semantics. | Implemented |
| Checkbox | Uses native checkbox input state. | Implemented |
| Input | Supports `aria-invalid` for invalid state. | Implemented |
| Label | Can be associated with a form control by the app. | Implemented |
| Switch | Needs explicit switch semantics before stability. | Planned |
| Textarea | Supports `aria-invalid` for invalid state. | Implemented |

## Disclosure And Selection

| Component | Contract | Status |
| --- | --- | --- |
| Accordion | Needs trigger/content ARIA relationships. | Planned |
| Tabs | Needs tablist, tab, and tabpanel roles. | Planned |
| Select | Needs listbox semantics and keyboard navigation. | Planned |

## Overlays

| Component | Contract | Status |
| --- | --- | --- |
| Dialog | Exposes dialog role and modal state. | Implemented |
| Dialog | Needs focus trap and focus return verification. | Planned |
| Dropdown | Needs menu roles, roving focus, and typeahead. | Planned |
| Popover | Needs dismissal and focus behavior verification. | Planned |
| Tooltip | Should be discoverable by hover and focus. | Planned |

## Data And Navigation

| Component | Contract | Status |
| --- | --- | --- |
| Pagination | Uses navigation region and current-page state. | Implemented |
| Progress | Uses progressbar value attributes. | Implemented |
| Table | Uses semantic table elements. | Implemented |

## Complex Component Gates

Before complex interaction components are marked stable:

- Keyboard behavior must be documented.
- Role and ARIA attributes must be listed in component docs.
- Focus entry, movement, and return behavior must be tested or explicitly
  deferred.
- Web and Desktop behavior must be checked separately when portals or DOM focus
  are involved.
- Mobile behavior must not rely on hover-only interaction.
