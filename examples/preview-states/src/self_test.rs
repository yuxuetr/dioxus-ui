use dioxus::prelude::*;
use serde_json::Value;

/// Interaction scenarios the Desktop and Mobile previews run in their own
/// WebView (RFC 0017, RFC 0018). The script sends one `{ ok, passed, error }`
/// result.
pub const INTERACTION_SELF_TEST_SCRIPT: &str = include_str!("../self-test/interactions.js");

/// Runs the interaction scenarios once, prints the result prefixed with
/// `label`, and exits the process with status 0 when every scenario passed.
/// The Mobile preview also prints each control's size at defaults (M211.1).
#[component]
pub fn InteractionSelfTest(label: &'static str) -> Element {
  use_effect(move || {
    spawn(async move {
      let mut eval = document::eval(INTERACTION_SELF_TEST_SCRIPT);
      let code = match eval.recv::<Value>().await {
        Ok(result) => {
          if label == "mobile" {
            report_touch_targets(&result);
          }
          report(label, &result)
        }
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

/// The touch target guideline (WCAG 2.5.5, Apple's Human Interface
/// Guidelines) in CSS pixels.
const TOUCH_TARGET: f64 = 44.0;

/// Prints one `touch target` line per control, smallest first, then a count
/// of the controls under 44 by 44 CSS pixels.
fn report_touch_targets(result: &Value) {
  let mut targets: Vec<(&str, &str, &str, f64, f64)> = result["targets"]
    .as_array()
    .map(|targets| {
      targets
        .iter()
        .map(|target| {
          (
            target["target"].as_str().unwrap_or_default(),
            target["control"].as_str().unwrap_or_default(),
            target["name"].as_str().unwrap_or_default(),
            target["width"].as_f64().unwrap_or_default(),
            target["height"].as_f64().unwrap_or_default(),
          )
        })
        .collect()
    })
    .unwrap_or_default();
  targets.sort_by(|left, right| left.3.min(left.4).total_cmp(&right.3.min(right.4)));
  for (target, control, name, width, height) in &targets {
    println!("touch target {width}x{height} {target} {control} \"{name}\"");
  }
  let small =
    targets.iter().filter(|target| target.3 < TOUCH_TARGET || target.4 < TOUCH_TARGET).count();
  println!("touch targets: {small} of {} controls under 44x44", targets.len());
}
