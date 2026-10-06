# Theme Controller

Theme Controller applies the color theme to the document root, follows the
system's light or dark scheme while the theme is `System`, and remembers the
choice across visits.

## Source Copy

```bash
dxui add theme-controller
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.2", default-features = false, features = ["theme-controller"] }
```

## API Surface

- `ThemeController`
- `Theme`
- `theme_init_script`
- `THEME_STORAGE_KEY`

```rust
let mut theme = use_signal(|| Theme::System);

rsx! {
  ThemeController {
    theme: theme(),
    on_theme_change: move |stored| theme.set(stored),
  }
  ToggleGroup {
    "aria-label": "Theme",
    on_toggle: move |value: String| theme.set(Theme::parse(&value)),
    for (option, label) in [(Theme::System, "System"), (Theme::Light, "Light"), (Theme::Dark, "Dark")] {
      ToggleGroupItem {
        key: "{label}",
        value: option.as_str().to_string(),
        pressed: theme() == option,
        "{label}"
      }
    }
  }
}
```

## Behavior

See [RFC 0071](../rfcs/0071-theme-controller.md).

- `Theme::Light` and `Theme::Dark` set or remove the opt-in `dark` class on
  the root element ([RFC 0047](../rfcs/0047-opt-in-dark-theme.md)).
  `Theme::System` follows `prefers-color-scheme` and changes with it.
- `Theme::Preset(name)` sets `data-theme` on the root to a preset added with
  `dxui theme add` ([RFC 0057](../rfcs/0057-theme-presets.md)); presets bring
  their own scheme, so the `dark` class is removed.
- Each theme the app passes is stored in `localStorage` under `storage_key`
  (`THEME_STORAGE_KEY`, `dxui-theme`, by default; empty stores nothing). When
  the controller mounts, a stored theme that differs from `theme` is applied
  and reported through `on_theme_change`, so the app can adopt it. Without a
  stored theme the default is `System`.
- `theme` stays controlled: the controller applies what the app passes and
  never changes it except to report the stored theme.
- The controller renders a hidden `span`; keep it mounted, usually in the
  app's root component.

### Before first paint

The controller runs after the app's first render, so a stored dark or preset
theme would flash light first. Put the output of
`theme_init_script(THEME_STORAGE_KEY)` in a `<script>` in the page's `<head>`
to apply it before the app loads. In a Dioxus web app, add it to the
`index.html` template; Desktop and Mobile apps can pass it to the WebView's
custom head.

## Accessibility Notes

The controller renders no content. The control that picks the theme is the
app's; give it a name, such as a `ToggleGroup` labelled "Theme", and show the
current choice, since `System` may look like either scheme.
