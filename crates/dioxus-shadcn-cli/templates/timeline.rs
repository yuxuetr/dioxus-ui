use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TimelineOrientation {
  #[default]
  Vertical,
  Horizontal,
}

impl TimelineOrientation {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Vertical => "vertical",
      Self::Horizontal => "horizontal",
    }
  }
}

// The parts read the orientation from the list's `data-orientation` through
// the `timeline` group, so they need no context and follow a changed prop.
pub const TIMELINE_BASE_CLASS: &str =
  "group/timeline flex flex-col data-[orientation=horizontal]:flex-row";
pub const TIMELINE_ITEM_BASE_CLASS: &str = "group/item grid grid-cols-[minmax(0,8rem)_auto_minmax(0,1fr)] gap-x-3 group-data-[orientation=horizontal]/timeline:flex-1 group-data-[orientation=horizontal]/timeline:grid-cols-1 group-data-[orientation=horizontal]/timeline:grid-rows-[auto_auto_1fr]";
pub const TIMELINE_TIME_BASE_CLASS: &str = "col-start-1 row-start-1 pt-0.5 text-right text-sm text-muted-foreground group-data-[orientation=horizontal]/timeline:pt-0 group-data-[orientation=horizontal]/timeline:pb-2 group-data-[orientation=horizontal]/timeline:text-center";
// The line above the dot is hidden on the first item and the line below it
// on the last, so the connector runs only between markers.
pub const TIMELINE_MARKER_BASE_CLASS: &str = "col-start-2 row-start-1 flex flex-col items-center self-stretch before:h-1.5 before:w-px before:bg-border after:w-px after:flex-1 after:bg-border group-first/item:before:invisible group-last/item:after:invisible group-data-[orientation=horizontal]/timeline:col-start-1 group-data-[orientation=horizontal]/timeline:row-start-2 group-data-[orientation=horizontal]/timeline:flex-row group-data-[orientation=horizontal]/timeline:before:h-px group-data-[orientation=horizontal]/timeline:before:w-auto group-data-[orientation=horizontal]/timeline:before:flex-1 group-data-[orientation=horizontal]/timeline:after:h-px group-data-[orientation=horizontal]/timeline:after:w-auto";
// An empty slot draws the default dot; an icon child replaces it.
pub const TIMELINE_MARKER_SLOT_CLASS: &str =
  "flex shrink-0 items-center justify-center empty:size-3 empty:rounded-full empty:bg-primary";
pub const TIMELINE_CONTENT_BASE_CLASS: &str = "col-start-3 row-start-1 pb-8 group-last/item:pb-0 group-data-[orientation=horizontal]/timeline:col-start-1 group-data-[orientation=horizontal]/timeline:row-start-3 group-data-[orientation=horizontal]/timeline:px-2 group-data-[orientation=horizontal]/timeline:pt-2 group-data-[orientation=horizontal]/timeline:pb-0 group-data-[orientation=horizontal]/timeline:text-center";

pub fn timeline_class(class: &str) -> String {
  classes([Some(TIMELINE_BASE_CLASS), Some(class)])
}

pub fn timeline_item_class(class: &str) -> String {
  classes([Some(TIMELINE_ITEM_BASE_CLASS), Some(class)])
}

pub fn timeline_time_class(class: &str) -> String {
  classes([Some(TIMELINE_TIME_BASE_CLASS), Some(class)])
}

pub fn timeline_marker_class(class: &str) -> String {
  classes([Some(TIMELINE_MARKER_BASE_CLASS), Some(class)])
}

pub fn timeline_content_class(class: &str) -> String {
  classes([Some(TIMELINE_CONTENT_BASE_CLASS), Some(class)])
}

/// An ordered list of events. `TimelineOrientation::Vertical`, the default,
/// puts the time beside the marker; `Horizontal` puts it above.
#[component]
pub fn Timeline(
  #[props(default)] orientation: TimelineOrientation,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = timeline_class(&class);

  rsx! {
    ol { class, "data-orientation": orientation.as_str(), ..attributes, {children} }
  }
}

#[component]
pub fn TimelineItem(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = timeline_item_class(&class);

  rsx! {
    li { class, ..attributes, {children} }
  }
}

/// The event's time, a `time` element; pass `datetime` with a machine-readable
/// value.
#[component]
pub fn TimelineTime(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = time)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = timeline_time_class(&class);

  rsx! {
    time { class, ..attributes, {children} }
  }
}

/// The dot on the connector, hidden from assistive technology. Without
/// children it draws a primary dot; an icon child replaces the dot.
#[component]
pub fn TimelineMarker(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = timeline_marker_class(&class);

  rsx! {
    div { class, "aria-hidden": "true", ..attributes,
      span { class: TIMELINE_MARKER_SLOT_CLASS, {children} }
    }
  }
}

#[component]
pub fn TimelineContent(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = timeline_content_class(&class);

  rsx! {
    div { class, ..attributes, {children} }
  }
}
