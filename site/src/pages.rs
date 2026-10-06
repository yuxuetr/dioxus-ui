use dioxus::prelude::*;
use dioxus_shadcn::{
  Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Tabs, TabsContent, TabsList, TabsTrigger,
};

use crate::Route;
use crate::blocks::BLOCKS;
use crate::catalog::{CATEGORIES, Component};
use crate::examples::EXAMPLES;

const H1: &str = "text-3xl font-bold tracking-normal";
const H2: &str = "mt-10 border-b border-border pb-2 text-xl font-semibold";
const LEAD: &str = "mt-2 text-lg text-muted-foreground";
const P: &str = "mt-4 leading-7";
const CODE_BLOCK: &str =
  "overflow-x-auto rounded-md border border-border bg-muted p-4 pe-20 font-mono text-sm";
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

// `REFERENCES`: each component docs page from its API Surface section on,
// rendered by `build.rs`.
include!(concat!(env!("OUT_DIR"), "/references.rs"));

/// Styles the rendered docs, which arrive as plain HTML elements.
const PROSE: &str = "[&_h2]:mt-10 [&_h2]:border-b [&_h2]:border-border [&_h2]:pb-2 [&_h2]:text-xl [&_h2]:font-semibold [&_h3]:mt-8 [&_h3]:text-lg [&_h3]:font-semibold [&_p]:mt-4 [&_p]:leading-7 [&_ul]:mt-4 [&_ul]:list-disc [&_ul]:ps-6 [&_ol]:mt-4 [&_ol]:list-decimal [&_ol]:ps-6 [&_li]:mt-1.5 [&_li]:leading-7 [&_a]:font-medium [&_a]:underline [&_a]:underline-offset-4 [&_pre]:mt-4 [&_pre]:overflow-x-auto [&_pre]:rounded-md [&_pre]:border [&_pre]:border-border [&_pre]:bg-muted [&_pre]:p-4 [&_pre]:text-sm [&_code]:font-mono [&_code]:text-sm [&_:not(pre)>code]:rounded [&_:not(pre)>code]:bg-muted [&_:not(pre)>code]:px-1.5 [&_:not(pre)>code]:py-0.5 [&_:not(pre)>code]:[overflow-wrap:anywhere] [&_table]:mt-4 [&_table]:w-full [&_table]:text-sm [&_th]:border-b [&_th]:border-border [&_th]:py-2 [&_th]:text-start [&_td]:border-b [&_td]:border-border [&_td]:py-2";

/// A component's rendered reference, with links to other component pages
/// under the site's base path.
fn reference_html(slug: &str) -> Option<String> {
  let (_, html) = REFERENCES.iter().find(|(reference, _)| *reference == slug)?;
  let base = dioxus::cli_config::base_path().unwrap_or_default();
  let base = base.trim_matches('/');
  let prefix = if base.is_empty() { String::new() } else { format!("/{base}") };
  Some(html.replace("__SITE_BASE__", &prefix))
}

/// The crate version these pages document, such as `0.3.0`.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The version requirement for `Cargo.toml`, such as `0.3`.
fn version_requirement() -> &'static str {
  VERSION.rsplit_once('.').map_or(VERSION, |(requirement, _)| requirement)
}

// Copies the text it receives, reports whether that worked, and reports
// again two seconds later so the button can show its label again.
const COPY_SCRIPT: &str = r#"
const text = await dioxus.recv();
try {
  await navigator.clipboard.writeText(text);
  dioxus.send(true);
} catch {
  dioxus.send(false);
  return;
}
await new Promise((resolve) => setTimeout(resolve, 2000));
dioxus.send(false);
"#;

/// A button over a code block's corner that copies `text`; it reads
/// "Copied" for two seconds, also announced through a status message.
#[component]
fn CopyButton(text: String, label: String) -> Element {
  let mut copied = use_signal(|| false);

  rsx! {
    Button {
      class: "absolute end-2 top-2",
      variant: ButtonVariant::Outline,
      size: ButtonSize::Sm,
      "aria-label": "{label}",
      "data-site-copy": "",
      onclick: move |_| {
        let text = text.clone();
        spawn(async move {
          let mut eval = document::eval(COPY_SCRIPT);
          // A send error means the page already finished the script.
          let _ = eval.send(text);
          while let Ok(state) = eval.recv::<bool>().await {
            copied.set(state);
          }
        });
      },
      if copied() { "Copied" } else { "Copy" }
    }
    span { class: "sr-only", role: "status", if copied() { "Copied to the clipboard" } }
  }
}

#[component]
fn CodeBlock(code: String) -> Element {
  rsx! {
    div { class: "relative mt-4",
      pre { class: CODE_BLOCK, code { "{code}" } }
      CopyButton { text: code.clone(), label: "Copy code" }
    }
  }
}

#[component]
pub fn Home() -> Element {
  let count: usize = CATEGORIES.iter().map(|category| category.components.len()).sum();

  rsx! {
    article { "data-site-page": "home",
      h1 { class: H1, "dioxus-shadcn" }
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
      CodeBlock { code: "cargo install dioxus-shadcn-cli\n\ndxui init\ndxui add button\ndxui add dialog" }
      p { class: P,
        code { class: INLINE_CODE, "dxui add" }
        " keeps existing files; pass "
        code { class: INLINE_CODE, "--overwrite" }
        " to replace a previously generated component."
      }
      h2 { class: H2, "Crate mode" }
      p { class: P, "Enable one feature per component:" }
      CodeBlock {
        code: format!(
          "[dependencies]\ndioxus-shadcn = {{ version = \"{}\", default-features = false, features = [\"button\", \"dialog\"] }}",
          version_requirement()
        )
          .to_string()
      }
      CodeBlock { code: "use dioxus_shadcn::{Button, ButtonVariant};".to_string() }
      h2 { class: H2, "Stylesheet" }
      p { class: P,
        code { class: INLINE_CODE, "dxui init" }
        " writes "
        code { class: INLINE_CODE, "assets/dioxus-shadcn.css" }
        ", a Tailwind CSS v4 input with the semantic color tokens. Crate-mode apps need it too: run "
        code { class: INLINE_CODE, "dxui init" }
        " or copy the token blocks into your own input, and let your build compile it."
      }
      p { class: P,
        "Crate mode also needs an "
        code { class: INLINE_CODE, "@source" }
        " line for the crate's source after the import, since Tailwind generates only the classes it finds in scanned files. In an app that depends on the crate, "
        code { class: INLINE_CODE, "dxui init" }
        " writes it from "
        code { class: INLINE_CODE, "cargo metadata" }
        ". The path names one version, so run "
        code { class: INLINE_CODE, "dxui init" }
        " again after upgrading the crate:"
      }
      CodeBlock { code: format!("@import \"tailwindcss\";\n@source \"/path/to/dioxus-shadcn-{VERSION}/src\";") }
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
      h2 { class: H2, "Theme presets" }
      p { class: P,
        "The CLI ships 33 presets ported from daisyUI, with every text color checked against WCAG AA. "
        code { class: INLINE_CODE, "dxui theme add" }
        " appends them to the stylesheet; each is one "
        code { class: INLINE_CODE, "[data-theme]" }
        " rule, so setting the attribute themes that element's subtree. "
        code { class: INLINE_CODE, "ThemeController" }
        " sets it on the page root, follows the system's light or dark scheme by default, and remembers the choice; this site's header menu uses it."
      }
      CodeBlock {
        code: "dxui theme add nord dracula\n\n// main.rs\nlet mut theme = use_signal(|| Theme::System);\nrsx! {\n  ThemeController { theme: theme(), on_theme_change: move |stored| theme.set(stored) }\n  // theme.set(Theme::Preset(\"nord\".into())) from any control\n  App {}\n}"
          .to_string()
      }
      ul { class: "mt-4 grid gap-3 sm:grid-cols-2 lg:grid-cols-3", "data-site-presets": "",
        for theme in crate::themes::THEMES {
          li {
            key: "{theme.name}",
            "data-theme": theme.name,
            class: "rounded-lg border border-border bg-background p-4 text-foreground",
            div { class: "flex items-baseline justify-between gap-2",
              h3 { class: "font-semibold", "{theme.title}" }
              span { class: "text-xs text-muted-foreground", if theme.dark { "dark" } else { "light" } }
            }
            div { class: "mt-3 flex gap-1", "aria-hidden": "true",
              for swatch in ["bg-primary", "bg-secondary", "bg-chart-3", "bg-muted", "bg-destructive", "bg-success", "bg-warning", "bg-info"] {
                span { key: "{swatch}", class: "h-6 flex-1 rounded-sm border border-border {swatch}" }
              }
            }
            div { class: "mt-3 flex items-center gap-2",
              span { class: "rounded-md bg-primary px-2 py-1 text-xs font-medium text-primary-foreground", "Primary" }
              span { class: "rounded-md bg-secondary px-2 py-1 text-xs font-medium text-secondary-foreground", "Secondary" }
            }
            code { class: "mt-3 block text-xs text-muted-foreground", "dxui theme add {theme.name}" }
          }
        }
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
        code: format!(
          "dioxus-shadcn = {{ version = \"{}\", features = [\"{}\"] }}",
          version_requirement(),
          component.feature
        )
      }
      if has_examples {
        h2 { class: H2, "Examples" }
        for (index, example) in EXAMPLES.iter().enumerate().filter(|(_, example)| example.slug == component.slug) {
          ExampleCard { key: "{component.slug}-{example.title}", index }
        }
      }
      if let Some(reference) = reference_html(component.slug) {
        div { class: PROSE, "data-site-reference": "", dangerous_inner_html: reference }
      }
      p { class: "mt-10 text-sm text-muted-foreground",
        a {
          class: "font-medium underline underline-offset-4",
          href: "{DOCS_URL}/{component.slug}.md",
          "View this page on GitHub"
        }
      }
    }
  }
}

/// The blocks (RFC 0073), each with its add command.
#[component]
pub fn Blocks() -> Element {
  rsx! {
    article { "data-site-page": "blocks",
      h1 { class: H1, "Blocks" }
      p { class: LEAD, "Whole screens built from the components, copied into your app with their components." }
      CodeBlock { code: "dxui list blocks\ndxui add dashboard" }
      ul { class: "mt-8 grid gap-4 sm:grid-cols-2",
        for block in BLOCKS {
          li { key: "{block.slug}", class: "rounded-lg border border-border p-4",
            Link {
              class: "font-semibold underline-offset-4 hover:underline",
              to: Route::BlockPage { slug: block.slug.to_string() },
              "{block.title}"
            }
            p { class: "mt-1 text-sm text-muted-foreground", "{block.description}" }
            code { class: "{INLINE_CODE} mt-3 inline-block", "dxui add {block.slug}" }
          }
        }
      }
    }
  }
}

/// One block: its add command, a full-width preview or its source, and its
/// docs page.
#[component]
pub fn BlockPage(slug: String) -> Element {
  let mut tab = use_signal(|| "preview".to_string());
  let Some(block) = BLOCKS.iter().find(|block| block.slug == slug) else {
    return rsx! { NotFoundContent { path: format!("/blocks/{slug}") } };
  };
  let on_preview = tab() == "preview";
  let reference =
    BLOCK_REFERENCES.iter().find(|(reference, _)| *reference == block.slug).map(|(_, html)| *html);

  rsx! {
    article { "data-site-page": "block", "data-block": block.slug,
      p { class: "text-sm text-muted-foreground", "Blocks" }
      h1 { class: "{H1} mt-1", "{block.title}" }
      p { class: LEAD, "{block.description}" }
      CodeBlock { code: format!("dxui add {}", block.slug) }
      Tabs { class: "mt-8", on_value_change: move |value: String| tab.set(value),
        TabsList { "aria-label": "{block.title} block",
          TabsTrigger { value: "preview", active: on_preview, "Preview" }
          TabsTrigger { value: "code", active: !on_preview, "Code" }
        }
        TabsContent { value: "preview", active: on_preview,
          // The transform makes the frame the containing block of the
          // block's fixed parts, such as the off-canvas sidebar.
          div {
            class: "h-[44rem] overflow-auto rounded-lg border border-border [transform:translateZ(0)]",
            tabindex: "0",
            "aria-label": "{block.title} preview",
            "data-site-block-preview": "",
            BlockRender { slug: block.slug }
          }
        }
        TabsContent { value: "code", active: !on_preview, class: "relative",
          pre { class: "{CODE_BLOCK} max-h-[44rem]", tabindex: "0", "data-site-block-source": "",
            code { "{block.source}" }
          }
          CopyButton { text: block.source.to_string(), label: "Copy {block.title} source" }
        }
      }
      if let Some(reference) = reference {
        div { class: PROSE, "data-site-reference": "", dangerous_inner_html: reference }
      }
    }
  }
}

/// Renders a block in its own scope, so its hooks belong to it.
#[component]
fn BlockRender(slug: &'static str) -> Element {
  match BLOCKS.iter().find(|block| block.slug == slug) {
    Some(block) => (block.render)(),
    None => rsx! {},
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
          div { "data-site-example-preview": "", ExampleRender { index } }
        }
        TabsContent { value: "code", active: !on_preview, class: "relative",
          pre { class: "{CODE_BLOCK} max-h-[32rem]", "data-site-example-source": "",
            code { "{example.source}" }
          }
          CopyButton { text: example.source.to_string(), label: "Copy {example.title} source" }
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
