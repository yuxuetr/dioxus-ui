//! Unstyled behavior primitives for dioxus-ui components.

#[cfg(feature = "dialog")]
pub mod dialog;
pub mod overlay;
#[cfg(feature = "popover")]
pub mod popover;
#[cfg(feature = "tooltip")]
pub mod tooltip;

pub use dioxus_ui_core::UiDensity;
pub use overlay::{
  DismissBehavior, FocusReturn, FocusStrategy, OverlayAlign, OverlaySide, PortalTarget,
};

#[cfg(feature = "dialog")]
pub use dialog::DialogPrimitiveConfig;
#[cfg(feature = "popover")]
pub use popover::PopoverPrimitiveConfig;
#[cfg(feature = "tooltip")]
pub use tooltip::TooltipPrimitiveConfig;
