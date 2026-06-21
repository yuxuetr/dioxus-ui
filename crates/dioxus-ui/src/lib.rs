//! Styled Dioxus UI components.

#[cfg(feature = "button")]
pub mod button;

#[cfg(feature = "checkbox")]
pub mod checkbox;

#[cfg(feature = "input")]
pub mod input;

#[cfg(feature = "label")]
pub mod label;

#[cfg(feature = "textarea")]
pub mod textarea;

#[cfg(feature = "button")]
pub use button::{button_class, Button, ButtonSize, ButtonVariant, BUTTON_BASE_CLASS};
#[cfg(feature = "checkbox")]
pub use checkbox::{checkbox_class, Checkbox, CHECKBOX_BASE_CLASS};
#[cfg(feature = "input")]
pub use input::{input_class, Input, INPUT_BASE_CLASS};
#[cfg(feature = "label")]
pub use label::{label_class, Label, LABEL_BASE_CLASS};
#[cfg(feature = "textarea")]
pub use textarea::{textarea_class, Textarea, TEXTAREA_BASE_CLASS};
#[cfg(feature = "switch")]
pub mod switch;
#[cfg(feature = "switch")]
pub use switch::{switch_class, switch_thumb_class, Switch, SWITCH_BASE_CLASS, SWITCH_THUMB_BASE_CLASS};
pub use dioxus_ui_core::UiDensity;
