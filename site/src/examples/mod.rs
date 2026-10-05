//! Live examples (RFC 0052). Each example is one file whose `Demo` component
//! the component page renders and whose text, read with `include_str!`, the
//! page shows, so the shown source is the code that runs.

use dioxus::prelude::*;

pub struct Example {
  pub slug: &'static str,
  pub title: &'static str,
  pub source: &'static str,
  pub render: fn() -> Element,
}

macro_rules! examples {
  ($($module:ident => $slug:literal, $title:literal;)*) => {
    $(mod $module;)*

    pub const EXAMPLES: &[Example] = &[
      $(Example {
        slug: $slug,
        title: $title,
        source: include_str!(concat!(stringify!($module), ".rs")),
        render: $module::Demo,
      },)*
    ];
  };
}

examples! {
  button_variants => "button", "Variants";
}
