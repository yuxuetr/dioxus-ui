//! Unstyled behavior primitives for dioxus-ui components.

pub mod active_descendant;
pub mod calendar;
pub mod chart;
pub mod data_table;
#[cfg(feature = "dialog")]
pub mod dialog;
pub mod dismissal;
pub mod feedback;
pub mod layout;
#[cfg(feature = "dropdown")]
pub mod dropdown;
pub mod overlay;
pub mod placement;
#[cfg(feature = "popover")]
pub mod popover;
pub mod roving_focus;
#[cfg(feature = "select")]
pub mod select;
pub mod slider;
#[cfg(feature = "tooltip")]
pub mod tooltip;
pub mod typeahead;

pub use active_descendant::{
  ActiveDescendantContainerAttributes, ActiveDescendantItemAttributes, ActiveDescendantState,
};
pub use calendar::{
  calendar_month_grid, calendar_move_date, calendar_range_state, days_in_month, is_leap_year,
  CalendarDate, CalendarDay, CalendarKeyMove, CalendarMonth, CalendarMonthGrid,
  CalendarRangeState, CalendarWeekday,
};
pub use chart::{
  chart_color_attribute, chart_color_class, chart_domain, chart_domain_normalize,
  chart_scale_value, chart_series_x_domain, chart_series_y_domain, ChartColorToken, ChartDomain,
  ChartPoint, ChartScale, ChartSeries,
};
pub use data_table::{
  data_table_clamp_page, data_table_is_column_visible, data_table_page_count,
  data_table_page_window, data_table_toggle_all_rows, data_table_toggle_column,
  data_table_toggle_row, data_table_toggle_sort, DataTableColumnState,
  DataTablePaginationState, DataTableSelectionState, DataTableSortDirection, DataTableSortState,
};
pub use dismissal::{DismissalDecision, DismissalEvent};
pub use feedback::{
  toast_dismiss_reason_attribute, toast_is_expired, toast_placement_attribute,
  toast_queue_dismiss, toast_queue_limit, toast_queue_push, toast_variant_attribute, ToastDismissReason,
  ToastItem, ToastPlacement, ToastQueue, ToastVariant,
};
pub use layout::{
  carousel_can_go_next, carousel_can_go_previous, carousel_clamp_index, carousel_next,
  carousel_previous, layout_orientation_attribute, resizable_clamp, resizable_resize_pair,
  scroll_area_orientation_attribute, sidebar_toggle, CarouselState, LayoutOrientation,
  ResizablePanelState, ScrollAreaOrientation, SidebarState,
};
pub use dioxus_ui_core::UiDensity;
pub use overlay::{
  DismissBehavior, FocusReturn, FocusStrategy, OverlayAlign, OverlaySide, PortalTarget,
};
pub use placement::{
  compute_overlay_placement, CollisionPadding, CollisionStrategy, OverlayOffset, OverlayPlacement,
  OverlayPlacementInput, OverlayRect, OverlaySize,
};
pub use roving_focus::{FocusMove, NavigationOrientation, RovingFocusItem, RovingFocusState};
pub use slider::{
  slider_clamp, slider_percent, slider_snap, SliderAriaAttributes, SliderKeyMove, SliderState,
};
pub use typeahead::{match_typeahead, TypeaheadItem, TypeaheadState};

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
