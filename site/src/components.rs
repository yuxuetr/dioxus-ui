//! The `components::ui` modules that blocks import (RFC 0073). An app gets
//! them from `dxui add`; the site points each at the crate, which exports
//! the same items, so it compiles the block sources unchanged.

pub mod ui {
  pub mod attachment {
    pub use dioxus_shadcn::*;
  }
  pub mod avatar {
    pub use dioxus_shadcn::*;
  }
  pub mod badge {
    pub use dioxus_shadcn::*;
  }
  pub mod bubble {
    pub use dioxus_shadcn::*;
  }
  pub mod button {
    pub use dioxus_shadcn::*;
  }
  pub mod card {
    pub use dioxus_shadcn::*;
  }
  pub mod chart {
    pub use dioxus_shadcn::*;
  }
  pub mod checkbox {
    pub use dioxus_shadcn::*;
  }
  pub mod data_table {
    pub use dioxus_shadcn::*;
  }
  pub mod field {
    pub use dioxus_shadcn::*;
  }
  pub mod input {
    pub use dioxus_shadcn::*;
  }
  pub mod item {
    pub use dioxus_shadcn::*;
  }
  pub mod label {
    pub use dioxus_shadcn::*;
  }
  pub mod message {
    pub use dioxus_shadcn::*;
  }
  pub mod message_scroller {
    pub use dioxus_shadcn::*;
  }
  pub mod native_select {
    pub use dioxus_shadcn::*;
  }
  pub mod progress {
    pub use dioxus_shadcn::*;
  }
  pub mod resizable {
    pub use dioxus_shadcn::*;
  }
  pub mod scroll_area {
    pub use dioxus_shadcn::*;
  }
  pub mod separator {
    pub use dioxus_shadcn::*;
  }
  pub mod sidebar {
    pub use dioxus_shadcn::*;
  }
  pub mod stat {
    pub use dioxus_shadcn::*;
  }
  pub mod switch {
    pub use dioxus_shadcn::*;
  }
  pub mod tabs {
    pub use dioxus_shadcn::*;
  }
  pub mod textarea {
    pub use dioxus_shadcn::*;
  }
  pub mod toggle_group {
    pub use dioxus_shadcn::*;
  }
}
