//! Unstyled behavior primitives for dioxus-ui components.

#[cfg(feature = "dialog")]
pub mod dialog;
#[cfg(feature = "dropdown")]
pub mod dropdown;
pub mod overlay;
#[cfg(feature = "popover")]
pub mod popover;
#[cfg(feature = "select")]
pub mod select;
#[cfg(feature = "tooltip")]
pub mod tooltip;

pub use dioxus_ui_core::UiDensity;
pub use overlay::{
  DismissBehavior, FocusReturn, FocusStrategy, OverlayAlign, OverlaySide, PortalTarget,
};

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
