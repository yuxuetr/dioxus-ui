use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonVariant, Step, StepStatus, Steps, StepsOrientation};

const STEPS: [&str; 4] = ["Cart", "Shipping", "Payment", "Review"];

#[component]
pub fn StepsCheckoutDemo() -> Element {
  let mut current = use_signal(|| 1_usize);
  let status = move |index: usize| {
    if index < current() {
      StepStatus::Complete
    } else if index == current() {
      StepStatus::Current
    } else {
      StepStatus::Upcoming
    }
  };

  rsx! {
    div { class: "grid gap-8",
      Steps { "aria-label": "Checkout progress",
        for (index, label) in STEPS.iter().enumerate() {
          Step { key: "{label}", status: status(index), "{label}" }
        }
      }
      div { class: "flex gap-2",
        Button {
          variant: ButtonVariant::Outline,
          disabled: current() == 0,
          onclick: move |_| current -= 1,
          "Back"
        }
        Button {
          disabled: current() + 1 == STEPS.len(),
          onclick: move |_| current += 1,
          "Next"
        }
      }
      Steps { orientation: StepsOrientation::Vertical, "aria-label": "Account setup",
        Step { status: StepStatus::Complete, "Create account" }
        Step { status: StepStatus::Current,
          p { "Verify email" }
          p { class: "text-xs font-normal text-muted-foreground", "Check your inbox for a link." }
        }
        Step { "Invite your team" }
      }
    }
  }
}
