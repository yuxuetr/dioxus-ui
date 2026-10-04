# RFC 0034: Carousel Slide Changes

- Status: Accepted
- Created: 2026-10-05

## Summary

Make Carousel move. `CarouselContent` takes the selected `index` and its items
shift to show it. `CarouselPrevious`, `CarouselNext`, and `CarouselIndicator`
take an `onclick` callback, and the `Carousel` root reports arrow keys
through `on_key_step`. All parts pass through global and element attributes,
and a passed `aria-label` replaces the English default on the controls.

## Current State

As of M158:

- The viewport has `overflow-hidden`, and every item has `basis-full`, but
  nothing translates the content. Whatever the app selects, the first slide
  stays on screen. `selected` only sets `data-selected`.
- Previous, Next, and the indicators are buttons with no click handler, so
  the app cannot learn that one was pressed.
- The root handles no keys. The docs tell apps to add keyboard shortcuts, but
  the root has no event to hang them on.
- Every indicator has `aria-label="Go to slide"`, so screen reader users hear
  the same name for each one. The labels on Previous and Next are fixed
  English.
- No part accepts `id`, `aria-label`, or other attributes, so the region
  cannot be named and a slide cannot say "2 of 5".

## Decision

### API

```rust
let mut state = use_signal(|| CarouselState::new(0, 3));

rsx! {
  Carousel {
    "aria-label": "Featured",
    on_key_step: move |step| state.set(match step {
      CarouselStep::Previous => state().previous(),
      CarouselStep::Next => state().next(),
    }),
    CarouselViewport {
      CarouselContent { index: state().index,
        for slide in 0..3 {
          CarouselItem { key: "{slide}", selected: state().index == slide, "aria-label": "{slide + 1} of 3", "Slide {slide + 1}" }
        }
      }
    }
    CarouselPrevious {
      disabled: !carousel_can_go_previous(state().index, 3, false),
      onclick: move |_| state.set(state().previous()),
    }
    CarouselNext {
      disabled: !carousel_can_go_next(state().index, 3, false),
      onclick: move |_| state.set(state().next()),
    }
  }
}
```

- `CarouselContent` gains `index: usize` (default `0`) and renders it as the
  `--dxui-carousel-index` custom property in its `style`.
- `CarouselItem` sets the `transform` style to
  `translateX(calc(var(--dxui-carousel-index, 0) * -100%))`, or `translateY`
  when vertical. A percentage translate is relative to the item itself, so
  each item moves by `index` item widths. This works for `basis-full` and for
  views that show several items, like `basis-1/2`, without measuring
  anything. The item base class gains `transition-transform duration-300
  motion-reduce:transition-none`.
- `onclick: Option<EventHandler<MouseEvent>>` on `CarouselPrevious`,
  `CarouselNext`, and `CarouselIndicator`. Native `disabled` already blocks
  clicks, and the callback is not called while disabled.
- `Carousel` gains `on_key_step: Option<EventHandler<CarouselStep>>`, with
  `CarouselStep::{Previous, Next}`. ArrowLeft and ArrowRight map to it when
  horizontal, and ArrowUp and ArrowDown when vertical. A handled key calls
  `prevent_default`, so vertical steps do not scroll the page. The app applies
  the step with `CarouselState`, so its looping and bounds rules still decide
  the index.
- `Carousel`, `CarouselViewport`, `CarouselContent`, and `CarouselItem`
  extend `GlobalAttributes` and `div`. The three buttons extend
  `GlobalAttributes` and `button`. The spread comes after the explicit
  attributes.
- Previous, Next, and Indicator keep their English `aria-label` only when the
  passed attributes have none, so an app can name each indicator or localize
  the controls.

## Scope

In scope:

- the index shift, the callbacks, the root key step, attribute spreading, and
  the label override in the crate and the template
- the Carousel docs page
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Hiding off-screen slides from Tab and assistive technology | `inert` on unselected items would also hide the visible neighbors in a `basis-1/2` view; the app knows which slides are on screen and can pass `inert` or `aria-hidden` through the spread | A consumer reports focus moving into an off-screen slide |
| Swipe and drag gestures | Needs pointer capture and a threshold policy; buttons and keys cover input on every target | A consumer ships a touch-first carousel |
| Autoplay and a rotation control | Needs a timer, pause on hover and focus, and a stop button per the APG pattern | A consumer needs rotating slides |
| Right-to-left arrow mirroring | A Rust key handler cannot read the computed direction; the translate would also need to flip | A consumer ships a right-to-left Carousel |
| Ignoring arrows typed into a field inside a slide | Dioxus 0.7 key events do not expose the target | A consumer reports a form inside a slide |
| Desktop and Mobile self-test scenarios | The parts use plain click and key events with no script | They gain script behavior |

## Verification

- Unit tests cover the key mapping for both orientations and the item
  transform.
- The CLI generated fixture smoke keeps the template compiling.
- The Web preview renders a labelled three-slide Carousel with indicators,
  and `npm run verify:runtime-interactions` asserts:
  - Next, Previous, and an indicator change the index, and the selected slide
    lines up with the viewport while the others sit outside it;
  - ArrowRight and ArrowLeft on the region step the index, and Previous and
    Next are disabled at the ends;
  - passed attributes render, and a passed indicator `aria-label` replaces the
    default.
- Reverse checks: removing the click callbacks, removing the key step, not
  translating the items, keeping the default label when one is passed, or not
  spreading the attributes each make the verifier fail.
