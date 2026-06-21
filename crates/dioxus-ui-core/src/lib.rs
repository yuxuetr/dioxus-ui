//! Shared foundation types and utilities for dioxus-ui.

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default_density_is_comfortable() {
    assert_eq!(UiDensity::default(), UiDensity::Comfortable);
  }
}
