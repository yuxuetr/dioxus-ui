use super::element_id::next_element_id;
use super::root_state::use_controllable;
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

pub const MENU_BASE_CLASS: &str = "flex w-full flex-col gap-0.5 text-sm";
pub const MENU_TITLE_BASE_CLASS: &str = "px-3 pb-1 pt-3 text-xs font-medium text-muted-foreground";
pub const MENU_ITEM_BASE_CLASS: &str = "flex w-full items-center gap-2 rounded-md px-3 py-2 text-start transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
pub const MENU_ITEM_ACTIVE_CLASS: &str = "bg-accent font-medium text-accent-foreground";
/// A down chevron at the end of a group's button that turns up while it is
/// open; up and down need no mirroring in right-to-left.
pub const MENU_GROUP_TRIGGER_CLASS: &str = "after:ms-auto after:size-4 after:shrink-0 after:bg-current after:transition-transform data-[state=open]:after:rotate-180 after:[mask:url(data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20viewBox=%270%200%2016%2016%27%20fill=%27none%27%20stroke=%27black%27%20stroke-width=%272%27%20stroke-linecap=%27round%27%20stroke-linejoin=%27round%27%3E%3Cpath%20d=%27M3.5%206l4.5%204.5%204.5-4.5%27/%3E%3C/svg%3E)_center/contain_no-repeat]";
pub const MENU_GROUP_LIST_BASE_CLASS: &str =
  "ms-4 mt-0.5 flex flex-col gap-0.5 border-s border-border ps-2";

pub fn menu_class(class: &str) -> String {
  merge_classes(classes([Some(MENU_BASE_CLASS)]), class)
}

pub fn menu_title_class(class: &str) -> String {
  merge_classes(classes([Some(MENU_TITLE_BASE_CLASS)]), class)
}

pub fn menu_item_class(active: bool, class: &str) -> String {
  let state_class = if active { MENU_ITEM_ACTIVE_CLASS } else { "text-foreground" };
  merge_classes(classes([Some(MENU_ITEM_BASE_CLASS), Some(state_class)]), class)
}

pub fn menu_group_list_class(class: &str) -> String {
  merge_classes(classes([Some(MENU_GROUP_LIST_BASE_CLASS)]), class)
}

/// A vertical navigation list. Put it in a `nav` named for what it lists,
/// with `MenuTitle`, `MenuItem`, and `MenuGroup` children.
#[component]
pub fn Menu(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = ul)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = menu_class(&class);

  rsx! {
    ul {
      class,
      ..attributes,
      {children}
    }
  }
}

/// A section label between items.
#[component]
pub fn MenuTitle(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = li)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = menu_title_class(&class);

  rsx! {
    li {
      class,
      ..attributes,
      {children}
    }
  }
}

/// A list item holding a link with an `href`, a button with an `onclick`,
/// or otherwise the app's own content. `active` marks the current page with
/// `aria-current="page"`; a disabled link drops its `href`.
#[component]
pub fn MenuItem(
  #[props(default)] active: bool,
  #[props(default)] disabled: bool,
  #[props(default)] href: String,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = menu_item_class(active, &class);
  let aria_current = active.then_some("page");
  let has_onclick = onclick.is_some();
  let onclick = move |event: MouseEvent| {
    if !disabled {
      if let Some(handler) = onclick {
        handler.call(event);
      }
    }
  };

  rsx! {
    li {
      if !href.is_empty() {
        a {
          class,
          href: (!disabled).then_some(href),
          role: disabled.then_some("link"),
          "aria-current": aria_current,
          "aria-disabled": disabled.then_some("true"),
          "data-active": active.to_string(),
          "data-disabled": disabled.to_string(),
          onclick,
          {children}
        }
      } else if has_onclick {
        button {
          r#type: "button",
          class,
          disabled,
          "aria-current": aria_current,
          "data-active": active.to_string(),
          onclick,
          {children}
        }
      } else {
        div {
          class,
          "aria-disabled": disabled.then_some("true"),
          "data-active": active.to_string(),
          "data-disabled": disabled.to_string(),
          {children}
        }
      }
    }
  }
}

/// A collapsible group of items. Its button shows `label` and toggles the
/// nested list, which is hidden while closed. The group owns whether it is
/// open (RFC 0077): pass `open` to control it, or `default_open` to start it;
/// `on_open_change` hears every change the user makes either way.
#[component]
pub fn MenuGroup(
  label: Element,
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] disabled: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default)] class: String,
  #[props(default)] list_class: String,
  children: Element,
) -> Element {
  let group = use_controllable(move || open.cloned(), move || default_open, on_open_change);
  let open = group.get();
  let list_id = use_hook(|| format!("dxui-menu-group-{}", next_element_id()));
  let class =
    menu_item_class(false, &merge_classes(classes([Some(MENU_GROUP_TRIGGER_CLASS)]), &class));
  let list_class = menu_group_list_class(&list_class);
  let state = if open { "open" } else { "closed" };

  rsx! {
    li {
      button {
        r#type: "button",
        class,
        disabled,
        "aria-expanded": open.to_string(),
        "aria-controls": list_id.clone(),
        "data-state": state,
        onclick: move |_| {
          if !disabled {
            group.set(!group.get());
          }
        },
        {label}
      }
      ul {
        id: list_id,
        class: list_class,
        hidden: !open,
        "data-state": state,
        {children}
      }
    }
  }
}
