use std::cell::Cell;
use std::rc::Rc;

use dioxus::prelude::*;

use super::element_id::next_element_id;
use super::script::{Script, component_script};

// Select, Combobox, and Command keep focus on the anchor (trigger or input) and track
// the highlighted option through `aria-activedescendant`; menus move DOM focus
// to the highlighted item instead. Items are read from the DOM on every key so
// items re-rendered while filtering are picked up. Sends the chosen item's
// `data-value`, empty for menu items. A submenu (RFC 0067) is a nested menu
// with its own script: each script acts on the items whose closest
// `[data-dxui-listbox]` is its own, and ignores keys from a nested menu.
component_script!(listbox_script = r#"
export async function run(dioxus) {
  const [scopeId, anchorId, mode] = await dioxus.recv();
  const listbox = document.querySelector(`[data-dxui-listbox="${scopeId}"]`);
  if (!listbox) return;
  const anchor = anchorId ? document.getElementById(anchorId) : null;
  const previous = document.activeElement;
  const isMenu = mode === "menu";
  const jumps = mode !== "combobox";
  // Space and typeahead are for modes where no input takes the typing.
  const searchesByKey = mode === "select" || isMenu;
  // Command goes back to the first option when the query changes.
  const resetsOnInput = mode === "command";
  const alwaysOpen = mode === "command";
  const itemSelector = isMenu
    ? '[role="menuitem"], [role="menuitemcheckbox"], [role="menuitemradio"]'
    : '[role="option"]';
  const isSubmenu = listbox.hasAttribute("data-dxui-submenu");
  const rtl = getComputedStyle(listbox).direction === "rtl";
  const forwardKey = rtl ? "ArrowLeft" : "ArrowRight";
  const backKey = rtl ? "ArrowRight" : "ArrowLeft";
  const owns = (element) => element.closest("[data-dxui-listbox]") === listbox;
  const opensSubmenu = (option) => isMenu && option?.getAttribute("aria-haspopup") === "menu";
  const enabled = (option) =>
    option.dataset.disabled !== "true" && option.getAttribute("aria-disabled") !== "true";
  const options = () => {
    const all = Array.from(listbox.querySelectorAll(itemSelector)).filter(owns);
    all.forEach((option, index) => {
      if (!option.id) option.id = `${scopeId}-option-${index}`;
      if (isMenu && !option.hasAttribute("tabindex")) option.tabIndex = -1;
    });
    return all.filter(enabled);
  };
  let highlighted = null;
  const highlight = (option) => {
    if (highlighted && highlighted !== option) delete highlighted.dataset.highlighted;
    highlighted = option;
    if (option) {
      option.dataset.highlighted = "";
      if (isMenu) {
        // The menu may still be in normal flow before placement makes it fixed;
        // scrolling the page then would move it away from its anchor point.
        option.focus({ preventScroll: true });
      } else {
        option.scrollIntoView({ block: "nearest" });
        if (anchor) anchor.setAttribute("aria-activedescendant", option.id);
      }
    } else if (anchor && !isMenu) {
      anchor.removeAttribute("aria-activedescendant");
    }
  };
  const initial = () => {
    if (mode === "combobox") return null;
    const list = options();
    const selected = isMenu ? null : list.find((option) => option.getAttribute("aria-selected") === "true");
    return selected || list[0] || null;
  };
  const choose = (option) => {
    if (option && enabled(option)) dioxus.send(option.dataset.value || "");
  };
  // A menu item is activated by clicking it, so its own click handler runs and
  // the click listener below reports the choice.
  const activate = (option) => (isMenu ? option.click() : choose(option));
  // A sub trigger's click asks the app to open its submenu, whose script then
  // focuses its first item; an already open one gets that focus here.
  const enterSubmenu = (trigger) => {
    const submenu = document.getElementById(trigger.getAttribute("aria-controls") || "");
    if (trigger.getAttribute("aria-expanded") !== "true" || !submenu) return trigger.click();
    const first = Array.from(submenu.querySelectorAll(itemSelector)).find(
      (option) => option.closest("[data-dxui-listbox]") === submenu && enabled(option),
    );
    if (first) first.focus({ preventScroll: true });
  };
  let buffer = "";
  let bufferedAt = 0;
  const typing = () => buffer !== "" && performance.now() - bufferedAt <= 500;
  const typeahead = (character) => {
    buffer = typing() ? buffer + character : character;
    bufferedAt = performance.now();
    // Repeating one letter cycles through its matches; a longer prefix keeps
    // the current option while it still matches.
    const repeated = Array.from(buffer).every((letter) => letter === buffer[0]);
    const query = repeated ? buffer[0] : buffer;
    const list = options();
    const current = list.indexOf(highlighted);
    const start = current < 0 ? 0 : current + (repeated ? 1 : 0);
    for (let offset = 0; offset < list.length; offset += 1) {
      const option = list[(start + offset) % list.length];
      if (option.textContent.trim().toLowerCase().startsWith(query)) return highlight(option);
    }
  };
  const onKeyDown = (event) => {
    if (event.defaultPrevented) return;
    if (isMenu && event.target instanceof Element && !owns(event.target)) return;
    if (isSubmenu && (event.key === backKey || event.key === "Escape")) {
      // Closes this level only: the outer menu and the document-level
      // Escape dismissal must not see the key.
      event.preventDefault();
      event.stopPropagation();
      dioxus.send("");
      return;
    }
    const list = options();
    const index = list.indexOf(highlighted);
    const last = list.length - 1;
    let handled = true;
    if (event.key === "ArrowDown") {
      const next = isMenu ? (index + 1) % list.length : Math.min(index + 1, last);
      highlight(list[next] || null);
    } else if (event.key === "ArrowUp") {
      const next = index < 0 ? last : isMenu ? (index - 1 + list.length) % list.length : Math.max(index - 1, 0);
      highlight(list[next] || null);
    } else if (jumps && event.key === "Home") {
      highlight(list[0] || null);
    } else if (jumps && event.key === "End") {
      highlight(list[last] || null);
    } else if (event.key === forwardKey && opensSubmenu(highlighted)) {
      enterSubmenu(highlighted);
    } else if (event.key === "Enter" && highlighted) {
      activate(highlighted);
    } else if (searchesByKey && event.key === " " && highlighted && !typing()) {
      activate(highlighted);
    } else if (searchesByKey && event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey) {
      typeahead(event.key.toLowerCase());
    } else {
      handled = false;
    }
    if (handled) event.preventDefault();
  };
  // Any enabled item inside, nested menus included: choosing a nested item
  // closes this menu too.
  const itemFrom = (target) => {
    const option = target instanceof Element ? target.closest(itemSelector) : null;
    return option && listbox.contains(option) && enabled(option) ? option : null;
  };
  const optionFrom = (target) => {
    const option = itemFrom(target);
    return option && owns(option) ? option : null;
  };
  const onPointerMove = (event) => {
    const option = optionFrom(event.target);
    if (option && option !== highlighted) highlight(option);
    if (opensSubmenu(option) && option.getAttribute("aria-expanded") !== "true") {
      // Hover opens the submenu without moving focus into it.
      option.dataset.dxuiPointerOpen = "";
      option.click();
    }
  };
  // Keeps the highlight on the focused item when focus arrives some other way,
  // such as from an outer menu entering this one.
  const onFocusIn = (event) => {
    const option = optionFrom(event.target);
    if (!option || option === highlighted) return;
    if (highlighted) delete highlighted.dataset.highlighted;
    highlighted = option;
    option.dataset.highlighted = "";
  };
  // Keep focus where the keys are read (the anchor, or the highlighted menu
  // item) when the pointer presses inside.
  const onPointerDown = (event) => event.preventDefault();
  const onClick = (event) => {
    const option = itemFrom(event.target);
    if (!opensSubmenu(option)) choose(option);
  };
  const keySource = isMenu ? listbox : anchor;
  // Resets at once, and again on the next DOM change, which is the app's
  // re-render of the filtered list.
  let resetPending = false;
  const onInput = () => {
    resetPending = true;
    highlight(initial());
  };
  let finish;
  const ended = new Promise((resolve) => {
    finish = resolve;
  });
  // A menu inside a closed menu is hidden by its ancestor. Command is always
  // open, so a hidden ancestor (a closed Dialog, Popover, or tab) only pauses
  // it: it keeps no highlight while hidden and starts over when shown.
  const hidden = () => listbox.closest("[hidden]") !== null;
  const closed = () => !listbox.isConnected || (!alwaysOpen && hidden());
  let wasHidden = false;
  const observer = new MutationObserver(() => {
    if (closed()) return finish();
    const isHidden = hidden();
    const shown = wasHidden && !isHidden;
    wasHidden = isHidden;
    if (isHidden) {
      if (highlighted) highlight(null);
    } else if (shown) {
      highlight(initial());
    } else if (resetPending) {
      resetPending = false;
      highlight(initial());
    } else if (highlighted && !options().includes(highlighted)) {
      highlight(initial());
    } else if (alwaysOpen && !highlighted) {
      // Results can arrive after the change that took the reset, as from a
      // debounced server search; Command highlights them when they do.
      const first = initial();
      if (first) highlight(first);
    }
  });
  await new Promise((resolve) => requestAnimationFrame(resolve));
  if (closed()) return;
  const pointerOpened = isSubmenu && anchor !== null && "dxuiPointerOpen" in anchor.dataset;
  if (anchor) delete anchor.dataset.dxuiPointerOpen;
  wasHidden = hidden();
  if (!pointerOpened && !wasHidden) highlight(initial());
  observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["hidden", "data-disabled", "aria-disabled"] });
  if (keySource) keySource.addEventListener("keydown", onKeyDown);
  if (resetsOnInput && anchor) anchor.addEventListener("input", onInput);
  listbox.addEventListener("pointermove", onPointerMove);
  listbox.addEventListener("pointerdown", onPointerDown);
  listbox.addEventListener("click", onClick);
  if (isMenu) listbox.addEventListener("focusin", onFocusIn);
  await ended;
  observer.disconnect();
  if (keySource) keySource.removeEventListener("keydown", onKeyDown);
  if (resetsOnInput && anchor) anchor.removeEventListener("input", onInput);
  listbox.removeEventListener("pointermove", onPointerMove);
  listbox.removeEventListener("pointerdown", onPointerDown);
  listbox.removeEventListener("click", onClick);
  if (isMenu) listbox.removeEventListener("focusin", onFocusIn);
  highlight(null);
  // Closed with the menu around it: ask the app to close this one as well.
  if (isSubmenu && listbox.isConnected && !listbox.hidden) dioxus.send("");
  if (isMenu) {
    // Focus still inside the hidden menu, or blurred to the body, goes back to
    // the anchor (a menubar may have switched menus since this one opened), or
    // without one to where it was before opening; focus moved to another
    // control stays there.
    const active = document.activeElement;
    const focusLeft = active !== null && active !== document.body && !listbox.contains(active);
    const returnTo = anchor || previous;
    if (!focusLeft && returnTo instanceof HTMLElement && returnTo.isConnected) returnTo.focus();
  }
}
"#);

/// How the listbox script reads keys and shows the highlighted item.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ListboxMode {
  /// Select: starts on the selected option, handles Home, End, Space, and
  /// typeahead.
  Select,
  /// Combobox: starts without a highlight and leaves typing to the input.
  Combobox,
  /// Command: like Select for Home and End, but leaves typing to the input
  /// and goes back to the first option when the query changes.
  Command,
  /// Menu: focuses the first item, moves DOM focus with wrapping arrows, Home,
  /// End, and typeahead, activates items by clicking them, and returns focus
  /// on close.
  Menu,
}

impl ListboxMode {
  fn name(self) -> &'static str {
    match self {
      Self::Select => "select",
      Self::Combobox => "combobox",
      Self::Command => "command",
      Self::Menu => "menu",
    }
  }

  /// Select, Combobox, and Command read keys from their anchor; menus read them from
  /// the focused item and also run without an anchor.
  fn needs_anchor(self) -> bool {
    match self {
      Self::Select => true,
      Self::Combobox => true,
      Self::Command => true,
      Self::Menu => false,
    }
  }
}

/// Runs the listbox script each time `open` turns true, with an `anchor_id`
/// unless the mode is a menu. Choosing an option calls `on_value_change(value)` and then
/// `on_open_change(false)`. Command is always open: its script runs until the
/// listbox leaves the page and pauses while an ancestor is hidden.
///
/// Returns the value for the content's `data-dxui-listbox` attribute.
pub(crate) fn use_listbox(
  open: bool,
  anchor_id: Option<String>,
  mode: ListboxMode,
  on_value_change: Option<EventHandler<String>>,
  on_open_change: Option<EventHandler<bool>>,
) -> String {
  let scope_id = use_hook(|| format!("dxui-listbox-{}", next_element_id()));
  let was_open = use_hook(|| Rc::new(Cell::new(false)));
  let effect_scope_id = scope_id.clone();

  use_effect(use_reactive(
    (&open, &anchor_id, &on_value_change, &on_open_change),
    move |(open, anchor_id, on_value_change, on_open_change)| {
      if open == was_open.replace(open) || !open || (anchor_id.is_none() && mode.needs_anchor()) {
        return;
      }
      let script = listbox_script::start();
      // A send error means the page already finished the script; nothing to track.
      let _ = script.send((effect_scope_id.as_str(), anchor_id.as_deref(), mode.name()));
      spawn(async move {
        while let Ok(value) = script.recv::<String>().await {
          if let Some(handler) = on_value_change {
            handler.call(value);
          }
          if let Some(handler) = on_open_change {
            handler.call(false);
          }
        }
      });
    },
  ));

  scope_id
}
