//! Tree: nested items, such as folders, that expand and collapse, with
//! WAI-ARIA tree keyboard navigation.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

use crate::element_id::next_element_id;
use crate::root_state::{Controllable, use_controllable, use_root_context};
use crate::script::{Script, component_script};

const TREE_BASE_CLASS: &str = "grid gap-0.5 text-sm";
const TREE_GROUP_BASE_CLASS: &str = "grid gap-0.5";
const TREE_ITEM_BASE_CLASS: &str = "outline-none";
const TREE_ITEM_ROW_BASE_CLASS: &str = "flex min-h-8 cursor-pointer select-none items-center gap-1.5 rounded-md py-1 pe-2 hover:bg-accent hover:text-accent-foreground [[aria-selected=true]>&]:bg-accent [[aria-selected=true]>&]:font-medium [[aria-selected=true]>&]:text-accent-foreground [[role=treeitem]:focus-visible>&]:ring-2 [[role=treeitem]:focus-visible>&]:ring-ring [[aria-disabled=true]>&]:cursor-not-allowed [[aria-disabled=true]>&]:opacity-50";
const TREE_ITEM_CHEVRON_CLASS: &str = "size-4 shrink-0 text-muted-foreground transition-transform";

// Runs for the tree's lifetime and reads the items from the DOM on every
// event. Up, Down, Home, End, and typeahead move focus among visible items;
// Right opens a closed branch or moves into an open one, Left closes an open
// branch or moves to the parent (mirrored right to left); Enter and Space
// select. Sends `[action, value]` with action "expand", "collapse", or
// "select". One visible item is the Tab stop: the selected one, else the one
// focused last, else the first.
// Keep in sync with `tree_script` in the CLI `tree.rs` template.
component_script!(
  tree_script = r#"
export async function run(dioxus) {
  const scopeId = await dioxus.recv();
  const root = document.querySelector(`[data-dxui-tree="${scopeId}"]`);
  if (!root) return;
  const own = (item) => item.closest("[data-dxui-tree]") === root;
  const items = () =>
    Array.from(root.querySelectorAll('[role="treeitem"]')).filter(
      (item) => own(item) && !item.closest('[role="group"][hidden]'),
    );
  const enabled = () => items().filter((item) => item.getAttribute("aria-disabled") !== "true");
  const label = (item) => (item.querySelector(":scope > [data-dxui-tree-row]")?.textContent || "").trim().toLowerCase();
  let lastFocused = null;
  const setTabStop = () => {
    const visible = enabled();
    const stop =
      visible.find((item) => item.getAttribute("aria-selected") === "true") ||
      (visible.includes(lastFocused) ? lastFocused : null) ||
      visible[0];
    root.querySelectorAll('[role="treeitem"]').forEach((item) => {
      if (own(item)) item.tabIndex = item === stop ? 0 : -1;
    });
  };
  let typed = "";
  let typedAt = 0;
  const typeahead = (item, key) => {
    const now = Date.now();
    typed = now - typedAt > 500 ? key : typed + key;
    typedAt = now;
    const visible = enabled();
    const start = visible.indexOf(item);
    // A repeated letter steps through the items that start with it.
    const repeated = typed.length > 1 && [...typed].every((letter) => letter === typed[0]);
    const search = repeated ? typed[0] : typed;
    const offset = search.length === 1 ? 1 : 0;
    for (let index = 0; index < visible.length; index += 1) {
      const candidate = visible[(start + offset + index) % visible.length];
      if (label(candidate).startsWith(search)) return candidate;
    }
    return null;
  };
  const visualKey = (key) => {
    if (getComputedStyle(root).direction !== "rtl") return key;
    if (key === "ArrowLeft") return "ArrowRight";
    if (key === "ArrowRight") return "ArrowLeft";
    return key;
  };
  const onKeyDown = (event) => {
    if (event.defaultPrevented || event.altKey || event.ctrlKey || event.metaKey) return;
    const item = event.target;
    if (!(item instanceof Element) || item.getAttribute("role") !== "treeitem" || !own(item)) return;
    const visible = enabled();
    const index = visible.indexOf(item);
    const expanded = item.getAttribute("aria-expanded");
    let next = null;
    switch (visualKey(event.key)) {
      case "ArrowDown":
        next = visible[index + 1] || item;
        break;
      case "ArrowUp":
        next = index > 0 ? visible[index - 1] : item;
        break;
      case "Home":
        next = visible[0];
        break;
      case "End":
        next = visible[visible.length - 1];
        break;
      case "ArrowRight":
        if (expanded === "false") dioxus.send(["expand", item.dataset.value || ""]);
        else if (expanded === "true") next = enabled().find((child) => child.parentElement.closest('[role="treeitem"]') === item);
        next = next || item;
        break;
      case "ArrowLeft":
        if (expanded === "true") {
          dioxus.send(["collapse", item.dataset.value || ""]);
          next = item;
        } else {
          next = item.parentElement.closest('[role="treeitem"]');
          next = next && own(next) ? next : item;
        }
        break;
      case "Enter":
      case " ":
        if (item.getAttribute("aria-disabled") !== "true") dioxus.send(["select", item.dataset.value || ""]);
        next = item;
        break;
      default:
        if (event.key.length === 1 && event.key !== " ") next = typeahead(item, event.key.toLowerCase());
        if (!next) return;
    }
    event.preventDefault();
    if (next !== item) next.focus();
  };
  const onFocusIn = (event) => {
    if (!(event.target instanceof Element) || event.target.getAttribute("role") !== "treeitem" || !own(event.target)) return;
    lastFocused = event.target;
    root.querySelectorAll('[role="treeitem"]').forEach((item) => {
      if (own(item)) item.tabIndex = item === event.target ? 0 : -1;
    });
  };
  const onFocusOut = (event) => {
    if (!root.contains(event.relatedTarget)) setTabStop();
  };
  let finish;
  const ended = new Promise((resolve) => {
    finish = resolve;
  });
  const observer = new MutationObserver(() => {
    if (!root.isConnected) return finish();
    if (!root.contains(document.activeElement)) setTabStop();
  });
  setTabStop();
  observer.observe(root, { subtree: true, childList: true, attributes: true, attributeFilter: ["aria-selected", "aria-disabled", "hidden"] });
  root.addEventListener("keydown", onKeyDown);
  root.addEventListener("focusin", onFocusIn);
  root.addEventListener("focusout", onFocusOut);
  await ended;
  observer.disconnect();
}
"#
);

/// Classes for the tree: base classes with `class` merged over them.
pub fn tree_class(class: &str) -> String {
  merge_classes(classes([Some(TREE_BASE_CLASS)]), class)
}

/// Classes for a branch's group of child items: base classes with `class`
/// merged over them.
pub fn tree_group_class(class: &str) -> String {
  merge_classes(classes([Some(TREE_GROUP_BASE_CLASS)]), class)
}

/// Classes for an item's row: base classes, which follow the item's
/// `aria-selected`, focus, and `aria-disabled`, with `class` merged over them.
pub fn tree_item_row_class(class: &str) -> String {
  merge_classes(classes([Some(TREE_ITEM_ROW_BASE_CLASS)]), class)
}

#[derive(Clone, Copy)]
struct TreeContext {
  expanded: Controllable<Vec<String>>,
  selected: Controllable<String>,
}

impl TreeContext {
  fn set_expanded(&self, value: &str, open: bool) {
    let mut expanded = self.expanded.get();
    let position = expanded.iter().position(|item| item == value);
    match (open, position) {
      (true, None) => expanded.push(value.to_string()),
      (false, Some(index)) => {
        expanded.remove(index);
      }
      _ => return,
    }
    self.expanded.set(expanded);
  }
}

/// How deep an item sits: 1 for the tree's own items.
#[derive(Clone, Copy)]
struct TreeLevel(usize);

/// A tree of items (WAI-ARIA tree). The tree owns which branches are open
/// and which item is selected (RFC 0077): `expanded` and `selected` control
/// them, or `default_expanded` and `default_selected` start them, and
/// `on_expanded_change` and `on_selected_change` hear each change. Name it
/// with `aria-label` or `aria-labelledby`.
#[component]
pub fn Tree(
  #[props(default)] expanded: ReadSignal<Option<Vec<String>>>,
  #[props(default)] default_expanded: Vec<String>,
  #[props(default)] on_expanded_change: Option<EventHandler<Vec<String>>>,
  #[props(default)] selected: ReadSignal<Option<String>>,
  #[props(default)] default_selected: String,
  #[props(default)] on_selected_change: Option<EventHandler<String>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = ul)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = tree_class(&class);
  let expanded =
    use_controllable(move || expanded.cloned(), move || default_expanded, on_expanded_change);
  let selected =
    use_controllable(move || selected.cloned(), move || default_selected, on_selected_change);
  let context = use_context_provider(|| TreeContext { expanded, selected });
  use_context_provider(|| TreeLevel(1));
  let scope_id = use_hook(|| format!("dxui-tree-{}", next_element_id()));
  let effect_scope_id = scope_id.clone();

  use_effect(move || {
    let script = tree_script::start();
    // A send error means the page already finished the script; nothing to track.
    let _ = script.send(effect_scope_id.as_str());
    spawn(async move {
      while let Ok((action, value)) = script.recv::<(String, String)>().await {
        match action.as_str() {
          "expand" => context.set_expanded(&value, true),
          "collapse" => context.set_expanded(&value, false),
          "select" => context.selected.set(value),
          _ => {}
        }
      }
    });
  });

  rsx! {
    ul {
      role: "tree",
      class,
      "data-dxui-tree": scope_id,
      ..attributes,
      {children}
    }
  }
}

/// An item: `children` is its label and `group` its child items, which make
/// it a branch that opens and closes. A click selects it and, on a branch,
/// opens or closes it. A disabled item can be reached but not selected.
#[component]
pub fn TreeItem(
  value: String,
  #[props(default)] group: Option<Element>,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] group_class: String,
  children: Element,
) -> Element {
  let context = use_root_context::<TreeContext>("TreeItem", "Tree");
  let TreeLevel(level) = use_root_context::<TreeLevel>("TreeItem", "Tree");
  use_context_provider(|| TreeLevel(level + 1));
  let class = tree_item_row_class(&class);
  let group_class = tree_group_class(&group_class);
  let branch = group.is_some();
  let open = branch && context.expanded.get().contains(&value);
  let selected = context.selected.get() == value;
  // Each level indents by 1rem from the row's own 0.5rem.
  let indent = format!("padding-inline-start: {}rem;", 0.5 + (level - 1) as f32);
  let click_value = value.clone();

  rsx! {
    li {
      role: "treeitem",
      class: TREE_ITEM_BASE_CLASS,
      "aria-level": level.to_string(),
      "aria-selected": selected.to_string(),
      "aria-expanded": branch.then(|| open.to_string()),
      "aria-disabled": disabled.then_some("true"),
      "data-value": value,
      div {
        class,
        style: indent,
        "data-dxui-tree-row": "",
        onclick: move |_| {
          if disabled {
            return;
          }
          context.selected.set(click_value.clone());
          if branch {
            context.set_expanded(&click_value, !open);
          }
        },
        if branch {
          svg {
            class: if open { format!("{TREE_ITEM_CHEVRON_CLASS} rotate-90") } else { TREE_ITEM_CHEVRON_CLASS.to_string() },
            "aria-hidden": "true",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "m9 18 6-6-6-6" }
          }
        } else {
          span { class: "size-4 shrink-0", "aria-hidden": "true" }
        }
        {children}
      }
      if let Some(group) = group {
        ul { role: "group", class: group_class, hidden: !open, {group} }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn items_carry_roles_levels_and_state() {
    let html = render(|| {
      rsx! {
        Tree { "aria-label": "Files", default_expanded: vec!["src".to_string()], default_selected: "main",
          TreeItem {
            value: "src",
            group: rsx! {
              TreeItem { value: "main", "main.rs" }
              TreeItem {
                value: "ui",
                group: rsx! {
                  TreeItem { value: "button", "button.rs" }
                },
                "ui"
              }
            },
            "src"
          }
          TreeItem { value: "readme", disabled: true, "README.md" }
        }
      }
    });

    assert!(html.contains(r#"role="tree""#), "{html}");
    assert!(html.contains(r#"aria-label="Files""#));
    assert_eq!(html.matches(r#"role="treeitem""#).count(), 5);
    assert_eq!(html.matches(r#"role="group""#).count(), 2);
    assert!(
      html
        .contains(r#"aria-level="1" aria-selected="false" aria-expanded="true" data-value="src""#),
      "{html}"
    );
    assert!(html.contains(r#"aria-level="2" aria-selected="true" data-value="main""#), "{html}");
    assert!(
      html
        .contains(r#"aria-level="2" aria-selected="false" aria-expanded="false" data-value="ui""#),
      "{html}"
    );
    assert!(html.contains(r#"aria-level="3" aria-selected="false" data-value="button""#), "{html}");
    assert!(html.contains(r#"aria-disabled="true" data-value="readme""#), "{html}");
    // The closed branch hides its group; the open one does not.
    assert_eq!(html.matches(r#"role="group" class="grid gap-0.5" hidden"#).count(), 1, "{html}");
    assert!(html.contains("padding-inline-start: 2.5rem;"), "{html}");
  }

  #[test]
  fn row_class_lets_the_caller_override() {
    assert!(tree_item_row_class("rounded-none").contains("rounded-none"));
    assert!(!tree_item_row_class("rounded-none").contains("rounded-md"));
  }
}
