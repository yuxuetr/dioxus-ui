use dioxus::prelude::*;
use dioxus_shadcn::{Theme, ToggleGroup, ToggleGroupItem};

/// The site's root renders `ThemeController { theme: theme(), on_theme_change }`
/// and shares the theme signal through context; this picker sets it, so the
/// whole site follows, and the choice is remembered on the next visit.
#[component]
pub fn ThemeControllerPickerDemo() -> Element {
  let mut theme = use_context::<Signal<Theme>>();

  rsx! {
    ToggleGroup {
      "aria-label": "Theme",
      value: theme().as_str().to_string(),
      // Pressing the current theme again asks to clear it; a theme stays set.
      on_value_change: move |value: String| {
        if !value.is_empty() {
          theme.set(Theme::parse(&value));
        }
      },
      for (option, label) in [(Theme::System, "System"), (Theme::Light, "Light"), (Theme::Dark, "Dark")] {
        ToggleGroupItem {
          key: "{label}",
          value: option.as_str().to_string(),
          "{label}"
        }
      }
    }
    p { class: "mt-3 text-sm text-muted-foreground", "Theme: {theme().as_str()}" }
  }
}
