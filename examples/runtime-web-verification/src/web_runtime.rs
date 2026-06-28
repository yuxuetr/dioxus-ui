use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use dioxus_ui_primitives::{
  DuplicateAnnouncementPolicy, LiveRegionRuntime, LiveRegionRuntimeRequest,
  LiveRegionRuntimeResult, TimerRuntime, TimerRuntimeRequest, TimerRuntimeResult,
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
