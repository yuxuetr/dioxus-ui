use dioxus::prelude::*;
use dioxus_ui_preview_states::{PreviewSurface, PreviewTarget};
use serde_json::Value;

const PREVIEW_CSS: Asset = asset!("/assets/preview.css");
// Interaction scenarios run in this WebView when DIOXUS_UI_DESKTOP_SELF_TEST
// is set (RFC 0017).
const SELF_TEST_SCRIPT: &str = include_str!("../../self-test/interactions.js");

fn main() {
  dioxus::launch(PreviewApp);
}

#[component]
fn PreviewApp() -> Element {
  let self_test = std::env::var_os("DIOXUS_UI_DESKTOP_SELF_TEST").is_some();

  rsx! {
    document::Title { "dioxus-ui desktop preview" }
    document::Stylesheet { href: PREVIEW_CSS }
    PreviewSurface {
      target: PreviewTarget::Desktop,
      title: "dioxus-ui Desktop Preview".to_string(),
    }
    if self_test {
      SelfTest {}
    }
  }
}

/// Runs the interaction scenarios once, prints the result, and exits with
/// status 0 when every scenario passed.
#[component]
fn SelfTest() -> Element {
  use_effect(|| {
    spawn(async {
      let mut eval = document::eval(SELF_TEST_SCRIPT);
      let code = match eval.recv::<Value>().await {
        Ok(result) => report(&result),
        Err(error) => {
          eprintln!("desktop interaction verification failed: {error}");
          1
        }
      };
      std::process::exit(code);
    });
  });

  rsx! {}
}

fn report(result: &Value) -> i32 {
  let passed: Vec<&str> = result["passed"]
    .as_array()
    .map(|names| names.iter().filter_map(Value::as_str).collect())
    .unwrap_or_default();

  if result["ok"].as_bool() == Some(true) {
    println!(
      "desktop interaction verification passed ({} scenarios: {})",
      passed.len(),
      passed.join(", ")
    );
    return 0;
  }

  eprintln!(
    "desktop interaction verification failed: {}",
    result["error"].as_str().unwrap_or("no error reported")
  );
  if !passed.is_empty() {
    eprintln!("passed before the failure: {}", passed.join(", "));
  }
  1
}
