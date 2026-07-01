use dioxus::prelude::*;
use dioxus_ui_preview_states::{PreviewSurface, PreviewTarget};

const PREVIEW_CSS: Asset = asset!("/assets/preview.css");

fn main() {
  dioxus::launch(PreviewApp);
}

#[component]
fn PreviewApp() -> Element {
  rsx! {
    document::Title { "dioxus-ui desktop preview" }
    document::Stylesheet { href: PREVIEW_CSS }
    PreviewSurface {
      target: PreviewTarget::Desktop,
      title: "dioxus-ui Desktop Preview".to_string(),
    }
  }
}
