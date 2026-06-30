use dioxus::prelude::*;
use dioxus_ui::{button_class, ButtonSize, ButtonVariant, UiDensity};
use dioxus_ui_preview_states::{preview_lines, PreviewTarget};

const PREVIEW_CSS: Asset = asset!("/assets/preview.css");

fn main() {
  dioxus::launch(PreviewApp);
}

#[component]
fn PreviewApp() -> Element {
  let states = preview_lines(PreviewTarget::Web);
  let primary_button_class = button_class(
    ButtonVariant::Primary,
    ButtonSize::Md,
    UiDensity::Comfortable,
    "",
  );
  let secondary_button_class = button_class(
    ButtonVariant::Secondary,
    ButtonSize::Sm,
    UiDensity::Compact,
    "",
  );

  rsx! {
    document::Title { "dioxus-ui preview" }
    document::Stylesheet { href: PREVIEW_CSS }
    main {
      class: "min-h-screen bg-white text-zinc-950",
      "data-preview-root": "web",
      section {
        class: "mx-auto flex w-full max-w-6xl flex-col gap-6 px-6 py-8",
        "data-preview-panel": "overview",
        header {
          class: "flex flex-col gap-2 border-b border-zinc-200 pb-4",
          h1 { class: "text-2xl font-semibold", "dioxus-ui Web Preview" }
          p {
            class: "max-w-3xl text-sm text-zinc-600",
            "Rendered preview shell for representative component states. This is a component preview surface, not a landing page."
          }
        }
        section {
          class: "grid gap-3 md:grid-cols-2",
          "data-preview-panel": "actions",
          button { class: "{primary_button_class}", "Primary action" }
          button { class: "{secondary_button_class}", "Secondary action" }
        }
        section {
          class: "grid gap-3 md:grid-cols-2 xl:grid-cols-3",
          "data-preview-panel": "inventory",
          for state in states {
            article {
              key: "{state.label}",
              class: "rounded-md border border-zinc-200 bg-white p-4 shadow-sm",
              "data-preview-state": "{state.label}",
              h2 { class: "text-sm font-medium text-zinc-950", "{state.label}" }
              code {
                class: "mt-3 block overflow-hidden text-ellipsis whitespace-nowrap rounded bg-zinc-50 px-2 py-1 text-xs text-zinc-700",
                title: "{state.value}",
                "{state.value}"
              }
            }
          }
        }
      }
    }
  }
}
