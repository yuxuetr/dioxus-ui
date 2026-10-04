use dioxus::prelude::*;

/// Keeps a component's default `aria-label` only when the app passed none.
/// The browser applies the later spread value anyway, but SSR writes both
/// attributes and an HTML parser keeps the first.
pub(crate) fn default_aria_label(
  attributes: &[Attribute],
  label: &'static str,
) -> Option<&'static str> {
  (!attributes.iter().any(|attribute| attribute.name == "aria-label")).then_some(label)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_passed_aria_label_replaces_the_default() {
    let passed = vec![Attribute::new("aria-label", "Show product 1", None, false)];

    assert_eq!(default_aria_label(&passed, "Go to slide"), None);
    assert_eq!(default_aria_label(&[], "Go to slide"), Some("Go to slide"));
  }
}
