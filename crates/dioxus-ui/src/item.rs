use dioxus::prelude::*;
use dioxus_ui_core::classes;

pub const ITEM_BASE_CLASS: &str = "flex items-start gap-3 rounded-md p-3 text-sm transition-colors data-[disabled=true]:opacity-50 data-[selected=true]:bg-accent";
pub const ITEM_SELECTED_CLASS: &str = "bg-accent";
pub const ITEM_DISABLED_CLASS: &str = "pointer-events-none opacity-50";
pub const ITEM_MEDIA_BASE_CLASS: &str = "flex shrink-0 items-center justify-center";
pub const ITEM_CONTENT_BASE_CLASS: &str = "grid min-w-0 flex-1 gap-1";
pub const ITEM_TITLE_BASE_CLASS: &str = "truncate font-medium text-foreground";
pub const ITEM_DESCRIPTION_BASE_CLASS: &str = "line-clamp-2 text-sm text-muted-foreground";
pub const ITEM_ACTIONS_BASE_CLASS: &str = "ml-auto flex shrink-0 items-center gap-2";

pub fn item_class(selected: bool, disabled: bool, class: &str) -> String {
  classes([
    Some(ITEM_BASE_CLASS),
    selected.then_some(ITEM_SELECTED_CLASS),
    disabled.then_some(ITEM_DISABLED_CLASS),
    Some(class),
  ])
}

pub fn item_media_class(class: &str) -> String {
  classes([Some(ITEM_MEDIA_BASE_CLASS), Some(class)])
}

pub fn item_content_class(class: &str) -> String {
  classes([Some(ITEM_CONTENT_BASE_CLASS), Some(class)])
}

pub fn item_title_class(class: &str) -> String {
  classes([Some(ITEM_TITLE_BASE_CLASS), Some(class)])
}

pub fn item_description_class(class: &str) -> String {
  classes([Some(ITEM_DESCRIPTION_BASE_CLASS), Some(class)])
}

pub fn item_actions_class(class: &str) -> String {
  classes([Some(ITEM_ACTIONS_BASE_CLASS), Some(class)])
}

#[component]
pub fn Item(
  #[props(default)] selected: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = item_class(selected, disabled, &class);

  rsx! {
    div {
      class,
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      "data-selected": selected.to_string(),
      {children}
    }
  }
}

#[component]
pub fn ItemMedia(#[props(default)] class: String, children: Element) -> Element {
  let class = item_media_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn ItemContent(#[props(default)] class: String, children: Element) -> Element {
  let class = item_content_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn ItemTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = item_title_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn ItemDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = item_description_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn ItemActions(#[props(default)] class: String, children: Element) -> Element {
  let class = item_actions_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn item_class_reflects_state() {
    let actual = item_class(true, true, "border");

    assert!(actual.contains(ITEM_BASE_CLASS));
    assert!(actual.contains(ITEM_SELECTED_CLASS));
    assert!(actual.contains(ITEM_DISABLED_CLASS));
    assert!(actual.ends_with("border"));
  }

  #[test]
  fn item_title_class_appends_user_class() {
    let actual = item_title_class("text-base");

    assert!(actual.contains(ITEM_TITLE_BASE_CLASS));
    assert!(actual.ends_with("text-base"));
  }
}
