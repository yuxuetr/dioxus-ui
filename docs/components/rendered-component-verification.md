# Rendered Component Verification

This document defines the M109 plan for moving from representative preview
states to rendered coverage for every public shadcn/ui-aligned component.

Status: Metadata gate and shared preview targets implemented in M109.2-M109.3.

## Problem

The project now has complete component wiring for the current shadcn/ui
component catalog:

- registry entries
- source-copy templates
- crate features
- styled crate modules
- component docs
- source preview metadata

That proves every component is available through project entry points. It does
not prove every component appears in a rendered preview surface or that future
browser checks can locate a deterministic DOM target for every component.

## Decision

M109 should add rendered component coverage in layers:

1. Define a static rendered coverage manifest for all public components.
2. Verify the manifest stays aligned with the docs catalog.
3. Add deterministic preview target ids for catalog coverage.
4. Expand Web/Desktop preview checks to assert the coverage targets exist.
5. Keep full screenshot pixel assertions as a later, narrower milestone.

The first gate is metadata-first and read-only. It does not require a browser,
start a dev server, write screenshots, or claim visual parity.

## Coverage Contract

Every public component from the docs catalog should have a rendered coverage
record with:

- component slug
- display label
- category
- preview panel
- stable test id
- coverage level
- notes for app-owned or runtime-owned behavior

Coverage levels should be explicit:

| Level | Meaning |
| --- | --- |
| `static` | Pure rendered markup or class-state preview is enough. |
| `controlled` | Preview shows controlled state, but app owns mutation. |
| `runtime-planned` | Preview has a deterministic target, but renderer behavior needs separate adapter verification. |
| `app-owned` | Component composition is present, but domain behavior remains outside the UI library. |

The committed manifest lives at
[`rendered-coverage.json`](rendered-coverage.json). The shared preview surface
uses the same stable target ids in
[`examples/preview-states/src/lib.rs`](../../examples/preview-states/src/lib.rs).

## Initial Panel Groups

The manifest should group components by the existing docs catalog categories:

- Actions
- Forms
- Overlays
- Navigation
- Layout
- Data Display
- Feedback
- Messaging

The rendered preview expansion reuses the existing
`examples/preview-states` crate and add stable `data-component-preview`
targets. It does not replace existing `data-preview-panel` or
`data-preview-state` markers.

## Verification Scope

`npm run verify:rendered-component-coverage` should confirm that:

- the coverage manifest contains exactly the public docs catalog components
- every component has a stable `data-component-preview` test id
- every stable target id is present in the shared preview source
- every coverage record has a known panel and coverage level
- runtime-sensitive components are marked `runtime-planned` or `controlled`
  rather than falsely claimed as fully browser-verified
- package scripts expose the focused coverage check
- the release aggregate includes the focused coverage check
- README, quality gates, release docs, docs-site notes, and this document stay
  aligned

The gate should stay read-only. It must not start a server, launch a browser,
write screenshots, update generated docs, change component APIs, edit
templates, or claim visual parity.

## Non-goals

- no visual diffing in M109.1 or M109.2
- no Playwright screenshot artifact generation until the manifest is stable
- no runtime adapter stabilization
- no component API changes
- no replacement of source-copy smoke tests
- no claim that rendered coverage equals shadcn/ui visual parity

## Follow-up Milestones

After the metadata gate and shared preview targets, useful next steps are:

1. Add Web preview DOM checks for `data-component-preview` coverage targets.
2. Add Desktop preview structural checks for the same coverage targets.
3. Add targeted screenshot assertions for chart, form, message, and overlay
   panels only after DOM coverage is deterministic.

M110 plans the first browser-backed DOM check in
[Browser DOM Component Verification](browser-dom-component-verification.md).
