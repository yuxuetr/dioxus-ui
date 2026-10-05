use dioxus::prelude::*;
use dioxus_ui_preview_states::{InteractionSelfTest, PreviewSurface, PreviewTarget};

fn main() {
  dioxus::launch(PreviewApp);
}

#[component]
fn PreviewApp() -> Element {
  // Interaction scenarios run in this WebView when requested (RFC 0018,
  // RFC 0020).
  let self_test = use_hook(self_test_requested);

  rsx! {
    PreviewSurface {
      target: PreviewTarget::Mobile,
      title: "dioxus-shadcn Mobile Preview".to_string(),
    }
    if self_test {
      InteractionSelfTest { label: "mobile" }
    }
  }
}

/// The iOS Simulator passes the request as an environment variable.
#[cfg(not(target_os = "android"))]
fn self_test_requested() -> bool {
  std::env::var_os("DIOXUS_UI_MOBILE_SELF_TEST").is_some()
}

/// Apps started with `am start` get no environment, so the Android command
/// sets the `debug.dioxus_shadcn.self_test` system property to `1` instead.
#[cfg(target_os = "android")]
fn self_test_requested() -> bool {
  std::process::Command::new("getprop")
    .arg("debug.dioxus_shadcn.self_test")
    .output()
    .map(|output| output.stdout.trim_ascii() == b"1")
    .unwrap_or(false)
}
