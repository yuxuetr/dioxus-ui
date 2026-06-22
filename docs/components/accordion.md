# Accordion

Accordion provides controlled styled parts for collapsible content sections.

## Source Copy

```bash
dxui add accordion
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["accordion"] }
```

## API Surface

- `AccordionItem`
- `AccordionTrigger`
- `AccordionContent`
- `accordion_item_class`
- `accordion_trigger_class`
- `accordion_content_class`

## Accessibility Notes

Use button semantics for triggers and keep expanded state controlled by the
application. Content should remain associated with its trigger through stable
IDs once full ARIA wiring is added.
