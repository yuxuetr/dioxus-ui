use dioxus::prelude::*;
use dioxus_ui_core::classes;

pub const AVATAR_BASE_CLASS: &str = "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full";
pub const AVATAR_IMAGE_BASE_CLASS: &str = "aspect-square h-full w-full object-cover";
pub const AVATAR_FALLBACK_BASE_CLASS: &str = "flex h-full w-full items-center justify-center rounded-full bg-muted text-sm font-medium text-foreground";

pub fn avatar_class(class: &str) -> String {
  classes([Some(AVATAR_BASE_CLASS), Some(class)])
}

pub fn avatar_image_class(class: &str) -> String {
  classes([Some(AVATAR_IMAGE_BASE_CLASS), Some(class)])
}

pub fn avatar_fallback_class(class: &str) -> String {
  classes([Some(AVATAR_FALLBACK_BASE_CLASS), Some(class)])
}

#[component]
pub fn Avatar(#[props(default)] class: String, children: Element) -> Element {
  let class = avatar_class(&class);

  rsx! {
    span {
      class,
      {children}
    }
  }
}

#[component]
pub fn AvatarImage(
  src: String,
  #[props(default)] alt: String,
  #[props(default)] class: String,
) -> Element {
  let class = avatar_image_class(&class);

  rsx! {
    img {
      class,
      src,
      alt,
    }
  }
}

#[component]
pub fn AvatarFallback(#[props(default)] class: String, children: Element) -> Element {
  let class = avatar_fallback_class(&class);

  rsx! {
    span {
      class,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn avatar_class_appends_user_class() {
    let actual = avatar_class("h-12 w-12");

    assert!(actual.contains(AVATAR_BASE_CLASS));
    assert!(actual.ends_with("h-12 w-12"));
  }

  #[test]
  fn avatar_fallback_class_appends_user_class() {
    let actual = avatar_fallback_class("bg-blue-100");

    assert!(actual.contains(AVATAR_FALLBACK_BASE_CLASS));
    assert!(actual.ends_with("bg-blue-100"));
  }
}
