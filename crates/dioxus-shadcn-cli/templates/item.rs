//! Item: parts for a list or result row, such as a settings row or search result,
//! with media, content, and actions; the app owns the collection.
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

const ITEM_BASE_CLASS: &str = "flex items-start gap-3 rounded-md p-3 text-sm transition-colors data-[disabled=true]:opacity-50 data-[selected=true]:bg-accent";
const ITEM_SELECTED_CLASS: &str = "bg-accent";
const ITEM_DISABLED_CLASS: &str = "pointer-events-none opacity-50";
const ITEM_MEDIA_BASE_CLASS: &str = "flex shrink-0 items-center justify-center";
const ITEM_CONTENT_BASE_CLASS: &str = "grid min-w-0 flex-1 gap-1";
const ITEM_TITLE_BASE_CLASS: &str = "truncate font-medium text-foreground";
const ITEM_DESCRIPTION_BASE_CLASS: &str = "line-clamp-2 text-sm text-muted-foreground";
const ITEM_ACTIONS_BASE_CLASS: &str = "ml-auto flex shrink-0 items-center gap-2";

/// Classes for the row: base classes, the selected background, the disabled dimming,
/// then `class` merged over them.
pub fn item_class(selected: bool, disabled: bool, class: &str) -> String {
  merge_classes(classes([Some(ITEM_BASE_CLASS), selected.then_some(ITEM_SELECTED_CLASS), disabled.then_some(ITEM_DISABLED_CLASS)]), class)
}

/// Classes for the media slot, such as an icon or avatar, then `class` merged over them.
pub fn item_media_class(class: &str) -> String {
  merge_classes(classes([Some(ITEM_MEDIA_BASE_CLASS)]), class)
}

/// Classes for the content column that holds the title and description, then
/// `class` merged over them.
pub fn item_content_class(class: &str) -> String {
  merge_classes(classes([Some(ITEM_CONTENT_BASE_CLASS)]), class)
}

/// Classes for the title: one truncated line of medium text, then `class` merged
/// over them.
pub fn item_title_class(class: &str) -> String {
  merge_classes(classes([Some(ITEM_TITLE_BASE_CLASS)]), class)
}

/// Classes for the description: muted text clamped to two lines, then `class`
/// merged over them.
pub fn item_description_class(class: &str) -> String {
  merge_classes(classes([Some(ITEM_DESCRIPTION_BASE_CLASS)]), class)
}

/// Classes for the actions slot, pushed to the end of the row, then `class` merged
/// over them.
pub fn item_actions_class(class: &str) -> String {
  merge_classes(classes([Some(ITEM_ACTIONS_BASE_CLASS)]), class)
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
