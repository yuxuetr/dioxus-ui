//! Unstyled behavior primitives for dioxus-shadcn components.

// Every public item says what it is for (RFC 0079).
#![deny(missing_docs)]

pub mod active_descendant;
pub mod calendar;
pub mod chart;
pub mod data_table;
#[cfg(feature = "dialog")]
pub mod dialog;
#[cfg(feature = "dropdown")]
pub mod dropdown;
pub mod feedback;
pub mod input_otp;
pub mod layout;
pub mod message_scroller;
pub mod overlay;
pub mod placement;
#[cfg(feature = "popover")]
pub mod popover;
pub mod roving_focus;
#[cfg(feature = "runtime")]
pub mod runtime;
#[cfg(feature = "select")]
pub mod select;
pub mod slider;
#[cfg(feature = "tooltip")]
pub mod tooltip;

pub use active_descendant::{
  ActiveDescendantContainerAttributes, ActiveDescendantItemAttributes, ActiveDescendantState,
};
pub use calendar::{
  CalendarDate, CalendarDay, CalendarKeyMove, CalendarMonth, CalendarMonthGrid, CalendarRangeState,
  CalendarWeekday, calendar_month_grid, calendar_move_date, calendar_range_state, days_in_month,
  is_leap_year,
};
pub use chart::{
  ChartColorToken, ChartDomain, ChartFallbackRow, ChartPoint, ChartScale, ChartSeries,
  chart_color_attribute, chart_color_class, chart_domain, chart_fallback_rows, chart_number_label,
  chart_scale_value, chart_series_label, chart_series_x_domain, chart_series_y_domain,
  chart_summary, chart_value_label,
};
pub use data_table::{
  DataTableColumnState, DataTablePaginationState, DataTableSelectionState, DataTableSortDirection,
  DataTableSortState, data_table_is_column_visible, data_table_page_count, data_table_page_window,
  data_table_toggle_all_rows, data_table_toggle_column, data_table_toggle_row,
  data_table_toggle_sort,
};
pub use dioxus_shadcn_core::UiDensity;
pub use feedback::{
  ToastDismissReason, ToastItem, ToastPlacement, ToastQueue, ToastVariant,
  toast_dismiss_reason_attribute, toast_is_expired, toast_placement_attribute, toast_queue_dismiss,
  toast_queue_limit, toast_queue_push, toast_variant_attribute,
};
pub use input_otp::{
  OtpSlotState, otp_apply_paste, otp_apply_paste_filtered, otp_clamp_value, otp_delete_char,
  otp_insert_char, otp_insert_char_filtered, otp_is_complete, otp_next_index, otp_previous_index,
  otp_slots, otp_slots_with_disabled,
};
pub use layout::{
  CarouselState, LayoutOrientation, ResizablePanelState, ScrollAreaOrientation, SidebarState,
  carousel_can_go_next, carousel_can_go_previous, carousel_clamp_index, carousel_next,
  carousel_previous, layout_orientation_attribute, resizable_clamp, resizable_resize_pair,
  scroll_area_orientation_attribute, sidebar_toggle,
};
pub use message_scroller::{
  MessageScrollerEvent, MessageScrollerIntent, MessageScrollerMetrics,
  message_scroller_distance_to_bottom, message_scroller_is_at_bottom, message_scroller_next_intent,
  message_scroller_should_follow, message_scroller_show_unread_marker,
};
pub use overlay::{
  DismissBehavior, FocusReturn, FocusStrategy, OverlayAlign, OverlaySide, PortalTarget,
};
pub use placement::{
  CollisionPadding, CollisionStrategy, OverlayOffset, OverlayPlacement, OverlayPlacementInput,
  OverlayRect, OverlaySize, compute_overlay_placement,
};
pub use roving_focus::{FocusMove, NavigationOrientation, RovingFocusItem, RovingFocusState};
#[cfg(feature = "runtime")]
pub use runtime::{
  AnnouncementPriority, DuplicateAnnouncementPolicy, FocusCommandResult, FocusRuntime,
  FocusRuntimeRequest, FocusRuntimeUnsupported, GestureAxis, GestureOutcome, GestureRuntime,
  GestureRuntimeRequest, GestureRuntimeResult, GestureRuntimeUnsupported, GestureState,
  LiveRegionRuntime, LiveRegionRuntimeRequest, LiveRegionRuntimeResult,
  LiveRegionRuntimeUnsupported, MeasurementRuntime, MeasurementRuntimeRequest,
  MeasurementRuntimeResult, MeasurementRuntimeUnsupported, PointerDelta, PointerPhase,
  PointerRuntime, PointerRuntimeRequest, PointerRuntimeResult, PointerRuntimeUnsupported,
  PortalMountResult, PortalRuntime, PortalRuntimeRequest, PortalRuntimeUnsupported, RuntimeRect,
  TimerReason, TimerRuntime, TimerRuntimeRequest, TimerRuntimeResult, TimerRuntimeUnsupported,
  carousel_apply_gesture,
};
pub use slider::{SliderAriaAttributes, SliderKeyMove, SliderState};

#[cfg(feature = "dialog")]
pub use dialog::DialogPrimitiveConfig;
#[cfg(feature = "dropdown")]
pub use dropdown::DropdownPrimitiveConfig;
#[cfg(feature = "popover")]
pub use popover::PopoverPrimitiveConfig;
#[cfg(feature = "select")]
pub use select::SelectPrimitiveConfig;
#[cfg(feature = "tooltip")]
pub use tooltip::TooltipPrimitiveConfig;
