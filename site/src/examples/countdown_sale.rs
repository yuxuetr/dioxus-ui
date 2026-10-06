use dioxus::prelude::*;
use dioxus_shadcn::{Countdown, countdown_parts};

#[component]
pub fn CountdownSaleDemo() -> Element {
  let mut remaining = use_signal(|| 2 * 86_400 + 10 * 3_600 + 24 * 60 + 59_u64);

  // The app owns the clock: wait a second in the page, then count down.
  use_future(move || async move {
    loop {
      let tick = document::eval("await new Promise((resolve) => setTimeout(resolve, 1000)); return 0;");
      if tick.await.is_err() {
        break;
      }
      let next = remaining().saturating_sub(1);
      remaining.set(next);
      if next == 0 {
        break;
      }
    }
  });

  let parts = countdown_parts(remaining());

  rsx! {
    div { class: "grid gap-6",
      p { class: "text-sm text-muted-foreground",
        "Sale ends in "
        Countdown { class: "text-foreground", remaining: remaining(), "aria-label": "Sale ends in" }
      }
      // countdown_parts gives the numbers for a layout with unit labels.
      div { class: "flex gap-3", role: "timer", "aria-label": "Sale ends in",
        for (value, unit) in [(parts.days, "days"), (parts.hours, "hours"), (parts.minutes, "min"), (parts.seconds, "sec")] {
          div { key: "{unit}", class: "grid min-w-16 place-items-center rounded-md bg-muted px-3 py-2",
            span { class: "text-3xl font-semibold tabular-nums", "{value:02}" }
            span { class: "text-xs text-muted-foreground", "{unit}" }
          }
        }
      }
    }
  }
}
