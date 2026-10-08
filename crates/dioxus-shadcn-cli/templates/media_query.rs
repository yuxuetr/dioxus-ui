//! Viewport media queries reported to Rust (RFC 0069).

use std::cell::Cell;
use std::rc::Rc;

use dioxus::prelude::*;

use super::element_id::next_element_id;
use super::script::{Script, component_script};

// Reports whether the query matches, and again on every change, until the
// element carrying the scope id is removed.
component_script!(media_query_script = r#"
export async function run(dioxus) {
  const [scopeId, query] = await dioxus.recv();
  const present = () => document.querySelector(`[data-dxui-media="${scopeId}"]`) !== null;
  await new Promise((resolve) => requestAnimationFrame(resolve));
  if (!present()) return;
  const list = window.matchMedia(query);
  const report = () => dioxus.send(list.matches);
  report();
  list.addEventListener("change", report);
  await new Promise((resolve) => {
    const observer = new MutationObserver(() => {
      if (!present()) {
        observer.disconnect();
        resolve();
      }
    });
    observer.observe(document.documentElement, { subtree: true, childList: true });
  });
  list.removeEventListener("change", report);
}
"#);

/// Whether `query` matches the viewport, once `enabled`; false until the page
/// answers, and in server-side rendering.
///
/// Returns the value and the `data-dxui-media` attribute for an element that
/// stays mounted while the value is needed.
pub(crate) fn use_media_query(query: &'static str, enabled: bool) -> (bool, String) {
  let scope_id = use_hook(|| format!("dxui-media-{}", next_element_id()));
  let mut matches = use_signal(|| false);
  let started = use_hook(|| Rc::new(Cell::new(false)));
  let effect_scope_id = scope_id.clone();

  use_effect(use_reactive((&enabled,), move |(enabled,)| {
    if !enabled || started.replace(true) {
      return;
    }
    let script = media_query_script::start();
    // A send error means the page already finished the script; nothing to track.
    let _ = script.send((effect_scope_id.as_str(), query));
    spawn(async move {
      while let Ok(value) = script.recv::<bool>().await {
        matches.set(value);
      }
    });
  }));

  (matches(), scope_id)
}
