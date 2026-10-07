//! Input group: lays out addons, a control, and actions around a native input inside
//! one bordered field.
use super::density::{density_control_class, use_density, with_density};
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

/// Which side of the control an `InputGroupAddon` sits on.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum InputGroupAddonPosition {
  /// Before the control, with a border on its end side.
  #[default]
  Start,
  /// After the control, with a border on its start side.
  End,
}

const INPUT_GROUP_BASE_CLASS: &str = "flex min-h-10 w-full items-center overflow-hidden rounded-md border bg-background text-sm transition-colors focus-within:ring-2 data-[disabled=true]:opacity-50";
const INPUT_GROUP_INVALID_CLASS: &str = "border-destructive focus-within:ring-destructive";
const INPUT_GROUP_DISABLED_CLASS: &str = "cursor-not-allowed";
const INPUT_GROUP_ADDON_BASE_CLASS: &str =
  "flex h-full shrink-0 items-center gap-2 bg-muted px-3 text-sm text-muted-foreground";
const INPUT_GROUP_ADDON_START_CLASS: &str = "border-r border-input";
const INPUT_GROUP_ADDON_END_CLASS: &str = "border-l border-input";
const INPUT_GROUP_CONTROL_BASE_CLASS: &str = "flex min-w-0 flex-1 items-center [&>input]:border-0 [&>input]:bg-transparent [&>input]:shadow-none [&>input]:focus-visible:ring-0";
const INPUT_GROUP_ACTION_BASE_CLASS: &str = "inline-flex h-full shrink-0 items-center justify-center px-3 text-sm font-medium text-muted-foreground transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

impl InputGroupAddonPosition {
  /// The border that separates the addon from the control on this side.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Start => INPUT_GROUP_ADDON_START_CLASS,
      Self::End => INPUT_GROUP_ADDON_END_CLASS,
    }
  }

  /// The `data-position` value: `start` or `end`.
  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Start => "start",
      Self::End => "end",
    }
  }
}

/// Classes for the group: base classes, the invalid or normal border and focus ring,
/// the disabled cursor, then `class` merged over them.
pub fn input_group_class(invalid: bool, disabled: bool, class: &str) -> String {
  merge_classes(classes([Some(INPUT_GROUP_BASE_CLASS), Some(if invalid { INPUT_GROUP_INVALID_CLASS } else { "border-input focus-within:ring-ring" }), disabled.then_some(INPUT_GROUP_DISABLED_CLASS)]), class)
}

/// Classes for an addon: muted base classes, the position's border, then `class`
/// merged over them.
pub fn input_group_addon_class(position: InputGroupAddonPosition, class: &str) -> String {
  merge_classes(classes([Some(INPUT_GROUP_ADDON_BASE_CLASS), Some(position.class())]), class)
}

/// Classes for the control slot: it fills the space and strips the input's own
/// border, background, and ring, then `class` merged over them.
pub fn input_group_control_class(class: &str) -> String {
  merge_classes(classes([Some(INPUT_GROUP_CONTROL_BASE_CLASS)]), class)
}

/// Classes for an action button inside the group, then `class` merged over them.
pub fn input_group_action_class(class: &str) -> String {
  merge_classes(classes([Some(INPUT_GROUP_ACTION_BASE_CLASS)]), class)
}

#[component]
pub fn InputGroup(
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = input_group_class(invalid, disabled, &class);

  rsx! {
    div {
      class,
      "data-disabled": disabled.to_string(),
      "data-invalid": invalid.to_string(),
      {children}
    }
  }
}

#[component]
pub fn InputGroupAddon(
  #[props(default)] position: InputGroupAddonPosition,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = input_group_addon_class(position, &class);

  rsx! {
    div {
      class,
      "data-position": position.attribute(),
      {children}
    }
  }
}

#[component]
pub fn InputGroupControl(#[props(default)] class: String, children: Element) -> Element {
  let class = input_group_control_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn InputGroupAction(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = input_group_action_class(&with_density(density_control_class(use_density()), &class));

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      onclick: move |event| {
        if let Some(handler) = onclick {
          handler.call(event);
        }
      },
      ..attributes,
      {children}
    }
  }
}
