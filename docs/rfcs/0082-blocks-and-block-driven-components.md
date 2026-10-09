# RFC 0082: Blocks and Block-Driven Components

- Status: Accepted
- Created: 2026-10-09

## Summary

Add eight blocks, designed here from the library's own components, and add a
component only when a block needs one that a block should not carry itself.
Five blocks need no new component and ship as 0.6.4. Three more drive a
Tree component and two decisions measured on the blocks: shared form errors
and a file drop area. They ship as 0.6.5. Neither release waits for Dioxus
0.8.

## Current State

Read at `v0.6.3`:

- **83 components, 3 blocks.** The blocks are `dashboard`, `login`, and
  `settings` ([RFC 0073](0073-blocks.md)). A block is the shortest way from
  `dxui add` to a working screen, and building one is how the components get
  tested together: the dashboard found the off-canvas sidebar height bug
  (`ec6a0d0`) and a reversed chart range (`b5847ff`).
- **The components cover shadcn/ui except Form.** Form state and validation
  is in Deferred. Against daisyUI, the missing parts (hero, footer, navbar,
  stack, mask, link, filter) are class strings with no behavior and stay out.
  Against Ant Design, the missing parts with behavior are Tree, TreeSelect,
  Cascader, Transfer, TimePicker, ColorPicker, Tour, Mentions, and a file
  drop area. Nothing shows a user needs them: the repository has no outside
  issue.
- **Hand-written form errors.** `login` and `settings` each keep a
  "submitted" signal, one `Option<&str>` error per field, the field's
  `invalid` and `aria-describedby`, and a `FieldError`. Two blocks are too few
  to tell what a shared piece should look like.
- **Files can be dropped without a script.** `DragData` in Dioxus 0.7
  implements `HasFileData`, so an `ondrop` handler reads the dropped files.
  `FileInput` styles the native picker and has no drop area.
- **The scope rule.** The TODO plan and the roadmap allow no new component or
  block without an issue that asks for one. The release owner changed this
  on 2026-10-09: add the blocks below, and the components they need.

## Decision

### Sources

Blocks and components are designed from this library's components and from
openly licensed references such as shadcn/ui and daisyUI (both MIT). No
markup, class lists, or assets come from commercial kits whose licenses
forbid using them in a library or template others build from (Tailwind Plus,
Catalyst, Preline Pro, Refactoring UI). A screen's type, such as an inbox or
a pricing page, is a common idea; its layout here is our own.

### Blocks

Each block follows RFC 0073: one file, a `<Name>Block` component, sample
data in signals, and callbacks for what an app connects.

| Block | Screen | Components | Needs |
| --- | --- | --- | --- |
| `signup` | Account creation: name, email, password with a strength meter, terms | Card, Field, Input, Checkbox, Progress, Button | nothing new |
| `pricing` | Plans with a monthly or yearly toggle, a highlighted plan, and a feature list | Card, Badge, ToggleGroup, Button, Separator | nothing new |
| `landing` | Product page: hero, features, a testimonial, a call to action, and a footer | Button, Badge, Card, Avatar, Separator | nothing new |
| `inbox` | Mail: folder list, message list with search, and a reading pane | Resizable, Item, Input, Badge, Avatar, ScrollArea, Separator, Button | nothing new |
| `chat` | Conversation list, messages, attachments, and a composer that takes dropped files | Item, Avatar, Bubble, Message, MessageScroller, Attachment, Textarea, Button | nothing new |
| `files` | Folder tree, file table with breadcrumbs, row menu, and an upload area | Tree, DataTable, Breadcrumb, ContextMenu, Attachment, Button | **Tree** |
| `schedule` | Week view with events, a month picker, and an event dialog | Calendar, Dialog, Field, Input, DatePicker, Button, Badge | nothing new |
| `checkout` | Address and payment form, shipping choice, and an order summary | Card, Field, Input, NativeSelect, RadioGroup, Separator, Button | nothing new |

Hero and footer stay part of the `landing` block, not components: they are
layout an app changes on every page.

### When a block adds a component

A block gets a new component when it needs either of these:

- **Behavior a block should not own.** Tree is the one case among the eight
  blocks. The ARIA tree pattern has roving focus, ArrowLeft and ArrowRight to
  collapse and expand, typeahead, and `aria-expanded`, `aria-level`, and
  `aria-selected` on each item. Written inside the `files` block, every app
  that adds it would keep a copy of that keyboard code.
- **A piece that two or more blocks write the same way.** This is measured
  once the blocks exist, not guessed before. Two candidates are known.
  - **Form errors**: `login`, `settings`, `signup`, and `checkout` will
    write them.
  - **A file drop area**: `chat` and `files` will write one.

Not added:

- **TimePicker.** The `schedule` block uses `Input` with `type="time"`, which
  every target browser and webview renders with its own picker and keyboard.
  Re-evaluate when an issue asks for a styled time picker.
- **TreeSelect, Cascader, Transfer, ColorPicker, Tour, and Mentions.** No
  block needs them; they stay in Deferred under the issue rule.

### Tree

`Tree`, `TreeItem`, and `TreeGroup` in the styled crate and a CLI template.
The parts:

- **Roles.** `role="tree"` on the root, `treeitem` items, and nested
  `group`s.
- **State.** The app owns the expanded and selected ids, the same way Select
  and Tabs take a value and a change callback
  ([RFC 0077](0077-component-owned-state.md)).
- **Keyboard.**
  - Up and Down move between visible items; Home and End go to the first and
    last.
  - Right expands a closed item, then moves to its first child. Left
    collapses an open item, then moves to its parent.
  - Enter selects; typing moves to the next item that starts with the typed
    text.
- **Focus.** One item is in the tab order (`tabindex="0"`), the rest `-1`.

The keys run in the existing page-script path (`script.rs`, no eval), as
roving focus does. The styled crate builds it on the primitives' roving
focus where that fits, and adds a tree primitive only for what roving focus
cannot express (levels and parent moves).

### Lifting a repeated piece

Each candidate gets one task after its blocks exist:

1. **Count.** For each block, count the lines that only wire the piece. For
   form errors these are the submitted signal, the error `let`s, `invalid`,
   `aria-describedby`, and the `FieldError` branch. For the drop area they
   are the drag handlers, the drag-over state, and the file read.
2. **Lift or defer.** If one small component or function removes at least
   half of those lines across the blocks, with no block losing behavior, it
   lands with the blocks rewritten on it. Otherwise the candidate goes back
   to Deferred, recorded with the numbers.

A Form framework (field registration, schema validation, async rules) is not
in scope either way.

### Releases

- **Version numbers.** Blocks and an added component are compatible changes.
  Before 1.0, Cargo treats a change of the last version number as
  compatible, so they ship as 0.6.4 and 0.6.5. That keeps 0.7.0 for the
  Dioxus 0.8 move (Stage 14). `verify:semver` against the previous tag must
  pass at each release.
- **If Dioxus 0.8 is released first.** If Stage 14's gate opens during these
  milestones, the task in progress finishes and M216 goes next. Any remaining
  tasks then ship as 0.7.x.
- **Tree and 1.x.** A component added before the `rc` period joins the 1.x
  promise.

## Alternatives

- **Port the commercial kits.** Their licenses forbid it, whether paid for or
  not (see Sources).
- **Add the missing Ant Design components first.** Without a screen that
  uses them, nothing tests whether their API fits an app, and each one adds
  public API to keep through 1.x.
- **Build Form now.** Two blocks show one pattern, not what varies. The
  four blocks of these milestones measure it first.
- **Add hero, footer, and navbar components.** They would be class strings
  with props for every variation; a block shows the layout and the app edits
  it.

## Validation

- **Every block:**
  - Registry entry and source pass the block registry tests: the source
    exists, targets `src/blocks/`, names only known components, and imports
    no internal crate.
  - The generated fixture adds and compiles it.
  - It has a docs page, a row in the blocks table, and a site page.
  - `site-verify` runs one of its interactions:
    - signup: a weak password shows its meter and blocks submit;
    - pricing: the yearly toggle changes the prices;
    - landing: the call to action is reachable by keyboard;
    - inbox: picking a message shows it in the pane;
    - chat: sending adds a message;
    - files: a folder in the tree filters the table;
    - schedule: a new event appears in its day;
    - checkout: a missing field blocks submit with its error.
- **Tree:**
  - Unit tests for the roles and levels.
  - A browser fixture walks the keys above and checks focus,
    `aria-expanded`, and the selection callback.
  - Template parity, `verify:csp`, and Clippy pass.
- **Each lift task** records its counts in the TODO note, whichever way it
  decides.
- **Each release:** the release gate, `verify:semver` against the previous
  tag, a publish dry run, CI, and fresh apps from crates.io in both modes
  that add a new block.
