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
  button_sizes => "button", "Sizes and states";
  button_group_basic => "button-group", "Orientation";
  command_palette => "command", "Command palette";
  kbd_shortcuts => "kbd", "Shortcuts";
  toggle_basic => "toggle", "Variants";
  toggle_group_single => "toggle-group", "Single selection";
  calendar_month => "calendar", "Month";
  checkbox_basic => "checkbox", "States";
  date_picker_basic => "date-picker", "Date picker";
  field_basic => "field", "Description and error";
  input_states => "input", "States";
  input_group_addons => "input-group", "Addons";
  input_otp_basic => "input-otp", "Six digits";
  label_basic => "label", "Labels";
  native_select_basic => "native-select", "Groups";
  radio_group_basic => "radio-group", "Plan picker";
  select_basic => "select", "Select";
  slider_basic => "slider", "Orientation and states";
  switch_basic => "switch", "States";
  textarea_basic => "textarea", "Character count";
}
