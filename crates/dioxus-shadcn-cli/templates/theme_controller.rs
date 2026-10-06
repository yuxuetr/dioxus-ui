use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

static NEXT_THEME_CONTROLLER_ID: AtomicUsize = AtomicUsize::new(0);

/// Where `ThemeController` keeps the chosen theme unless told otherwise.
pub const THEME_STORAGE_KEY: &str = "dxui-theme";

/// A color theme: the system's light or dark scheme, one of the two, or a
/// preset that brings its own scheme (RFC 0057).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Theme {
  #[default]
  System,
  Light,
  Dark,
  Preset(String),
}

impl Theme {
  /// The stored form: `system`, `light`, `dark`, or the preset's name.
  pub fn as_str(&self) -> &str {
    match self {
      Self::System => "system",
      Self::Light => "light",
      Self::Dark => "dark",
      Self::Preset(name) => name,
    }
  }

  /// Reads the stored form; empty text is `System`.
  pub fn parse(value: &str) -> Self {
    match value {
      "" | "system" => Self::System,
      "light" => Self::Light,
      "dark" => Self::Dark,
      name => Self::Preset(name.to_string()),
    }
  }
}

// Applies the theme to the root element: `data-theme` for a preset, and the
// `dark` class for the dark scheme, following the system while the theme is
// `system`. Reports a stored theme that differs from the app's, then applies
// and stores each theme the app sends, until the controller is removed.
// Keep in sync with `THEME_CONTROLLER_SCRIPT` in the crate `theme_controller.rs`.
pub(crate) const THEME_CONTROLLER_SCRIPT: &str = r#"
const [scopeId, storageKey, initial] = await dioxus.recv();
const root = document.documentElement;
const media = window.matchMedia("(prefers-color-scheme: dark)");
const present = () => document.querySelector(`[data-dxui-theme-controller="${scopeId}"]`) !== null;
let theme = initial;
const apply = () => {
  const preset = !["system", "light", "dark"].includes(theme);
  if (preset) root.dataset.theme = theme;
  else delete root.dataset.theme;
  root.classList.toggle("dark", !preset && (theme === "dark" || (theme === "system" && media.matches)));
};
// Storage can be missing or blocked, as in some private windows.
const read = () => {
  try {
    return storageKey ? window.localStorage.getItem(storageKey) : null;
  } catch {
    return null;
  }
};
const write = () => {
  try {
    if (storageKey) window.localStorage.setItem(storageKey, theme);
  } catch {}
};
const stored = read();
if (stored !== null && stored !== theme) {
  theme = stored;
  dioxus.send(stored);
}
apply();
media.addEventListener("change", apply);
let running = true;
(async () => {
  while (running) {
    theme = await dioxus.recv();
    apply();
    write();
  }
})();
await new Promise((resolve) => {
  const observer = new MutationObserver(() => {
    if (!present()) {
      observer.disconnect();
      resolve();
    }
  });
  observer.observe(document.documentElement, { subtree: true, childList: true });
});
running = false;
media.removeEventListener("change", apply);
"#;

/// A script for the page's `<head>` that applies the stored theme before the
/// app renders, so a dark or preset theme does not flash light first. Use
/// the `storage_key` the `ThemeController` uses.
pub fn theme_init_script(storage_key: &str) -> String {
  format!(
    r#"(() => {{ try {{ const t = localStorage.getItem({storage_key:?}) || "system"; const r = document.documentElement; const p = !["system", "light", "dark"].includes(t); if (p) r.dataset.theme = t; r.classList.toggle("dark", !p && (t === "dark" || (t === "system" && matchMedia("(prefers-color-scheme: dark)").matches))); }} catch {{}} }})();"#
  )
}

/// Applies `theme` to the document root and keeps it there: `data-theme`
/// for a preset and the `dark` class for the dark scheme, following the
/// system's scheme while the theme is `System`. Each theme is stored under
/// `storage_key` (empty to store nothing); a stored theme that differs from
/// `theme` when the controller mounts is reported through `on_theme_change`,
/// so the app can adopt it. Renders a hidden marker element.
#[component]
pub fn ThemeController(
  #[props(default)] theme: Theme,
  #[props(default = THEME_STORAGE_KEY.to_string())] storage_key: String,
  #[props(default)] on_theme_change: Option<EventHandler<Theme>>,
) -> Element {
  let scope_id = use_hook(|| {
    format!("dxui-theme-controller-{}", NEXT_THEME_CONTROLLER_ID.fetch_add(1, Ordering::Relaxed))
  });
  let channel = use_hook(|| Rc::new(Cell::new(None::<document::Eval>)));
  // The theme last given to the script, so the first render sends it once.
  let sent = use_hook(|| Rc::new(RefCell::new(None::<String>)));
  let effect_scope_id = scope_id.clone();

  use_effect(use_reactive((&theme,), move |(theme,)| {
    let current = theme.as_str().to_string();
    if let Some(eval) = channel.get() {
      if sent.borrow().as_deref() != Some(current.as_str()) {
        // A send error means the page already finished the script.
        let _ = eval.send(current.as_str());
      }
    } else {
      let mut eval = document::eval(THEME_CONTROLLER_SCRIPT);
      let _ = eval.send((effect_scope_id.as_str(), storage_key.as_str(), current.as_str()));
      channel.set(Some(eval));
      spawn(async move {
        while let Ok(stored) = eval.recv::<String>().await {
          if let Some(handler) = on_theme_change {
            handler.call(Theme::parse(&stored));
          }
        }
      });
    }
    *sent.borrow_mut() = Some(current);
  }));

  rsx! {
    span { hidden: true, "data-dxui-theme-controller": scope_id }
  }
}
