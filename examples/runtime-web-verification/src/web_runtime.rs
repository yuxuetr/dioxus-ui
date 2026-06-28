use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use dioxus_ui_primitives::{
  DuplicateAnnouncementPolicy, FocusCommandResult, FocusRuntime, FocusRuntimeRequest,
  LiveRegionRuntime, LiveRegionRuntimeRequest, LiveRegionRuntimeResult, MeasurementRuntime,
  MeasurementRuntimeRequest, MeasurementRuntimeResult, PortalMountResult, PortalRuntime,
  PortalRuntimeRequest, RuntimeRect, TimerRuntime, TimerRuntimeRequest, TimerRuntimeResult,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebTimerId(u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebTimerRecord {
  pub id: WebTimerId,
  pub delay_ms: u64,
  pub cancelled: bool,
}

#[derive(Debug, Default)]
pub struct WebTimerRuntime {
  next_id: AtomicU64,
  records: Mutex<Vec<WebTimerRecord>>,
}

impl WebTimerRuntime {
  pub fn new() -> Self {
    Self { next_id: AtomicU64::new(1), records: Mutex::new(Vec::new()) }
  }

  #[cfg(test)]
  pub fn records(&self) -> Vec<WebTimerRecord> {
    match self.records.lock() {
      Ok(records) => records.clone(),
      Err(_) => Vec::new(),
    }
  }
}

impl TimerRuntime for WebTimerRuntime {
  type TimerId = WebTimerId;

  fn schedule_once(&self, request: &TimerRuntimeRequest) -> TimerRuntimeResult<Self::TimerId> {
    if !request.is_enabled() {
      return TimerRuntimeResult::Disabled;
    }

    let id = WebTimerId(self.next_id.fetch_add(1, Ordering::Relaxed));
    let record = WebTimerRecord { id, delay_ms: request.delay_ms, cancelled: false };

    match self.records.lock() {
      Ok(mut records) => {
        records.push(record);
        TimerRuntimeResult::Scheduled(id)
      }
      Err(_) => TimerRuntimeResult::Unsupported,
    }
  }

  fn cancel(&self, id: &Self::TimerId) -> TimerRuntimeResult<Self::TimerId> {
    match self.records.lock() {
      Ok(mut records) => {
        if let Some(record) = records.iter_mut().find(|record| record.id == *id) {
          record.cancelled = true;
          TimerRuntimeResult::Cancelled
        } else {
          TimerRuntimeResult::Missing
        }
      }
      Err(_) => TimerRuntimeResult::Unsupported,
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebFocusNode {
  pub id: String,
  pub mounted: bool,
}

impl WebFocusNode {
  pub fn new(id: impl Into<String>) -> Self {
    Self { id: id.into(), mounted: true }
  }

  pub fn missing(id: impl Into<String>) -> Self {
    Self { id: id.into(), mounted: false }
  }

  fn is_available(&self) -> bool {
    self.mounted && !self.id.trim().is_empty()
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WebFocusCommand {
  Initial,
  Trap,
  Restore,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebFocusRecord {
  pub node_id: String,
  pub command: WebFocusCommand,
  pub request: FocusRuntimeRequest,
}

#[derive(Debug, Default)]
pub struct WebFocusRuntime {
  records: Mutex<Vec<WebFocusRecord>>,
}

impl WebFocusRuntime {
  pub fn new() -> Self {
    Self { records: Mutex::new(Vec::new()) }
  }

  #[cfg(test)]
  pub fn records(&self) -> Vec<WebFocusRecord> {
    match self.records.lock() {
      Ok(records) => records.clone(),
      Err(_) => Vec::new(),
    }
  }

  fn apply_command(
    &self,
    node: &WebFocusNode,
    request: FocusRuntimeRequest,
    command: WebFocusCommand,
  ) -> FocusCommandResult {
    if !node.is_available() {
      return FocusCommandResult::MissingTarget;
    }

    let record = WebFocusRecord { node_id: node.id.clone(), command, request };

    match self.records.lock() {
      Ok(mut records) => {
        records.push(record);
        FocusCommandResult::Applied
      }
      Err(_) => FocusCommandResult::Unsupported,
    }
  }
}

impl FocusRuntime for WebFocusRuntime {
  type NodeId = WebFocusNode;

  fn focus_initial(
    &self,
    scope: &Self::NodeId,
    request: FocusRuntimeRequest,
  ) -> FocusCommandResult {
    self.apply_command(scope, request, WebFocusCommand::Initial)
  }

  fn trap_focus(&self, scope: &Self::NodeId, request: FocusRuntimeRequest) -> FocusCommandResult {
    self.apply_command(scope, request, WebFocusCommand::Trap)
  }

  fn restore_focus(
    &self,
    target: &Self::NodeId,
    request: FocusRuntimeRequest,
  ) -> FocusCommandResult {
    self.apply_command(target, request, WebFocusCommand::Restore)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebPortalMountId(u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebPortalRecord {
  pub id: WebPortalMountId,
  pub target: String,
  pub modal: bool,
}

#[derive(Debug, Default)]
pub struct WebPortalRuntime {
  next_id: AtomicU64,
  available_selectors: Vec<String>,
  records: Mutex<Vec<WebPortalRecord>>,
}

impl WebPortalRuntime {
  #[cfg(test)]
  pub fn new() -> Self {
    Self {
      next_id: AtomicU64::new(1),
      available_selectors: Vec::new(),
      records: Mutex::new(Vec::new()),
    }
  }

  pub fn with_selectors(selectors: impl IntoIterator<Item = impl Into<String>>) -> Self {
    Self {
      next_id: AtomicU64::new(1),
      available_selectors: selectors.into_iter().map(Into::into).collect(),
      records: Mutex::new(Vec::new()),
    }
  }

  #[cfg(test)]
  pub fn records(&self) -> Vec<WebPortalRecord> {
    match self.records.lock() {
      Ok(records) => records.clone(),
      Err(_) => Vec::new(),
    }
  }

  fn mount(&self, target: String, modal: bool) -> PortalMountResult<WebPortalMountId> {
    let id = WebPortalMountId(self.next_id.fetch_add(1, Ordering::Relaxed));
    let record = WebPortalRecord { id, target, modal };

    match self.records.lock() {
      Ok(mut records) => {
        records.push(record);
        PortalMountResult::Mounted(id)
      }
      Err(_) => PortalMountResult::Unsupported,
    }
  }
}

impl PortalRuntime for WebPortalRuntime {
  type MountId = WebPortalMountId;

  fn mount_target(&self, request: &PortalRuntimeRequest) -> PortalMountResult<Self::MountId> {
    match &request.target {
      dioxus_ui_primitives::PortalTarget::Inline => PortalMountResult::Inline,
      dioxus_ui_primitives::PortalTarget::Body => self.mount("body".to_string(), request.modal),
      dioxus_ui_primitives::PortalTarget::Selector(selector) => {
        if self.available_selectors.iter().any(|available_selector| available_selector == selector)
        {
          self.mount(selector.clone(), request.modal)
        } else {
          PortalMountResult::MissingTarget
        }
      }
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebMeasurementNode {
  pub id: String,
  pub mounted: bool,
}

impl WebMeasurementNode {
  pub fn new(id: impl Into<String>) -> Self {
    Self { id: id.into(), mounted: true }
  }

  pub fn missing(id: impl Into<String>) -> Self {
    Self { id: id.into(), mounted: false }
  }

  fn is_available(&self) -> bool {
    self.mounted && !self.id.trim().is_empty()
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WebMeasurementRecord {
  pub node_id: String,
  pub rect: RuntimeRect,
}

#[derive(Debug)]
pub struct WebMeasurementRuntime {
  viewport: Mutex<RuntimeRect>,
  records: Mutex<Vec<WebMeasurementRecord>>,
}

impl WebMeasurementRuntime {
  pub fn with_rects(
    viewport: RuntimeRect,
    records: impl IntoIterator<Item = (impl Into<String>, RuntimeRect)>,
  ) -> Self {
    Self {
      viewport: Mutex::new(viewport),
      records: Mutex::new(
        records
          .into_iter()
          .map(|(node_id, rect)| WebMeasurementRecord { node_id: node_id.into(), rect })
          .collect(),
      ),
    }
  }

  pub fn update_viewport(&self, rect: RuntimeRect) -> MeasurementRuntimeResult {
    if rect.is_empty() {
      return MeasurementRuntimeResult::Missing;
    }

    match self.viewport.lock() {
      Ok(mut viewport) => {
        *viewport = rect;
        MeasurementRuntimeResult::Rect(rect)
      }
      Err(_) => MeasurementRuntimeResult::Unsupported,
    }
  }

  #[cfg(test)]
  pub fn records(&self) -> Vec<WebMeasurementRecord> {
    match self.records.lock() {
      Ok(records) => records.clone(),
      Err(_) => Vec::new(),
    }
  }
}

impl MeasurementRuntime for WebMeasurementRuntime {
  type NodeId = WebMeasurementNode;

  fn measure(&self, request: &MeasurementRuntimeRequest<Self::NodeId>) -> MeasurementRuntimeResult {
    match request {
      MeasurementRuntimeRequest::Viewport => match self.viewport.lock() {
        Ok(viewport) => MeasurementRuntimeResult::Rect(*viewport),
        Err(_) => MeasurementRuntimeResult::Unsupported,
      },
      MeasurementRuntimeRequest::Node(node) => {
        if !node.is_available() {
          return MeasurementRuntimeResult::Missing;
        }

        match self.records.lock() {
          Ok(records) => records
            .iter()
            .find(|record| record.node_id == node.id)
            .map(|record| MeasurementRuntimeResult::Rect(record.rect))
            .unwrap_or(MeasurementRuntimeResult::Missing),
          Err(_) => MeasurementRuntimeResult::Unsupported,
        }
      }
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebLiveRegionAnnouncement {
  pub message: String,
}

#[derive(Debug, Default)]
pub struct WebLiveRegionRuntime {
  announcements: Mutex<Vec<WebLiveRegionAnnouncement>>,
  last_message: Mutex<Option<String>>,
}

impl WebLiveRegionRuntime {
  pub fn new() -> Self {
    Self { announcements: Mutex::new(Vec::new()), last_message: Mutex::new(None) }
  }

  #[cfg(test)]
  pub fn announcements(&self) -> Vec<WebLiveRegionAnnouncement> {
    match self.announcements.lock() {
      Ok(announcements) => announcements.clone(),
      Err(_) => Vec::new(),
    }
  }
}

impl LiveRegionRuntime for WebLiveRegionRuntime {
  fn announce(&self, request: &LiveRegionRuntimeRequest) -> LiveRegionRuntimeResult {
    if request.is_empty() {
      return LiveRegionRuntimeResult::EmptyMessage;
    }

    let message = request.message.trim().to_string();

    match self.last_message.lock() {
      Ok(mut last_message) => {
        if matches!(request.duplicate_policy, DuplicateAnnouncementPolicy::SuppressConsecutive)
          && last_message.as_deref() == Some(message.as_str())
        {
          return LiveRegionRuntimeResult::SuppressedDuplicate;
        }

        *last_message = Some(message.clone());
      }
      Err(_) => return LiveRegionRuntimeResult::Unsupported,
    }

    match self.announcements.lock() {
      Ok(mut announcements) => {
        announcements.push(WebLiveRegionAnnouncement { message });
        LiveRegionRuntimeResult::Queued
      }
      Err(_) => LiveRegionRuntimeResult::Unsupported,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn web_timer_schedules_and_cancels() {
    let runtime = WebTimerRuntime::new();
    let scheduled = runtime.schedule_once(&TimerRuntimeRequest::toast_dismiss(3000));

    let timer_id = match scheduled {
      TimerRuntimeResult::Scheduled(timer_id) => timer_id,
      other => panic!("expected scheduled timer, got {other:?}"),
    };

    assert_eq!(runtime.cancel(&timer_id), TimerRuntimeResult::Cancelled);
    assert_eq!(runtime.records()[0].cancelled, true);
  }

  #[test]
  fn web_timer_reports_disabled_and_missing() {
    let runtime = WebTimerRuntime::new();

    assert_eq!(
      runtime.schedule_once(&TimerRuntimeRequest::toast_dismiss(0)),
      TimerRuntimeResult::Disabled
    );
    assert_eq!(runtime.cancel(&WebTimerId(99)), TimerRuntimeResult::Missing);
  }

  #[test]
  fn web_focus_applies_commands_and_records_requests() {
    let runtime = WebFocusRuntime::new();
    let node = WebFocusNode::new("dialog-content");
    let request = FocusRuntimeRequest::dialog_default();

    assert_eq!(runtime.focus_initial(&node, request), FocusCommandResult::Applied);
    assert_eq!(runtime.trap_focus(&node, request), FocusCommandResult::Applied);
    assert_eq!(runtime.restore_focus(&node, request), FocusCommandResult::Applied);
    assert_eq!(runtime.records().len(), 3);
  }

  #[test]
  fn web_focus_reports_missing_targets() {
    let runtime = WebFocusRuntime::new();
    let node = WebFocusNode::missing("missing-dialog-content");

    assert_eq!(
      runtime.focus_initial(&node, FocusRuntimeRequest::dialog_default()),
      FocusCommandResult::MissingTarget
    );
  }

  #[test]
  fn web_portal_mounts_body_and_known_selectors() {
    let runtime = WebPortalRuntime::with_selectors(["#runtime-overlay-root"]);

    assert!(matches!(
      runtime.mount_target(&PortalRuntimeRequest::body(true)),
      PortalMountResult::Mounted(_)
    ));
    assert!(matches!(
      runtime.mount_target(&PortalRuntimeRequest::selector("#runtime-overlay-root", false)),
      PortalMountResult::Mounted(_)
    ));
    assert_eq!(runtime.records().len(), 2);
  }

  #[test]
  fn web_portal_reports_inline_and_missing_targets() {
    let runtime = WebPortalRuntime::new();

    assert_eq!(
      runtime.mount_target(&PortalRuntimeRequest::inline(false)),
      PortalMountResult::Inline
    );
    assert_eq!(
      runtime.mount_target(&PortalRuntimeRequest::selector("#missing", false)),
      PortalMountResult::MissingTarget
    );
  }

  #[test]
  fn web_measurement_reports_node_and_viewport_rects() {
    let runtime = WebMeasurementRuntime::with_rects(
      RuntimeRect::new(0.0, 0.0, 1280.0, 720.0),
      [
        ("trigger", RuntimeRect::new(24.0, 48.0, 160.0, 32.0)),
        ("content", RuntimeRect::new(24.0, 88.0, 320.0, 240.0)),
      ],
    );

    assert_eq!(runtime.records().len(), 2);
    assert_eq!(
      runtime.measure(&MeasurementRuntimeRequest::Node(WebMeasurementNode::new("trigger"))),
      MeasurementRuntimeResult::Rect(RuntimeRect::new(24.0, 48.0, 160.0, 32.0))
    );
    assert_eq!(
      runtime.measure(&MeasurementRuntimeRequest::Viewport),
      MeasurementRuntimeResult::Rect(RuntimeRect::new(0.0, 0.0, 1280.0, 720.0))
    );
  }

  #[test]
  fn web_measurement_reports_missing_and_updated_viewport() {
    let runtime = WebMeasurementRuntime::with_rects(
      RuntimeRect::new(0.0, 0.0, 1280.0, 720.0),
      [("trigger", RuntimeRect::new(24.0, 48.0, 160.0, 32.0))],
    );

    assert_eq!(
      runtime.measure(&MeasurementRuntimeRequest::Node(WebMeasurementNode::missing("missing"))),
      MeasurementRuntimeResult::Missing
    );
    assert_eq!(
      runtime.update_viewport(RuntimeRect::new(0.0, 0.0, 1024.0, 640.0)),
      MeasurementRuntimeResult::Rect(RuntimeRect::new(0.0, 0.0, 1024.0, 640.0))
    );
  }

  #[test]
  fn web_live_region_queues_and_suppresses_duplicates() {
    let runtime = WebLiveRegionRuntime::new();
    let request = LiveRegionRuntimeRequest::polite("Saved");

    assert_eq!(runtime.announce(&request), LiveRegionRuntimeResult::Queued);
    assert_eq!(runtime.announce(&request), LiveRegionRuntimeResult::SuppressedDuplicate);
    assert_eq!(runtime.announcements().len(), 1);
  }

  #[test]
  fn web_live_region_reports_empty_messages() {
    let runtime = WebLiveRegionRuntime::new();

    assert_eq!(
      runtime.announce(&LiveRegionRuntimeRequest::polite(" ")),
      LiveRegionRuntimeResult::EmptyMessage
    );
  }
}
