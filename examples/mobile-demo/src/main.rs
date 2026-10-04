use dioxus::prelude::*;
use dioxus_ui_preview_states::{InteractionSelfTest, PreviewSurface, PreviewTarget};

fn main() {
  dioxus::launch(PreviewApp);
}

#[component]
fn PreviewApp() -> Element {
  // Interaction scenarios run in this WebView when set (RFC 0018).
  let self_test = std::env::var_os("DIOXUS_UI_MOBILE_SELF_TEST").is_some();

  rsx! {
    PreviewSurface {
      target: PreviewTarget::Mobile,
      title: "dioxus-ui Mobile Preview".to_string(),
    }
    if self_test {
      InteractionSelfTest { label: "mobile" }
    }
  }
}
