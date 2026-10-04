# Accordion

Accordion provides controlled styled root, item, trigger, and content parts
for collapsible content sections.

## Source Copy

```bash
dxui add accordion
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["accordion"] }
```

## API Surface

- `Accordion`
- `AccordionItem`
- `AccordionTrigger`
- `AccordionContent`
- `accordion_item_class`
- `accordion_trigger_class`
- `accordion_content_class`
- `accordion_single_open`
- `accordion_multiple_open`

## Behavior

The open items stay controlled by the app. Keep the open value, pass `open` to
each trigger and content part, and handle `Accordion` `on_toggle`, which
reports the toggled item's value:

```rust
let mut open = use_signal(|| None::<String>);
let is_open = move |value: &str| open().as_deref() == Some(value);

rsx! {
  Accordion {
    on_toggle: move |value: String| open.set(accordion_single_open(open().as_deref(), &value)),
    AccordionItem { value: "shipping",
      AccordionTrigger { open: is_open("shipping"), "Shipping" }
      AccordionContent { open: is_open("shipping"), "Ships in two days." }
    }
    AccordionItem { value: "returns",
      AccordionTrigger { open: is_open("returns"), "Returns" }
      AccordionContent { open: is_open("returns"), "Returns within 30 days." }
    }
  }
}
```

- `accordion_single_open` keeps at most one item open; toggling the open item
  closes it. `accordion_multiple_open` adds or removes the toggled value from a
  list.
- A click, Enter, or Space on a trigger calls `on_toggle` with its item's
  value.
- Up and Down move focus between enabled triggers and wrap; Home and End jump
  to the first and last. Moving focus does not toggle.
- Every enabled trigger stays in the Tab order.
- `AccordionItem` takes a required `value`. Inside `Accordion`, each trigger
  renders `aria-controls` and each content region renders `aria-labelledby`.
  Without `Accordion`, the parts render no ids and report nothing.

The Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

Each trigger is a `button` with `aria-expanded`, wrapped in an `h3`. Content
uses `role="region"`, named by its trigger. An expanded item that cannot
collapse, other heading levels, horizontal accordions, and animation are not
implemented (see [RFC 0021](../rfcs/0021-accordion-interaction.md)).
