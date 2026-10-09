# Tree

Tree shows nested items, such as folders and files, whose branches open and
close, with the keyboard model of the WAI-ARIA tree pattern.

## Source Copy

```bash
dxui add tree
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["tree"] }
```

## API Surface

- `Tree`
- `TreeItem`
- `tree_class`
- `tree_group_class`
- `tree_item_row_class`

## Behavior

`Tree` owns which branches are open and which item is selected
([RFC 0077](../rfcs/0077-component-owned-state.md)). `default_expanded` and
`default_selected` start them; `expanded` and `selected` control them.
`on_expanded_change` hears the open branches after each change and
`on_selected_change` the selected value.

A `TreeItem`'s children are its label. Its `group` holds its child items,
which make it a branch:

```rust
rsx! {
  Tree { "aria-label": "Project files", default_expanded: vec!["src".to_string()],
    TreeItem {
      value: "src",
      group: rsx! {
        TreeItem { value: "main", "main.rs" }
      },
      "src"
    }
    TreeItem { value: "cargo", "Cargo.toml" }
  }
}
```

- The visible items form one Tab stop: the selected item, or the one that
  last had focus, or the first.
- Up and Down move between visible items, and Home and End go to the first
  and last. Typing moves to the next item whose label starts with the typed
  text; repeating a letter steps through the items that start with it.
- Right opens a closed branch, or moves into an open one. Left closes an open
  branch, or moves to the parent. In a right-to-left layout the two swap.
- Enter or Space selects the focused item. A click selects an item and opens
  or closes a branch.
- A disabled item is skipped by the keys and cannot be selected.
- Each level indents its rows by 1rem; `class` styles an item's row and
  `group_class` its group.
- A `TreeItem` outside a `Tree` renders nothing and logs which root it is
  missing.

The Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

`Tree` renders `role="tree"` and each item `role="treeitem"` with
`aria-level`, `aria-selected`, and, on a branch, `aria-expanded`; a branch's
children sit in a `role="group"` that is `hidden` while it is closed. The
item element takes focus, and its row shows the focus ring and the selection.
Name the tree with `aria-label` or `aria-labelledby` (see
[RFC 0040](../rfcs/0040-composite-widget-names.md)). Selecting several items
and `*` to open sibling branches are not implemented.
