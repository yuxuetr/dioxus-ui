mod web_runtime;

use dioxus_ui_primitives::{
  CarouselState, FocusRuntime, FocusRuntimeRequest, FocusRuntimeUnsupported, GestureRuntime,
  GestureRuntimeRequest, GestureRuntimeUnsupported, LiveRegionRuntime, LiveRegionRuntimeRequest,
  LiveRegionRuntimeUnsupported, MeasurementRuntime, MeasurementRuntimeRequest,
  MeasurementRuntimeUnsupported, PointerDelta, PointerRuntime, PointerRuntimeRequest,
  PointerRuntimeUnsupported, PortalRuntime, PortalRuntimeRequest, PortalRuntimeUnsupported,
  TimerRuntime, TimerRuntimeRequest, TimerRuntimeUnsupported, carousel_apply_gesture,
};
use web_runtime::{WebLiveRegionRuntime, WebTimerRuntime};

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

const TIMER_CHECKS: [VerificationCheck; 4] = [
  VerificationCheck {
    test_id: "runtime-timer-schedule",
    label: "schedule timer",
    expected: "Scheduled or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-timer-cancel",
    label: "cancel timer",
    expected: "Cancelled, Missing, or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-timer-disabled",
    label: "disabled timer",
    expected: "Disabled",
  },
  VerificationCheck {
    test_id: "runtime-timer-cleanup",
    label: "timer cleanup",
    expected: "no stale callback after unmount",
  },
];

const LIVE_REGION_CHECKS: [VerificationCheck; 4] = [
  VerificationCheck {
    test_id: "runtime-live-region-polite",
    label: "polite announcement",
    expected: "Queued or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-live-region-assertive",
    label: "assertive announcement",
    expected: "Queued or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-live-region-duplicate",
    label: "duplicate suppression",
    expected: "SuppressedDuplicate or true policy check",
  },
  VerificationCheck {
    test_id: "runtime-live-region-empty",
    label: "empty announcement",
    expected: "EmptyMessage",
  },
];

const MEASUREMENT_CHECKS: [VerificationCheck; 4] = [
  VerificationCheck {
    test_id: "runtime-measurement-node",
    label: "node measurement",
    expected: "Rect, Missing, or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-measurement-viewport",
    label: "viewport measurement",
    expected: "Rect or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-measurement-scroll-resize",
    label: "scroll and resize invalidation",
    expected: "pending-browser-assertion",
  },
  VerificationCheck {
    test_id: "runtime-measurement-missing",
    label: "missing measurement target",
    expected: "Missing or Unsupported",
  },
];

const POINTER_CHECKS: [VerificationCheck; 5] = [
  VerificationCheck {
    test_id: "runtime-pointer-start",
    label: "pointer start",
    expected: "Started or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-pointer-move",
    label: "pointer move",
    expected: "Moved or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-pointer-end",
    label: "pointer end",
    expected: "Ended or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-pointer-cancel",
    label: "pointer cancel",
    expected: "Cancelled or Unsupported",
  },
  VerificationCheck {
    test_id: "runtime-pointer-capture-release",
    label: "pointer capture release",
    expected: "pending-browser-assertion",
  },
];

const GESTURE_CHECKS: [VerificationCheck; 4] = [
  VerificationCheck {
    test_id: "runtime-gesture-next",
    label: "carousel next gesture",
    expected: "CommitNext",
  },
  VerificationCheck {
    test_id: "runtime-gesture-previous",
    label: "carousel previous gesture",
    expected: "CommitPrevious",
  },
  VerificationCheck {
    test_id: "runtime-gesture-cancel",
    label: "gesture cancel",
    expected: "Cancel",
  },
  VerificationCheck {
    test_id: "runtime-gesture-native-scroll",
    label: "native scroll escape",
    expected: "pending-browser-assertion",
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

  for state in timer_live_region_measurement_panel_states() {
    println!("{state}");
  }

  for state in pointer_gesture_panel_states() {
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

fn timer_live_region_measurement_panel_states() -> Vec<String> {
  let web_timer_runtime = WebTimerRuntime::new();
  let unsupported_timer_runtime = TimerRuntimeUnsupported;
  let timer_request = TimerRuntimeRequest::toast_dismiss(3000);
  let disabled_timer_request = TimerRuntimeRequest::toast_dismiss(0);
  let scheduled_timer = web_timer_runtime.schedule_once(&timer_request);
  let cancelled_timer = match scheduled_timer {
    dioxus_ui_primitives::TimerRuntimeResult::Scheduled(timer_id) => {
      web_timer_runtime.cancel(&timer_id)
    }
    _ => dioxus_ui_primitives::TimerRuntimeResult::Unsupported,
  };
  let unsupported_timer_id = ();
  let web_live_region_runtime = WebLiveRegionRuntime::new();
  let unsupported_live_region_runtime = LiveRegionRuntimeUnsupported;
  let polite_request = LiveRegionRuntimeRequest::polite("Saved");
  let assertive_request = LiveRegionRuntimeRequest::assertive("Failed");
  let empty_request = LiveRegionRuntimeRequest::polite(" ");
  let polite_result = web_live_region_runtime.announce(&polite_request);
  let duplicate_result = web_live_region_runtime.announce(&polite_request);
  let assertive_result = web_live_region_runtime.announce(&assertive_request);
  let measurement_runtime = MeasurementRuntimeUnsupported;
  let node = ();

  vec![
    format!(
      "timer_schedule testid={} expected={} result={:?}",
      TIMER_CHECKS[0].test_id, TIMER_CHECKS[0].expected, scheduled_timer
    ),
    format!(
      "timer_cancel testid={} expected={} result={:?}",
      TIMER_CHECKS[1].test_id, TIMER_CHECKS[1].expected, cancelled_timer
    ),
    format!(
      "timer_disabled testid={} expected={} result={:?}",
      TIMER_CHECKS[2].test_id,
      TIMER_CHECKS[2].expected,
      web_timer_runtime.schedule_once(&disabled_timer_request)
    ),
    format!(
      "timer_cleanup testid={} expected={} result=pending-browser-assertion",
      TIMER_CHECKS[3].test_id, TIMER_CHECKS[3].expected
    ),
    format!(
      "timer_unsupported testid=runtime-timer-unsupported expected=Unsupported result={:?}",
      unsupported_timer_runtime.cancel(&unsupported_timer_id)
    ),
    format!(
      "live_region_polite testid={} expected={} result={:?}",
      LIVE_REGION_CHECKS[0].test_id, LIVE_REGION_CHECKS[0].expected, polite_result
    ),
    format!(
      "live_region_assertive testid={} expected={} result={:?}",
      LIVE_REGION_CHECKS[1].test_id, LIVE_REGION_CHECKS[1].expected, assertive_result
    ),
    format!(
      "live_region_duplicate testid={} expected={} result={}",
      LIVE_REGION_CHECKS[2].test_id,
      LIVE_REGION_CHECKS[2].expected,
      duplicate_result == dioxus_ui_primitives::LiveRegionRuntimeResult::SuppressedDuplicate
    ),
    format!(
      "live_region_empty testid={} expected={} result={:?}",
      LIVE_REGION_CHECKS[3].test_id,
      LIVE_REGION_CHECKS[3].expected,
      web_live_region_runtime.announce(&empty_request)
    ),
    format!(
      "live_region_unsupported testid=runtime-live-region-unsupported expected=Unsupported result={:?}",
      unsupported_live_region_runtime.announce(&polite_request)
    ),
    format!(
      "measurement_node testid={} expected={} result={:?}",
      MEASUREMENT_CHECKS[0].test_id,
      MEASUREMENT_CHECKS[0].expected,
      measurement_runtime.measure(&MeasurementRuntimeRequest::Node(node))
    ),
    format!(
      "measurement_viewport testid={} expected={} result={:?}",
      MEASUREMENT_CHECKS[1].test_id,
      MEASUREMENT_CHECKS[1].expected,
      measurement_runtime.measure(&MeasurementRuntimeRequest::Viewport)
    ),
    format!(
      "measurement_scroll_resize testid={} expected={} result=pending-browser-assertion",
      MEASUREMENT_CHECKS[2].test_id, MEASUREMENT_CHECKS[2].expected
    ),
    format!(
      "measurement_missing testid={} expected={} result=pending-adapter-missing-target",
      MEASUREMENT_CHECKS[3].test_id, MEASUREMENT_CHECKS[3].expected
    ),
  ]
}

fn pointer_gesture_panel_states() -> Vec<String> {
  let pointer_runtime = PointerRuntimeUnsupported;
  let next_request = GestureRuntimeRequest::horizontal(-48.0, 0.0, 32.0, 500.0);
  let previous_request = GestureRuntimeRequest::horizontal(48.0, 0.0, 32.0, 500.0);
  let cancel_request = GestureRuntimeRequest::horizontal(8.0, 0.0, 32.0, 500.0);
  let gesture_runtime = GestureRuntimeUnsupported;
  let carousel_state = CarouselState::new(1, 3);
  let next_outcome = next_request.resolve_outcome();
  let previous_outcome = previous_request.resolve_outcome();
  let cancel_outcome = cancel_request.resolve_outcome();

  vec![
    format!(
      "pointer_start testid={} expected={} result={:?}",
      POINTER_CHECKS[0].test_id,
      POINTER_CHECKS[0].expected,
      pointer_runtime.handle_pointer(&PointerRuntimeRequest::start())
    ),
    format!(
      "pointer_move testid={} expected={} result={:?}",
      POINTER_CHECKS[1].test_id,
      POINTER_CHECKS[1].expected,
      pointer_runtime.handle_pointer(&PointerRuntimeRequest::move_by(PointerDelta::new(12.0, 0.0)))
    ),
    format!(
      "pointer_end testid={} expected={} result={:?}",
      POINTER_CHECKS[2].test_id,
      POINTER_CHECKS[2].expected,
      pointer_runtime.handle_pointer(&PointerRuntimeRequest::end())
    ),
    format!(
      "pointer_cancel testid={} expected={} result={:?}",
      POINTER_CHECKS[3].test_id,
      POINTER_CHECKS[3].expected,
      pointer_runtime.handle_pointer(&PointerRuntimeRequest::cancel())
    ),
    format!(
      "pointer_capture_release testid={} expected={} result=pending-browser-assertion",
      POINTER_CHECKS[4].test_id, POINTER_CHECKS[4].expected
    ),
    format!(
      "gesture_next testid={} expected={} outcome={:?} index={}",
      GESTURE_CHECKS[0].test_id,
      GESTURE_CHECKS[0].expected,
      next_outcome,
      carousel_apply_gesture(carousel_state, next_outcome).index
    ),
    format!(
      "gesture_previous testid={} expected={} outcome={:?} index={}",
      GESTURE_CHECKS[1].test_id,
      GESTURE_CHECKS[1].expected,
      previous_outcome,
      carousel_apply_gesture(carousel_state, previous_outcome).index
    ),
    format!(
      "gesture_cancel testid={} expected={} outcome={:?} index={}",
      GESTURE_CHECKS[2].test_id,
      GESTURE_CHECKS[2].expected,
      cancel_outcome,
      carousel_apply_gesture(carousel_state, cancel_outcome).index
    ),
    format!(
      "gesture_native_scroll testid={} expected={} result=pending-browser-assertion",
      GESTURE_CHECKS[3].test_id, GESTURE_CHECKS[3].expected
    ),
    format!(
      "gesture_unsupported testid=runtime-gesture-unsupported expected=Unsupported result={:?}",
      gesture_runtime.resolve_gesture(&next_request)
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

  #[test]
  fn timer_panel_checks_expose_stable_test_ids() {
    assert_eq!(TIMER_CHECKS.len(), 4);
    assert!(TIMER_CHECKS.iter().any(|check| check.test_id == "runtime-timer-schedule"));
    assert!(TIMER_CHECKS.iter().any(|check| check.test_id == "runtime-timer-cancel"));
    assert!(TIMER_CHECKS.iter().any(|check| check.test_id == "runtime-timer-disabled"));
    assert!(TIMER_CHECKS.iter().any(|check| check.test_id == "runtime-timer-cleanup"));
  }

  #[test]
  fn live_region_panel_checks_expose_stable_test_ids() {
    assert_eq!(LIVE_REGION_CHECKS.len(), 4);
    assert!(LIVE_REGION_CHECKS.iter().any(|check| check.test_id == "runtime-live-region-polite"));
    assert!(
      LIVE_REGION_CHECKS.iter().any(|check| check.test_id == "runtime-live-region-assertive")
    );
    assert!(
      LIVE_REGION_CHECKS.iter().any(|check| check.test_id == "runtime-live-region-duplicate")
    );
    assert!(LIVE_REGION_CHECKS.iter().any(|check| check.test_id == "runtime-live-region-empty"));
  }

  #[test]
  fn measurement_panel_checks_expose_stable_test_ids() {
    assert_eq!(MEASUREMENT_CHECKS.len(), 4);
    assert!(MEASUREMENT_CHECKS.iter().any(|check| check.test_id == "runtime-measurement-node"));
    assert!(MEASUREMENT_CHECKS.iter().any(|check| check.test_id == "runtime-measurement-viewport"));
    assert!(
      MEASUREMENT_CHECKS.iter().any(|check| check.test_id == "runtime-measurement-scroll-resize")
    );
    assert!(MEASUREMENT_CHECKS.iter().any(|check| check.test_id == "runtime-measurement-missing"));
  }

  #[test]
  fn timer_live_region_measurement_states_report_fallbacks() {
    let states = timer_live_region_measurement_panel_states();

    assert_eq!(
      states.len(),
      TIMER_CHECKS.len() + LIVE_REGION_CHECKS.len() + MEASUREMENT_CHECKS.len() + 2
    );
    assert!(states.iter().any(|state| state.contains("timer_schedule")));
    assert!(states.iter().any(|state| state.contains("timer_cancel")));
    assert!(states.iter().any(|state| state.contains("timer_disabled")));
    assert!(states.iter().any(|state| state.contains("timer_unsupported")));
    assert!(states.iter().any(|state| state.contains("live_region_polite")));
    assert!(states.iter().any(|state| state.contains("live_region_empty")));
    assert!(states.iter().any(|state| state.contains("live_region_unsupported")));
    assert!(states.iter().any(|state| state.contains("measurement_node")));
    assert!(states.iter().any(|state| state.contains("measurement_viewport")));
    assert!(states.iter().any(|state| state.contains("measurement_missing")));
  }

  #[test]
  fn unsupported_timer_live_region_and_measurement_results_are_explicit() {
    let timer_runtime = TimerRuntimeUnsupported;
    let live_region_runtime = LiveRegionRuntimeUnsupported;
    let measurement_runtime = MeasurementRuntimeUnsupported;
    let node = ();

    assert_eq!(
      timer_runtime.schedule_once(&TimerRuntimeRequest::toast_dismiss(0)),
      dioxus_ui_primitives::TimerRuntimeResult::Disabled
    );
    assert_eq!(
      live_region_runtime.announce(&LiveRegionRuntimeRequest::polite(" ")),
      dioxus_ui_primitives::LiveRegionRuntimeResult::EmptyMessage
    );
    assert_eq!(
      measurement_runtime.measure(&MeasurementRuntimeRequest::Node(node)),
      dioxus_ui_primitives::MeasurementRuntimeResult::Unsupported
    );
  }

  #[test]
  fn web_feedback_adapters_report_success_and_policy_results() {
    let timer_runtime = WebTimerRuntime::new();
    let live_region_runtime = WebLiveRegionRuntime::new();
    let timer_request = TimerRuntimeRequest::toast_dismiss(3000);
    let scheduled = timer_runtime.schedule_once(&timer_request);

    let timer_id = match scheduled {
      dioxus_ui_primitives::TimerRuntimeResult::Scheduled(timer_id) => timer_id,
      other => panic!("expected scheduled timer, got {other:?}"),
    };

    assert_eq!(
      timer_runtime.cancel(&timer_id),
      dioxus_ui_primitives::TimerRuntimeResult::Cancelled
    );
    assert_eq!(
      live_region_runtime.announce(&LiveRegionRuntimeRequest::polite("Saved")),
      dioxus_ui_primitives::LiveRegionRuntimeResult::Queued
    );
    assert_eq!(
      live_region_runtime.announce(&LiveRegionRuntimeRequest::polite("Saved")),
      dioxus_ui_primitives::LiveRegionRuntimeResult::SuppressedDuplicate
    );
  }

  #[test]
  fn pointer_panel_checks_expose_stable_test_ids() {
    assert_eq!(POINTER_CHECKS.len(), 5);
    assert!(POINTER_CHECKS.iter().any(|check| check.test_id == "runtime-pointer-start"));
    assert!(POINTER_CHECKS.iter().any(|check| check.test_id == "runtime-pointer-move"));
    assert!(POINTER_CHECKS.iter().any(|check| check.test_id == "runtime-pointer-end"));
    assert!(POINTER_CHECKS.iter().any(|check| check.test_id == "runtime-pointer-cancel"));
    assert!(POINTER_CHECKS.iter().any(|check| check.test_id == "runtime-pointer-capture-release"));
  }

  #[test]
  fn gesture_panel_checks_expose_stable_test_ids() {
    assert_eq!(GESTURE_CHECKS.len(), 4);
    assert!(GESTURE_CHECKS.iter().any(|check| check.test_id == "runtime-gesture-next"));
    assert!(GESTURE_CHECKS.iter().any(|check| check.test_id == "runtime-gesture-previous"));
    assert!(GESTURE_CHECKS.iter().any(|check| check.test_id == "runtime-gesture-cancel"));
    assert!(GESTURE_CHECKS.iter().any(|check| check.test_id == "runtime-gesture-native-scroll"));
  }

  #[test]
  fn pointer_gesture_states_report_fallbacks_and_carousel_outcomes() {
    let states = pointer_gesture_panel_states();

    assert_eq!(states.len(), POINTER_CHECKS.len() + GESTURE_CHECKS.len() + 1);
    assert!(states.iter().any(|state| state.contains("pointer_start")));
    assert!(states.iter().any(|state| state.contains("pointer_move")));
    assert!(states.iter().any(|state| state.contains("pointer_end")));
    assert!(states.iter().any(|state| state.contains("pointer_cancel")));
    assert!(states.iter().any(|state| state.contains("gesture_next")));
    assert!(states.iter().any(|state| state.contains("gesture_previous")));
    assert!(states.iter().any(|state| state.contains("gesture_cancel")));
    assert!(states.iter().any(|state| state.contains("gesture_unsupported")));
  }

  #[test]
  fn pointer_and_gesture_results_are_explicit() {
    let pointer_runtime = PointerRuntimeUnsupported;
    let gesture_runtime = GestureRuntimeUnsupported;
    let next_request = GestureRuntimeRequest::horizontal(-48.0, 0.0, 32.0, 500.0);
    let carousel_state = CarouselState::new(1, 3);

    assert_eq!(
      pointer_runtime.handle_pointer(&PointerRuntimeRequest::start()),
      dioxus_ui_primitives::PointerRuntimeResult::Unsupported
    );
    assert_eq!(
      gesture_runtime.resolve_gesture(&next_request),
      dioxus_ui_primitives::GestureRuntimeResult::Unsupported
    );
    assert_eq!(carousel_apply_gesture(carousel_state, next_request.resolve_outcome()).index, 2);
  }
}
