use dioxus_shadcn::{
  AlertDialogActionVariant, AlertDialogPrimitiveConfig, AlertVariant, AttachmentMediaVariant,
  AttachmentOrientation, AttachmentSize, AttachmentState, BadgeVariant, BubbleAlign,
  BubbleReactionAlign, BubbleReactionSide, BubbleVariant, ButtonGroupOrientation, ButtonSize,
  ButtonVariant, CalendarDate, CalendarKeyMove, CalendarMonth, CalendarRangeState, CalendarWeekday,
  CarouselOrientation, CarouselState, ChartColorToken, ChartDomain, ChartPoint, ChartScale,
  ChartSeries, ComboboxPrimitiveConfig, ContextMenuPrimitiveConfig, DataTableSortDirection,
  DatePickerAlign, DatePickerPrimitiveConfig, DatePickerSide, DialogPrimitiveConfig,
  DrawerPrimitiveConfig, DropdownPrimitiveConfig, FocusMove, HoverCardAlign,
  HoverCardPrimitiveConfig, HoverCardSide, InputGroupAddonPosition, InputOtpInputMode, KbdSize,
  LayoutOrientation, MarkerVariant, MenubarPrimitiveConfig, MessageAlign, MessageScrollerIntent,
  MessageScrollerMetrics, NavigationMenuPrimitiveConfig, NavigationOrientation,
  PopoverPrimitiveConfig, ResizablePanelState, RovingFocusItem, ScrollAreaOrientation,
  SelectPrimitiveConfig, SeparatorOrientation, SheetPrimitiveConfig, SheetSide, SidebarSide,
  SidebarState, SliderOrientation, SonnerItem, SonnerPlacement, SonnerQueue, SonnerVariant,
  SpinnerSize, TextDirection, ToastItem, ToastPlacement, ToastQueue, ToastVariant, ToggleSize,
  ToggleVariant, TooltipPrimitiveConfig, UiDensity, accordion_content_class, accordion_item_class,
  accordion_trigger_class, alert_class, alert_description_class, alert_dialog_action_class,
  alert_dialog_content_class, alert_dialog_overlay_class, alert_title_class, aspect_ratio_style,
  attachment_action_class, attachment_actions_class, attachment_class, attachment_content_class,
  attachment_description_class, attachment_group_class, attachment_media_class,
  attachment_title_class, attachment_trigger_class, avatar_class, avatar_fallback_class,
  avatar_image_class, badge_class, breadcrumb_link_class, breadcrumb_list_class, bubble_class,
  bubble_content_class, bubble_group_class, bubble_reactions_class, button_class,
  button_group_class, button_group_item_class, calendar_day_class, calendar_month_grid,
  calendar_move_date, card_class, card_content_class, card_description_class, card_footer_class,
  card_header_class, card_title_class, carousel_can_go_previous, carousel_content_class,
  carousel_control_class, carousel_indicator_class, carousel_item_class, carousel_previous,
  chart_area_series_class, chart_bar_rects, chart_bar_series_class, chart_class,
  chart_fallback_rows, chart_line_path, chart_line_series_class, chart_view_box, checkbox_class,
  collapsible_class, collapsible_content_class, collapsible_trigger_class, combobox_input_class,
  combobox_item_class, combobox_trigger_class, command_active_descendant_state, command_class,
  command_input_class, command_item_class, context_menu_content_class, context_menu_item_class,
  context_menu_shortcut_class, data_table_header_cell_class, data_table_page_window,
  data_table_row_class, data_table_sort_attribute, data_table_toggle_row,
  date_picker_align_attribute, date_picker_content_class, date_picker_side_attribute,
  date_picker_trigger_class, date_picker_value_class, dialog_content_class, dialog_overlay_class,
  direction_class, drawer_content_class, drawer_overlay_class, dropdown_content_class,
  dropdown_item_class, dropdown_label_class, dropdown_separator_class, empty_actions_class,
  empty_class, empty_title_class, field_class, field_error_class, field_group_class,
  hover_card_align_attribute, hover_card_content_class, hover_card_side_attribute, input_class,
  input_group_action_class, input_group_addon_class, input_group_class, input_group_control_class,
  input_otp_class, input_otp_group_class, input_otp_hidden_input_class, input_otp_separator_class,
  input_otp_slot_class, item_class, item_description_class, item_title_class, kbd_class,
  label_class, marker_class, marker_content_class, marker_icon_class, menubar_class,
  menubar_item_class, menubar_trigger_class, message_avatar_class, message_class,
  message_content_class, message_footer_class, message_group_class, message_header_class,
  message_scroller_bottom_anchor_class, message_scroller_class, message_scroller_content_class,
  message_scroller_intent_attribute, message_scroller_is_at_bottom,
  message_scroller_jump_button_class, message_scroller_show_unread_marker,
  message_scroller_unread_marker_class, message_scroller_viewport_class, native_select_class,
  native_select_group_class, native_select_option_class, navigation_menu_class,
  navigation_menu_link_class, navigation_menu_trigger_class, otp_apply_paste_filtered, otp_slots,
  pagination_class, pagination_link_class, popover_content_class, popover_description_class,
  popover_header_class, popover_title_class, progress_class, progress_indicator_class,
  progress_percent, radio_group_class, radio_group_item_class, radio_group_move_value,
  resizable_handle_class, resizable_panel_group_class, resizable_panel_style,
  resizable_resize_pair, scroll_area_class, scroll_area_orientation_attribute,
  scroll_area_scrollbar_class, scroll_area_thumb_class, scroll_area_viewport_class,
  select_content_class, select_item_class, select_label_class, select_separator_class,
  select_trigger_class, select_value_class, separator_class, sheet_content_class,
  sheet_overlay_class, sidebar_class, sidebar_item_class, sidebar_side_attribute, sidebar_toggle,
  sidebar_trigger_class, skeleton_class, slider_percent, slider_range_style, slider_root_class,
  slider_thumb_style, slider_track_class, sonner_icon_class, sonner_queue_push, sonner_toast_class,
  sonner_viewport_class, spinner_class, switch_class, switch_thumb_class, table_class,
  table_row_class, tabs_content_class, tabs_list_class, tabs_trigger_class, textarea_class,
  toast_action_class, toast_close_class, toast_is_expired, toast_queue_push, toast_root_class,
  toast_viewport_class, toggle_class, toggle_group_class, toggle_group_item_class,
  toggle_group_move_value, toggle_group_single_selection, tooltip_content_class,
  typography_h2_class, typography_inline_code_class, typography_p_class,
};

fn main() {
  for line in
    dioxus_ui_preview_states::preview_smoke_lines(dioxus_ui_preview_states::PreviewTarget::Desktop)
  {
    println!("{line}");
  }

  let class =
    button_class(ButtonVariant::Secondary, ButtonSize::Sm, UiDensity::Compact, "justify-start");

  println!("dioxus-shadcn desktop demo button class: {class}");
  println!(
    "dioxus-shadcn desktop demo button group class: {}",
    button_group_class(ButtonGroupOrientation::Vertical, false, "items-start")
  );
  println!(
    "dioxus-shadcn desktop demo button group item class: {}",
    button_group_item_class("min-w-16")
  );
  println!(
    "dioxus-shadcn desktop demo attachment class: {}",
    attachment_class(
      AttachmentState::Error,
      AttachmentSize::Sm,
      AttachmentOrientation::Vertical,
      "max-w-xs"
    )
  );
  println!(
    "dioxus-shadcn desktop demo attachment parts: {}/{}/{}/{}",
    attachment_group_class("pb-2"),
    attachment_media_class(AttachmentMediaVariant::Image, "rounded-none"),
    attachment_content_class("gap-0"),
    attachment_title_class("text-sm")
  );
  println!(
    "dioxus-shadcn desktop demo attachment actions: {}/{}/{}",
    attachment_description_class("text-red-700"),
    attachment_actions_class("justify-end"),
    attachment_action_class("text-red-600")
  );
  println!(
    "dioxus-shadcn desktop demo attachment trigger/state: {}/{}",
    attachment_trigger_class("w-full"),
    AttachmentState::Error.attribute()
  );
  println!(
    "dioxus-shadcn desktop demo bubble class: {}",
    bubble_class(BubbleAlign::Start, "max-w-sm")
  );
  println!(
    "dioxus-shadcn desktop demo bubble content class: {}",
    bubble_content_class(BubbleVariant::Destructive, "rounded-lg")
  );
  println!(
    "dioxus-shadcn desktop demo bubble reactions class: {}",
    bubble_reactions_class(BubbleReactionSide::Top, BubbleReactionAlign::Start, "opacity-80")
  );
  println!(
    "dioxus-shadcn desktop demo bubble group/variant: {}/{}",
    bubble_group_class("gap-1"),
    BubbleVariant::Destructive.attribute()
  );
  println!(
    "dioxus-shadcn desktop demo message class: {}",
    message_class(MessageAlign::Start, "max-w-xl")
  );
  println!(
    "dioxus-shadcn desktop demo message parts: {}/{}/{}/{}",
    message_group_class("gap-3"),
    message_avatar_class("bg-red-100"),
    message_content_class(MessageAlign::Start, "gap-1"),
    message_header_class("font-medium")
  );
  println!(
    "dioxus-shadcn desktop demo message footer/align: {}/{}",
    message_footer_class("justify-start"),
    MessageAlign::Start.attribute()
  );
  let desktop_message_metrics = MessageScrollerMetrics::new(500.0, 240.0, 1200.0);
  let desktop_unread_visible = message_scroller_show_unread_marker(MessageScrollerIntent::Hold, 3);
  println!(
    "dioxus-shadcn desktop demo message scroller class: {}",
    message_scroller_class(MessageScrollerIntent::JumpToLatest, "h-80")
  );
  println!(
    "dioxus-shadcn desktop demo message scroller parts: {}/{}/{}/{}",
    message_scroller_viewport_class("px-1"),
    message_scroller_content_class("gap-3"),
    message_scroller_bottom_anchor_class("scroll-mb-6"),
    message_scroller_unread_marker_class(desktop_unread_visible, "bottom-5")
  );
  println!(
    "dioxus-shadcn desktop demo message scroller helper: {}/{}/{}",
    message_scroller_is_at_bottom(desktop_message_metrics, 16.0),
    message_scroller_jump_button_class(desktop_unread_visible, "text-red-700"),
    message_scroller_intent_attribute(MessageScrollerIntent::JumpToLatest)
  );
  let desktop_chart_series = ChartSeries::new(
    "cost",
    "Cost",
    vec![
      ChartPoint::new(0.0, 8.0),
      ChartPoint::new(1.0, 11.0),
      ChartPoint::new(2.0, 10.0),
      ChartPoint::new(3.0, 13.0),
    ],
  );
  let desktop_chart_x = ChartScale::new(ChartDomain::new(0.0, 3.0), ChartDomain::new(24.0, 456.0));
  let desktop_chart_y = ChartScale::new(ChartDomain::new(0.0, 16.0), ChartDomain::new(220.0, 24.0));
  println!(
    "dioxus-shadcn desktop demo chart classes: {}/{}/{}/{}",
    chart_class("max-w-xl"),
    chart_line_series_class(ChartColorToken::Secondary, ""),
    chart_area_series_class(ChartColorToken::Secondary, "opacity-15"),
    chart_bar_series_class(ChartColorToken::Neutral, "")
  );
  println!(
    "dioxus-shadcn desktop demo chart helper: {}/{}/{}",
    chart_view_box(480.0, 240.0),
    chart_line_path(&desktop_chart_series, desktop_chart_x, desktop_chart_y),
    chart_fallback_rows(std::slice::from_ref(&desktop_chart_series)).len()
  );
  println!(
    "dioxus-shadcn desktop demo chart bars: {}",
    chart_bar_rects(&desktop_chart_series, desktop_chart_x, desktop_chart_y, 0.0, 14.0).len()
  );
  println!(
    "dioxus-shadcn desktop demo marker class: {}",
    marker_class(MarkerVariant::Separator, "text-red-700")
  );
  println!(
    "dioxus-shadcn desktop demo marker parts: {}/{}/{}",
    marker_icon_class("text-red-600"),
    marker_content_class("font-medium"),
    MarkerVariant::Separator.attribute()
  );
  println!("dioxus-shadcn desktop demo input class: {}", input_class(false, "h-8"));
  println!(
    "dioxus-shadcn desktop demo input group class: {}",
    input_group_class(true, true, "max-w-xs")
  );
  println!(
    "dioxus-shadcn desktop demo input group addon class: {}",
    input_group_addon_class(InputGroupAddonPosition::End, "text-xs")
  );
  println!(
    "dioxus-shadcn desktop demo input group control class: {}",
    input_group_control_class("min-w-32")
  );
  println!(
    "dioxus-shadcn desktop demo input group action class: {}",
    input_group_action_class("text-red-600")
  );
  println!("dioxus-shadcn desktop demo input otp class: {}", input_otp_class(true, "max-w-xs"));
  println!("dioxus-shadcn desktop demo input otp group class: {}", input_otp_group_class("gap-1"));
  println!(
    "dioxus-shadcn desktop demo input otp slot class: {}",
    input_otp_slot_class(false, true, true, "h-9 w-9")
  );
  println!(
    "dioxus-shadcn desktop demo input otp separator/input: {}/{}",
    input_otp_separator_class("text-red-600"),
    input_otp_hidden_input_class("absolute")
  );
  let desktop_otp = otp_apply_paste_filtered("1", 1, "2b34", 4, |ch| ch.is_ascii_digit());
  println!(
    "dioxus-shadcn desktop demo input otp helper: {}/{}",
    desktop_otp,
    otp_slots(&desktop_otp, 4, 3).len()
  );
  println!("dioxus-shadcn desktop demo input otp mode: {}", InputOtpInputMode::Text.attribute());
  println!("dioxus-shadcn desktop demo textarea class: {}", textarea_class(false, "min-h-20"));
  println!("dioxus-shadcn desktop demo label class: {}", label_class("text-xs"));
  println!(
    "dioxus-shadcn desktop demo alert class: {}",
    alert_class(AlertVariant::Destructive, "mb-2")
  );
  println!("dioxus-shadcn desktop demo alert title class: {}", alert_title_class("text-sm"));
  println!(
    "dioxus-shadcn desktop demo alert description class: {}",
    alert_description_class(AlertVariant::Destructive, "")
  );
  println!(
    "dioxus-shadcn desktop demo alert dialog overlay class: {}",
    alert_dialog_overlay_class("")
  );
  println!(
    "dioxus-shadcn desktop demo alert dialog content class: {}",
    alert_dialog_content_class("max-w-sm")
  );
  println!(
    "dioxus-shadcn desktop demo alert dialog action class: {}",
    alert_dialog_action_class(AlertDialogActionVariant::Default, "")
  );
  println!(
    "dioxus-shadcn desktop demo alert dialog primitive open: {}",
    AlertDialogPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-shadcn desktop demo direction class/attr: {}/{}",
    direction_class("inline"),
    TextDirection::Ltr.attribute()
  );
  println!("dioxus-shadcn desktop demo aspect ratio style: {}", aspect_ratio_style(4.0 / 3.0));
  println!("dioxus-shadcn desktop demo avatar class: {}", avatar_class("h-8 w-8"));
  println!("dioxus-shadcn desktop demo avatar image class: {}", avatar_image_class(""));
  println!(
    "dioxus-shadcn desktop demo avatar fallback class: {}",
    avatar_fallback_class("text-xs")
  );
  println!("dioxus-shadcn desktop demo badge class: {}", badge_class(BadgeVariant::Secondary, ""));
  println!("dioxus-shadcn desktop demo breadcrumb list class: {}", breadcrumb_list_class("gap-2"));
  println!(
    "dioxus-shadcn desktop demo breadcrumb current link class: {}",
    breadcrumb_link_class(true, "")
  );
  println!("dioxus-shadcn desktop demo empty class: {}", empty_class("min-h-48"));
  println!("dioxus-shadcn desktop demo empty title class: {}", empty_title_class("text-base"));
  println!(
    "dioxus-shadcn desktop demo empty actions class: {}",
    empty_actions_class("justify-end")
  );
  println!("dioxus-shadcn desktop demo field class: {}", field_class(false, "max-w-md"));
  println!("dioxus-shadcn desktop demo field error class: {}", field_error_class("text-xs"));
  println!("dioxus-shadcn desktop demo field group class: {}", field_group_class("gap-3"));
  println!("dioxus-shadcn desktop demo item class: {}", item_class(false, true, ""));
  println!("dioxus-shadcn desktop demo item title class: {}", item_title_class("text-sm"));
  println!("dioxus-shadcn desktop demo item description class: {}", item_description_class(""));
  println!("dioxus-shadcn desktop demo kbd class: {}", kbd_class(KbdSize::Sm, ""));
  println!("dioxus-shadcn desktop demo typography h2 class: {}", typography_h2_class(""));
  println!("dioxus-shadcn desktop demo typography p class: {}", typography_p_class(""));
  println!(
    "dioxus-shadcn desktop demo typography inline code class: {}",
    typography_inline_code_class("")
  );
  println!(
    "dioxus-shadcn desktop demo calendar day class: {}",
    calendar_day_class(false, true, true, false, CalendarRangeState::Middle, "")
  );
  println!(
    "dioxus-shadcn desktop demo calendar grid first day: {:?}",
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
    "dioxus-shadcn desktop demo calendar moved date: {:?}",
    calendar_move_date(
      CalendarDate::unchecked(2024, 2, 29),
      CalendarKeyMove::NextYear,
      CalendarWeekday::Monday,
    )
  );
  println!(
    "dioxus-shadcn desktop demo carousel content class: {}",
    carousel_content_class(CarouselOrientation::Vertical, "")
  );
  println!(
    "dioxus-shadcn desktop demo carousel item class: {}",
    carousel_item_class(CarouselOrientation::Vertical, false, "basis-full")
  );
  println!(
    "dioxus-shadcn desktop demo carousel previous control class: {}",
    carousel_control_class(true, "")
  );
  println!(
    "dioxus-shadcn desktop demo carousel indicator class: {}",
    carousel_indicator_class(false, "")
  );
  println!(
    "dioxus-shadcn desktop demo carousel previous/can: {}/{}",
    carousel_previous(0, 3, true),
    carousel_can_go_previous(
      CarouselState::new(0, 3).index,
      CarouselState::new(0, 3).item_count,
      true,
    )
  );
  println!("dioxus-shadcn desktop demo card class: {}", card_class("shadow-none"));
  println!("dioxus-shadcn desktop demo card header class: {}", card_header_class("p-4"));
  println!("dioxus-shadcn desktop demo card title class: {}", card_title_class("text-lg"));
  println!("dioxus-shadcn desktop demo card description class: {}", card_description_class(""));
  println!("dioxus-shadcn desktop demo card content class: {}", card_content_class("p-4 pt-0"));
  println!("dioxus-shadcn desktop demo card footer class: {}", card_footer_class("p-4 pt-0"));
  println!("dioxus-shadcn desktop demo pagination class: {}", pagination_class("mt-2"));
  println!(
    "dioxus-shadcn desktop demo pagination link class: {}",
    pagination_link_class(false, true, "")
  );
  println!("dioxus-shadcn desktop demo progress class: {}", progress_class("h-1.5"));
  println!(
    "dioxus-shadcn desktop demo progress indicator class: {}",
    progress_indicator_class("bg-zinc-900")
  );
  println!("dioxus-shadcn desktop demo progress percent: {}", progress_percent(3.0, 4.0));
  println!(
    "dioxus-shadcn desktop demo toast viewport class: {}",
    toast_viewport_class(ToastPlacement::TopRight, "")
  );
  println!(
    "dioxus-shadcn desktop demo toast root class: {}",
    toast_root_class(ToastVariant::Error, "")
  );
  println!(
    "dioxus-shadcn desktop demo toast action/close class: {}/{}",
    toast_action_class(true, ""),
    toast_close_class(false, "")
  );
  println!(
    "dioxus-shadcn desktop demo toast queue/expired: {}/{}",
    toast_queue_push(ToastQueue::new(1), ToastItem::new("failed", "Failed")).items.len(),
    toast_is_expired(1000, 5000)
  );
  println!(
    "dioxus-shadcn desktop demo sonner viewport class: {}",
    sonner_viewport_class(SonnerPlacement::TopRight, "")
  );
  println!(
    "dioxus-shadcn desktop demo sonner toast/icon class: {}/{}",
    sonner_toast_class(SonnerVariant::Error, ""),
    sonner_icon_class(SonnerVariant::Error, "")
  );
  println!(
    "dioxus-shadcn desktop demo sonner queue: {}",
    sonner_queue_push(SonnerQueue::new(1), SonnerItem::new("offline", "Offline")).items.len()
  );
  println!("dioxus-shadcn desktop demo slider class: {}", slider_root_class("mt-2"));
  println!("dioxus-shadcn desktop demo slider track class: {}", slider_track_class("h-1.5"));
  println!("dioxus-shadcn desktop demo slider percent: {}", slider_percent(3.0, 0.0, 4.0, 1.0));
  println!(
    "dioxus-shadcn desktop demo slider range style: {}",
    slider_range_style(SliderOrientation::Horizontal, 75.0)
  );
  println!(
    "dioxus-shadcn desktop demo slider thumb style: {}",
    slider_thumb_style(SliderOrientation::Horizontal, 75.0)
  );
  println!(
    "dioxus-shadcn desktop demo radio group class: {}",
    radio_group_class(NavigationOrientation::Vertical, "gap-2")
  );
  println!(
    "dioxus-shadcn desktop demo radio group item class: {}",
    radio_group_item_class(false, "")
  );
  println!(
    "dioxus-shadcn desktop demo radio group next value: {:?}",
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
    "dioxus-shadcn desktop demo toggle group class: {}",
    toggle_group_class(NavigationOrientation::Vertical, "gap-2")
  );
  println!(
    "dioxus-shadcn desktop demo toggle group item class: {}",
    toggle_group_item_class(false, "")
  );
  println!(
    "dioxus-shadcn desktop demo toggle group next value: {:?}",
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
    "dioxus-shadcn desktop demo toggle group value: {:?}",
    toggle_group_single_selection(Some("left"), "right")
  );
  println!(
    "dioxus-shadcn desktop demo separator class: {}",
    separator_class(SeparatorOrientation::Vertical, "mx-2")
  );
  println!("dioxus-shadcn desktop demo skeleton class: {}", skeleton_class("h-3 w-24"));
  println!(
    "dioxus-shadcn desktop demo spinner class: {}",
    spinner_class(SpinnerSize::Sm, "text-zinc-700")
  );
  println!("dioxus-shadcn desktop demo table class: {}", table_class("text-xs"));
  println!("dioxus-shadcn desktop demo table row class: {}", table_row_class(""));
  println!("dioxus-shadcn desktop demo checkbox class: {}", checkbox_class(false, "mt-1"));
  println!("dioxus-shadcn desktop demo collapsible class: {}", collapsible_class(true, "max-w-xs"));
  println!(
    "dioxus-shadcn desktop demo collapsible trigger class: {}",
    collapsible_trigger_class("w-full")
  );
  println!(
    "dioxus-shadcn desktop demo collapsible content class: {}",
    collapsible_content_class(false, "pt-1")
  );
  println!("dioxus-shadcn desktop demo command class: {}", command_class("max-w-sm"));
  println!("dioxus-shadcn desktop demo command input class: {}", command_input_class("h-9"));
  println!(
    "dioxus-shadcn desktop demo command item class: {}",
    command_item_class(false, true, "")
  );
  println!(
    "dioxus-shadcn desktop demo command active descendant: {:?}",
    command_active_descendant_state(Some("quick-open".to_string())).active_id
  );
  println!(
    "dioxus-shadcn desktop demo combobox trigger class: {}",
    combobox_trigger_class(true, "w-56")
  );
  println!("dioxus-shadcn desktop demo combobox input class: {}", combobox_input_class("h-9"));
  println!(
    "dioxus-shadcn desktop demo combobox item class: {}",
    combobox_item_class(false, true, "")
  );
  println!(
    "dioxus-shadcn desktop demo combobox primitive open: {}",
    ComboboxPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-shadcn desktop demo context menu content class: {}",
    context_menu_content_class("min-w-40")
  );
  println!(
    "dioxus-shadcn desktop demo context menu item class: {}",
    context_menu_item_class(false, true, "")
  );
  println!(
    "dioxus-shadcn desktop demo context menu shortcut class: {}",
    context_menu_shortcut_class("")
  );
  println!(
    "dioxus-shadcn desktop demo context menu primitive open: {}",
    ContextMenuPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-shadcn desktop demo data table header class: {}",
    data_table_header_cell_class(false, "w-32")
  );
  println!(
    "dioxus-shadcn desktop demo data table row class: {}",
    data_table_row_class(false, true, "")
  );
  println!(
    "dioxus-shadcn desktop demo data table page window: {:?}",
    data_table_page_window(5, 10, 22)
  );
  println!(
    "dioxus-shadcn desktop demo data table selected rows: {:?}",
    data_table_toggle_row(&["row-2".to_string()], "row-2")
  );
  println!(
    "dioxus-shadcn desktop demo data table sort: {}",
    data_table_sort_attribute(Some(DataTableSortDirection::Descending))
  );
  println!("dioxus-shadcn desktop demo scroll area class: {}", scroll_area_class("h-64"));
  println!(
    "dioxus-shadcn desktop demo scroll area viewport class: {}",
    scroll_area_viewport_class("")
  );
  println!(
    "dioxus-shadcn desktop demo scroll area scrollbar class: {}",
    scroll_area_scrollbar_class(ScrollAreaOrientation::Horizontal, "")
  );
  println!("dioxus-shadcn desktop demo scroll area thumb class: {}", scroll_area_thumb_class(""));
  println!(
    "dioxus-shadcn desktop demo scroll area orientation: {}",
    scroll_area_orientation_attribute(ScrollAreaOrientation::Horizontal)
  );
  println!(
    "dioxus-shadcn desktop demo resizable group class: {}",
    resizable_panel_group_class(LayoutOrientation::Vertical, "h-52")
  );
  println!(
    "dioxus-shadcn desktop demo resizable handle class: {}",
    resizable_handle_class(true, "")
  );
  println!(
    "dioxus-shadcn desktop demo resizable panel style: {}",
    resizable_panel_style(10.0, 20.0, 80.0)
  );
  println!(
    "dioxus-shadcn desktop demo resizable resize: {:?}",
    resizable_resize_pair(
      ResizablePanelState::new(60.0, 20.0, 80.0),
      ResizablePanelState::new(40.0, 20.0, 80.0),
      -15.0,
    )
  );
  println!(
    "dioxus-shadcn desktop demo sidebar class: {}",
    sidebar_class(true, SidebarSide::Right, "shrink-0")
  );
  println!(
    "dioxus-shadcn desktop demo sidebar item class: {}",
    sidebar_item_class(false, true, "")
  );
  println!("dioxus-shadcn desktop demo sidebar trigger class: {}", sidebar_trigger_class(""));
  println!(
    "dioxus-shadcn desktop demo sidebar side/toggle: {}/{}",
    sidebar_side_attribute(SidebarSide::Right),
    sidebar_toggle(SidebarState::new(true).collapsed)
  );
  println!(
    "dioxus-shadcn desktop demo date picker trigger class: {}",
    date_picker_trigger_class(true, "w-56")
  );
  println!(
    "dioxus-shadcn desktop demo date picker value class: {}",
    date_picker_value_class("text-xs")
  );
  println!(
    "dioxus-shadcn desktop demo date picker content class: {}",
    date_picker_content_class("p-2")
  );
  println!(
    "dioxus-shadcn desktop demo date picker side/align: {}/{}",
    date_picker_side_attribute(DatePickerSide::Right),
    date_picker_align_attribute(DatePickerAlign::End)
  );
  println!(
    "dioxus-shadcn desktop demo date picker primitive open: {}",
    DatePickerPrimitiveConfig::controlled(false).open
  );
  println!("dioxus-shadcn desktop demo menubar class: {}", menubar_class("w-fit"));
  println!(
    "dioxus-shadcn desktop demo menubar trigger class: {}",
    menubar_trigger_class(false, "")
  );
  println!(
    "dioxus-shadcn desktop demo menubar item class: {}",
    menubar_item_class(false, true, "")
  );
  println!(
    "dioxus-shadcn desktop demo menubar primitive open: {}",
    MenubarPrimitiveConfig::controlled(false).open
  );
  println!("dioxus-shadcn desktop demo navigation menu class: {}", navigation_menu_class("w-full"));
  println!(
    "dioxus-shadcn desktop demo navigation menu trigger class: {}",
    navigation_menu_trigger_class(false, "")
  );
  println!(
    "dioxus-shadcn desktop demo navigation menu link class: {}",
    navigation_menu_link_class(false, "")
  );
  println!(
    "dioxus-shadcn desktop demo navigation menu primitive open: {}",
    NavigationMenuPrimitiveConfig::controlled(false).open
  );
  println!("dioxus-shadcn desktop demo switch class: {}", switch_class(false, "mt-1"));
  println!("dioxus-shadcn desktop demo switch thumb class: {}", switch_thumb_class(false));
  println!("dioxus-shadcn desktop demo tabs list class: {}", tabs_list_class("mt-2"));
  println!(
    "dioxus-shadcn desktop demo tabs trigger class: {}",
    tabs_trigger_class(false, "min-w-20")
  );
  println!("dioxus-shadcn desktop demo tabs content class: {}", tabs_content_class("p-2"));
  println!(
    "dioxus-shadcn desktop demo toggle class: {}",
    toggle_class(ToggleVariant::Outline, ToggleSize::Sm, false, "")
  );
  println!("dioxus-shadcn desktop demo accordion item class: {}", accordion_item_class(""));
  println!("dioxus-shadcn desktop demo accordion trigger class: {}", accordion_trigger_class(""));
  println!(
    "dioxus-shadcn desktop demo accordion content class: {}",
    accordion_content_class("px-1")
  );
  println!("dioxus-shadcn desktop demo dialog overlay class: {}", dialog_overlay_class(""));
  println!("dioxus-shadcn desktop demo dialog content class: {}", dialog_content_class("max-w-md"));
  println!(
    "dioxus-shadcn desktop demo dialog primitive open: {}",
    DialogPrimitiveConfig::controlled(false).open
  );
  println!("dioxus-shadcn desktop demo drawer overlay class: {}", drawer_overlay_class(""));
  println!(
    "dioxus-shadcn desktop demo drawer content class: {}",
    drawer_content_class("max-h-[60vh]")
  );
  println!(
    "dioxus-shadcn desktop demo drawer primitive open: {}",
    DrawerPrimitiveConfig::controlled(false).open
  );
  println!("dioxus-shadcn desktop demo popover content class: {}", popover_content_class("w-64"));
  println!("dioxus-shadcn desktop demo popover header class: {}", popover_header_class(""));
  println!("dioxus-shadcn desktop demo popover title class: {}", popover_title_class(""));
  println!(
    "dioxus-shadcn desktop demo popover description class: {}",
    popover_description_class("")
  );
  println!(
    "dioxus-shadcn desktop demo popover primitive open: {}",
    PopoverPrimitiveConfig::controlled(false).open
  );
  println!(
    "dioxus-shadcn desktop demo hover card content class: {}",
    hover_card_content_class("w-72")
  );
  println!(
    "dioxus-shadcn desktop demo hover card side/align: {}/{}",
    hover_card_side_attribute(HoverCardSide::Right),
    hover_card_align_attribute(HoverCardAlign::Start)
  );
  println!(
    "dioxus-shadcn desktop demo hover card primitive open: {}",
    HoverCardPrimitiveConfig::controlled(false).open
  );
  println!("dioxus-shadcn desktop demo tooltip content class: {}", tooltip_content_class(""));
  println!(
    "dioxus-shadcn desktop demo tooltip primitive delay: {}",
    TooltipPrimitiveConfig::controlled(false).delay_ms
  );
  println!(
    "dioxus-shadcn desktop demo select trigger class: {}",
    select_trigger_class(false, "w-40")
  );
  println!("dioxus-shadcn desktop demo select value class: {}", select_value_class(""));
  println!("dioxus-shadcn desktop demo select content class: {}", select_content_class(""));
  println!("dioxus-shadcn desktop demo select label class: {}", select_label_class(""));
  println!("dioxus-shadcn desktop demo select item class: {}", select_item_class(false, ""));
  println!("dioxus-shadcn desktop demo select separator class: {}", select_separator_class(""));
  println!(
    "dioxus-shadcn desktop demo select primitive value: {:?}",
    SelectPrimitiveConfig::controlled(false, None).value
  );
  println!("dioxus-shadcn desktop demo native select class: {}", native_select_class(true, "w-40"));
  println!(
    "dioxus-shadcn desktop demo native select group class: {}",
    native_select_group_class("text-xs")
  );
  println!(
    "dioxus-shadcn desktop demo native select option class: {}",
    native_select_option_class("")
  );
  println!("dioxus-shadcn desktop demo sheet overlay class: {}", sheet_overlay_class(""));
  println!(
    "dioxus-shadcn desktop demo sheet content class: {}",
    sheet_content_class(SheetSide::Left, "w-72")
  );
  println!(
    "dioxus-shadcn desktop demo sheet primitive open: {}",
    SheetPrimitiveConfig::controlled(false).open
  );
  println!("dioxus-shadcn desktop demo dropdown content class: {}", dropdown_content_class(""));
  println!("dioxus-shadcn desktop demo dropdown label class: {}", dropdown_label_class(""));
  println!("dioxus-shadcn desktop demo dropdown item class: {}", dropdown_item_class(true, ""));
  println!("dioxus-shadcn desktop demo dropdown separator class: {}", dropdown_separator_class(""));
  println!(
    "dioxus-shadcn desktop demo dropdown primitive open: {}",
    DropdownPrimitiveConfig::controlled(false).open
  );
}
