use dioxus_ui::{
  accordion_content_class, accordion_item_class, accordion_trigger_class,
  alert_dialog_action_class, alert_dialog_content_class, alert_dialog_overlay_class, alert_class,
  alert_description_class, alert_title_class, avatar_class, avatar_fallback_class,
  avatar_image_class, badge_class, button_class, calendar_day_class, calendar_month_grid,
  calendar_move_date, card_class, card_content_class, card_description_class, card_footer_class,
  card_header_class, card_title_class, checkbox_class, command_active_descendant_state,
  command_class, command_input_class, command_item_class, combobox_input_class,
  combobox_item_class, combobox_trigger_class, context_menu_content_class,
  context_menu_item_class, context_menu_shortcut_class, data_table_header_cell_class,
  data_table_page_window, data_table_row_class, data_table_sort_attribute,
  data_table_toggle_row, date_picker_align_attribute, date_picker_content_class,
  date_picker_side_attribute, date_picker_trigger_class, date_picker_value_class,
  dialog_content_class, dialog_overlay_class, drawer_content_class,
  drawer_overlay_class, dropdown_content_class, dropdown_item_class, dropdown_label_class,
  dropdown_separator_class, hover_card_align_attribute, hover_card_content_class,
  hover_card_side_attribute, input_class, label_class, menubar_class, menubar_item_class,
  menubar_trigger_class, native_select_class, native_select_group_class,
  native_select_option_class, navigation_menu_class, navigation_menu_link_class,
  navigation_menu_trigger_class, pagination_class, pagination_link_class,
  popover_content_class, popover_description_class, popover_header_class, popover_title_class,
  progress_class, progress_indicator_class, progress_percent, radio_group_class,
  radio_group_item_class, radio_group_move_value, resizable_handle_class,
  resizable_panel_group_class, resizable_panel_style, resizable_resize_pair, scroll_area_class,
  scroll_area_orientation_attribute, scroll_area_scrollbar_class, scroll_area_thumb_class,
  scroll_area_viewport_class, select_content_class, select_item_class, select_label_class,
  select_separator_class, select_trigger_class, select_value_class, separator_class,
  sheet_content_class, sheet_overlay_class, skeleton_class, slider_percent, slider_range_style,
  slider_root_class, slider_thumb_style, slider_track_class, switch_class, spinner_class,
  switch_thumb_class, table_class, table_row_class, tabs_content_class, tabs_list_class,
  tabs_trigger_class, textarea_class, toggle_class, toggle_group_class, toggle_group_item_class,
  toggle_group_move_value,
  toggle_group_multiple_selection, tooltip_content_class,
  AlertDialogActionVariant, AlertDialogPrimitiveConfig, AlertVariant, BadgeVariant, ButtonSize,
  ButtonVariant, CalendarDate, CalendarKeyMove, CalendarMonth, CalendarRangeState,
  CalendarWeekday, ComboboxPrimitiveConfig, ContextMenuPrimitiveConfig, DatePickerAlign,
  DatePickerPrimitiveConfig, DatePickerSide, DataTableSortDirection, DialogPrimitiveConfig,
  DrawerPrimitiveConfig, DropdownPrimitiveConfig, FocusMove, HoverCardAlign,
  HoverCardPrimitiveConfig, HoverCardSide, MenubarPrimitiveConfig, NavigationMenuPrimitiveConfig,
  NavigationOrientation, PopoverPrimitiveConfig, RovingFocusItem,
  LayoutOrientation, ResizablePanelState, ScrollAreaOrientation, SelectPrimitiveConfig,
  SeparatorOrientation, SheetPrimitiveConfig, SheetSide, SpinnerSize, ToggleSize, ToggleVariant,
  TooltipPrimitiveConfig, UiDensity,
};

fn main() {
  let class = button_class(
    ButtonVariant::Primary,
    ButtonSize::Md,
    UiDensity::Comfortable,
    "w-full",
  );

  println!("dioxus-ui web demo button class: {class}");
  println!("dioxus-ui web demo input class: {}", input_class(false, "mt-2"));
  println!("dioxus-ui web demo textarea class: {}", textarea_class(false, "mt-2"));
  println!("dioxus-ui web demo label class: {}", label_class("mb-2"));
  println!(
    "dioxus-ui web demo alert class: {}",
    alert_class(AlertVariant::Default, "mb-4")
  );
  println!("dioxus-ui web demo alert title class: {}", alert_title_class(""));
  println!(
    "dioxus-ui web demo alert description class: {}",
    alert_description_class(AlertVariant::Default, "")
  );
  println!(
    "dioxus-ui web demo alert dialog overlay class: {}",
    alert_dialog_overlay_class("")
  );
  println!(
    "dioxus-ui web demo alert dialog content class: {}",
    alert_dialog_content_class("max-w-md")
  );
  println!(
    "dioxus-ui web demo alert dialog action class: {}",
    alert_dialog_action_class(AlertDialogActionVariant::Destructive, "")
  );
  println!(
    "dioxus-ui web demo alert dialog primitive open: {}",
    AlertDialogPrimitiveConfig::controlled(true).open
  );
  println!(
    "dioxus-ui web demo avatar class: {}",
    avatar_class("h-12 w-12")
  );
  println!("dioxus-ui web demo avatar image class: {}", avatar_image_class(""));
  println!(
    "dioxus-ui web demo avatar fallback class: {}",
    avatar_fallback_class("bg-blue-100")
  );
  println!(
    "dioxus-ui web demo badge class: {}",
    badge_class(BadgeVariant::Default, "")
  );
  println!(
    "dioxus-ui web demo calendar day class: {}",
    calendar_day_class(true, false, false, false, CalendarRangeState::Single, "")
  );
  println!(
    "dioxus-ui web demo calendar grid first day: {:?}",
    calendar_month_grid(
      CalendarMonth::unchecked(2024, 6),
      CalendarWeekday::Sunday,
      Some(CalendarDate::unchecked(2024, 6, 10)),
      Some(CalendarDate::unchecked(2024, 6, 11)),
      None,
      None,
      &[],
    )
    .weeks[0][0]
    .date
  );
  println!(
    "dioxus-ui web demo calendar moved date: {:?}",
    calendar_move_date(
      CalendarDate::unchecked(2024, 6, 5),
      CalendarKeyMove::NextWeek,
      CalendarWeekday::Sunday,
    )
  );
  println!("dioxus-ui web demo card class: {}", card_class("max-w-sm"));
  println!("dioxus-ui web demo card header class: {}", card_header_class(""));
  println!("dioxus-ui web demo card title class: {}", card_title_class(""));
  println!(
    "dioxus-ui web demo card description class: {}",
    card_description_class("")
  );
  println!("dioxus-ui web demo card content class: {}", card_content_class(""));
  println!(
    "dioxus-ui web demo card footer class: {}",
    card_footer_class("justify-end")
  );
  println!(
    "dioxus-ui web demo pagination class: {}",
    pagination_class("mt-6")
  );
  println!(
    "dioxus-ui web demo pagination link class: {}",
    pagination_link_class(true, false, "")
  );
  println!("dioxus-ui web demo progress class: {}", progress_class("h-2"));
  println!(
    "dioxus-ui web demo progress indicator class: {}",
    progress_indicator_class("")
  );
  println!(
    "dioxus-ui web demo progress percent: {}",
    progress_percent(64.0, 100.0)
  );
  println!("dioxus-ui web demo slider class: {}", slider_root_class("mt-3"));
  println!(
    "dioxus-ui web demo slider track class: {}",
    slider_track_class("")
  );
  println!(
    "dioxus-ui web demo slider percent: {}",
    slider_percent(42.0, 0.0, 100.0, 1.0)
  );
  println!(
    "dioxus-ui web demo slider range style: {}",
    slider_range_style(42.0)
  );
  println!(
    "dioxus-ui web demo slider thumb style: {}",
    slider_thumb_style(42.0)
  );
  println!(
    "dioxus-ui web demo radio group class: {}",
    radio_group_class(NavigationOrientation::Horizontal, "gap-3")
  );
  println!(
    "dioxus-ui web demo radio group item class: {}",
    radio_group_item_class(true, "")
  );
  println!(
    "dioxus-ui web demo radio group next value: {:?}",
    radio_group_move_value(
      Some("sm"),
      &[
        RovingFocusItem::enabled("sm"),
        RovingFocusItem::disabled("md"),
        RovingFocusItem::enabled("lg"),
      ],
      FocusMove::Next,
      NavigationOrientation::Horizontal,
      true,
    )
  );
  println!(
    "dioxus-ui web demo toggle group class: {}",
    toggle_group_class(NavigationOrientation::Horizontal, "gap-1")
  );
  println!(
    "dioxus-ui web demo toggle group item class: {}",
    toggle_group_item_class(true, "")
  );
  println!(
    "dioxus-ui web demo toggle group next value: {:?}",
    toggle_group_move_value(
      Some("bold"),
      &[
        RovingFocusItem::enabled("bold"),
        RovingFocusItem::disabled("italic"),
        RovingFocusItem::enabled("underline"),
      ],
      FocusMove::Next,
      NavigationOrientation::Horizontal,
      true,
    )
  );
  println!(
    "dioxus-ui web demo toggle group values: {:?}",
    toggle_group_multiple_selection(&["bold".to_string()], "underline")
  );
  println!(
    "dioxus-ui web demo separator class: {}",
    separator_class(SeparatorOrientation::Horizontal, "my-4")
  );
  println!("dioxus-ui web demo skeleton class: {}", skeleton_class("h-4 w-32"));
  println!(
    "dioxus-ui web demo spinner class: {}",
    spinner_class(SpinnerSize::Md, "text-blue-600")
  );
  println!("dioxus-ui web demo table class: {}", table_class("min-w-lg"));
  println!("dioxus-ui web demo table row class: {}", table_row_class(""));
  println!("dioxus-ui web demo checkbox class: {}", checkbox_class(true, "mt-2"));
  println!("dioxus-ui web demo command class: {}", command_class("max-w-md"));
  println!(
    "dioxus-ui web demo command input class: {}",
    command_input_class("")
  );
  println!(
    "dioxus-ui web demo command item class: {}",
    command_item_class(true, false, "")
  );
  println!(
    "dioxus-ui web demo command active descendant: {:?}",
    command_active_descendant_state(Some("open-file".to_string())).active_id
  );
  println!(
    "dioxus-ui web demo combobox trigger class: {}",
    combobox_trigger_class(false, "w-64")
  );
  println!(
    "dioxus-ui web demo combobox input class: {}",
    combobox_input_class("")
  );
  println!(
    "dioxus-ui web demo combobox item class: {}",
    combobox_item_class(true, false, "")
  );
  println!(
    "dioxus-ui web demo combobox primitive open: {}",
    ComboboxPrimitiveConfig::controlled(true).open
  );
  println!(
    "dioxus-ui web demo context menu content class: {}",
    context_menu_content_class("min-w-48")
  );
  println!(
    "dioxus-ui web demo context menu item class: {}",
    context_menu_item_class(true, false, "")
  );
  println!(
    "dioxus-ui web demo context menu shortcut class: {}",
    context_menu_shortcut_class("")
  );
  println!(
    "dioxus-ui web demo context menu primitive open: {}",
    ContextMenuPrimitiveConfig::controlled(true).open
  );
  println!(
    "dioxus-ui web demo data table header class: {}",
    data_table_header_cell_class(true, "w-40")
  );
  println!(
    "dioxus-ui web demo data table row class: {}",
    data_table_row_class(true, false, "")
  );
  println!(
    "dioxus-ui web demo data table page window: {:?}",
    data_table_page_window(1, 10, 24)
  );
  println!(
    "dioxus-ui web demo data table selected rows: {:?}",
    data_table_toggle_row(&["row-1".to_string()], "row-2")
  );
  println!(
    "dioxus-ui web demo data table sort: {}",
    data_table_sort_attribute(Some(DataTableSortDirection::Ascending))
  );
  println!(
    "dioxus-ui web demo scroll area class: {}",
    scroll_area_class("h-72")
  );
  println!(
    "dioxus-ui web demo scroll area viewport class: {}",
    scroll_area_viewport_class("")
  );
  println!(
    "dioxus-ui web demo scroll area scrollbar class: {}",
    scroll_area_scrollbar_class(ScrollAreaOrientation::Vertical, "")
  );
  println!(
    "dioxus-ui web demo scroll area thumb class: {}",
    scroll_area_thumb_class("")
  );
  println!(
    "dioxus-ui web demo scroll area orientation: {}",
    scroll_area_orientation_attribute(ScrollAreaOrientation::Both)
  );
  println!(
    "dioxus-ui web demo resizable group class: {}",
    resizable_panel_group_class(LayoutOrientation::Horizontal, "h-64")
  );
  println!(
    "dioxus-ui web demo resizable handle class: {}",
    resizable_handle_class(false, "")
  );
  println!(
    "dioxus-ui web demo resizable panel style: {}",
    resizable_panel_style(75.0, 20.0, 80.0)
  );
  println!(
    "dioxus-ui web demo resizable resize: {:?}",
    resizable_resize_pair(
      ResizablePanelState::new(50.0, 20.0, 80.0),
      ResizablePanelState::new(50.0, 20.0, 80.0),
      10.0,
    )
  );
  println!(
    "dioxus-ui web demo date picker trigger class: {}",
    date_picker_trigger_class(false, "w-64")
  );
  println!(
    "dioxus-ui web demo date picker value class: {}",
    date_picker_value_class("")
  );
  println!(
    "dioxus-ui web demo date picker content class: {}",
    date_picker_content_class("p-3")
  );
  println!(
    "dioxus-ui web demo date picker side/align: {}/{}",
    date_picker_side_attribute(DatePickerSide::Bottom),
    date_picker_align_attribute(DatePickerAlign::Start)
  );
  println!(
    "dioxus-ui web demo date picker primitive open: {}",
    DatePickerPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-ui web demo menubar class: {}", menubar_class("w-fit"));
  println!(
    "dioxus-ui web demo menubar trigger class: {}",
    menubar_trigger_class(true, "")
  );
  println!(
    "dioxus-ui web demo menubar item class: {}",
    menubar_item_class(true, false, "")
  );
  println!(
    "dioxus-ui web demo menubar primitive open: {}",
    MenubarPrimitiveConfig::controlled(true).open
  );
  println!(
    "dioxus-ui web demo navigation menu class: {}",
    navigation_menu_class("w-full")
  );
  println!(
    "dioxus-ui web demo navigation menu trigger class: {}",
    navigation_menu_trigger_class(true, "")
  );
  println!(
    "dioxus-ui web demo navigation menu link class: {}",
    navigation_menu_link_class(true, "")
  );
  println!(
    "dioxus-ui web demo navigation menu primitive open: {}",
    NavigationMenuPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-ui web demo switch class: {}", switch_class(true, "mt-2"));
  println!("dioxus-ui web demo switch thumb class: {}", switch_thumb_class(true));
  println!("dioxus-ui web demo tabs list class: {}", tabs_list_class("mt-4"));
  println!(
    "dioxus-ui web demo tabs trigger class: {}",
    tabs_trigger_class(true, "min-w-24")
  );
  println!("dioxus-ui web demo tabs content class: {}", tabs_content_class("p-4"));
  println!(
    "dioxus-ui web demo toggle class: {}",
    toggle_class(ToggleVariant::Default, ToggleSize::Md, true, "")
  );
  println!("dioxus-ui web demo accordion item class: {}", accordion_item_class(""));
  println!(
    "dioxus-ui web demo accordion trigger class: {}",
    accordion_trigger_class(true, "")
  );
  println!(
    "dioxus-ui web demo accordion content class: {}",
    accordion_content_class("px-1")
  );
  println!("dioxus-ui web demo dialog overlay class: {}", dialog_overlay_class(""));
  println!(
    "dioxus-ui web demo dialog content class: {}",
    dialog_content_class("max-w-xl")
  );
  println!(
    "dioxus-ui web demo dialog primitive open: {}",
    DialogPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-ui web demo drawer overlay class: {}", drawer_overlay_class(""));
  println!(
    "dioxus-ui web demo drawer content class: {}",
    drawer_content_class("max-h-[70vh]")
  );
  println!(
    "dioxus-ui web demo drawer primitive open: {}",
    DrawerPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-ui web demo popover content class: {}", popover_content_class("w-80"));
  println!("dioxus-ui web demo popover header class: {}", popover_header_class(""));
  println!("dioxus-ui web demo popover title class: {}", popover_title_class(""));
  println!(
    "dioxus-ui web demo popover description class: {}",
    popover_description_class("")
  );
  println!(
    "dioxus-ui web demo popover primitive open: {}",
    PopoverPrimitiveConfig::controlled(true).open
  );
  println!(
    "dioxus-ui web demo hover card content class: {}",
    hover_card_content_class("w-96")
  );
  println!(
    "dioxus-ui web demo hover card side/align: {}/{}",
    hover_card_side_attribute(HoverCardSide::Bottom),
    hover_card_align_attribute(HoverCardAlign::Center)
  );
  println!(
    "dioxus-ui web demo hover card primitive open: {}",
    HoverCardPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-ui web demo tooltip content class: {}", tooltip_content_class(""));
  println!(
    "dioxus-ui web demo tooltip primitive delay: {}",
    TooltipPrimitiveConfig::controlled(true).delay_ms
  );
  println!("dioxus-ui web demo select trigger class: {}", select_trigger_class(false, "w-44"));
  println!("dioxus-ui web demo select value class: {}", select_value_class(""));
  println!("dioxus-ui web demo select content class: {}", select_content_class(""));
  println!("dioxus-ui web demo select label class: {}", select_label_class(""));
  println!("dioxus-ui web demo select item class: {}", select_item_class(true, ""));
  println!("dioxus-ui web demo select separator class: {}", select_separator_class(""));
  println!(
    "dioxus-ui web demo select primitive value: {:?}",
    SelectPrimitiveConfig::controlled(true, Some("system".to_string())).value
  );
  println!(
    "dioxus-ui web demo native select class: {}",
    native_select_class(false, "w-48")
  );
  println!(
    "dioxus-ui web demo native select group class: {}",
    native_select_group_class("")
  );
  println!(
    "dioxus-ui web demo native select option class: {}",
    native_select_option_class("")
  );
  println!("dioxus-ui web demo sheet overlay class: {}", sheet_overlay_class(""));
  println!(
    "dioxus-ui web demo sheet content class: {}",
    sheet_content_class(SheetSide::Right, "w-80")
  );
  println!(
    "dioxus-ui web demo sheet primitive open: {}",
    SheetPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-ui web demo dropdown content class: {}", dropdown_content_class(""));
  println!("dioxus-ui web demo dropdown label class: {}", dropdown_label_class(""));
  println!(
    "dioxus-ui web demo dropdown item class: {}",
    dropdown_item_class(false, "")
  );
  println!(
    "dioxus-ui web demo dropdown separator class: {}",
    dropdown_separator_class("")
  );
  println!(
    "dioxus-ui web demo dropdown primitive open: {}",
    DropdownPrimitiveConfig::controlled(true).open
  );
}
