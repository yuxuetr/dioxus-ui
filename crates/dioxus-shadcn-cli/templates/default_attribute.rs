use dioxus::prelude::*;

/// Keeps a component's default value for attribute `name` only when the app
/// passed none. The browser applies the later spread value anyway, but SSR
/// writes both attributes and an HTML parser keeps the first.
pub(crate) fn default_attribute<T>(attributes: &[Attribute], name: &str, value: T) -> Option<T> {
  (!attributes.iter().any(|attribute| attribute.name == name)).then_some(value)
}
