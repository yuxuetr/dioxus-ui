use dioxus::prelude::*;
use dioxus_ui::{
  Carousel, CarouselContent, CarouselIndicator, CarouselItem, CarouselNext, CarouselPrevious,
  CarouselState, CarouselStep, CarouselViewport, carousel_can_go_next, carousel_can_go_previous,
};

const SLIDES: usize = 4;

#[component]
pub fn Demo() -> Element {
  let mut state = use_signal(|| CarouselState::new(0, SLIDES));
  let index = state().index;

  rsx! {
    Carousel {
      class: "max-w-xs",
      "aria-label": "Featured products",
      on_key_step: move |step| {
        state.set(match step {
          CarouselStep::Previous => state().previous(),
          CarouselStep::Next => state().next(),
        })
      },
      CarouselViewport {
        CarouselContent { index,
          for slide in 0..SLIDES {
            CarouselItem { key: "{slide}", selected: index == slide, "aria-label": "{slide + 1} of {SLIDES}",
              div { class: "flex h-32 items-center justify-center rounded-md bg-muted text-2xl font-semibold", "{slide + 1}" }
            }
          }
        }
      }
      div { class: "mt-3 flex items-center gap-2",
        CarouselPrevious {
          disabled: !carousel_can_go_previous(index, SLIDES, false),
          "aria-label": "Previous slide",
          onclick: move |_| state.set(state().previous()),
          "‹"
        }
        for slide in 0..SLIDES {
          CarouselIndicator {
            key: "{slide}",
            selected: index == slide,
            "aria-label": "Show slide {slide + 1}",
            onclick: move |_| state.set(CarouselState::new(slide, SLIDES)),
          }
        }
        CarouselNext {
          disabled: !carousel_can_go_next(index, SLIDES, false),
          "aria-label": "Next slide",
          onclick: move |_| state.set(state().next()),
          "›"
        }
      }
    }
  }
}
