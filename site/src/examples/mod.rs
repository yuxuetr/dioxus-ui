//! Live examples (RFC 0052). Each example is one file whose component the
//! component page renders and whose text, read with `include_str!`, the page
//! shows, so the shown source is the code that runs. The component is named
//! after the file, such as `CalendarMonthDemo` in `calendar_month.rs`, so
//! examples copied into one app keep distinct names.

use dioxus::prelude::*;

pub struct Example {
  pub slug: &'static str,
  pub title: &'static str,
  pub source: &'static str,
  pub render: fn() -> Element,
}

macro_rules! examples {
  ($($module:ident => $demo:ident, $slug:literal, $title:literal;)*) => {
    $(mod $module;)*

    pub const EXAMPLES: &[Example] = &[
      $(Example {
        slug: $slug,
        title: $title,
        source: include_str!(concat!(stringify!($module), ".rs")),
        render: $module::$demo,
      },)*
    ];

    #[cfg(test)]
    #[test]
    fn demos_are_named_after_their_files() {
      $(assert_eq!(demo_name(stringify!($module)), stringify!($demo));)*
    }
  };
}

examples! {
  button_variants => ButtonVariantsDemo, "button", "Variants";
  button_sizes => ButtonSizesDemo, "button", "Sizes and states";
  button_group_basic => ButtonGroupBasicDemo, "button-group", "Orientation";
  command_palette => CommandPaletteDemo, "command", "Command palette";
  kbd_shortcuts => KbdShortcutsDemo, "kbd", "Shortcuts";
  menu_docs => MenuDocsDemo, "menu", "Docs navigation";
  mockup_frames => MockupFramesDemo, "mockup", "Browser, code, and phone";
  theme_controller_picker => ThemeControllerPickerDemo, "theme-controller", "System, light, and dark";
  toggle_basic => ToggleBasicDemo, "toggle", "Variants";
  toggle_group_single => ToggleGroupSingleDemo, "toggle-group", "Single selection";
  calendar_month => CalendarMonthDemo, "calendar", "Month";
  checkbox_basic => CheckboxBasicDemo, "checkbox", "States";
  date_picker_basic => DatePickerBasicDemo, "date-picker", "Date picker";
  date_picker_input => DatePickerInputDemo, "date-picker", "Typed date";
  field_basic => FieldBasicDemo, "field", "Description and error";
  input_states => InputStatesDemo, "input", "States";
  input_group_addons => InputGroupAddonsDemo, "input-group", "Addons";
  input_otp_basic => InputOtpBasicDemo, "input-otp", "Six digits";
  label_basic => LabelBasicDemo, "label", "Labels";
  native_select_basic => NativeSelectBasicDemo, "native-select", "Groups";
  radio_group_basic => RadioGroupBasicDemo, "radio-group", "Plan picker";
  select_basic => SelectBasicDemo, "select", "Select";
  select_multiple => SelectMultipleDemo, "select", "Multiple";
  slider_basic => SliderBasicDemo, "slider", "Orientation and states";
  slider_range => SliderRangeDemo, "slider", "Price range";
  switch_basic => SwitchBasicDemo, "switch", "States";
  textarea_basic => TextareaBasicDemo, "textarea", "Character count";
  alert_dialog_confirm => AlertDialogConfirmDemo, "alert-dialog", "Confirm deletion";
  combobox_search => ComboboxSearchDemo, "combobox", "Search";
  context_menu_basic => ContextMenuBasicDemo, "context-menu", "Right-click menu";
  dialog_form => DialogFormDemo, "dialog", "Form in a dialog";
  drawer_basic => DrawerBasicDemo, "drawer", "Bottom drawer";
  dropdown_actions => DropdownActionsDemo, "dropdown", "Actions menu";
  dropdown_options => DropdownOptionsDemo, "dropdown", "View options";
  hover_card_profile => HoverCardProfileDemo, "hover-card", "Profile card";
  menubar_editor => MenubarEditorDemo, "menubar", "Editor menus";
  popover_basic => PopoverBasicDemo, "popover", "Form in a popover";
  sheet_side => SheetSideDemo, "sheet", "Sides";
  tooltip_basic => TooltipBasicDemo, "tooltip", "Tooltip";
  breadcrumb_basic => BreadcrumbBasicDemo, "breadcrumb", "Collapsed path";
  navigation_menu_basic => NavigationMenuBasicDemo, "navigation-menu", "Product menu";
  navigation_menu_mega => NavigationMenuMegaDemo, "navigation-menu", "Mega menu";
  pagination_basic => PaginationBasicDemo, "pagination", "Pages";
  sidebar_collapsible => SidebarCollapsibleDemo, "sidebar", "Collapsible";
  tabs_account => TabsAccountDemo, "tabs", "Account settings";
  accordion_faq => AccordionFaqDemo, "accordion", "FAQ";
  aspect_ratio_basic => AspectRatioBasicDemo, "aspect-ratio", "Ratios";
  card_basic => CardBasicDemo, "card", "Form card";
  carousel_basic => CarouselBasicDemo, "carousel", "Slides";
  collapsible_basic => CollapsibleBasicDemo, "collapsible", "Show more";
  direction_rtl => DirectionRtlDemo, "direction", "Right to left";
  item_list => ItemListDemo, "item", "People";
  resizable_panels => ResizablePanelsDemo, "resizable", "Two panels";
  scroll_area_tags => ScrollAreaTagsDemo, "scroll-area", "Tags";
  separator_basic => SeparatorBasicDemo, "separator", "Orientation";
  avatar_basic => AvatarBasicDemo, "avatar", "Image and fallback";
  badge_variants => BadgeVariantsDemo, "badge", "Variants";
  badge_status => BadgeStatusDemo, "badge", "Status";
  stat_revenue => StatRevenueDemo, "stat", "Revenue";
  timeline_release => TimelineReleaseDemo, "timeline", "Release history";
  steps_checkout => StepsCheckoutDemo, "steps", "Checkout";
  indicator_counts => IndicatorCountsDemo, "indicator", "Counts and presence";
  status_presence => StatusPresenceDemo, "status", "Service health";
  radial_progress_usage => RadialProgressUsageDemo, "radial-progress", "Sizes and labels";
  countdown_sale => CountdownSaleDemo, "countdown", "Sale timer";
  diff_compare => DiffCompareDemo, "diff", "Design comparison";
  rating_review => RatingReviewDemo, "rating", "Review";
  number_input_cart => NumberInputCartDemo, "number-input", "Quantity and weight";
  tags_input_topics => TagsInputTopicsDemo, "tags-input", "Topics";
  file_input_upload => FileInputUploadDemo, "file-input", "Documents";
  swap_icons => SwapIconsDemo, "swap", "Icons and text";
  dock_phone => DockPhoneDemo, "dock", "Phone tabs";
  fab_speed_dial => FabSpeedDialDemo, "fab", "Speed dial";
  chart_revenue => ChartRevenueDemo, "chart", "Area and line";
  chart_traffic => ChartTrafficDemo, "chart", "Donut";
  data_table_users => DataTableUsersDemo, "data-table", "Filter, sort, and select";
  empty_projects => EmptyProjectsDemo, "empty", "No projects";
  progress_upload => ProgressUploadDemo, "progress", "Upload";
  table_invoices => TableInvoicesDemo, "table", "Invoices";
  typography_article => TypographyArticleDemo, "typography", "Article";
  alert_variants => AlertVariantsDemo, "alert", "Variants";
  alert_status => AlertStatusDemo, "alert", "Status";
  skeleton_card => SkeletonCardDemo, "skeleton", "Loading profile";
  sonner_variants => SonnerVariantsDemo, "sonner", "Variants";
  spinner_sizes => SpinnerSizesDemo, "spinner", "Sizes";
  toast_undo => ToastUndoDemo, "toast", "Undo action";
  attachment_states => AttachmentStatesDemo, "attachment", "Upload states";
  bubble_chat => BubbleChatDemo, "bubble", "Variants";
  marker_variants => MarkerVariantsDemo, "marker", "Variants";
  message_thread => MessageThreadDemo, "message", "Thread";
  message_scroller_chat => MessageScrollerChatDemo, "message-scroller", "Follow new messages";
}

#[cfg(test)]
fn demo_name(module: &str) -> String {
  let mut name: String = module
    .split('_')
    .map(|word| {
      let mut chars = word.chars();
      chars
        .next()
        .map(|first| first.to_ascii_uppercase().to_string() + chars.as_str())
        .unwrap_or_default()
    })
    .collect();
  name.push_str("Demo");
  name
}
