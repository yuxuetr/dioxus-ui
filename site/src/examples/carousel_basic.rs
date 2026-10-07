use dioxus::prelude::*;
use dioxus_shadcn::{
  Carousel, CarouselContent, CarouselIndicator, CarouselItem, CarouselNext, CarouselPrevious,
  CarouselViewport,
};

const SLIDES: usize = 4;

#[component]
pub fn CarouselBasicDemo() -> Element {
  rsx! {
    Carousel { class: "max-w-xs", "aria-label": "Featured products", count: SLIDES,
      CarouselViewport {
        CarouselContent {
          for slide in 0..SLIDES {
            CarouselItem { key: "{slide}", index: slide, "aria-label": "{slide + 1} of {SLIDES}",
              div { class: "flex h-32 items-center justify-center rounded-md bg-muted text-2xl font-semibold", "{slide + 1}" }
            }
          }
        }
      }
      div { class: "mt-3 flex items-center gap-2",
        CarouselPrevious { "‹" }
        for slide in 0..SLIDES {
          CarouselIndicator { key: "{slide}", index: slide, "aria-label": "Show slide {slide + 1}" }
        }
        CarouselNext { "›" }
      }
    }
  }
}
