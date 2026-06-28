use dioxus_ui_primitives::{
  FocusRuntimeRequest, GestureRuntimeRequest, LiveRegionRuntimeRequest, MeasurementRuntimeRequest,
  PointerDelta, PointerRuntimeRequest, PortalRuntimeRequest, TimerRuntimeRequest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RuntimeFamily {
  Focus,
  Portal,
  Timer,
  LiveRegion,
  Measurement,
  Pointer,
  Gesture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RuntimePanel {
  family: RuntimeFamily,
  test_id: &'static str,
  label: &'static str,
  fallback: &'static str,
}

const RUNTIME_PANELS: [RuntimePanel; 7] = [
  RuntimePanel {
    family: RuntimeFamily::Focus,
    test_id: "runtime-focus-panel",
    label: "Focus",
    fallback: "Unsupported or MissingTarget",
  },
  RuntimePanel {
    family: RuntimeFamily::Portal,
    test_id: "runtime-portal-panel",
    label: "Portal",
    fallback: "Inline, MissingTarget, or Unsupported",
  },
  RuntimePanel {
    family: RuntimeFamily::Timer,
    test_id: "runtime-timer-panel",
    label: "Timer",
    fallback: "Disabled or Unsupported",
  },
  RuntimePanel {
    family: RuntimeFamily::LiveRegion,
    test_id: "runtime-live-region-panel",
    label: "Live region",
    fallback: "EmptyMessage, SuppressedDuplicate, or Unsupported",
  },
  RuntimePanel {
    family: RuntimeFamily::Measurement,
    test_id: "runtime-measurement-panel",
    label: "Measurement",
    fallback: "Missing or Unsupported",
  },
  RuntimePanel {
    family: RuntimeFamily::Pointer,
    test_id: "runtime-pointer-panel",
    label: "Pointer",
    fallback: "Unsupported without state mutation",
  },
  RuntimePanel {
    family: RuntimeFamily::Gesture,
    test_id: "runtime-gesture-panel",
    label: "Gesture",
    fallback: "Cancel or Unsupported",
  },
];

fn main() {
  println!("dioxus-ui runtime web verification fixture scaffold");

  for panel in RUNTIME_PANELS {
    println!("panel={} testid={} fallback={}", panel.label, panel.test_id, panel.fallback);
  }

  for state in sample_contract_states() {
    println!("{state}");
  }
}

fn sample_contract_states() -> Vec<String> {
  vec![
    format!("focus={:?}", FocusRuntimeRequest::dialog_default()),
    format!("portal={:?}", PortalRuntimeRequest::body(true)),
    format!("timer={:?}", TimerRuntimeRequest::toast_dismiss(3000)),
    format!("live_region={:?}", LiveRegionRuntimeRequest::polite("Saved")),
    format!("measurement={:?}", MeasurementRuntimeRequest::<&str>::Viewport),
    format!("pointer={:?}", PointerRuntimeRequest::move_by(PointerDelta::new(4.0, 0.0))),
    format!(
      "gesture={:?}",
      GestureRuntimeRequest::horizontal(-48.0, 0.0, 32.0, 500.0).resolve_outcome()
    ),
  ]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn runtime_panels_cover_all_families() {
    assert_eq!(RUNTIME_PANELS.len(), 7);
    assert!(RUNTIME_PANELS.iter().any(|panel| panel.family == RuntimeFamily::Focus));
    assert!(RUNTIME_PANELS.iter().any(|panel| panel.family == RuntimeFamily::Portal));
    assert!(RUNTIME_PANELS.iter().any(|panel| panel.family == RuntimeFamily::Timer));
    assert!(RUNTIME_PANELS.iter().any(|panel| panel.family == RuntimeFamily::LiveRegion));
    assert!(RUNTIME_PANELS.iter().any(|panel| panel.family == RuntimeFamily::Measurement));
    assert!(RUNTIME_PANELS.iter().any(|panel| panel.family == RuntimeFamily::Pointer));
    assert!(RUNTIME_PANELS.iter().any(|panel| panel.family == RuntimeFamily::Gesture));
  }

  #[test]
  fn runtime_panels_expose_stable_test_ids() {
    for panel in RUNTIME_PANELS {
      assert!(panel.test_id.starts_with("runtime-"));
      assert!(panel.test_id.ends_with("-panel"));
    }
  }

  #[test]
  fn sample_contract_states_cover_every_runtime_family() {
    let states = sample_contract_states();

    assert_eq!(states.len(), RUNTIME_PANELS.len());
    assert!(states.iter().any(|state| state.starts_with("focus=")));
    assert!(states.iter().any(|state| state.starts_with("portal=")));
    assert!(states.iter().any(|state| state.starts_with("timer=")));
    assert!(states.iter().any(|state| state.starts_with("live_region=")));
    assert!(states.iter().any(|state| state.starts_with("measurement=")));
    assert!(states.iter().any(|state| state.starts_with("pointer=")));
    assert!(states.iter().any(|state| state.starts_with("gesture=")));
  }
}
