use dioxus::prelude::*;
use dioxus_ui_preview_states::{PreviewSurface, PreviewTarget};

fn main() {
  dioxus::launch(PreviewApp);
}

#[component]
fn PreviewApp() -> Element {
  rsx! {
    PreviewSurface {
      target: PreviewTarget::Web,
      title: "dioxus-shadcn Web Preview".to_string(),
    }
  }
}
