use dioxus_shadcn_primitives::{
  FocusRuntime, FocusRuntimeRequest, FocusRuntimeUnsupported, GestureRuntimeRequest,
  LiveRegionRuntimeRequest, MeasurementRuntime, MeasurementRuntimeRequest,
  MeasurementRuntimeUnsupported, PointerDelta, PointerRuntime, PointerRuntimeRequest,
  PointerRuntimeUnsupported, PortalRuntime, PortalRuntimeRequest, PortalRuntimeUnsupported,
  TimerRuntime, TimerRuntimeRequest, TimerRuntimeUnsupported,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DesktopRuntimeFamily {
  Focus,
  Portal,
  Timer,
  LiveStatus,
  Measurement,
  Pointer,
  Gesture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DesktopRuntimePanel {
  family: DesktopRuntimeFamily,
  test_id: &'static str,
  label: &'static str,
  desktop_risk: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DesktopSmokeCheck {
  test_id: &'static str,
  label: &'static str,
  expected: &'static str,
}

const DESKTOP_RUNTIME_PANELS: [DesktopRuntimePanel; 7] = [
  DesktopRuntimePanel {
    family: DesktopRuntimeFamily::Focus,
    test_id: "desktop-runtime-focus-panel",
    label: "Focus",
    desktop_risk: "WebView focus return, trap, and escape behavior",
  },
  DesktopRuntimePanel {
    family: DesktopRuntimeFamily::Portal,
    test_id: "desktop-runtime-portal-panel",
    label: "Portal",
    desktop_risk: "WebView body and named target stacking",
  },
  DesktopRuntimePanel {
    family: DesktopRuntimeFamily::Timer,
    test_id: "desktop-runtime-timer-panel",
    label: "Timer",
    desktop_risk: "window hidden or background timer behavior",
  },
  DesktopRuntimePanel {
    family: DesktopRuntimeFamily::LiveStatus,
    test_id: "desktop-runtime-live-status-panel",
    label: "Live status",
    desktop_risk: "manual assistive behavior before support claims",
  },
  DesktopRuntimePanel {
    family: DesktopRuntimeFamily::Measurement,
    test_id: "desktop-runtime-measurement-panel",
    label: "Measurement",
    desktop_risk: "WebView rect consistency and device scale",
  },
  DesktopRuntimePanel {
    family: DesktopRuntimeFamily::Pointer,
    test_id: "desktop-runtime-pointer-panel",
    label: "Pointer",
    desktop_risk: "pointer capture and lost-capture behavior",
  },
  DesktopRuntimePanel {
    family: DesktopRuntimeFamily::Gesture,
    test_id: "desktop-runtime-gesture-panel",
    label: "Gesture",
    desktop_risk: "conservative support after pointer behavior is proven",
  },
];

const DESKTOP_SMOKE_CHECKS: [DesktopSmokeCheck; 18] = [
  DesktopSmokeCheck {
    test_id: "desktop-runtime-focus-initial",
    label: "initial focus",
    expected: "Applied, MissingTarget, or Unsupported",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-focus-trap",
    label: "modal focus trap",
    expected: "Applied, MissingTarget, or Unsupported",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-focus-return",
    label: "focus return",
    expected: "Applied, MissingTarget, or Unsupported",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-focus-escape",
    label: "escape close",
    expected: "pending-desktop-webview-smoke",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-portal-inline",
    label: "inline portal",
    expected: "Inline",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-portal-body-stack",
    label: "body portal stack",
    expected: "Mounted, MissingTarget, or Unsupported",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-portal-named-stack",
    label: "named portal stack",
    expected: "Mounted, MissingTarget, or Unsupported",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-portal-missing",
    label: "missing portal target",
    expected: "MissingTarget or Unsupported",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-timer-schedule",
    label: "schedule timer",
    expected: "Scheduled or Unsupported",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-timer-disabled",
    label: "disabled timer",
    expected: "Disabled",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-timer-cleanup",
    label: "timer cleanup",
    expected: "pending-desktop-webview-smoke",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-live-status-visible",
    label: "visible live status",
    expected: "manual-assistive-check-required",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-measurement-anchor",
    label: "anchor measurement",
    expected: "Rect, Missing, or Unsupported",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-measurement-viewport",
    label: "viewport measurement",
    expected: "Rect or Unsupported",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-measurement-device-scale",
    label: "device scale",
    expected: "pending-desktop-webview-smoke",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-pointer-start",
    label: "pointer start",
    expected: "Started or Unsupported",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-pointer-capture-release",
    label: "pointer capture release",
    expected: "pending-desktop-webview-smoke",
  },
  DesktopSmokeCheck {
    test_id: "desktop-runtime-gesture-conservative",
    label: "gesture conservative support",
    expected: "Cancel or deferred until pointer smoke passes",
  },
];

fn main() {
  println!("dioxus-shadcn runtime desktop verification smoke fixture");

  for panel in DESKTOP_RUNTIME_PANELS {
    println!("panel={} testid={} desktop_risk={}", panel.label, panel.test_id, panel.desktop_risk);
  }

  for state in desktop_smoke_states() {
    println!("{state}");
  }
}

fn desktop_smoke_states() -> Vec<String> {
  let focus_runtime = FocusRuntimeUnsupported;
  let focus_node = ();
  let focus_request = FocusRuntimeRequest::dialog_default();
  let portal_runtime = PortalRuntimeUnsupported;
  let timer_runtime = TimerRuntimeUnsupported;
  let timer_id = ();
  let measurement_runtime = MeasurementRuntimeUnsupported;
  let measurement_node = ();
  let pointer_runtime = PointerRuntimeUnsupported;
  let visible_status = LiveRegionRuntimeRequest::polite("Desktop status visible");
  let gesture_request = GestureRuntimeRequest::horizontal(8.0, 0.0, 32.0, 500.0);

  vec![
    format!(
      "focus_initial testid={} expected={} result={:?}",
      DESKTOP_SMOKE_CHECKS[0].test_id,
      DESKTOP_SMOKE_CHECKS[0].expected,
      focus_runtime.focus_initial(&focus_node, focus_request)
    ),
    format!(
      "focus_trap testid={} expected={} result={:?}",
      DESKTOP_SMOKE_CHECKS[1].test_id,
      DESKTOP_SMOKE_CHECKS[1].expected,
      focus_runtime.trap_focus(&focus_node, focus_request)
    ),
    format!(
      "focus_return testid={} expected={} result={:?}",
      DESKTOP_SMOKE_CHECKS[2].test_id,
      DESKTOP_SMOKE_CHECKS[2].expected,
      focus_runtime.restore_focus(&focus_node, focus_request)
    ),
    format!(
      "focus_escape testid={} expected={} result=pending-desktop-webview-smoke",
      DESKTOP_SMOKE_CHECKS[3].test_id, DESKTOP_SMOKE_CHECKS[3].expected
    ),
    format!(
      "portal_inline testid={} expected={} result={:?}",
      DESKTOP_SMOKE_CHECKS[4].test_id,
      DESKTOP_SMOKE_CHECKS[4].expected,
      portal_runtime.mount_target(&PortalRuntimeRequest::inline(false))
    ),
    format!(
      "portal_body_stack testid={} expected={} result={:?}",
      DESKTOP_SMOKE_CHECKS[5].test_id,
      DESKTOP_SMOKE_CHECKS[5].expected,
      portal_runtime.mount_target(&PortalRuntimeRequest::body(true))
    ),
    format!(
      "portal_named_stack testid={} expected={} result={:?}",
      DESKTOP_SMOKE_CHECKS[6].test_id,
      DESKTOP_SMOKE_CHECKS[6].expected,
      portal_runtime.mount_target(&PortalRuntimeRequest::selector("#desktop-overlay-root", true))
    ),
    format!(
      "portal_missing testid={} expected={} result={:?}",
      DESKTOP_SMOKE_CHECKS[7].test_id,
      DESKTOP_SMOKE_CHECKS[7].expected,
      portal_runtime.mount_target(&PortalRuntimeRequest::selector("#missing-desktop-root", true))
    ),
    format!(
      "timer_schedule testid={} expected={} result={:?}",
      DESKTOP_SMOKE_CHECKS[8].test_id,
      DESKTOP_SMOKE_CHECKS[8].expected,
      timer_runtime.schedule_once(&TimerRuntimeRequest::toast_dismiss(3000))
    ),
    format!(
      "timer_disabled testid={} expected={} result={:?}",
      DESKTOP_SMOKE_CHECKS[9].test_id,
      DESKTOP_SMOKE_CHECKS[9].expected,
      timer_runtime.schedule_once(&TimerRuntimeRequest::toast_dismiss(0))
    ),
    format!(
      "timer_cleanup testid={} expected={} result={:?}",
      DESKTOP_SMOKE_CHECKS[10].test_id,
      DESKTOP_SMOKE_CHECKS[10].expected,
      timer_runtime.cancel(&timer_id)
    ),
    format!(
      "live_status_visible testid={} expected={} message={:?}",
      DESKTOP_SMOKE_CHECKS[11].test_id, DESKTOP_SMOKE_CHECKS[11].expected, visible_status.message
    ),
    format!(
      "measurement_anchor testid={} expected={} result={:?}",
      DESKTOP_SMOKE_CHECKS[12].test_id,
      DESKTOP_SMOKE_CHECKS[12].expected,
      measurement_runtime.measure(&MeasurementRuntimeRequest::Node(measurement_node))
    ),
    format!(
      "measurement_viewport testid={} expected={} result={:?}",
      DESKTOP_SMOKE_CHECKS[13].test_id,
      DESKTOP_SMOKE_CHECKS[13].expected,
      measurement_runtime.measure(&MeasurementRuntimeRequest::Viewport)
    ),
    format!(
      "measurement_device_scale testid={} expected={} result=pending-desktop-webview-smoke",
      DESKTOP_SMOKE_CHECKS[14].test_id, DESKTOP_SMOKE_CHECKS[14].expected
    ),
    format!(
      "pointer_start testid={} expected={} result={:?}",
      DESKTOP_SMOKE_CHECKS[15].test_id,
      DESKTOP_SMOKE_CHECKS[15].expected,
      pointer_runtime.handle_pointer(&PointerRuntimeRequest::start())
    ),
    format!(
      "pointer_capture_release testid={} expected={} result=pending-desktop-webview-smoke",
      DESKTOP_SMOKE_CHECKS[16].test_id, DESKTOP_SMOKE_CHECKS[16].expected
    ),
    format!(
      "gesture_conservative testid={} expected={} outcome={:?}",
      DESKTOP_SMOKE_CHECKS[17].test_id,
      DESKTOP_SMOKE_CHECKS[17].expected,
      gesture_request.resolve_outcome()
    ),
    format!(
      "pointer_delta_sample testid=desktop-runtime-pointer-delta expected=normalized-delta result={:?}",
      PointerRuntimeRequest::move_by(PointerDelta::new(4.0, 0.0))
    ),
  ]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn panels_cover_desktop_runtime_families() {
    assert_eq!(DESKTOP_RUNTIME_PANELS.len(), 7);
    assert!(DESKTOP_RUNTIME_PANELS.iter().any(|panel| panel.family == DesktopRuntimeFamily::Focus));
    assert!(
      DESKTOP_RUNTIME_PANELS.iter().any(|panel| panel.family == DesktopRuntimeFamily::Portal)
    );
    assert!(DESKTOP_RUNTIME_PANELS.iter().any(|panel| panel.family == DesktopRuntimeFamily::Timer));
    assert!(
      DESKTOP_RUNTIME_PANELS.iter().any(|panel| panel.family == DesktopRuntimeFamily::Measurement)
    );
    assert!(
      DESKTOP_RUNTIME_PANELS.iter().any(|panel| panel.family == DesktopRuntimeFamily::Pointer)
    );
  }

  #[test]
  fn smoke_checks_have_stable_desktop_test_ids() {
    assert_eq!(DESKTOP_SMOKE_CHECKS.len(), 18);

    for check in DESKTOP_SMOKE_CHECKS {
      assert!(check.test_id.starts_with("desktop-runtime-"));
    }
  }

  #[test]
  fn smoke_states_cover_desktop_webview_risks() {
    let states = desktop_smoke_states();

    assert_eq!(states.len(), DESKTOP_SMOKE_CHECKS.len() + 1);
    assert!(states.iter().any(|state| state.contains("focus_initial")));
    assert!(states.iter().any(|state| state.contains("portal_body_stack")));
    assert!(states.iter().any(|state| state.contains("timer_cleanup")));
    assert!(states.iter().any(|state| state.contains("live_status_visible")));
    assert!(states.iter().any(|state| state.contains("measurement_device_scale")));
    assert!(states.iter().any(|state| state.contains("pointer_capture_release")));
    assert!(states.iter().any(|state| state.contains("gesture_conservative")));
  }

  #[test]
  fn unsupported_fallbacks_are_explicit() {
    let focus_runtime = FocusRuntimeUnsupported;
    let portal_runtime = PortalRuntimeUnsupported;
    let timer_runtime = TimerRuntimeUnsupported;
    let pointer_runtime = PointerRuntimeUnsupported;
    let focus_node = ();

    assert_eq!(
      focus_runtime.focus_initial(&focus_node, FocusRuntimeRequest::dialog_default()),
      dioxus_shadcn_primitives::FocusCommandResult::Unsupported
    );
    assert_eq!(
      portal_runtime.mount_target(&PortalRuntimeRequest::inline(false)),
      dioxus_shadcn_primitives::PortalMountResult::Inline
    );
    assert_eq!(
      timer_runtime.schedule_once(&TimerRuntimeRequest::toast_dismiss(0)),
      dioxus_shadcn_primitives::TimerRuntimeResult::Disabled
    );
    assert_eq!(
      pointer_runtime.handle_pointer(&PointerRuntimeRequest::start()),
      dioxus_shadcn_primitives::PointerRuntimeResult::Unsupported
    );
  }
}
