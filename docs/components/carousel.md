# Carousel

Carousel provides controlled slide composition parts and pure index helpers.
The app owns the selected index; the parts show it and report clicks and arrow
keys. It does not own gesture recognition, autoplay, or scroll snapping.

## Source Copy

```bash
dxui add carousel
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["carousel"] }
```

## API Surface

- `Carousel`
- `CarouselViewport`
- `CarouselContent`
- `CarouselItem`
- `CarouselPrevious`
- `CarouselNext`
- `CarouselIndicator`
- `CarouselOrientation`
- `CarouselStep`
- `carousel_key_step`
- `carousel_next`
- `carousel_previous`
- `carousel_can_go_next`
- `carousel_can_go_previous`

## Slide Changes

`Carousel` owns the selected slide
([RFC 0077](../rfcs/0077-component-owned-state.md)). Give it the number of
slides in `count`, and each `CarouselItem` and `CarouselIndicator` its
`index`. The root starts at `default_index`, or follows `index` when the app
controls it; `on_index_change` hears every change the user makes:

```rust
rsx! {
  Carousel { "aria-label": "Featured", count: 3,
    CarouselViewport {
      CarouselContent {
        for slide in 0..3 {
          CarouselItem { key: "{slide}", index: slide, "aria-label": "{slide + 1} of 3", "Slide {slide + 1}" }
        }
      }
    }
    CarouselPrevious {}
    CarouselNext {}
  }
}
```

- Previous and Next step the index and disable themselves at the ends, unless
  `looping` is set; `disabled` disables either one as well. An indicator
  selects its slide and sets `aria-current` while it is selected.
- Each item translates by `index` times its own size, so `basis-1/2` and other
  multi-item views work without measurement. The transition is turned off for
  reduced motion.
- `CarouselContent` sets the index as the `--dxui-carousel-index` style
  property, which merges with a passed `style`. Items own the `transform`
  style.
- ArrowLeft and ArrowRight step a horizontal carousel, ArrowUp and ArrowDown a
  vertical one, while focus is anywhere inside the root, including fields in a
  slide.
- The parts must be inside `Carousel`; otherwise they render nothing and log
  which root they are missing.
- Global and element attributes pass through every part. A passed
  `aria-label` replaces the English default on Previous, Next, and
  Indicator, so each indicator can say which slide it shows.
- Off-screen slides stay in the Tab order and the accessibility tree. When
  slides hold focusable content, pass `inert` to the slides that are not on
  screen.

## Accessibility Notes

The root uses region semantics with `aria-roledescription="carousel"`. Items
use slide group semantics. Previous and next controls are native buttons with
disabled state. Apps should name the region and each slide, such as "2 of 5", through
`aria-label`, and name each indicator.
