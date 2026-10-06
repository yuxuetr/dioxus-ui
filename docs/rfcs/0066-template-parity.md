# RFC 0066: Template Parity

- Status: Accepted
- Created: 2026-10-06

## Summary

Keep the source-copy templates as hand-maintained copies of the crate
modules, and add a test that compares them item by item, so a change made to
one copy fails `cargo test` until it is made to the other. Generating the
templates from the crate is deferred until the measured cost calls for it.

## Current State

Every component exists twice: a crate module in `crates/dioxus-shadcn/src`
and a template in `crates/dioxus-shadcn-cli/templates`, which `dxui add`
copies into an app. A template cannot import the core or primitives crates
([RFC 0002](0002-cli-registry-and-code-generation.md)), so it imports
`classes` and the shared helpers from its own `utils.rs` and inlines the
primitive types it uses.

All 21 commits between `v0.1.0` and `v0.2.0` that changed a crate module also
edited `templates/`, and nothing checked that the edits matched. Comparing
the two copies after removing imports, tests, comments, and formatting,
measured on 2026-10-06:

| Comparison | Result |
| --- | --- |
| Templates identical to their module | 38 of 79 |
| Items differing, as committed | 160 |
| After formatting the templates with rustfmt | 101 |
| After also ignoring visibility, feature gates, and trailing commas, and comparing `impl` blocks per method | 97 |

The real differences were:

- **Behavior.** The Select and Combobox templates still highlighted the
  selected option with the accent background; the 0.2.0 check mark reached
  only the crate. The Calendar template's `CalendarDate::new` did not
  validate and had no `unchecked`, though the Calendar docs call it.
- **API.** 25 enums, and the two Sonner enums, derived `Default` in the
  crate but not in the templates,
  so `BadgeVariant::default()` compiled only in crate mode.
- **Stale inlined primitives.** The Resizable, Slider, Toast, and overlay
  copies lagged their primitives (a missing `collapsed` field and
  `page_step`, a missing `Copy` derive).
- **Shape.** The Sonner template renamed its copies of the toast primitives
  instead of aliasing them as the crate does.
- **Style.** The crate used `if ... && let` chains, which the templates avoid
  so they compile in edition 2021 apps.

## Decision

### The test

`crates/dioxus-shadcn-cli/tests/template_parity.rs` parses every template and
its crate module with `syn` and compares their items by name:

- a template item must equal its crate module's item, or, when the module
  has no such item, an item of the same name in the primitives crate, the
  core crate, or another crate module (the shared helpers in `utils.rs`);
- a crate module item missing from its template fails, unless `CRATE_ONLY`
  lists it with a reason, and a `CRATE_ONLY` entry that is no longer crate
  only fails too;
- an `impl` block counts as one item per method, since a template inlines
  only the primitive methods it calls.

The comparison ignores what does not change behavior or what templates differ
in on purpose: formatting, comments and doc attributes, how a string literal
is spelled, trailing commas, visibility (templates keep inlined helpers
private), and `#[cfg(feature = ...)]` gates (templates have no features). A
failure names the item and shows both copies around the first difference.

### The fixes

The crate is the reference: the templates took the crate's items and the
current primitive copies, and the Sonner template inlines the toast
primitives under their own names and aliases them with `pub use`, as the
crate does. The crate's let chains became nested `if` statements, the one
change made on the crate side. The templates are formatted with the
workspace `rustfmt.toml`, so the copies also read the same.

`CRATE_ONLY` starts with three items, all documented as crate only:
`CHART_COLOR_CLASSES` and the Combobox and Command
`*_active_descendant_state` helpers.

## Alternatives

- **Generate templates from the crate.** A generator would rewrite imports,
  drop tests, and inline the primitive items each template needs, which
  means resolving which items a module uses, transitively. The parity test
  needs only the item comparison, and catches the same drift. Re-evaluate
  generation when `CRATE_ONLY` passes 10 entries or a template needs a
  difference these rules cannot express; either means the two copies are
  diverging on purpose often enough that one source would be cheaper.
- **Compare text.** A text diff after rustfmt still breaks on the raw
  JavaScript strings in Rust files, and reports trailing commas and match
  arm braces that rustfmt places by line width.

## Verification

- The test passes on the fixed templates, and the generated fixture app,
  which compiles every template, builds.
- It fails for a changed template class string, a changed primitive whose
  inlined copy is now stale, a new crate item, and a stale `CRATE_ONLY`
  entry, and passes for a doc comment added to the crate.
