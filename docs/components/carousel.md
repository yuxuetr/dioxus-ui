# Carousel

Carousel provides controlled slide composition parts and pure index helpers. It
does not own gesture recognition, autoplay, scroll snapping, or slide
measurement.

## Source Copy

```bash
dxui add carousel
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["carousel"] }
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
- `CarouselState`
- `carousel_next`
- `carousel_previous`
- `carousel_can_go_next`
- `carousel_can_go_previous`

## Accessibility Notes

The root uses region semantics with `aria-roledescription="carousel"`. Items
use slide group semantics. Previous and next controls are native buttons with
disabled state. Apps should provide visible labels, slide counts, keyboard
shortcuts, and live announcements when the carousel changes slides.
