use dioxus::prelude::*;
use dioxus_shadcn_core::classes;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StepsOrientation {
  #[default]
  Horizontal,
  Vertical,
}

impl StepsOrientation {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Horizontal => "horizontal",
      Self::Vertical => "vertical",
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StepStatus {
  Complete,
  Current,
  #[default]
  Upcoming,
}

impl StepStatus {
  /// The circle's colors.
  pub const fn indicator_class(self) -> &'static str {
    match self {
      Self::Complete => "border-primary bg-primary text-primary-foreground",
      Self::Current => "border-primary bg-background text-foreground",
      Self::Upcoming => "border-border bg-background text-muted-foreground",
    }
  }

  /// The connector halves before and after the circle: primary up to the
  /// current step, so the line shows progress.
  pub const fn connector_class(self) -> &'static str {
    match self {
      Self::Complete => "before:bg-primary after:bg-primary",
      Self::Current => "before:bg-primary after:bg-border",
      Self::Upcoming => "before:bg-border after:bg-border",
    }
  }

  pub const fn label_class(self) -> &'static str {
    match self {
      Self::Complete | Self::Current => "font-medium text-foreground",
      Self::Upcoming => "text-muted-foreground",
    }
  }
}

// The parts read the orientation from the list's `data-orientation` through
// the `steps` group, and the list numbers its steps with a CSS counter.
pub const STEPS_BASE_CLASS: &str =
  "group/steps flex [counter-reset:step] data-[orientation=vertical]:flex-col";
pub const STEP_BASE_CLASS: &str = "group/step flex flex-1 flex-col items-center gap-2 text-center text-sm [counter-increment:step] group-data-[orientation=vertical]/steps:flex-row group-data-[orientation=vertical]/steps:items-stretch group-data-[orientation=vertical]/steps:gap-3 group-data-[orientation=vertical]/steps:text-left";
pub const STEP_TRACK_BASE_CLASS: &str = "flex w-full items-center before:h-0.5 before:flex-1 after:h-0.5 after:flex-1 group-first/step:before:invisible group-last/step:after:invisible group-data-[orientation=vertical]/steps:w-auto group-data-[orientation=vertical]/steps:flex-col group-data-[orientation=vertical]/steps:before:h-2 group-data-[orientation=vertical]/steps:before:w-0.5 group-data-[orientation=vertical]/steps:before:flex-none group-data-[orientation=vertical]/steps:after:h-auto group-data-[orientation=vertical]/steps:after:w-0.5";
pub const STEP_INDICATOR_BASE_CLASS: &str = "flex size-8 shrink-0 items-center justify-center rounded-full border-2 text-sm font-medium before:content-[counter(step)]";
pub const STEP_LABEL_BASE_CLASS: &str = "px-2 group-data-[orientation=vertical]/steps:px-0 group-data-[orientation=vertical]/steps:pt-3 group-data-[orientation=vertical]/steps:pb-6";

pub fn steps_class(class: &str) -> String {
  classes([Some(STEPS_BASE_CLASS), Some(class)])
}

pub fn step_class(class: &str) -> String {
  classes([Some(STEP_BASE_CLASS), Some(class)])
}

pub fn step_track_class(status: StepStatus) -> String {
  classes([Some(STEP_TRACK_BASE_CLASS), Some(status.connector_class())])
}

pub fn step_indicator_class(status: StepStatus) -> String {
  classes([Some(STEP_INDICATOR_BASE_CLASS), Some(status.indicator_class())])
}

pub fn step_label_class(status: StepStatus) -> String {
  classes([Some(STEP_LABEL_BASE_CLASS), Some(status.label_class())])
}

/// An ordered list of the steps in a process, numbered in order.
/// `StepsOrientation::Horizontal` is the default.
#[component]
pub fn Steps(
  #[props(default)] orientation: StepsOrientation,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = steps_class(&class);

  rsx! {
    ol { class, "data-orientation": orientation.as_str(), ..attributes, {children} }
  }
}

/// One step; children are its label. The current step sets
/// `aria-current="step"`, and a complete step adds hidden "completed" text,
/// since its color alone cannot say it.
#[component]
pub fn Step(
  #[props(default)] status: StepStatus,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = step_class(&class);

  rsx! {
    li {
      class,
      "data-status": match status {
        StepStatus::Complete => "complete",
        StepStatus::Current => "current",
        StepStatus::Upcoming => "upcoming",
      },
      "aria-current": if status == StepStatus::Current { Some("step") } else { None },
      ..attributes,
      div { class: step_track_class(status), "aria-hidden": "true",
        span { class: step_indicator_class(status) }
      }
      div { class: step_label_class(status),
        {children}
        if status == StepStatus::Complete {
          span { class: "sr-only", " (completed)" }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_steps_mark_the_current_and_completed_steps() {
    fn app() -> Element {
      rsx! {
        Steps {
          Step { status: StepStatus::Complete, "Cart" }
          Step { status: StepStatus::Current, "Shipping" }
          Step { "Payment" }
        }
      }
    }
    let html = render(app);

    assert!(html.starts_with("<ol"));
    assert!(html.contains("data-orientation=\"horizontal\""));
    assert_eq!(html.matches("aria-current=\"step\"").count(), 1);
    assert!(html.contains("Cart<span class=\"sr-only\"> (completed)</span>"));
    assert!(!html.contains("Payment<span"));
    assert_eq!(html.matches("aria-hidden=\"true\"").count(), 3);
  }

  #[test]
  fn step_status_colors_the_circle_and_connector() {
    assert!(
      step_indicator_class(StepStatus::Complete).contains("bg-primary text-primary-foreground")
    );
    assert!(step_track_class(StepStatus::Current).contains("before:bg-primary after:bg-border"));
    assert!(step_label_class(StepStatus::Upcoming).contains("text-muted-foreground"));
    assert_eq!(StepStatus::default(), StepStatus::Upcoming);
  }
}
