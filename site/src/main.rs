//! The dioxus-ui component site (RFC 0052): the catalog, each component's
//! page, and the setup and theming guides, built on the published components.

mod catalog;
mod pages;

use dioxus::prelude::*;
use dioxus_ui::{
  Button, ButtonSize, ButtonVariant, SheetContent, SheetOverlay, SheetSide, SheetTitle, Toggle,
};
use pages::{ComponentPage, GettingStarted, Home, NotFound, Theming};

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
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    document::Title { "dioxus-ui" }
    document::Stylesheet { href: SITE_CSS }
    Router::<Route> {}
  }
}

/// The header, the catalog sidebar, and the routed page. The site starts in
/// the light theme; the header toggle adds the opt-in `dark` class.
#[component]
fn Shell() -> Element {
  let mut dark_theme = use_signal(|| false);
  let mut menu_open = use_signal(|| false);
  let theme_class = if dark_theme() { "dark " } else { "" };

  rsx! {
    div { class: "{theme_class}min-h-screen bg-background text-foreground", "data-site-root": "",
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
          Link { class: "font-semibold", to: Route::Home {}, "dioxus-ui" }
          nav { class: "hidden items-center gap-4 text-sm text-muted-foreground sm:flex",
            Link { class: "hover:text-foreground", to: Route::GettingStarted {}, "Docs" }
            Link {
              class: "hover:text-foreground",
              to: Route::ComponentPage { slug: "button".to_string() },
              "Components"
            }
            Link { class: "hover:text-foreground", to: Route::Theming {}, "Theming" }
          }
          div { class: "ml-auto",
            Toggle {
              id: "site-theme-toggle",
              pressed: dark_theme(),
              on_pressed_change: move |pressed| dark_theme.set(pressed),
              "Dark theme"
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
