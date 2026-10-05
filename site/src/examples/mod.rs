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
  alert_dialog_confirm => "alert-dialog", "Confirm deletion";
  combobox_search => "combobox", "Search";
  context_menu_basic => "context-menu", "Right-click menu";
  dialog_form => "dialog", "Form in a dialog";
  drawer_basic => "drawer", "Bottom drawer";
  dropdown_actions => "dropdown", "Actions menu";
  hover_card_profile => "hover-card", "Profile card";
  menubar_editor => "menubar", "Editor menus";
  popover_basic => "popover", "Form in a popover";
  sheet_side => "sheet", "Sides";
  tooltip_basic => "tooltip", "Tooltip";
  breadcrumb_basic => "breadcrumb", "Collapsed path";
  navigation_menu_basic => "navigation-menu", "Product menu";
  pagination_basic => "pagination", "Pages";
  sidebar_collapsible => "sidebar", "Collapsible";
  tabs_account => "tabs", "Account settings";
  accordion_faq => "accordion", "FAQ";
  aspect_ratio_basic => "aspect-ratio", "Ratios";
  card_basic => "card", "Form card";
  carousel_basic => "carousel", "Slides";
  collapsible_basic => "collapsible", "Show more";
  direction_rtl => "direction", "Right to left";
  item_list => "item", "People";
  resizable_panels => "resizable", "Two panels";
  scroll_area_tags => "scroll-area", "Tags";
  separator_basic => "separator", "Orientation";
  avatar_basic => "avatar", "Image and fallback";
  badge_variants => "badge", "Variants";
  chart_revenue => "chart", "Area and line";
  data_table_users => "data-table", "Filter, sort, and select";
  empty_projects => "empty", "No projects";
  progress_upload => "progress", "Upload";
  table_invoices => "table", "Invoices";
  typography_article => "typography", "Article";
  alert_variants => "alert", "Variants";
  skeleton_card => "skeleton", "Loading profile";
  sonner_variants => "sonner", "Variants";
  spinner_sizes => "spinner", "Sizes";
  toast_undo => "toast", "Undo action";
  attachment_states => "attachment", "Upload states";
  bubble_chat => "bubble", "Variants";
  marker_variants => "marker", "Variants";
  message_thread => "message", "Thread";
  message_scroller_chat => "message-scroller", "Follow new messages";
}
