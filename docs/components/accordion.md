# Accordion

Accordion provides styled root, item, trigger, and content parts for
collapsible content sections. The root owns which items are open
([RFC 0077](../rfcs/0077-component-owned-state.md)).

## Source Copy

```bash
dxui add accordion
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["accordion"] }
```

## API Surface

- `Accordion`
- `AccordionItem`
- `AccordionTrigger`
- `AccordionContent`
- `accordion_item_class`
- `accordion_trigger_class`
- `accordion_content_class`

## Behavior

`Accordion` opens one item at a time. Start it with `default_value`, or
control it with `value`; the empty string means every item is closed.
`on_value_change` hears every change the user makes, including `""` when the
open item closes:

```rust
rsx! {
  Accordion { default_value: "shipping",
    AccordionItem { value: "shipping",
      AccordionTrigger { "Shipping" }
      AccordionContent { "Ships in two days." }
    }
    AccordionItem { value: "returns",
      AccordionTrigger { "Returns" }
      AccordionContent { "Returns within 30 days." }
    }
  }
}
```

- With `multiple: true`, any number of items open; the root takes `values`,
  `default_values`, and `on_values_change` instead.
- A click, Enter, or Space on a trigger opens its item, or closes it when
  open.
- Up and Down move focus between enabled triggers and wrap; Home and End jump
  to the first and last. Moving focus does not toggle.
- Every enabled trigger stays in the Tab order.
- `AccordionItem` takes a required `value`. Each trigger renders
  `aria-controls` and each content region renders `aria-labelledby`, with ids
  the root generates.
- `AccordionItem` must be inside `Accordion`, and a trigger or content inside
  an `AccordionItem`; otherwise the part renders nothing and logs which root
  it is missing.

The Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

Each trigger is a `button` with `aria-expanded`, wrapped in an `h3`. Content
uses `role="region"`, named by its trigger. An expanded item that cannot
collapse, other heading levels, horizontal accordions, and animation are not
implemented (see [RFC 0021](../rfcs/0021-accordion-interaction.md)).
