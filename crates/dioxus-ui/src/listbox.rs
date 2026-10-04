use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

static NEXT_LISTBOX_ID: AtomicUsize = AtomicUsize::new(0);

// Keeps focus on the anchor (Select trigger or Combobox input) and tracks the
// highlighted option through `aria-activedescendant`. Options are read from
// the DOM on every key so items re-rendered while filtering are picked up.
// Sends the chosen option's `data-value`.
// Keep in sync with `LISTBOX_SCRIPT` in the CLI `utils.rs` template.
pub(crate) const LISTBOX_SCRIPT: &str = r#"
const [scopeId, anchorId, mode] = await dioxus.recv();
const listbox = document.querySelector(`[data-dxui-listbox="${scopeId}"]`);
if (!listbox) return;
const anchor = anchorId ? document.getElementById(anchorId) : null;
const isSelect = mode === "select";
const enabled = (option) =>
  option.dataset.disabled !== "true" && option.getAttribute("aria-disabled") !== "true";
const options = () => {
  const all = Array.from(listbox.querySelectorAll('[role="option"]'));
  all.forEach((option, index) => {
    if (!option.id) option.id = `${scopeId}-option-${index}`;
  });
  return all.filter(enabled);
};
let highlighted = null;
const highlight = (option) => {
  if (highlighted && highlighted !== option) delete highlighted.dataset.highlighted;
  highlighted = option;
  if (option) {
    option.dataset.highlighted = "";
    option.scrollIntoView({ block: "nearest" });
    if (anchor) anchor.setAttribute("aria-activedescendant", option.id);
  } else if (anchor) {
    anchor.removeAttribute("aria-activedescendant");
  }
};
const initial = () => {
  if (!isSelect) return null;
  const list = options();
  return list.find((option) => option.getAttribute("aria-selected") === "true") || list[0] || null;
};
const choose = (option) => {
  if (option && enabled(option)) dioxus.send(option.dataset.value || "");
};
let buffer = "";
let bufferedAt = 0;
const typing = () => buffer !== "" && performance.now() - bufferedAt <= 500;
const typeahead = (character) => {
  buffer = typing() ? buffer + character : character;
  bufferedAt = performance.now();
  const list = options();
  const start = list.indexOf(highlighted) + 1;
  for (let offset = 0; offset < list.length; offset += 1) {
    const option = list[(start + offset) % list.length];
    if (option.textContent.trim().toLowerCase().startsWith(buffer)) return highlight(option);
  }
};
const onKeyDown = (event) => {
  const list = options();
  const index = list.indexOf(highlighted);
  let handled = true;
  if (event.key === "ArrowDown") {
    highlight(list[Math.min(index + 1, list.length - 1)] || null);
  } else if (event.key === "ArrowUp") {
    highlight(list[index < 0 ? list.length - 1 : Math.max(index - 1, 0)] || null);
  } else if (isSelect && event.key === "Home") {
    highlight(list[0] || null);
  } else if (isSelect && event.key === "End") {
    highlight(list[list.length - 1] || null);
  } else if (event.key === "Enter" && highlighted) {
    choose(highlighted);
  } else if (isSelect && event.key === " " && !typing()) {
    choose(highlighted);
  } else if (isSelect && event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey) {
    typeahead(event.key.toLowerCase());
  } else {
    handled = false;
  }
  if (handled) event.preventDefault();
};
const optionFrom = (target) => {
  const option = target instanceof Element ? target.closest('[role="option"]') : null;
  return option && listbox.contains(option) && enabled(option) ? option : null;
};
const onPointerMove = (event) => {
  const option = optionFrom(event.target);
  if (option && option !== highlighted) highlight(option);
};
// Keep focus on the anchor so keys keep reaching it after a click.
const onPointerDown = (event) => event.preventDefault();
const onClick = (event) => choose(optionFrom(event.target));
let finish;
const ended = new Promise((resolve) => {
  finish = resolve;
});
const observer = new MutationObserver(() => {
  if (!listbox.isConnected || listbox.hidden) return finish();
  if (highlighted && !options().includes(highlighted)) highlight(initial());
});
await new Promise((resolve) => requestAnimationFrame(resolve));
if (!listbox.isConnected || listbox.hidden) return;
highlight(initial());
observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["hidden", "data-disabled", "aria-disabled"] });
if (anchor) anchor.addEventListener("keydown", onKeyDown);
listbox.addEventListener("pointermove", onPointerMove);
listbox.addEventListener("pointerdown", onPointerDown);
listbox.addEventListener("click", onClick);
await ended;
observer.disconnect();
if (anchor) anchor.removeEventListener("keydown", onKeyDown);
listbox.removeEventListener("pointermove", onPointerMove);
listbox.removeEventListener("pointerdown", onPointerDown);
listbox.removeEventListener("click", onClick);
highlight(null);
"#;

/// How the listbox script treats keys on the anchor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ListboxMode {
  /// Select: starts on the selected option, handles Home, End, Space, and
  /// typeahead.
  #[cfg(feature = "select")]
  Select,
}

impl ListboxMode {
  fn name(self) -> &'static str {
    match self {
      #[cfg(feature = "select")]
      Self::Select => "select",
    }
  }
}

/// Runs the listbox script each time `open` turns true with an `anchor_id`.
/// Choosing an option calls `on_value_change(value)` and then
/// `on_open_change(false)`.
///
/// Returns the value for the content's `data-dxui-listbox` attribute.
pub(crate) fn use_listbox(
  open: bool,
  anchor_id: Option<String>,
  mode: ListboxMode,
  on_value_change: Option<EventHandler<String>>,
  on_open_change: Option<EventHandler<bool>>,
) -> String {
  let scope_id =
    use_hook(|| format!("dxui-listbox-{}", NEXT_LISTBOX_ID.fetch_add(1, Ordering::Relaxed)));
  let was_open = use_hook(|| Rc::new(Cell::new(false)));
  let effect_scope_id = scope_id.clone();

  use_effect(use_reactive(
    (&open, &anchor_id, &on_value_change, &on_open_change),
    move |(open, anchor_id, on_value_change, on_open_change)| {
      if open == was_open.replace(open) || !open || anchor_id.is_none() {
        return;
      }
      let mut eval = document::eval(LISTBOX_SCRIPT);
      // A send error means the page already finished the script; nothing to track.
      let _ = eval.send((effect_scope_id.as_str(), anchor_id.as_deref(), mode.name()));
      spawn(async move {
        while let Ok(value) = eval.recv::<String>().await {
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
