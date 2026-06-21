//! Shared foundation types and utilities for dioxus-ui.

mod class;

use serde::{Deserialize, Serialize};

pub use class::classes;

/// Density controls spacing and hit target size across platforms.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UiDensity {
  /// Dense controls for data-heavy desktop-style interfaces.
  Compact,

  /// Default controls for general web applications.
  #[default]
  Comfortable,

  /// Larger controls for touch-first interfaces.
  Touch,
}

/// A component entry in the local dioxus-ui registry.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegistryComponent {
  /// Stable component name used by `dxui add`.
  pub name: String,

  /// Human-readable component description.
  #[serde(default)]
  pub description: String,

  /// Template files copied for this component.
  #[serde(default)]
  pub files: Vec<RegistryFile>,

  /// Other registry components required by this component.
  #[serde(default)]
  pub dependencies: Vec<String>,

  /// Asset files copied for this component.
  #[serde(default)]
  pub assets: Vec<RegistryAsset>,
}

/// A template file mapping from repository source to user-project target.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegistryFile {
  /// Source template path relative to the workspace root.
  pub source: String,

  /// Target path relative to the user project root.
  pub target: String,
}

/// An asset mapping from repository source to user-project target.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegistryAsset {
  /// Source asset path relative to the workspace root.
  pub source: String,

  /// Target path relative to the user project root.
  pub target: String,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default_density_is_comfortable() {
    assert_eq!(UiDensity::default(), UiDensity::Comfortable);
  }
}
