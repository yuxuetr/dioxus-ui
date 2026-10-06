//! The dioxus-shadcn component site (RFC 0052): the catalog, each component's
//! page, and the setup and theming guides, built on the published components.

mod blocks;
mod catalog;
mod components;
mod examples;
mod pages;
mod themes;

use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, NativeSelect, NativeSelectGroup, NativeSelectOption,
  SheetContent, SheetOverlay, SheetSide, SheetTitle, Theme, ThemeController,
};
use pages::{BlockPage, Blocks, ComponentPage, GettingStarted, Home, NotFound, Theming};

const SITE_CSS: Asset = asset!("/assets/site.generated.css");

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
  #[layout(Shell)]
    #[route("/")]
    Home {},
    #[route("/docs/getting-started")]
    GettingStarted {},
    #[route("/docs/theming")]
    Theming {},
    #[route("/components/:slug")]
    ComponentPage { slug: String },
    #[route("/blocks")]
    Blocks {},
    #[route("/blocks/:slug")]
    BlockPage { slug: String },
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    document::Title { "dioxus-shadcn" }
    document::Stylesheet { href: SITE_CSS }
    Router::<Route> {}
  }
}

/// The header, the catalog sidebar, and the routed page. The theme menu picks
/// the system scheme, light, dark, or a preset (RFC 0057), which the
/// `ThemeController` applies to the document root and remembers (RFC 0071);
/// pages read and set the same theme through context.
#[component]
fn Shell() -> Element {
  let mut theme = use_context_provider(|| Signal::new(Theme::System));
  let mut menu_open = use_signal(|| false);

  rsx! {
    ThemeController { theme: theme(), on_theme_change: move |stored| theme.set(stored) }
    div {
      class: "min-h-screen bg-background text-foreground",
      "data-site-root": "",
      header { class: "sticky top-0 z-40 border-b border-border bg-background",
        div { class: "mx-auto flex h-14 max-w-6xl items-center gap-3 px-4",
          Button {
            class: "md:hidden",
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Sm,
            "aria-label": "Open the component menu",
            onclick: move |_| menu_open.set(true),
            "Menu"
          }
          Link { class: "font-semibold", to: Route::Home {}, "dioxus-shadcn" }
          nav { class: "hidden items-center gap-4 text-sm text-muted-foreground sm:flex",
            Link { class: "hover:text-foreground", to: Route::GettingStarted {}, "Docs" }
            Link {
              class: "hover:text-foreground",
              to: Route::ComponentPage { slug: "button".to_string() },
              "Components"
            }
            Link { class: "hover:text-foreground", to: Route::Blocks {}, "Blocks" }
            Link { class: "hover:text-foreground", to: Route::Theming {}, "Theming" }
          }
          div { class: "ml-auto flex items-center gap-2",
            NativeSelect {
              id: "site-theme",
              class: "h-8 w-32",
              "aria-label": "Theme",
              on_value_change: move |value: String| theme.set(Theme::parse(&value)),
              NativeSelectGroup { label: "Default",
                for option in [Theme::System, Theme::Light, Theme::Dark] {
                  NativeSelectOption {
                    key: "{option.as_str()}",
                    value: option.as_str().to_string(),
                    selected: theme() == option,
                    match option {
                      Theme::Light => "Light",
                      Theme::Dark => "Dark",
                      _ => "System",
                    }
                  }
                }
              }
              for (label, dark) in [("Light presets", false), ("Dark presets", true)] {
                NativeSelectGroup { key: "{label}", label,
                  for preset in themes::THEMES.iter().filter(|preset| preset.dark == dark) {
                    NativeSelectOption {
                      key: "{preset.name}",
                      value: preset.name,
                      selected: theme().as_str() == preset.name,
                      "{preset.title}"
                    }
                  }
                }
              }
            }
          }
        }
      }
      div { class: "mx-auto flex max-w-6xl gap-8 px-4",
        aside { class: "sticky top-14 hidden h-[calc(100vh-3.5rem)] w-56 shrink-0 overflow-y-auto py-6 md:block",
          SiteNav {}
        }
        main { class: "min-w-0 flex-1 py-8", Outlet::<Route> {} }
      }
      SheetOverlay { open: menu_open(), on_open_change: move |open| menu_open.set(open) }
      SheetContent {
        class: "overflow-y-auto",
        open: menu_open(),
        side: SheetSide::Left,
        on_open_change: move |open| menu_open.set(open),
        SheetTitle { class: "mb-4", "Components" }
        // A followed link bubbles here, so the menu closes on navigation.
        div { onclick: move |_| menu_open.set(false), SiteNav {} }
      }
    }
  }
}

/// The guides and the catalog grouped by category, from `catalog.rs`.
#[component]
fn SiteNav() -> Element {
  let link = "block rounded-md px-2 py-1 text-sm text-muted-foreground hover:bg-accent hover:text-foreground";
  let active = "bg-accent text-foreground";

  rsx! {
    nav { class: "grid gap-6", "aria-label": "Site",
      section { class: "grid gap-1",
        h2 { class: "px-2 text-sm font-semibold", "Getting Started" }
        Link { class: link, active_class: active, to: Route::Home {}, "Introduction" }
        Link { class: link, active_class: active, to: Route::GettingStarted {}, "Installation" }
        Link { class: link, active_class: active, to: Route::Theming {}, "Theming" }
      }
      section { class: "grid gap-1",
        h2 { class: "px-2 text-sm font-semibold", "Blocks" }
        for block in blocks::BLOCKS {
          Link {
            key: "{block.slug}",
            class: link,
            active_class: active,
            to: Route::BlockPage { slug: block.slug.to_string() },
            "{block.title}"
          }
        }
      }
      for category in catalog::CATEGORIES {
        section { key: "{category.id}", id: "category-{category.id}", class: "grid gap-1",
          h2 { class: "px-2 text-sm font-semibold", "{category.label}" }
          for component in category.components {
            Link {
              key: "{component.slug}",
              class: link,
              active_class: active,
              to: Route::ComponentPage { slug: component.slug.to_string() },
              "{component.title}"
            }
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use dioxus_shadcn::{THEME_STORAGE_KEY, theme_init_script};

  #[test]
  fn page_template_applies_the_stored_theme_before_the_app_loads() {
    let template = include_str!("../index.html");

    assert!(
      template.contains(&format!("<script>{}</script>", theme_init_script(THEME_STORAGE_KEY)))
    );
    assert!(template.contains(r#"<html lang="en">"#));
  }
}
