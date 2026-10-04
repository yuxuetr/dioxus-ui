use dioxus::prelude::*;
use serde_json::Value;

/// Interaction scenarios the Desktop and Mobile previews run in their own
/// WebView (RFC 0017, RFC 0018). The script sends one `{ ok, passed, error }`
/// result.
pub const INTERACTION_SELF_TEST_SCRIPT: &str = include_str!("../self-test/interactions.js");

/// Runs the interaction scenarios once, prints the result prefixed with
/// `label`, and exits the process with status 0 when every scenario passed.
#[component]
pub fn InteractionSelfTest(label: &'static str) -> Element {
  use_effect(move || {
    spawn(async move {
      let mut eval = document::eval(INTERACTION_SELF_TEST_SCRIPT);
      let code = match eval.recv::<Value>().await {
        Ok(result) => report(label, &result),
        Err(error) => {
          eprintln!("{label} interaction verification failed: {error}");
          1
        }
      };
      std::process::exit(code);
    });
  });

  rsx! {}
}

fn report(label: &str, result: &Value) -> i32 {
  let passed: Vec<&str> = result["passed"]
    .as_array()
    .map(|names| names.iter().filter_map(Value::as_str).collect())
    .unwrap_or_default();

  if result["ok"].as_bool() == Some(true) {
    println!(
      "{label} interaction verification passed ({} scenarios: {})",
      passed.len(),
      passed.join(", ")
    );
    return 0;
  }

  eprintln!(
    "{label} interaction verification failed: {}",
    result["error"].as_str().unwrap_or("no error reported")
  );
  if !passed.is_empty() {
    eprintln!("passed before the failure: {}", passed.join(", "));
  }
  1
}
