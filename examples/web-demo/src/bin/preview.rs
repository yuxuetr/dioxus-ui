use dioxus::prelude::*;
use dioxus_ui_preview_states::{PreviewSurface, PreviewTarget};

fn main() {
  dioxus::launch(PreviewApp);
}

#[component]
fn PreviewApp() -> Element {
  rsx! {
    document::Title { "dioxus-ui preview" }
    PreviewSurface {
      target: PreviewTarget::Web,
      title: "dioxus-ui Web Preview".to_string(),
    }
  }
}
