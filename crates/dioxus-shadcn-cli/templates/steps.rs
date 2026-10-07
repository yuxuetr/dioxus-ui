//! Steps: where the user is in a process, such as checkout or onboarding, shown as
//! numbered steps joined by a line.
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

/// Which way the steps run.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StepsOrientation {
  /// Left to right, with labels under the circles.
  #[default]
  Horizontal,
  /// Top to bottom, with labels beside the circles.
  Vertical,
}

impl StepsOrientation {
  /// The value for the list's `data-orientation` attribute.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Horizontal => "horizontal",
      Self::Vertical => "vertical",
    }
  }
}

/// Where a step stands relative to the user's progress.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StepStatus {
  /// Already done.
  Complete,
  /// The step the user is on.
  Current,
  /// Not reached yet.
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

  /// The label's weight and color: emphasized up to the current step, muted after.
  pub const fn label_class(self) -> &'static str {
    match self {
      Self::Complete | Self::Current => "font-medium text-foreground",
      Self::Upcoming => "text-muted-foreground",
    }
  }
}

// The parts read the orientation from the list's `data-orientation` through
// the `steps` group, and the list numbers its steps with a CSS counter.
const STEPS_BASE_CLASS: &str =
  "group/steps flex [counter-reset:step] data-[orientation=vertical]:flex-col";
const STEP_BASE_CLASS: &str = "group/step flex flex-1 flex-col items-center gap-2 text-center text-sm [counter-increment:step] group-data-[orientation=vertical]/steps:flex-row group-data-[orientation=vertical]/steps:items-stretch group-data-[orientation=vertical]/steps:gap-3 group-data-[orientation=vertical]/steps:text-left";
const STEP_TRACK_BASE_CLASS: &str = "flex w-full items-center before:h-0.5 before:flex-1 after:h-0.5 after:flex-1 group-first/step:before:invisible group-last/step:after:invisible group-data-[orientation=vertical]/steps:w-auto group-data-[orientation=vertical]/steps:flex-col group-data-[orientation=vertical]/steps:before:h-2 group-data-[orientation=vertical]/steps:before:w-0.5 group-data-[orientation=vertical]/steps:before:flex-none group-data-[orientation=vertical]/steps:after:h-auto group-data-[orientation=vertical]/steps:after:w-0.5";
const STEP_INDICATOR_BASE_CLASS: &str = "flex size-8 shrink-0 items-center justify-center rounded-full border-2 text-sm font-medium before:content-[counter(step)]";
const STEP_LABEL_BASE_CLASS: &str = "px-2 group-data-[orientation=vertical]/steps:px-0 group-data-[orientation=vertical]/steps:pt-3 group-data-[orientation=vertical]/steps:pb-6";

/// Classes for the list: base classes with `class` merged over them.
pub fn steps_class(class: &str) -> String {
  merge_classes(classes([Some(STEPS_BASE_CLASS)]), class)
}

/// Classes for one step: base classes with `class` merged over them.
pub fn step_class(class: &str) -> String {
  merge_classes(classes([Some(STEP_BASE_CLASS)]), class)
}

/// Classes for the track holding the circle and its connector halves, colored by
/// `status`.
pub fn step_track_class(status: StepStatus) -> String {
  classes([Some(STEP_TRACK_BASE_CLASS), Some(status.connector_class())])
}

/// Classes for the numbered circle, colored by `status`.
pub fn step_indicator_class(status: StepStatus) -> String {
  classes([Some(STEP_INDICATOR_BASE_CLASS), Some(status.indicator_class())])
}

/// Classes for the label, weighted and colored by `status`.
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
