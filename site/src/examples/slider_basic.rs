use dioxus::prelude::*;
use dioxus_shadcn::{Label, Slider, SliderOrientation};

#[component]
pub fn Demo() -> Element {
  let mut volume = use_signal(|| 40.0);
  let mut balance = use_signal(|| 50.0);

  rsx! {
    div { class: "flex max-w-sm items-start gap-8",
      div { class: "grid flex-1 gap-3",
        Label { id: "slider-basic-volume", "Volume: {volume}" }
        Slider {
          "aria-labelledby": "slider-basic-volume",
          step: 5.0,
          value: volume(),
          on_value_change: move |value| volume.set(value),
        }
        Label { id: "slider-basic-locked", "Locked" }
        Slider { "aria-labelledby": "slider-basic-locked", value: 30.0, disabled: true }
      }
      div { class: "h-32",
        Slider {
          "aria-label": "Balance",
          orientation: SliderOrientation::Vertical,
          step: 10.0,
          value: balance(),
          on_value_change: move |value| balance.set(value),
        }
      }
    }
  }
}
