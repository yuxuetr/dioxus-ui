//! Unstyled behavior primitives for dioxus-ui components.

#[cfg(feature = "dialog")]
pub mod dialog;

pub use dioxus_ui_core::UiDensity;

#[cfg(feature = "dialog")]
pub use dialog::{
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
};
