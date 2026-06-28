use dioxus_ui_primitives::{
  FocusRuntime, FocusRuntimeRequest, FocusRuntimeUnsupported, GestureRuntimeRequest,
  LiveRegionRuntimeRequest, MeasurementRuntimeRequest, PointerDelta, PointerRuntimeRequest,
  PortalRuntime, PortalRuntimeRequest, PortalRuntimeUnsupported, TimerRuntimeRequest,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct VerificationCheck {
  test_id: &'static str,
  label: &'static str,
  expected: &'static str,
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

const FOCUS_CHECKS: [VerificationCheck; 4] = [
  VerificationCheck {
    test_id: "runtime-focus-initial",
    label: "initial focus",
    expected: "focus initial target or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-focus-trap",
    label: "focus trap",
    expected: "trap modal scope or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-focus-return",
    label: "focus return",
    expected: "restore trigger focus or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-focus-escape-close",
    label: "escape close",
    expected: "close overlay and restore focus",
  },
];

const PORTAL_CHECKS: [VerificationCheck; 4] = [
  VerificationCheck {
    test_id: "runtime-portal-inline",
    label: "inline portal",
    expected: "Inline",
  },
  VerificationCheck {
    test_id: "runtime-portal-body",
    label: "body portal",
    expected: "Mounted, MissingTarget, or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-portal-named-target",
    label: "named portal target",
    expected: "Mounted, MissingTarget, or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-portal-missing-target",
    label: "missing portal target",
    expected: "MissingTarget or Unsupported",
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

  for state in focus_portal_panel_states() {
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

fn focus_portal_panel_states() -> Vec<String> {
  let focus_request = FocusRuntimeRequest::dialog_default();
  let focus_runtime = FocusRuntimeUnsupported;
  let focus_node = ();
  let portal_runtime = PortalRuntimeUnsupported;
  let inline_request = PortalRuntimeRequest::inline(false);
  let body_request = PortalRuntimeRequest::body(true);
  let named_request = PortalRuntimeRequest::selector("#runtime-overlay-root", true);
  let missing_request = PortalRuntimeRequest::selector("#missing-runtime-target", true);

  vec![
    format!(
      "focus_initial testid={} expected={} result={:?}",
      FOCUS_CHECKS[0].test_id,
      FOCUS_CHECKS[0].expected,
      focus_runtime.focus_initial(&focus_node, focus_request)
    ),
    format!(
      "focus_trap testid={} expected={} result={:?}",
      FOCUS_CHECKS[1].test_id,
      FOCUS_CHECKS[1].expected,
      focus_runtime.trap_focus(&focus_node, focus_request)
    ),
    format!(
      "focus_return testid={} expected={} result={:?}",
      FOCUS_CHECKS[2].test_id,
      FOCUS_CHECKS[2].expected,
      focus_runtime.restore_focus(&focus_node, focus_request)
    ),
    format!(
      "focus_escape testid={} expected={} result=pending-browser-assertion",
      FOCUS_CHECKS[3].test_id, FOCUS_CHECKS[3].expected
    ),
    format!(
      "portal_inline testid={} expected={} result={:?}",
      PORTAL_CHECKS[0].test_id,
      PORTAL_CHECKS[0].expected,
      portal_runtime.mount_target(&inline_request)
    ),
    format!(
      "portal_body testid={} expected={} result={:?}",
      PORTAL_CHECKS[1].test_id,
      PORTAL_CHECKS[1].expected,
      portal_runtime.mount_target(&body_request)
    ),
    format!(
      "portal_named testid={} expected={} result={:?}",
      PORTAL_CHECKS[2].test_id,
      PORTAL_CHECKS[2].expected,
      portal_runtime.mount_target(&named_request)
    ),
    format!(
      "portal_missing testid={} expected={} result={:?}",
      PORTAL_CHECKS[3].test_id,
      PORTAL_CHECKS[3].expected,
      portal_runtime.mount_target(&missing_request)
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

  #[test]
  fn focus_panel_checks_expose_stable_test_ids() {
    assert_eq!(FOCUS_CHECKS.len(), 4);
    assert!(FOCUS_CHECKS.iter().any(|check| check.test_id == "runtime-focus-initial"));
    assert!(FOCUS_CHECKS.iter().any(|check| check.test_id == "runtime-focus-trap"));
    assert!(FOCUS_CHECKS.iter().any(|check| check.test_id == "runtime-focus-return"));
    assert!(FOCUS_CHECKS.iter().any(|check| check.test_id == "runtime-focus-escape-close"));
  }

  #[test]
  fn portal_panel_checks_expose_stable_test_ids() {
    assert_eq!(PORTAL_CHECKS.len(), 4);
    assert!(PORTAL_CHECKS.iter().any(|check| check.test_id == "runtime-portal-inline"));
    assert!(PORTAL_CHECKS.iter().any(|check| check.test_id == "runtime-portal-body"));
    assert!(PORTAL_CHECKS.iter().any(|check| check.test_id == "runtime-portal-named-target"));
    assert!(PORTAL_CHECKS.iter().any(|check| check.test_id == "runtime-portal-missing-target"));
  }

  #[test]
  fn focus_portal_states_report_unsupported_fallbacks() {
    let states = focus_portal_panel_states();

    assert_eq!(states.len(), FOCUS_CHECKS.len() + PORTAL_CHECKS.len());
    assert!(states.iter().any(|state| state.contains("focus_initial")));
    assert!(states.iter().any(|state| state.contains("focus_trap")));
    assert!(states.iter().any(|state| state.contains("focus_return")));
    assert!(states.iter().any(|state| state.contains("focus_escape")));
    assert!(states.iter().any(|state| state.contains("portal_inline")));
    assert!(states.iter().any(|state| state.contains("portal_body")));
    assert!(states.iter().any(|state| state.contains("portal_named")));
    assert!(states.iter().any(|state| state.contains("portal_missing")));
  }

  #[test]
  fn unsupported_focus_and_portal_results_are_explicit() {
    let focus_runtime = FocusRuntimeUnsupported;
    let focus_request = FocusRuntimeRequest::dialog_default();
    let focus_node = ();
    let portal_runtime = PortalRuntimeUnsupported;

    assert_eq!(
      focus_runtime.focus_initial(&focus_node, focus_request),
      dioxus_ui_primitives::FocusCommandResult::Unsupported
    );
    assert_eq!(
      portal_runtime.mount_target(&PortalRuntimeRequest::inline(false)),
      dioxus_ui_primitives::PortalMountResult::Inline
    );
    assert_eq!(
      portal_runtime.mount_target(&PortalRuntimeRequest::selector("#missing", true)),
      dioxus_ui_primitives::PortalMountResult::Unsupported
    );
  }
}
