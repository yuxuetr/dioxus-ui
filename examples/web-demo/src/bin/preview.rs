use dioxus::prelude::*;
use dioxus_ui_preview_states::{PreviewSurface, PreviewTarget};

const PREVIEW_CSS: Asset = asset!("/assets/preview.css");

fn main() {
  dioxus::launch(PreviewApp);
}

#[component]
fn PreviewApp() -> Element {
  rsx! {
    document::Title { "dioxus-ui preview" }
    document::Stylesheet { href: PREVIEW_CSS }
    PreviewSurface {
      target: PreviewTarget::Web,
      title: "dioxus-ui Web Preview".to_string(),
    }
  }
}
