use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputGroupAddonPosition {
  Start,
  End,
}

pub const INPUT_GROUP_BASE_CLASS: &str = "flex min-h-10 w-full items-center overflow-hidden rounded-md border bg-background text-sm transition-colors focus-within:ring-2 data-[disabled=true]:opacity-50";
pub const INPUT_GROUP_INVALID_CLASS: &str = "border-destructive focus-within:ring-destructive";
pub const INPUT_GROUP_DISABLED_CLASS: &str = "cursor-not-allowed";
pub const INPUT_GROUP_ADDON_BASE_CLASS: &str = "flex h-full shrink-0 items-center gap-2 bg-muted px-3 text-sm text-muted-foreground";
pub const INPUT_GROUP_ADDON_START_CLASS: &str = "border-r border-input";
pub const INPUT_GROUP_ADDON_END_CLASS: &str = "border-l border-input";
pub const INPUT_GROUP_CONTROL_BASE_CLASS: &str = "flex min-w-0 flex-1 items-center [&>input]:border-0 [&>input]:bg-transparent [&>input]:shadow-none [&>input]:focus-visible:ring-0";
pub const INPUT_GROUP_ACTION_BASE_CLASS: &str = "inline-flex h-full shrink-0 items-center justify-center px-3 text-sm font-medium text-muted-foreground transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

impl InputGroupAddonPosition {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Start => INPUT_GROUP_ADDON_START_CLASS,
      Self::End => INPUT_GROUP_ADDON_END_CLASS,
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Start => "start",
      Self::End => "end",
    }
  }
}

pub fn input_group_class(invalid: bool, disabled: bool, class: &str) -> String {
  classes([
    Some(INPUT_GROUP_BASE_CLASS),
    Some(if invalid { INPUT_GROUP_INVALID_CLASS } else { "border-input focus-within:ring-ring" }),
    disabled.then_some(INPUT_GROUP_DISABLED_CLASS),
    Some(class),
  ])
}

pub fn input_group_addon_class(position: InputGroupAddonPosition, class: &str) -> String {
  classes([
    Some(INPUT_GROUP_ADDON_BASE_CLASS),
    Some(position.class()),
    Some(class),
  ])
}

pub fn input_group_control_class(class: &str) -> String {
  classes([Some(INPUT_GROUP_CONTROL_BASE_CLASS), Some(class)])
}

pub fn input_group_action_class(class: &str) -> String {
  classes([Some(INPUT_GROUP_ACTION_BASE_CLASS), Some(class)])
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
  #[props(default = InputGroupAddonPosition::Start)] position: InputGroupAddonPosition,
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
  children: Element,
) -> Element {
  let class = input_group_action_class(&class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      {children}
    }
  }
}
