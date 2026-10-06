# RFC 0068: Modal Scroll Lock

- Status: Accepted
- Created: 2026-10-06

## Summary

Dialog, Alert Dialog, Sheet, and Drawer stop the page from scrolling while
they are open, and restore it when the last one closes.

## Current State

The 0.1.0 release notes exclude scroll lock. With a modal open, the wheel or
a touch drag over its overlay scrolls the page behind it, so content shifts
under the dialog and the page is somewhere else when it closes. Radix and
shadcn/ui lock scroll for modals.

## Decision

The modal focus scope script ([RFC 0010](0010-overlay-interaction-behavior.md))
already runs for exactly the time a modal is open, so it also holds the lock:

- On open it sets `overflow: hidden` on the root element and adds the
  scrollbar's width to the root's right padding, so the content does not
  shift when the scrollbar goes away.
- Nested modals share one lock: `data-dxui-scroll-locks` on the root counts
  them, and the first saves the root's inline `overflow` and `padding-right`
  in `data-dxui-scroll-restore`. The last to close restores them.
- The scroll position is kept, since the page is not moved, only stopped.

Date Picker content uses the same focus scope but is not modal, so it does
not lock: the scope takes a `lock_scroll` flag.

## Alternatives

- **`position: fixed` on the body.** It also stops iOS rubber-band scrolling
  but needs the scroll position saved and restored, and moves fixed headers.
  `overflow: hidden` on the root works in current WebKit, Blink, and Gecko.
- **A Rust-side counter.** The lock must also be released when the app stops
  rendering the content, which the script observes already.

## Verification

- A runtime check opens the Dialog: the root has `overflow: hidden`, one
  lock, and padding for the scrollbar, a wheel scroll leaves `scrollY`
  unchanged, and closing restores the styles and allows scrolling again.
  Reverse-verified with the lock turned off.
- A unit test for the script's flag.
