use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use dioxus_shadcn_core::classes;

static NEXT_MENU_GROUP_ID: AtomicUsize = AtomicUsize::new(0);

pub const MENU_BASE_CLASS: &str = "flex w-full flex-col gap-0.5 text-sm";
pub const MENU_TITLE_BASE_CLASS: &str = "px-3 pb-1 pt-3 text-xs font-medium text-muted-foreground";
pub const MENU_ITEM_BASE_CLASS: &str = "flex w-full items-center gap-2 rounded-md px-3 py-2 text-start text-foreground transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
pub const MENU_ITEM_ACTIVE_CLASS: &str = "bg-accent font-medium text-accent-foreground";
/// A down chevron at the end of a group's button that turns up while it is
/// open; up and down need no mirroring in right-to-left.
pub const MENU_GROUP_TRIGGER_CLASS: &str = "after:ms-auto after:size-4 after:shrink-0 after:bg-current after:transition-transform data-[state=open]:after:rotate-180 after:[mask:url(data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20viewBox=%270%200%2016%2016%27%20fill=%27none%27%20stroke=%27black%27%20stroke-width=%272%27%20stroke-linecap=%27round%27%20stroke-linejoin=%27round%27%3E%3Cpath%20d=%27M3.5%206l4.5%204.5%204.5-4.5%27/%3E%3C/svg%3E)_center/contain_no-repeat]";
pub const MENU_GROUP_LIST_BASE_CLASS: &str =
  "ms-4 mt-0.5 flex flex-col gap-0.5 border-s border-border ps-2";

pub fn menu_class(class: &str) -> String {
  classes([Some(MENU_BASE_CLASS), Some(class)])
}

pub fn menu_title_class(class: &str) -> String {
  classes([Some(MENU_TITLE_BASE_CLASS), Some(class)])
}

pub fn menu_item_class(active: bool, class: &str) -> String {
  classes([Some(MENU_ITEM_BASE_CLASS), active.then_some(MENU_ITEM_ACTIVE_CLASS), Some(class)])
}

pub fn menu_group_list_class(class: &str) -> String {
  classes([Some(MENU_GROUP_LIST_BASE_CLASS), Some(class)])
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

/// A collapsible group of items. Its button shows `label`, reports the
/// requested state through `on_open_change`, and controls the nested list,
/// which is hidden while `open` is false.
#[component]
pub fn MenuGroup(
  label: Element,
  #[props(default)] open: bool,
  #[props(default)] disabled: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default)] class: String,
  #[props(default)] list_class: String,
  children: Element,
) -> Element {
  let list_id =
    use_hook(|| format!("dxui-menu-group-{}", NEXT_MENU_GROUP_ID.fetch_add(1, Ordering::Relaxed)));
  let class = menu_item_class(false, &classes([Some(MENU_GROUP_TRIGGER_CLASS), Some(&class)]));
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
          if let Some(handler) = on_open_change.filter(|_| !disabled) {
            handler.call(!open);
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn menu_item_class_marks_the_active_item() {
    assert!(menu_item_class(true, "gap-3").contains(MENU_ITEM_ACTIVE_CLASS));
    assert!(!menu_item_class(false, "").contains(MENU_ITEM_ACTIVE_CLASS));
    assert!(menu_item_class(false, "gap-3").ends_with("gap-3"));
  }

  #[test]
  fn ssr_renders_items_titles_and_groups() {
    fn app() -> Element {
      rsx! {
        Menu { "aria-label": "Docs",
          MenuTitle { "Guides" }
          MenuItem { href: "/start", active: true, "Getting started" }
          MenuItem { href: "/old", disabled: true, "Old guide" }
          MenuItem { onclick: move |_| {}, "Search" }
          MenuGroup { label: rsx! { "Components" }, open: false,
            MenuItem { href: "/button", "Button" }
          }
        }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains(r#"<ul class="flex w-full"#));
    assert!(html.contains(r#"aria-current="page""#));
    assert!(html.contains(r#"href="/start""#));
    assert!(!html.contains(r#"href="/old""#));
    assert!(html.contains(r#"role="link""#));
    assert!(html.contains(r#"<button type="button""#));
    assert!(html.contains(r#"aria-expanded="false""#));
    assert!(html.contains("hidden"));
    let controls = html.split(r#"aria-controls=""#).nth(1).and_then(|rest| rest.split('"').next());
    assert!(controls.is_some_and(|id| html.contains(&format!(r#"id="{id}""#))));
  }
}
