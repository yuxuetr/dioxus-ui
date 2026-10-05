use dioxus::prelude::*;
use dioxus_ui_preview_states::{InteractionSelfTest, PreviewSurface, PreviewTarget};

fn main() {
  dioxus::launch(PreviewApp);
}

#[component]
fn PreviewApp() -> Element {
  // Interaction scenarios run in this WebView when set (RFC 0017).
  let self_test = std::env::var_os("DIOXUS_UI_DESKTOP_SELF_TEST").is_some();

  rsx! {
    document::Title { "dioxus-ui desktop preview" }
    PreviewSurface {
      target: PreviewTarget::Desktop,
      title: "dioxus-ui Desktop Preview".to_string(),
    }
    if self_test {
      InteractionSelfTest { label: "desktop" }
    }
  }
}
