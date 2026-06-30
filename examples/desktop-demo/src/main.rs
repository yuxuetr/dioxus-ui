use dioxus_ui::{
  accordion_content_class, accordion_item_class, accordion_trigger_class,
  alert_dialog_action_class, alert_dialog_content_class, alert_dialog_overlay_class, alert_class,
  alert_description_class, alert_title_class, aspect_ratio_style, attachment_action_class,
  attachment_actions_class, attachment_class, attachment_content_class,
  attachment_description_class, attachment_group_class, attachment_media_class,
  attachment_title_class, attachment_trigger_class, avatar_class, avatar_fallback_class,
  avatar_image_class, badge_class, breadcrumb_link_class, breadcrumb_list_class, bubble_class,
  bubble_content_class, bubble_group_class, bubble_reactions_class, button_class,
  button_group_class, button_group_item_class,
  calendar_day_class, calendar_month_grid, calendar_move_date, carousel_can_go_previous,
  carousel_content_class, carousel_control_class, carousel_indicator_class, carousel_item_class,
  carousel_previous, card_class,
  card_content_class, card_description_class, card_footer_class, card_header_class,
  card_title_class, checkbox_class, collapsible_class, collapsible_content_class,
  collapsible_trigger_class, command_active_descendant_state, command_class,
  command_input_class, command_item_class, combobox_input_class,
  combobox_item_class, combobox_trigger_class, context_menu_content_class,
  context_menu_item_class, context_menu_shortcut_class, data_table_header_cell_class,
  data_table_page_window, data_table_row_class, data_table_sort_attribute,
  data_table_toggle_row, date_picker_align_attribute, date_picker_content_class,
  date_picker_side_attribute, date_picker_trigger_class, date_picker_value_class,
  dialog_content_class, dialog_overlay_class, direction_class, drawer_content_class,
  drawer_overlay_class, dropdown_content_class, dropdown_item_class, dropdown_label_class,
  dropdown_separator_class, empty_actions_class, empty_class, empty_title_class,
  field_class, field_error_class, field_group_class, hover_card_align_attribute,
  hover_card_content_class, hover_card_side_attribute, input_class, input_group_action_class,
  input_group_addon_class, input_group_class, input_group_control_class, input_otp_class,
  input_otp_group_class, input_otp_hidden_input_class, input_otp_separator_class,
  input_otp_slot_class, otp_apply_paste_filtered, otp_slots, item_class,
  item_description_class, item_title_class, kbd_class, label_class, menubar_class,
  menubar_item_class, menubar_trigger_class, message_avatar_class, message_class,
  message_content_class, message_footer_class, message_group_class, message_header_class,
  native_select_class, native_select_group_class,
  native_select_option_class, navigation_menu_class, navigation_menu_link_class,
  navigation_menu_trigger_class, pagination_class, pagination_link_class,
  popover_content_class, popover_description_class, popover_header_class, popover_title_class,
  progress_class, progress_indicator_class, progress_percent, radio_group_class,
  radio_group_item_class, radio_group_move_value, resizable_handle_class,
  resizable_panel_group_class, resizable_panel_style, resizable_resize_pair, scroll_area_class,
  scroll_area_orientation_attribute, scroll_area_scrollbar_class, scroll_area_thumb_class,
  scroll_area_viewport_class, select_content_class, select_item_class, select_label_class,
  select_separator_class, select_trigger_class, select_value_class, separator_class,
  sheet_content_class, sheet_overlay_class, sidebar_class, sidebar_item_class,
  sidebar_side_attribute, sidebar_toggle, sidebar_trigger_class, skeleton_class, slider_percent,
  slider_range_style, slider_root_class, slider_thumb_style, slider_track_class,
  sonner_icon_class, sonner_queue_push, sonner_toast_class, sonner_viewport_class, switch_class,
  spinner_class, switch_thumb_class, table_class, table_row_class, tabs_content_class,
  tabs_list_class, tabs_trigger_class, textarea_class, toggle_class, toggle_group_class,
  toggle_group_item_class, toggle_group_move_value,
  toggle_group_single_selection, toast_action_class, toast_close_class, toast_is_expired,
  toast_queue_push, toast_root_class, toast_viewport_class, tooltip_content_class,
  typography_h2_class, typography_inline_code_class, typography_p_class,
  AlertDialogActionVariant, AlertDialogPrimitiveConfig, AlertVariant, AttachmentMediaVariant,
  AttachmentOrientation, AttachmentSize, AttachmentState, BadgeVariant, BubbleAlign,
  BubbleReactionAlign, BubbleReactionSide, BubbleVariant,
  ButtonGroupOrientation, ButtonSize, ButtonVariant, CalendarDate, CalendarKeyMove,
  CalendarMonth, CalendarRangeState, CalendarWeekday, CarouselOrientation, CarouselState,
  ComboboxPrimitiveConfig, InputGroupAddonPosition, InputOtpInputMode,
  ContextMenuPrimitiveConfig, DatePickerAlign, DatePickerPrimitiveConfig, DatePickerSide,
  DataTableSortDirection, DialogPrimitiveConfig, DrawerPrimitiveConfig, DropdownPrimitiveConfig,
  FocusMove, HoverCardAlign, HoverCardPrimitiveConfig, HoverCardSide, MenubarPrimitiveConfig,
  MessageAlign, NavigationMenuPrimitiveConfig, NavigationOrientation, PopoverPrimitiveConfig,
  RovingFocusItem, KbdSize, LayoutOrientation, ResizablePanelState, ScrollAreaOrientation, SelectPrimitiveConfig,
  SeparatorOrientation, SheetPrimitiveConfig, SheetSide, SidebarSide, SidebarState, SpinnerSize,
  SonnerItem, SonnerPlacement, SonnerQueue, SonnerVariant, TextDirection, ToastItem,
  ToastPlacement, ToastQueue, ToastVariant, ToggleSize, ToggleVariant, TooltipPrimitiveConfig,
  UiDensity,
};

fn main() {
  let class = button_class(
    ButtonVariant::Secondary,
    ButtonSize::Sm,
    UiDensity::Compact,
    "justify-start",
  );

  println!("dioxus-ui desktop demo button class: {class}");
  println!(
    "dioxus-ui desktop demo button group class: {}",
    button_group_class(ButtonGroupOrientation::Vertical, false, "items-start")
  );
  println!(
    "dioxus-ui desktop demo button group item class: {}",
    button_group_item_class("min-w-16")
  );
  println!(
    "dioxus-ui desktop demo attachment class: {}",
    attachment_class(
      AttachmentState::Error,
      AttachmentSize::Sm,
      AttachmentOrientation::Vertical,
      "max-w-xs"
    )
  );
  println!(
    "dioxus-ui desktop demo attachment parts: {}/{}/{}/{}",
    attachment_group_class("pb-2"),
    attachment_media_class(AttachmentMediaVariant::Image, "rounded-none"),
    attachment_content_class("gap-0"),
    attachment_title_class("text-sm")
  );
  println!(
    "dioxus-ui desktop demo attachment actions: {}/{}/{}",
    attachment_description_class("text-red-700"),
    attachment_actions_class("justify-end"),
    attachment_action_class("text-red-600")
  );
  println!(
    "dioxus-ui desktop demo attachment trigger/state: {}/{}",
    attachment_trigger_class("w-full"),
    AttachmentState::Error.attribute()
  );
  println!(
    "dioxus-ui desktop demo bubble class: {}",
    bubble_class(BubbleAlign::Start, "max-w-sm")
  );
  println!(
    "dioxus-ui desktop demo bubble content class: {}",
    bubble_content_class(BubbleVariant::Destructive, "rounded-lg")
  );
  println!(
    "dioxus-ui desktop demo bubble reactions class: {}",
    bubble_reactions_class(
      BubbleReactionSide::Top,
      BubbleReactionAlign::Start,
      "opacity-80"
    )
  );
  println!(
    "dioxus-ui desktop demo bubble group/variant: {}/{}",
    bubble_group_class("gap-1"),
    BubbleVariant::Destructive.attribute()
  );
  println!(
    "dioxus-ui desktop demo message class: {}",
    message_class(MessageAlign::Start, "max-w-xl")
  );
  println!(
    "dioxus-ui desktop demo message parts: {}/{}/{}/{}",
    message_group_class("gap-3"),
    message_avatar_class("bg-red-100"),
    message_content_class(MessageAlign::Start, "gap-1"),
    message_header_class("font-medium")
  );
  println!(
    "dioxus-ui desktop demo message footer/align: {}/{}",
    message_footer_class("justify-start"),
    MessageAlign::Start.attribute()
  );
  println!("dioxus-ui desktop demo input class: {}", input_class(false, "h-8"));
  println!(
    "dioxus-ui desktop demo input group class: {}",
    input_group_class(true, true, "max-w-xs")
  );
  println!(
    "dioxus-ui desktop demo input group addon class: {}",
    input_group_addon_class(InputGroupAddonPosition::End, "text-xs")
  );
  println!(
    "dioxus-ui desktop demo input group control class: {}",
    input_group_control_class("min-w-32")
  );
  println!(
    "dioxus-ui desktop demo input group action class: {}",
    input_group_action_class("text-red-600")
  );
  println!(
    "dioxus-ui desktop demo input otp class: {}",
    input_otp_class(true, "max-w-xs")
  );
  println!(
    "dioxus-ui desktop demo input otp group class: {}",
    input_otp_group_class("gap-1")
  );
  println!(
    "dioxus-ui desktop demo input otp slot class: {}",
    input_otp_slot_class(false, true, true, "h-9 w-9")
  );
  println!(
    "dioxus-ui desktop demo input otp separator/input: {}/{}",
    input_otp_separator_class("text-red-600"),
    input_otp_hidden_input_class("absolute")
  );
  let desktop_otp = otp_apply_paste_filtered("1", 1, "2b34", 4, |ch| ch.is_ascii_digit());
  println!(
    "dioxus-ui desktop demo input otp helper: {}/{}",
    desktop_otp,
    otp_slots(&desktop_otp, 4, 3).len()
  );
  println!(
    "dioxus-ui desktop demo input otp mode: {}",
    InputOtpInputMode::Text.attribute()
  );
  println!(
    "dioxus-ui desktop demo textarea class: {}",
    textarea_class(false, "min-h-20")
  );
  println!("dioxus-ui desktop demo label class: {}", label_class("text-xs"));
  println!(
    "dioxus-ui desktop demo alert class: {}",
    alert_class(AlertVariant::Destructive, "mb-2")
  );
  println!(
    "dioxus-ui desktop demo alert title class: {}",
    alert_title_class("text-sm")
  );
  println!(
    "dioxus-ui desktop demo alert description class: {}",
    alert_description_class(AlertVariant::Destructive, "")
  );
  println!(
    "dioxus-ui desktop demo alert dialog overlay class: {}",
    alert_dialog_overlay_class("")
  );
  println!(
    "dioxus-ui desktop demo alert dialog content class: {}",
    alert_dialog_content_class("max-w-sm")
  );
  println!(
    "dioxus-ui desktop demo alert dialog action class: {}",
    alert_dialog_action_class(AlertDialogActionVariant::Default, "")
  );
  println!(
    "dioxus-ui desktop demo alert dialog primitive open: {}",
    AlertDialogPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-ui desktop demo direction class/attr: {}/{}",
    direction_class("inline"),
    TextDirection::Ltr.attribute()
  );
  println!(
    "dioxus-ui desktop demo aspect ratio style: {}",
    aspect_ratio_style(4.0 / 3.0)
  );
  println!(
    "dioxus-ui desktop demo avatar class: {}",
    avatar_class("h-8 w-8")
  );
  println!(
    "dioxus-ui desktop demo avatar image class: {}",
    avatar_image_class("")
  );
  println!(
    "dioxus-ui desktop demo avatar fallback class: {}",
    avatar_fallback_class("text-xs")
  );
  println!(
    "dioxus-ui desktop demo badge class: {}",
    badge_class(BadgeVariant::Secondary, "")
  );
  println!(
    "dioxus-ui desktop demo breadcrumb list class: {}",
    breadcrumb_list_class("gap-2")
  );
  println!(
    "dioxus-ui desktop demo breadcrumb current link class: {}",
    breadcrumb_link_class(true, "")
  );
  println!(
    "dioxus-ui desktop demo empty class: {}",
    empty_class("min-h-48")
  );
  println!(
    "dioxus-ui desktop demo empty title class: {}",
    empty_title_class("text-base")
  );
  println!(
    "dioxus-ui desktop demo empty actions class: {}",
    empty_actions_class("justify-end")
  );
  println!(
    "dioxus-ui desktop demo field class: {}",
    field_class(false, "max-w-md")
  );
  println!(
    "dioxus-ui desktop demo field error class: {}",
    field_error_class("text-xs")
  );
  println!(
    "dioxus-ui desktop demo field group class: {}",
    field_group_class("gap-3")
  );
  println!(
    "dioxus-ui desktop demo item class: {}",
    item_class(false, true, "")
  );
  println!(
    "dioxus-ui desktop demo item title class: {}",
    item_title_class("text-sm")
  );
  println!(
    "dioxus-ui desktop demo item description class: {}",
    item_description_class("")
  );
  println!(
    "dioxus-ui desktop demo kbd class: {}",
    kbd_class(KbdSize::Sm, "")
  );
  println!(
    "dioxus-ui desktop demo typography h2 class: {}",
    typography_h2_class("")
  );
  println!(
    "dioxus-ui desktop demo typography p class: {}",
    typography_p_class("")
  );
  println!(
    "dioxus-ui desktop demo typography inline code class: {}",
    typography_inline_code_class("")
  );
  println!(
    "dioxus-ui desktop demo calendar day class: {}",
    calendar_day_class(false, true, true, false, CalendarRangeState::Middle, "")
  );
  println!(
    "dioxus-ui desktop demo calendar grid first day: {:?}",
    calendar_month_grid(
      CalendarMonth::unchecked(2024, 2),
      CalendarWeekday::Monday,
      Some(CalendarDate::unchecked(2024, 2, 29)),
      None,
      Some(CalendarDate::unchecked(2024, 2, 10)),
      Some(CalendarDate::unchecked(2024, 2, 12)),
      &[],
    )
    .weeks[0][0]
    .date
  );
  println!(
    "dioxus-ui desktop demo calendar moved date: {:?}",
    calendar_move_date(
      CalendarDate::unchecked(2024, 2, 29),
      CalendarKeyMove::NextYear,
      CalendarWeekday::Monday,
    )
  );
  println!(
    "dioxus-ui desktop demo carousel content class: {}",
    carousel_content_class(CarouselOrientation::Vertical, "")
  );
  println!(
    "dioxus-ui desktop demo carousel item class: {}",
    carousel_item_class(CarouselOrientation::Vertical, false, "basis-full")
  );
  println!(
    "dioxus-ui desktop demo carousel previous control class: {}",
    carousel_control_class(true, "")
  );
  println!(
    "dioxus-ui desktop demo carousel indicator class: {}",
    carousel_indicator_class(false, "")
  );
  println!(
    "dioxus-ui desktop demo carousel previous/can: {}/{}",
    carousel_previous(0, 3, true),
    carousel_can_go_previous(
      CarouselState::new(0, 3).index,
      CarouselState::new(0, 3).item_count,
      true,
    )
  );
  println!("dioxus-ui desktop demo card class: {}", card_class("shadow-none"));
  println!(
    "dioxus-ui desktop demo card header class: {}",
    card_header_class("p-4")
  );
  println!("dioxus-ui desktop demo card title class: {}", card_title_class("text-lg"));
  println!(
    "dioxus-ui desktop demo card description class: {}",
    card_description_class("")
  );
  println!(
    "dioxus-ui desktop demo card content class: {}",
    card_content_class("p-4 pt-0")
  );
  println!(
    "dioxus-ui desktop demo card footer class: {}",
    card_footer_class("p-4 pt-0")
  );
  println!(
    "dioxus-ui desktop demo pagination class: {}",
    pagination_class("mt-2")
  );
  println!(
    "dioxus-ui desktop demo pagination link class: {}",
    pagination_link_class(false, true, "")
  );
  println!(
    "dioxus-ui desktop demo progress class: {}",
    progress_class("h-1.5")
  );
  println!(
    "dioxus-ui desktop demo progress indicator class: {}",
    progress_indicator_class("bg-zinc-900")
  );
  println!(
    "dioxus-ui desktop demo progress percent: {}",
    progress_percent(3.0, 4.0)
  );
  println!(
    "dioxus-ui desktop demo toast viewport class: {}",
    toast_viewport_class(ToastPlacement::TopRight, "")
  );
  println!(
    "dioxus-ui desktop demo toast root class: {}",
    toast_root_class(ToastVariant::Error, "")
  );
  println!(
    "dioxus-ui desktop demo toast action/close class: {}/{}",
    toast_action_class(true, ""),
    toast_close_class(false, "")
  );
  println!(
    "dioxus-ui desktop demo toast queue/expired: {}/{}",
    toast_queue_push(ToastQueue::new(1), ToastItem::new("failed", "Failed")).items.len(),
    toast_is_expired(1000, 5000)
  );
  println!(
    "dioxus-ui desktop demo sonner viewport class: {}",
    sonner_viewport_class(SonnerPlacement::TopRight, "")
  );
  println!(
    "dioxus-ui desktop demo sonner toast/icon class: {}/{}",
    sonner_toast_class(SonnerVariant::Error, ""),
    sonner_icon_class(SonnerVariant::Error, "")
  );
  println!(
    "dioxus-ui desktop demo sonner queue: {}",
    sonner_queue_push(SonnerQueue::new(1), SonnerItem::new("offline", "Offline")).items.len()
  );
  println!(
    "dioxus-ui desktop demo slider class: {}",
    slider_root_class("mt-2")
  );
  println!(
    "dioxus-ui desktop demo slider track class: {}",
    slider_track_class("h-1.5")
  );
  println!(
    "dioxus-ui desktop demo slider percent: {}",
    slider_percent(3.0, 0.0, 4.0, 1.0)
  );
  println!(
    "dioxus-ui desktop demo slider range style: {}",
    slider_range_style(75.0)
  );
  println!(
    "dioxus-ui desktop demo slider thumb style: {}",
    slider_thumb_style(75.0)
  );
  println!(
    "dioxus-ui desktop demo radio group class: {}",
    radio_group_class(NavigationOrientation::Vertical, "gap-2")
  );
  println!(
    "dioxus-ui desktop demo radio group item class: {}",
    radio_group_item_class(false, "")
  );
  println!(
    "dioxus-ui desktop demo radio group next value: {:?}",
    radio_group_move_value(
      Some("compact"),
      &[
        RovingFocusItem::enabled("compact"),
        RovingFocusItem::disabled("comfortable"),
        RovingFocusItem::enabled("touch"),
      ],
      FocusMove::Next,
      NavigationOrientation::Vertical,
      true,
    )
  );
  println!(
    "dioxus-ui desktop demo toggle group class: {}",
    toggle_group_class(NavigationOrientation::Vertical, "gap-2")
  );
  println!(
    "dioxus-ui desktop demo toggle group item class: {}",
    toggle_group_item_class(false, "")
  );
  println!(
    "dioxus-ui desktop demo toggle group next value: {:?}",
    toggle_group_move_value(
      Some("left"),
      &[
        RovingFocusItem::enabled("left"),
        RovingFocusItem::disabled("center"),
        RovingFocusItem::enabled("right"),
      ],
      FocusMove::Next,
      NavigationOrientation::Vertical,
      true,
    )
  );
  println!(
    "dioxus-ui desktop demo toggle group value: {:?}",
    toggle_group_single_selection(Some("left"), "right")
  );
  println!(
    "dioxus-ui desktop demo separator class: {}",
    separator_class(SeparatorOrientation::Vertical, "mx-2")
  );
  println!(
    "dioxus-ui desktop demo skeleton class: {}",
    skeleton_class("h-3 w-24")
  );
  println!(
    "dioxus-ui desktop demo spinner class: {}",
    spinner_class(SpinnerSize::Sm, "text-zinc-700")
  );
  println!("dioxus-ui desktop demo table class: {}", table_class("text-xs"));
  println!("dioxus-ui desktop demo table row class: {}", table_row_class(""));
  println!(
    "dioxus-ui desktop demo checkbox class: {}",
    checkbox_class(false, "mt-1")
  );
  println!(
    "dioxus-ui desktop demo collapsible class: {}",
    collapsible_class(true, "max-w-xs")
  );
  println!(
    "dioxus-ui desktop demo collapsible trigger class: {}",
    collapsible_trigger_class(false, "w-full")
  );
  println!(
    "dioxus-ui desktop demo collapsible content class: {}",
    collapsible_content_class(false, "pt-1")
  );
  println!(
    "dioxus-ui desktop demo command class: {}",
    command_class("max-w-sm")
  );
  println!(
    "dioxus-ui desktop demo command input class: {}",
    command_input_class("h-9")
  );
  println!(
    "dioxus-ui desktop demo command item class: {}",
    command_item_class(false, true, "")
  );
  println!(
    "dioxus-ui desktop demo command active descendant: {:?}",
    command_active_descendant_state(Some("quick-open".to_string())).active_id
  );
  println!(
    "dioxus-ui desktop demo combobox trigger class: {}",
    combobox_trigger_class(true, "w-56")
  );
  println!(
    "dioxus-ui desktop demo combobox input class: {}",
    combobox_input_class("h-9")
  );
  println!(
    "dioxus-ui desktop demo combobox item class: {}",
    combobox_item_class(false, true, "")
  );
  println!(
    "dioxus-ui desktop demo combobox primitive open: {}",
    ComboboxPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-ui desktop demo context menu content class: {}",
    context_menu_content_class("min-w-40")
  );
  println!(
    "dioxus-ui desktop demo context menu item class: {}",
    context_menu_item_class(false, true, "")
  );
  println!(
    "dioxus-ui desktop demo context menu shortcut class: {}",
    context_menu_shortcut_class("")
  );
  println!(
    "dioxus-ui desktop demo context menu primitive open: {}",
    ContextMenuPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-ui desktop demo data table header class: {}",
    data_table_header_cell_class(false, "w-32")
  );
  println!(
    "dioxus-ui desktop demo data table row class: {}",
    data_table_row_class(false, true, "")
  );
  println!(
    "dioxus-ui desktop demo data table page window: {:?}",
    data_table_page_window(5, 10, 22)
  );
  println!(
    "dioxus-ui desktop demo data table selected rows: {:?}",
    data_table_toggle_row(&["row-2".to_string()], "row-2")
  );
  println!(
    "dioxus-ui desktop demo data table sort: {}",
    data_table_sort_attribute(Some(DataTableSortDirection::Descending))
  );
  println!(
    "dioxus-ui desktop demo scroll area class: {}",
    scroll_area_class("h-64")
  );
  println!(
    "dioxus-ui desktop demo scroll area viewport class: {}",
    scroll_area_viewport_class("")
  );
  println!(
    "dioxus-ui desktop demo scroll area scrollbar class: {}",
    scroll_area_scrollbar_class(ScrollAreaOrientation::Horizontal, "")
  );
  println!(
    "dioxus-ui desktop demo scroll area thumb class: {}",
    scroll_area_thumb_class("")
  );
  println!(
    "dioxus-ui desktop demo scroll area orientation: {}",
    scroll_area_orientation_attribute(ScrollAreaOrientation::Horizontal)
  );
  println!(
    "dioxus-ui desktop demo resizable group class: {}",
    resizable_panel_group_class(LayoutOrientation::Vertical, "h-52")
  );
  println!(
    "dioxus-ui desktop demo resizable handle class: {}",
    resizable_handle_class(true, "")
  );
  println!(
    "dioxus-ui desktop demo resizable panel style: {}",
    resizable_panel_style(10.0, 20.0, 80.0)
  );
  println!(
    "dioxus-ui desktop demo resizable resize: {:?}",
    resizable_resize_pair(
      ResizablePanelState::new(60.0, 20.0, 80.0),
      ResizablePanelState::new(40.0, 20.0, 80.0),
      -15.0,
    )
  );
  println!(
    "dioxus-ui desktop demo sidebar class: {}",
    sidebar_class(true, SidebarSide::Right, "shrink-0")
  );
  println!(
    "dioxus-ui desktop demo sidebar item class: {}",
    sidebar_item_class(false, true, "")
  );
  println!(
    "dioxus-ui desktop demo sidebar trigger class: {}",
    sidebar_trigger_class("")
  );
  println!(
    "dioxus-ui desktop demo sidebar side/toggle: {}/{}",
    sidebar_side_attribute(SidebarSide::Right),
    sidebar_toggle(SidebarState::new(true).collapsed)
  );
  println!(
    "dioxus-ui desktop demo date picker trigger class: {}",
    date_picker_trigger_class(true, "w-56")
  );
  println!(
    "dioxus-ui desktop demo date picker value class: {}",
    date_picker_value_class("text-xs")
  );
  println!(
    "dioxus-ui desktop demo date picker content class: {}",
    date_picker_content_class("p-2")
  );
  println!(
    "dioxus-ui desktop demo date picker side/align: {}/{}",
    date_picker_side_attribute(DatePickerSide::Right),
    date_picker_align_attribute(DatePickerAlign::End)
  );
  println!(
    "dioxus-ui desktop demo date picker primitive open: {}",
    DatePickerPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-ui desktop demo menubar class: {}",
    menubar_class("w-fit")
  );
  println!(
    "dioxus-ui desktop demo menubar trigger class: {}",
    menubar_trigger_class(false, "")
  );
  println!(
    "dioxus-ui desktop demo menubar item class: {}",
    menubar_item_class(false, true, "")
  );
  println!(
    "dioxus-ui desktop demo menubar primitive open: {}",
    MenubarPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-ui desktop demo navigation menu class: {}",
    navigation_menu_class("w-full")
  );
  println!(
    "dioxus-ui desktop demo navigation menu trigger class: {}",
    navigation_menu_trigger_class(false, "")
  );
  println!(
    "dioxus-ui desktop demo navigation menu link class: {}",
    navigation_menu_link_class(false, "")
  );
  println!(
    "dioxus-ui desktop demo navigation menu primitive open: {}",
    NavigationMenuPrimitiveConfig::controlled(false).open
  );
  println!("dioxus-ui desktop demo switch class: {}", switch_class(false, "mt-1"));
  println!(
    "dioxus-ui desktop demo switch thumb class: {}",
    switch_thumb_class(false)
  );
  println!("dioxus-ui desktop demo tabs list class: {}", tabs_list_class("mt-2"));
  println!(
    "dioxus-ui desktop demo tabs trigger class: {}",
    tabs_trigger_class(false, "min-w-20")
  );
  println!(
    "dioxus-ui desktop demo tabs content class: {}",
    tabs_content_class("p-2")
  );
  println!(
    "dioxus-ui desktop demo toggle class: {}",
    toggle_class(ToggleVariant::Outline, ToggleSize::Sm, false, "")
  );
  println!(
    "dioxus-ui desktop demo accordion item class: {}",
    accordion_item_class("")
  );
  println!(
    "dioxus-ui desktop demo accordion trigger class: {}",
    accordion_trigger_class(false, "")
  );
  println!(
    "dioxus-ui desktop demo accordion content class: {}",
    accordion_content_class("px-1")
  );
  println!(
    "dioxus-ui desktop demo dialog overlay class: {}",
    dialog_overlay_class("")
  );
  println!(
    "dioxus-ui desktop demo dialog content class: {}",
    dialog_content_class("max-w-md")
  );
  println!(
    "dioxus-ui desktop demo dialog primitive open: {}",
    DialogPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-ui desktop demo drawer overlay class: {}",
    drawer_overlay_class("")
  );
  println!(
    "dioxus-ui desktop demo drawer content class: {}",
    drawer_content_class("max-h-[60vh]")
  );
  println!(
    "dioxus-ui desktop demo drawer primitive open: {}",
    DrawerPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-ui desktop demo popover content class: {}",
    popover_content_class("w-64")
  );
  println!(
    "dioxus-ui desktop demo popover header class: {}",
    popover_header_class("")
  );
  println!(
    "dioxus-ui desktop demo popover title class: {}",
    popover_title_class("")
  );
  println!(
    "dioxus-ui desktop demo popover description class: {}",
    popover_description_class("")
  );
  println!(
    "dioxus-ui desktop demo popover primitive open: {}",
    PopoverPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-ui desktop demo hover card content class: {}",
    hover_card_content_class("w-72")
  );
  println!(
    "dioxus-ui desktop demo hover card side/align: {}/{}",
    hover_card_side_attribute(HoverCardSide::Right),
    hover_card_align_attribute(HoverCardAlign::Start)
  );
  println!(
    "dioxus-ui desktop demo hover card primitive open: {}",
    HoverCardPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-ui desktop demo tooltip content class: {}",
    tooltip_content_class("")
  );
  println!(
    "dioxus-ui desktop demo tooltip primitive delay: {}",
    TooltipPrimitiveConfig::controlled(false).delay_ms
  );
  println!(
    "dioxus-ui desktop demo select trigger class: {}",
    select_trigger_class(false, "w-40")
  );
  println!("dioxus-ui desktop demo select value class: {}", select_value_class(""));
  println!("dioxus-ui desktop demo select content class: {}", select_content_class(""));
  println!("dioxus-ui desktop demo select label class: {}", select_label_class(""));
  println!(
    "dioxus-ui desktop demo select item class: {}",
    select_item_class(false, "")
  );
  println!(
    "dioxus-ui desktop demo select separator class: {}",
    select_separator_class("")
  );
  println!(
    "dioxus-ui desktop demo select primitive value: {:?}",
    SelectPrimitiveConfig::controlled(false, None).value
  );
  println!(
    "dioxus-ui desktop demo native select class: {}",
    native_select_class(true, "w-40")
  );
  println!(
    "dioxus-ui desktop demo native select group class: {}",
    native_select_group_class("text-xs")
  );
  println!(
    "dioxus-ui desktop demo native select option class: {}",
    native_select_option_class("")
  );
  println!(
    "dioxus-ui desktop demo sheet overlay class: {}",
    sheet_overlay_class("")
  );
  println!(
    "dioxus-ui desktop demo sheet content class: {}",
    sheet_content_class(SheetSide::Left, "w-72")
  );
  println!(
    "dioxus-ui desktop demo sheet primitive open: {}",
    SheetPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-ui desktop demo dropdown content class: {}",
    dropdown_content_class("")
  );
  println!(
    "dioxus-ui desktop demo dropdown label class: {}",
    dropdown_label_class("")
  );
  println!(
    "dioxus-ui desktop demo dropdown item class: {}",
    dropdown_item_class(true, "")
  );
  println!(
    "dioxus-ui desktop demo dropdown separator class: {}",
    dropdown_separator_class("")
  );
  println!(
    "dioxus-ui desktop demo dropdown primitive open: {}",
    DropdownPrimitiveConfig::controlled(false).open
  );
}
