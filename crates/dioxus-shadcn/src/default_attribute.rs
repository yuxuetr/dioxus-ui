use dioxus::prelude::*;

/// Keeps a component's default value for attribute `name` only when the app
/// passed none. The browser applies the later spread value anyway, but SSR
/// writes both attributes and an HTML parser keeps the first.
pub(crate) fn default_attribute<T>(attributes: &[Attribute], name: &str, value: T) -> Option<T> {
  (!attributes.iter().any(|attribute| attribute.name == name)).then_some(value)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_passed_attribute_replaces_the_default() {
    let passed = vec![Attribute::new("aria-label", "Show product 1", None, false)];

    assert_eq!(default_attribute(&passed, "aria-label", "Go to slide"), None);
    assert_eq!(default_attribute(&passed, "tabindex", "0"), Some("0"));
    assert_eq!(default_attribute(&[], "aria-label", "Go to slide"), Some("Go to slide"));
  }
}
