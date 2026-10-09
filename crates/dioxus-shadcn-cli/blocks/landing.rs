use dioxus::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback};
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::ui::input::Input;
use crate::components::ui::separator::Separator;

struct Feature {
  title: &'static str,
  description: &'static str,
  // The inside of a 24 by 24 stroked icon.
  icon: &'static str,
}

const FEATURES: &[Feature] = &[
  Feature {
    title: "Fast by default",
    description: "Pages render on the server and hydrate only what moves.",
    icon: "M13 2 3 14h9l-1 8 10-12h-9l1-8z",
  },
  Feature {
    title: "Typed end to end",
    description: "One language from the database row to the button label.",
    icon: "m16 18 6-6-6-6M8 6l-6 6 6 6",
  },
  Feature {
    title: "Accessible parts",
    description: "Keyboard, focus, and labels work before you style anything.",
    icon: "M12 8a2 2 0 1 0 0-4 2 2 0 0 0 0 4zM5 9l7 1 7-1M12 10v4m0 0-3 6m3-6 3 6",
  },
  Feature {
    title: "Your code",
    description: "Copy a component and change it; nothing is locked in a package.",
    icon: "M4 4h16v16H4zM9 9h6v6H9z",
  },
  Feature {
    title: "Themes",
    description: "Switch a color preset or roll your own with a few tokens.",
    icon: "M12 3a9 9 0 1 0 0 18c1 0 1.5-.7 1.5-1.5 0-.4-.2-.8-.4-1.1-.3-.3-.4-.6-.4-1 0-.8.7-1.4 1.5-1.4H16a5 5 0 0 0 5-5c0-4.4-4-8-9-8z",
  },
  Feature {
    title: "Every platform",
    description: "The same components on the web, the desktop, and mobile.",
    icon: "M2 5h20v12H2zM8 21h8M12 17v4",
  },
];

/// A product page: a header, a hero with two actions, a feature grid, a
/// testimonial, an early-access form, and a footer, all as layout you edit.
/// `on_get_started` hears the hero's main action and `on_subscribe` an email
/// the form accepted.
#[component]
pub fn LandingBlock(
  #[props(default)] on_get_started: Option<EventHandler<()>>,
  #[props(default)] on_subscribe: Option<EventHandler<String>>,
) -> Element {
  let mut email = use_signal(String::new);
  let mut subscribed = use_signal(|| None::<String>);
  let mut invalid = use_signal(|| false);
  let get_started = move |_| {
    if let Some(handler) = on_get_started {
      handler.call(());
    }
  };

  rsx! {
    div { class: "min-h-screen bg-background text-foreground",
      header { class: "mx-auto flex max-w-6xl items-center justify-between gap-4 px-4 py-4",
        a { class: "text-lg font-semibold", href: "#", "Acme" }
        nav { "aria-label": "Main", class: "hidden items-center gap-6 text-sm text-muted-foreground sm:flex",
          a { class: "hover:text-foreground", href: "#features", "Features" }
          a { class: "hover:text-foreground", href: "#customers", "Customers" }
          a { class: "hover:text-foreground", href: "#early-access", "Early access" }
        }
        Button { onclick: get_started, "Get started" }
      }
      main {
        section { class: "mx-auto grid max-w-3xl justify-items-center gap-6 px-4 py-20 text-center",
          Badge { variant: BadgeVariant::Secondary, "New: offline sync" }
          h1 { class: "text-4xl font-semibold tracking-tight sm:text-5xl",
            "Ship the app, not the scaffolding"
          }
          p { class: "max-w-xl text-lg text-muted-foreground",
            "Acme gives your team the parts every product needs, so the first week goes to the idea instead of the plumbing."
          }
          div { class: "flex flex-wrap justify-center gap-3",
            Button { onclick: get_started, "Start free" }
            Button { variant: ButtonVariant::Outline, "Read the docs" }
          }
        }
        section {
          id: "features",
          class: "mx-auto grid max-w-6xl gap-8 px-4 py-16",
          "aria-labelledby": "landing-features",
          div { class: "grid gap-2 text-center",
            h2 { id: "landing-features", class: "text-3xl font-semibold tracking-tight", "What you get" }
            p { class: "text-muted-foreground", "Six things you would otherwise build first." }
          }
          div { class: "grid gap-4 sm:grid-cols-2 lg:grid-cols-3",
            for feature in FEATURES {
              Card { key: "{feature.title}",
                CardHeader {
                  svg {
                    class: "mb-2 size-6 text-primary",
                    "aria-hidden": "true",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: feature.icon }
                  }
                  CardTitle {
                    h3 { "{feature.title}" }
                  }
                  CardDescription { "{feature.description}" }
                }
              }
            }
          }
        }
        section {
          id: "customers",
          class: "mx-auto max-w-3xl px-4 py-16",
          "aria-label": "Customer story",
          Card {
            CardContent { class: "grid gap-6 pt-6",
              blockquote { class: "text-xl leading-relaxed",
                "\u{201c}We replaced three internal libraries in a month. New screens take an afternoon now.\u{201d}"
              }
              div { class: "flex items-center gap-3",
                Avatar {
                  AvatarFallback { "MR" }
                }
                div { class: "text-sm",
                  p { class: "font-medium", "Maya Rossi" }
                  p { class: "text-muted-foreground", "Engineering lead, Northwind" }
                }
              }
            }
          }
        }
        section {
          id: "early-access",
          class: "border-y bg-muted/50",
          "aria-labelledby": "landing-early-access",
          div { class: "mx-auto grid max-w-xl justify-items-center gap-4 px-4 py-16 text-center",
            h2 { id: "landing-early-access", class: "text-3xl font-semibold tracking-tight", "Get early access" }
            p { class: "text-muted-foreground", "One email when the next release is out. Nothing else." }
            if let Some(address) = subscribed() {
              p { role: "status", class: "font-medium", "Thanks. We will write to {address}." }
            } else {
              form {
                class: "flex w-full max-w-md flex-col gap-2 sm:flex-row",
                novalidate: true,
                onsubmit: move |event| {
                  event.prevent_default();
                  let address = email().trim().to_string();
                  if !address.contains('@') {
                    invalid.set(true);
                    return;
                  }
                  if let Some(handler) = on_subscribe {
                    handler.call(address.clone());
                  }
                  subscribed.set(Some(address));
                },
                Input {
                  class: "flex-1",
                  r#type: "email",
                  autocomplete: "email",
                  placeholder: "you@example.com",
                  "aria-label": "Email",
                  value: email(),
                  invalid: invalid(),
                  "aria-describedby": invalid().then_some("landing-email-error"),
                  on_value_change: move |value| {
                    invalid.set(false);
                    email.set(value);
                  },
                }
                Button { r#type: "submit", "Notify me" }
              }
              if invalid() {
                p { id: "landing-email-error", class: "text-sm text-destructive", "Enter your email address." }
              }
            }
          }
        }
      }
      footer { class: "mx-auto grid max-w-6xl gap-6 px-4 py-10 text-sm text-muted-foreground",
        div { class: "flex flex-wrap items-center justify-between gap-4",
          span { class: "font-semibold text-foreground", "Acme" }
          nav { "aria-label": "Footer", class: "flex flex-wrap gap-6",
            a { class: "hover:text-foreground", href: "#", "Docs" }
            a { class: "hover:text-foreground", href: "#", "Changelog" }
            a { class: "hover:text-foreground", href: "#", "Privacy" }
            a { class: "hover:text-foreground", href: "#", "Contact" }
          }
        }
        Separator { decorative: true }
        p { "\u{a9} 2026 Acme, Inc." }
      }
    }
  }
}
