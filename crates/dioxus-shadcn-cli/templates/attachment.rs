use super::utils::classes;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AttachmentState {
  #[default]
  Idle,
  Uploading,
  Processing,
  Error,
  Done,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AttachmentSize {
  #[default]
  Default,
  Sm,
  Xs,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AttachmentOrientation {
  #[default]
  Horizontal,
  Vertical,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AttachmentMediaVariant {
  #[default]
  Icon,
  Image,
}

pub const ATTACHMENT_BASE_CLASS: &str =
  "group flex min-w-0 rounded-md border text-foreground transition-colors";
pub const ATTACHMENT_HORIZONTAL_CLASS: &str = "items-center gap-3 p-3";
pub const ATTACHMENT_VERTICAL_CLASS: &str = "flex-col gap-3 p-3";
pub const ATTACHMENT_SIZE_DEFAULT_CLASS: &str = "min-h-16 text-sm";
pub const ATTACHMENT_SIZE_SM_CLASS: &str = "min-h-12 text-sm";
pub const ATTACHMENT_SIZE_XS_CLASS: &str = "min-h-10 text-xs";
pub const ATTACHMENT_UPLOADING_CLASS: &str = "border-info/30 bg-info/10";
pub const ATTACHMENT_PROCESSING_CLASS: &str = "border-border bg-muted";
pub const ATTACHMENT_ERROR_CLASS: &str = "border-destructive/30 bg-destructive/10";
pub const ATTACHMENT_DONE_CLASS: &str = "border-success/30 bg-success/10";
pub const ATTACHMENT_GROUP_BASE_CLASS: &str = "flex gap-2 overflow-x-auto";
pub const ATTACHMENT_MEDIA_BASE_CLASS: &str = "flex shrink-0 items-center justify-center overflow-hidden rounded-md border border-border bg-muted text-muted-foreground";
pub const ATTACHMENT_MEDIA_ICON_CLASS: &str = "h-10 w-10";
pub const ATTACHMENT_MEDIA_IMAGE_CLASS: &str =
  "h-14 w-14 [&>img]:h-full [&>img]:w-full [&>img]:object-cover";
pub const ATTACHMENT_CONTENT_BASE_CLASS: &str = "grid min-w-0 flex-1 gap-1";
pub const ATTACHMENT_TITLE_BASE_CLASS: &str = "truncate font-medium text-foreground";
pub const ATTACHMENT_DESCRIPTION_BASE_CLASS: &str = "line-clamp-2 text-xs text-muted-foreground";
pub const ATTACHMENT_ACTIONS_BASE_CLASS: &str = "flex shrink-0 items-center gap-1";
pub const ATTACHMENT_ACTION_BASE_CLASS: &str = "inline-flex h-8 min-w-8 items-center justify-center rounded-md px-2 text-xs font-medium text-muted-foreground transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
pub const ATTACHMENT_TRIGGER_BASE_CLASS: &str = "inline-flex min-h-10 items-center justify-center gap-2 rounded-md border border-dashed border-input bg-background px-3 py-2 text-sm font-medium text-muted-foreground transition-colors hover:bg-muted focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

impl AttachmentState {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Idle => "border-border bg-background",
      Self::Uploading => ATTACHMENT_UPLOADING_CLASS,
      Self::Processing => ATTACHMENT_PROCESSING_CLASS,
      Self::Error => ATTACHMENT_ERROR_CLASS,
      Self::Done => ATTACHMENT_DONE_CLASS,
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Idle => "idle",
      Self::Uploading => "uploading",
      Self::Processing => "processing",
      Self::Error => "error",
      Self::Done => "done",
    }
  }
}

impl AttachmentSize {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Default => ATTACHMENT_SIZE_DEFAULT_CLASS,
      Self::Sm => ATTACHMENT_SIZE_SM_CLASS,
      Self::Xs => ATTACHMENT_SIZE_XS_CLASS,
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Default => "default",
      Self::Sm => "sm",
      Self::Xs => "xs",
    }
  }
}

impl AttachmentOrientation {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Horizontal => ATTACHMENT_HORIZONTAL_CLASS,
      Self::Vertical => ATTACHMENT_VERTICAL_CLASS,
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Horizontal => "horizontal",
      Self::Vertical => "vertical",
    }
  }
}

impl AttachmentMediaVariant {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Icon => ATTACHMENT_MEDIA_ICON_CLASS,
      Self::Image => ATTACHMENT_MEDIA_IMAGE_CLASS,
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Icon => "icon",
      Self::Image => "image",
    }
  }
}

pub fn attachment_class(
  state: AttachmentState,
  size: AttachmentSize,
  orientation: AttachmentOrientation,
  class: &str,
) -> String {
  classes([
    Some(ATTACHMENT_BASE_CLASS),
    Some(orientation.class()),
    Some(size.class()),
    Some(state.class()),
    Some(class),
  ])
}

pub fn attachment_group_class(class: &str) -> String {
  classes([Some(ATTACHMENT_GROUP_BASE_CLASS), Some(class)])
}

pub fn attachment_media_class(variant: AttachmentMediaVariant, class: &str) -> String {
  classes([Some(ATTACHMENT_MEDIA_BASE_CLASS), Some(variant.class()), Some(class)])
}

pub fn attachment_content_class(class: &str) -> String {
  classes([Some(ATTACHMENT_CONTENT_BASE_CLASS), Some(class)])
}

pub fn attachment_title_class(class: &str) -> String {
  classes([Some(ATTACHMENT_TITLE_BASE_CLASS), Some(class)])
}

pub fn attachment_description_class(class: &str) -> String {
  classes([Some(ATTACHMENT_DESCRIPTION_BASE_CLASS), Some(class)])
}

pub fn attachment_actions_class(class: &str) -> String {
  classes([Some(ATTACHMENT_ACTIONS_BASE_CLASS), Some(class)])
}

pub fn attachment_action_class(class: &str) -> String {
  classes([Some(ATTACHMENT_ACTION_BASE_CLASS), Some(class)])
}

pub fn attachment_trigger_class(class: &str) -> String {
  classes([Some(ATTACHMENT_TRIGGER_BASE_CLASS), Some(class)])
}

#[component]
pub fn Attachment(
  #[props(default)] state: AttachmentState,
  #[props(default)] size: AttachmentSize,
  #[props(default)] orientation: AttachmentOrientation,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = attachment_class(state, size, orientation, &class);

  rsx! {
    div {
      class,
      "data-state": state.attribute(),
      "data-size": size.attribute(),
      "data-orientation": orientation.attribute(),
      {children}
    }
  }
}

#[component]
pub fn AttachmentGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = attachment_group_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn AttachmentMedia(
  #[props(default)] variant: AttachmentMediaVariant,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = attachment_media_class(variant, &class);

  rsx! {
    div {
      class,
      "data-variant": variant.attribute(),
      {children}
    }
  }
}

#[component]
pub fn AttachmentContent(#[props(default)] class: String, children: Element) -> Element {
  let class = attachment_content_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn AttachmentTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = attachment_title_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn AttachmentDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = attachment_description_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn AttachmentActions(#[props(default)] class: String, children: Element) -> Element {
  let class = attachment_actions_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn AttachmentAction(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = attachment_action_class(&class);

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

#[component]
pub fn AttachmentTrigger(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = attachment_trigger_class(&class);

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
