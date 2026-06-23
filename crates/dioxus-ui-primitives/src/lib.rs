//! Unstyled behavior primitives for dioxus-ui components.

pub mod active_descendant;
pub mod calendar;
#[cfg(feature = "dialog")]
pub mod dialog;
pub mod dismissal;
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
pub use dismissal::{DismissalDecision, DismissalEvent};
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
