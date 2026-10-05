use dioxus::prelude::*;
use dioxus_ui::{Badge, BadgeVariant, Tabs, TabsContent, TabsList, TabsTrigger};

use crate::Route;
use crate::catalog::{CATEGORIES, Component};
use crate::examples::EXAMPLES;

const H1: &str = "text-3xl font-bold tracking-normal";
const H2: &str = "mt-10 border-b border-border pb-2 text-xl font-semibold";
const LEAD: &str = "mt-2 text-lg text-muted-foreground";
const P: &str = "mt-4 leading-7";
const CODE_BLOCK: &str =
  "mt-4 overflow-x-auto rounded-md border border-border bg-muted p-4 font-mono text-sm";
const INLINE_CODE: &str = "rounded bg-muted px-1.5 py-0.5 font-mono text-sm";
const DOCS_URL: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/blob/main/docs/components");

fn find_component(slug: &str) -> Option<(&'static str, &'static Component)> {
  CATEGORIES.iter().find_map(|category| {
    category
      .components
      .iter()
      .find(|component| component.slug == slug)
      .map(|component| (category.label, component))
  })
}

#[component]
fn CodeBlock(code: String) -> Element {
  rsx! {
    pre { class: CODE_BLOCK, code { "{code}" } }
  }
}

#[component]
pub fn Home() -> Element {
  let count: usize = CATEGORIES.iter().map(|category| category.components.len()).sum();

  rsx! {
    article { "data-site-page": "home",
      h1 { class: H1, "dioxus-ui" }
      p { class: LEAD,
        "Accessible, Tailwind-styled components for Dioxus, in the manner of shadcn/ui. "
        "Copy a component's source into your app with the CLI, or depend on the crate."
      }
      div { class: "mt-6 flex flex-wrap gap-3",
        Link {
          class: "inline-flex h-10 items-center rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground hover:bg-primary/90",
          to: Route::GettingStarted {},
          "Get started"
        }
        Link {
          class: "inline-flex h-10 items-center rounded-md border border-input bg-background px-4 text-sm font-medium hover:bg-accent",
          to: Route::ComponentPage { slug: "button".to_string() },
          "Browse components"
        }
      }
      h2 { class: H2, "{count} components" }
      div { class: "mt-6 grid gap-4 sm:grid-cols-2",
        for category in CATEGORIES {
          section {
            key: "{category.id}",
            class: "rounded-md border border-border bg-card p-4 text-card-foreground",
            h3 { class: "font-semibold", "{category.label}" }
            ul { class: "mt-2 flex flex-wrap gap-x-3 gap-y-1 text-sm",
              for component in category.components {
                li { key: "{component.slug}",
                  Link {
                    class: "text-muted-foreground hover:text-foreground hover:underline",
                    to: Route::ComponentPage { slug: component.slug.to_string() },
                    "{component.title}"
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}

#[component]
pub fn GettingStarted() -> Element {
  rsx! {
    article { "data-site-page": "getting-started",
      h1 { class: H1, "Installation" }
      p { class: LEAD, "Use the components as copied source or as a crate dependency." }
      h2 { class: H2, "Source-copy mode" }
      p { class: P,
        "The CLI writes the Tailwind input stylesheet and copies component source into "
        code { class: INLINE_CODE, "src/components/ui" }
        ", where you own and edit it."
      }
      CodeBlock { code: "cargo install dioxus-ui-cli\n\ndxui init\ndxui add button\ndxui add dialog" }
      p { class: P,
        code { class: INLINE_CODE, "dxui add" }
        " keeps existing files; pass "
        code { class: INLINE_CODE, "--overwrite" }
        " to replace a previously generated component."
      }
      h2 { class: H2, "Crate mode" }
      p { class: P, "Enable one feature per component:" }
      CodeBlock {
        code: "[dependencies]\ndioxus-ui = { version = \"0.1\", default-features = false, features = [\"button\", \"dialog\"] }"
          .to_string()
      }
      CodeBlock { code: "use dioxus_ui::{Button, ButtonVariant};".to_string() }
      h2 { class: H2, "Stylesheet" }
      p { class: P,
        code { class: INLINE_CODE, "dxui init" }
        " writes "
        code { class: INLINE_CODE, "assets/dioxus-ui.css" }
        ", a Tailwind CSS v4 input with the semantic color tokens. Crate-mode apps need it too: run "
        code { class: INLINE_CODE, "dxui init" }
        " or copy the token blocks into your own input, and let your build compile it."
      }
      p { class: P,
        "Continue with "
        Link { class: "font-medium underline underline-offset-4", to: Route::Theming {}, "Theming" }
        "."
      }
    }
  }
}

#[component]
pub fn Theming() -> Element {
  rsx! {
    article { "data-site-page": "theming",
      h1 { class: H1, "Theming" }
      p { class: LEAD,
        "Components color through the shadcn/ui semantic tokens, so redefining a token restyles every component."
      }
      h2 { class: H2, "Tokens" }
      p { class: P,
        "The generated stylesheet defines the tokens in "
        code { class: INLINE_CODE, ":root" }
        " and "
        code { class: INLINE_CODE, ".dark" }
        ", and "
        code { class: INLINE_CODE, "@theme inline" }
        " maps each one to a Tailwind color, so "
        code { class: INLINE_CODE, "bg-primary" }
        " and "
        code { class: INLINE_CODE, "text-muted-foreground" }
        " work in your own markup too."
      }
      ul { class: "mt-4 flex flex-wrap gap-2",
        for token in [
          "background", "foreground", "card", "popover", "primary", "secondary", "muted",
          "accent", "destructive", "success", "warning", "info", "border", "input", "ring",
          "chart-1", "sidebar", "radius",
        ] {
          li { key: "{token}", Badge { variant: BadgeVariant::Outline, "--{token}" } }
        }
      }
      h2 { class: H2, "Rebrand" }
      p { class: P, "Set the tokens you want to change in both themes, for example a blue brand:" }
      CodeBlock {
        code: ":root {\n  --primary: oklch(0.546 0.245 262.881);\n  --primary-foreground: oklch(0.985 0 0);\n  --ring: oklch(0.546 0.245 262.881);\n}\n\n.dark {\n  --primary: oklch(0.623 0.214 259.815);\n  --ring: oklch(0.623 0.214 259.815);\n}"
          .to_string()
      }
      h2 { class: H2, "Dark theme" }
      p { class: P,
        "The dark theme is opt-in. Add the "
        code { class: INLINE_CODE, "dark" }
        " class to an ancestor, usually the app root. It redefines only the tokens, so your own palette classes keep their colors. The toggle in this site's header does the same."
      }
      CodeBlock {
        code: "div { class: \"dark min-h-screen bg-background text-foreground\", App {} }".to_string()
      }
    }
  }
}

#[component]
pub fn ComponentPage(slug: String) -> Element {
  let Some((category, component)) = find_component(&slug) else {
    return rsx! { NotFoundContent { path: format!("/components/{slug}") } };
  };

  let has_examples = EXAMPLES.iter().any(|example| example.slug == component.slug);

  rsx! {
    article { "data-site-page": "component", "data-component": component.slug,
      p { class: "text-sm text-muted-foreground", "{category}" }
      h1 { class: "{H1} mt-1", "{component.title}" }
      p { class: LEAD, "{component.description}" }
      h2 { class: H2, "Installation" }
      p { class: P, "Copy the source into your app:" }
      CodeBlock { code: "dxui add {component.slug}" }
      p { class: P, "Or enable the crate feature:" }
      CodeBlock {
        code: format!("dioxus-ui = {{ version = \"0.1\", features = [\"{}\"] }}", component.feature)
      }
      if has_examples {
        h2 { class: H2, "Examples" }
        for (index, example) in EXAMPLES.iter().enumerate().filter(|(_, example)| example.slug == component.slug) {
          ExampleCard { key: "{component.slug}-{example.title}", index }
        }
      }
      h2 { class: H2, "Reference" }
      ul { class: "mt-4 grid gap-2 text-sm",
        li {
          a {
            class: "font-medium underline underline-offset-4",
            href: "{DOCS_URL}/{component.slug}.md#api-surface",
            "API surface and behavior"
          }
        }
        li {
          a {
            class: "font-medium underline underline-offset-4",
            href: "{DOCS_URL}/{component.slug}.md#accessibility-notes",
            "Accessibility notes"
          }
        }
      }
    }
  }
}

/// One example: its live preview, or its source on the Code tab.
#[component]
fn ExampleCard(index: usize) -> Element {
  let mut tab = use_signal(|| "preview".to_string());
  let Some(example) = EXAMPLES.get(index) else {
    return rsx! {};
  };
  let on_preview = tab() == "preview";

  rsx! {
    section { class: "mt-6", "data-site-example": example.title,
      h3 { class: "font-semibold", "{example.title}" }
      Tabs { class: "mt-3", on_value_change: move |value: String| tab.set(value),
        TabsList { "aria-label": "{example.title} example",
          TabsTrigger { value: "preview", active: on_preview, "Preview" }
          TabsTrigger { value: "code", active: !on_preview, "Code" }
        }
        TabsContent {
          value: "preview",
          active: on_preview,
          class: "rounded-md border border-border p-6",
          ExampleRender { index }
        }
        TabsContent { value: "code", active: !on_preview,
          pre { class: "{CODE_BLOCK} max-h-[32rem]", "data-site-example-source": "",
            code { "{example.source}" }
          }
        }
      }
    }
  }
}

/// Renders an example in its own scope, so its hooks belong to it.
#[component]
fn ExampleRender(index: usize) -> Element {
  match EXAMPLES.get(index) {
    Some(example) => (example.render)(),
    None => rsx! {},
  }
}

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
  rsx! { NotFoundContent { path: format!("/{}", segments.join("/")) } }
}

#[component]
fn NotFoundContent(path: String) -> Element {
  rsx! {
    article { "data-site-page": "not-found",
      h1 { class: H1, "Page not found" }
      p { class: LEAD, "Nothing lives at {path}." }
      p { class: P,
        Link { class: "font-medium underline underline-offset-4", to: Route::Home {}, "Back to the introduction" }
      }
    }
  }
}
