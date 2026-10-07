use dioxus_shadcn::{
  AlertDialogActionVariant, AlertDialogPrimitiveConfig, AlertVariant, AttachmentMediaVariant,
  AttachmentOrientation, AttachmentSize, AttachmentState, BadgeVariant, BubbleAlign,
  BubbleReactionAlign, BubbleReactionSide, BubbleVariant, ButtonGroupOrientation, ButtonSize,
  ButtonVariant, CalendarDate, CalendarKeyMove, CalendarMonth, CalendarWeekday, CarouselState,
  ComboboxPrimitiveConfig, ContextMenuPrimitiveConfig, DatePickerPrimitiveConfig,
  DialogPrimitiveConfig, DrawerPrimitiveConfig, DropdownPrimitiveConfig, FocusMove,
  HoverCardPrimitiveConfig, InputGroupAddonPosition, InputOtpInputMode, KbdSize, MarkerVariant,
  MenubarPrimitiveConfig, MessageAlign, MessageScrollerIntent, NavigationMenuPrimitiveConfig,
  NavigationOrientation, PopoverPrimitiveConfig, ResizablePanelState, RovingFocusItem,
  ScrollAreaOrientation, SelectPrimitiveConfig, SeparatorOrientation, SheetPrimitiveConfig,
  SheetSide, SonnerItem, SonnerQueue, SpinnerSize, TextDirection, ToastItem, ToastQueue,
  ToggleSize, ToggleVariant, TooltipPrimitiveConfig, UiDensity, accordion_content_class,
  accordion_item_class, accordion_trigger_class, alert_class, alert_description_class,
  alert_dialog_action_class, alert_dialog_content_class, alert_dialog_overlay_class,
  alert_title_class, aspect_ratio_style, attachment_action_class, attachment_actions_class,
  attachment_class, attachment_content_class, attachment_description_class, attachment_group_class,
  attachment_media_class, attachment_title_class, attachment_trigger_class, avatar_class,
  avatar_fallback_class, avatar_image_class, badge_class, breadcrumb_link_class,
  breadcrumb_list_class, bubble_class, bubble_content_class, bubble_group_class,
  bubble_reactions_class, button_class, button_group_class, button_group_item_class,
  calendar_month_grid, calendar_move_date, card_class, carousel_can_go_next, carousel_next,
  checkbox_class, collapsible_class, collapsible_content_class, collapsible_trigger_class,
  combobox_input_class, combobox_item_class, combobox_trigger_class, command_class,
  command_input_class, command_item_class, context_menu_content_class, context_menu_item_class,
  data_table_page_window, data_table_toggle_row, date_picker_content_class,
  date_picker_trigger_class, date_picker_value_class, dialog_content_class, dialog_overlay_class,
  direction_class, drawer_content_class, drawer_overlay_class, dropdown_content_class,
  dropdown_item_class, empty_actions_class, empty_class, empty_title_class, field_class,
  field_error_class, field_group_class, hover_card_content_class, input_class,
  input_group_action_class, input_group_addon_class, input_group_class, input_group_control_class,
  input_otp_class, input_otp_group_class, input_otp_hidden_input_class, input_otp_separator_class,
  input_otp_slot_class, item_class, item_description_class, item_title_class, kbd_class,
  label_class, marker_class, marker_content_class, marker_icon_class, menubar_class,
  menubar_item_class, menubar_trigger_class, message_avatar_class, message_class,
  message_content_class, message_footer_class, message_group_class, message_header_class,
  message_scroller_bottom_anchor_class, message_scroller_class, message_scroller_content_class,
  message_scroller_show_unread_marker, message_scroller_unread_marker_class,
  message_scroller_viewport_class, native_select_class, native_select_group_class,
  native_select_option_class, navigation_menu_class, navigation_menu_link_class,
  navigation_menu_trigger_class, otp_apply_paste_filtered, otp_slots, pagination_link_class,
  popover_content_class, progress_class, progress_indicator_class, radio_group_class,
  radio_group_item_class, radio_group_move_value, resizable_resize_pair,
  scroll_area_orientation_attribute, select_item_class, select_trigger_class, separator_class,
  sheet_content_class, sheet_overlay_class, skeleton_class, sonner_queue_push, spinner_class,
  switch_class, switch_thumb_class, table_class, tabs_content_class, tabs_list_class,
  tabs_trigger_class, textarea_class, toast_is_expired, toast_queue_push, toggle_class,
  toggle_group_class, toggle_group_item_class, tooltip_content_class,
};

fn main() {
  for line in
    dioxus_ui_preview_states::preview_smoke_lines(dioxus_ui_preview_states::PreviewTarget::Web)
  {
    println!("{line}");
  }

  let class =
    button_class(ButtonVariant::Primary, ButtonSize::Md, UiDensity::Comfortable, "w-full");

  println!("dioxus-shadcn web demo button class: {class}");
  println!(
    "dioxus-shadcn web demo button group class: {}",
    button_group_class(ButtonGroupOrientation::Horizontal, true, "w-full")
  );
  println!(
    "dioxus-shadcn web demo button group item class: {}",
    button_group_item_class("min-w-20")
  );
  println!(
    "dioxus-shadcn web demo attachment class: {}",
    attachment_class(
      AttachmentState::Uploading,
      AttachmentSize::Default,
      AttachmentOrientation::Horizontal,
      "max-w-md"
    )
  );
  println!(
    "dioxus-shadcn web demo attachment parts: {}/{}/{}/{}",
    attachment_group_class("pb-1"),
    attachment_media_class(AttachmentMediaVariant::Icon, ""),
    attachment_content_class(""),
    attachment_title_class("")
  );
  println!(
    "dioxus-shadcn web demo attachment actions: {}/{}/{}",
    attachment_description_class("text-blue-700"),
    attachment_actions_class("ml-auto"),
    attachment_action_class("text-blue-600")
  );
  println!(
    "dioxus-shadcn web demo attachment trigger/state: {}/{}",
    attachment_trigger_class("w-full"),
    AttachmentState::Uploading.attribute()
  );
  println!("dioxus-shadcn web demo bubble class: {}", bubble_class(BubbleAlign::End, "max-w-lg"));
  println!(
    "dioxus-shadcn web demo bubble content class: {}",
    bubble_content_class(BubbleVariant::Tinted, "rounded-xl")
  );
  println!(
    "dioxus-shadcn web demo bubble reactions class: {}",
    bubble_reactions_class(BubbleReactionSide::Bottom, BubbleReactionAlign::End, "opacity-90")
  );
  println!(
    "dioxus-shadcn web demo bubble group/variant: {}/{}",
    bubble_group_class("gap-3"),
    BubbleVariant::Tinted.attribute()
  );
  println!(
    "dioxus-shadcn web demo message class: {}",
    message_class(MessageAlign::End, "max-w-2xl")
  );
  println!(
    "dioxus-shadcn web demo message parts: {}/{}/{}/{}",
    message_group_class("gap-5"),
    message_avatar_class("bg-blue-100"),
    message_content_class(MessageAlign::End, "gap-2"),
    message_header_class("justify-end")
  );
  println!(
    "dioxus-shadcn web demo message footer/align: {}/{}",
    message_footer_class("justify-end"),
    MessageAlign::End.attribute()
  );
  let web_unread_visible = message_scroller_show_unread_marker(MessageScrollerIntent::Hold, 2);
  println!("dioxus-shadcn web demo message scroller class: {}", message_scroller_class("h-96"));
  println!(
    "dioxus-shadcn web demo message scroller parts: {}/{}/{}/{}",
    message_scroller_viewport_class("px-2"),
    message_scroller_content_class("gap-5"),
    message_scroller_bottom_anchor_class("scroll-mb-8"),
    message_scroller_unread_marker_class(web_unread_visible, "bottom-6")
  );
  println!(
    "dioxus-shadcn web demo marker class: {}",
    marker_class(MarkerVariant::Border, "text-blue-700")
  );
  println!(
    "dioxus-shadcn web demo marker parts: {}/{}/{}",
    marker_icon_class("text-blue-600"),
    marker_content_class("font-medium"),
    MarkerVariant::Border.attribute()
  );
  println!("dioxus-shadcn web demo input class: {}", input_class(false, "mt-2"));
  println!(
    "dioxus-shadcn web demo input group class: {}",
    input_group_class(false, false, "max-w-sm")
  );
  println!(
    "dioxus-shadcn web demo input group addon class: {}",
    input_group_addon_class(InputGroupAddonPosition::Start, "")
  );
  println!("dioxus-shadcn web demo input group control class: {}", input_group_control_class(""));
  println!(
    "dioxus-shadcn web demo input group action class: {}",
    input_group_action_class("text-blue-600")
  );
  println!("dioxus-shadcn web demo input otp class: {}", input_otp_class(false, "max-w-xs"));
  println!("dioxus-shadcn web demo input otp group class: {}", input_otp_group_class("gap-2"));
  println!(
    "dioxus-shadcn web demo input otp slot class: {}",
    input_otp_slot_class(true, false, false, "h-12 w-12")
  );
  println!(
    "dioxus-shadcn web demo input otp separator/input: {}/{}",
    input_otp_separator_class("text-blue-600"),
    input_otp_hidden_input_class("")
  );
  let web_otp = otp_apply_paste_filtered("", 0, "12a3", 6, |ch| ch.is_ascii_digit());
  println!(
    "dioxus-shadcn web demo input otp helper: {}/{}",
    web_otp,
    otp_slots(&web_otp, 6, 3).len()
  );
  println!("dioxus-shadcn web demo input otp mode: {}", InputOtpInputMode::Numeric.attribute());
  println!("dioxus-shadcn web demo textarea class: {}", textarea_class(false, "mt-2"));
  println!("dioxus-shadcn web demo label class: {}", label_class("mb-2"));
  println!("dioxus-shadcn web demo alert class: {}", alert_class(AlertVariant::Default, "mb-4"));
  println!("dioxus-shadcn web demo alert title class: {}", alert_title_class(""));
  println!(
    "dioxus-shadcn web demo alert description class: {}",
    alert_description_class(AlertVariant::Default, "")
  );
  println!("dioxus-shadcn web demo alert dialog overlay class: {}", alert_dialog_overlay_class(""));
  println!(
    "dioxus-shadcn web demo alert dialog content class: {}",
    alert_dialog_content_class("max-w-md")
  );
  println!(
    "dioxus-shadcn web demo alert dialog action class: {}",
    alert_dialog_action_class(AlertDialogActionVariant::Destructive, "")
  );
  println!(
    "dioxus-shadcn web demo alert dialog primitive open: {}",
    AlertDialogPrimitiveConfig::controlled(true).open
  );
  println!(
    "dioxus-shadcn web demo direction class/attr: {}/{}",
    direction_class("block"),
    TextDirection::Rtl.attribute()
  );
  println!("dioxus-shadcn web demo aspect ratio style: {}", aspect_ratio_style(16.0 / 9.0));
  println!("dioxus-shadcn web demo avatar class: {}", avatar_class("h-12 w-12"));
  println!("dioxus-shadcn web demo avatar image class: {}", avatar_image_class(""));
  println!(
    "dioxus-shadcn web demo avatar fallback class: {}",
    avatar_fallback_class("bg-blue-100")
  );
  println!("dioxus-shadcn web demo badge class: {}", badge_class(BadgeVariant::Default, ""));
  println!("dioxus-shadcn web demo breadcrumb list class: {}", breadcrumb_list_class(""));
  println!(
    "dioxus-shadcn web demo breadcrumb current link class: {}",
    breadcrumb_link_class(true, "")
  );
  println!("dioxus-shadcn web demo empty class: {}", empty_class("min-h-56"));
  println!("dioxus-shadcn web demo empty title class: {}", empty_title_class(""));
  println!("dioxus-shadcn web demo empty actions class: {}", empty_actions_class(""));
  println!("dioxus-shadcn web demo field class: {}", field_class(true, "max-w-sm"));
  println!("dioxus-shadcn web demo field error class: {}", field_error_class(""));
  println!("dioxus-shadcn web demo field group class: {}", field_group_class(""));
  println!("dioxus-shadcn web demo item class: {}", item_class(true, false, ""));
  println!("dioxus-shadcn web demo item title class: {}", item_title_class(""));
  println!("dioxus-shadcn web demo item description class: {}", item_description_class(""));
  println!("dioxus-shadcn web demo kbd class: {}", kbd_class(KbdSize::Md, ""));
  println!(
    "dioxus-shadcn web demo calendar grid first day: {:?}",
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
    "dioxus-shadcn web demo calendar moved date: {:?}",
    calendar_move_date(
      CalendarDate::unchecked(2024, 6, 5),
      CalendarKeyMove::NextWeek,
      CalendarWeekday::Sunday,
    )
  );
  println!(
    "dioxus-shadcn web demo carousel next/can: {}/{}",
    carousel_next(0, 3, false),
    carousel_can_go_next(
      CarouselState::new(0, 3).index,
      CarouselState::new(0, 3).item_count,
      false,
    )
  );
  println!("dioxus-shadcn web demo card class: {}", card_class("max-w-sm"));
  println!(
    "dioxus-shadcn web demo pagination link class: {}",
    pagination_link_class(true, false, "")
  );
  println!("dioxus-shadcn web demo progress class: {}", progress_class("h-2"));
  println!("dioxus-shadcn web demo progress indicator class: {}", progress_indicator_class(""));
  println!(
    "dioxus-shadcn web demo toast queue/expired: {}/{}",
    toast_queue_push(ToastQueue::new(2), ToastItem::new("saved", "Saved")).items.len(),
    toast_is_expired(5000, 5000)
  );
  println!(
    "dioxus-shadcn web demo sonner queue: {}",
    sonner_queue_push(SonnerQueue::new(2), SonnerItem::new("synced", "Synced")).items.len()
  );
  println!(
    "dioxus-shadcn web demo radio group class: {}",
    radio_group_class(NavigationOrientation::Horizontal, "gap-3")
  );
  println!("dioxus-shadcn web demo radio group item class: {}", radio_group_item_class(true, ""));
  println!(
    "dioxus-shadcn web demo radio group next value: {:?}",
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
    "dioxus-shadcn web demo toggle group class: {}",
    toggle_group_class(NavigationOrientation::Horizontal, "gap-1")
  );
  println!("dioxus-shadcn web demo toggle group item class: {}", toggle_group_item_class(true, ""));
  println!(
    "dioxus-shadcn web demo separator class: {}",
    separator_class(SeparatorOrientation::Horizontal, "my-4")
  );
  println!("dioxus-shadcn web demo skeleton class: {}", skeleton_class("h-4 w-32"));
  println!(
    "dioxus-shadcn web demo spinner class: {}",
    spinner_class(SpinnerSize::Md, "text-blue-600")
  );
  println!("dioxus-shadcn web demo table class: {}", table_class("min-w-lg"));
  println!("dioxus-shadcn web demo checkbox class: {}", checkbox_class(true, "mt-2"));
  println!("dioxus-shadcn web demo collapsible class: {}", collapsible_class(false, "max-w-sm"));
  println!(
    "dioxus-shadcn web demo collapsible trigger class: {}",
    collapsible_trigger_class("w-full")
  );
  println!(
    "dioxus-shadcn web demo collapsible content class: {}",
    collapsible_content_class(true, "pt-2")
  );
  println!("dioxus-shadcn web demo command class: {}", command_class("max-w-md"));
  println!("dioxus-shadcn web demo command input class: {}", command_input_class(""));
  println!("dioxus-shadcn web demo command item class: {}", command_item_class(""));
  println!(
    "dioxus-shadcn web demo combobox trigger class: {}",
    combobox_trigger_class(false, "w-64")
  );
  println!("dioxus-shadcn web demo combobox input class: {}", combobox_input_class(""));
  println!("dioxus-shadcn web demo combobox item class: {}", combobox_item_class(true, false, ""));
  println!(
    "dioxus-shadcn web demo combobox primitive open: {}",
    ComboboxPrimitiveConfig::controlled(true).open
  );
  println!(
    "dioxus-shadcn web demo context menu content class: {}",
    context_menu_content_class("min-w-48")
  );
  println!(
    "dioxus-shadcn web demo context menu item class: {}",
    context_menu_item_class(true, false, "")
  );
  println!(
    "dioxus-shadcn web demo context menu primitive open: {}",
    ContextMenuPrimitiveConfig::controlled(true).open
  );
  println!(
    "dioxus-shadcn web demo data table page window: {:?}",
    data_table_page_window(1, 10, 24)
  );
  println!(
    "dioxus-shadcn web demo data table selected rows: {:?}",
    data_table_toggle_row(&["row-1".to_string()], "row-2")
  );
  println!(
    "dioxus-shadcn web demo scroll area orientation: {}",
    scroll_area_orientation_attribute(ScrollAreaOrientation::Both)
  );
  println!(
    "dioxus-shadcn web demo resizable resize: {:?}",
    resizable_resize_pair(
      ResizablePanelState::new(50.0, 20.0, 80.0),
      ResizablePanelState::new(50.0, 20.0, 80.0),
      10.0,
    )
  );
  println!(
    "dioxus-shadcn web demo date picker trigger class: {}",
    date_picker_trigger_class(false, "w-64")
  );
  println!("dioxus-shadcn web demo date picker value class: {}", date_picker_value_class(""));
  println!(
    "dioxus-shadcn web demo date picker content class: {}",
    date_picker_content_class("p-3")
  );
  println!(
    "dioxus-shadcn web demo date picker primitive open: {}",
    DatePickerPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-shadcn web demo menubar class: {}", menubar_class("w-fit"));
  println!("dioxus-shadcn web demo menubar trigger class: {}", menubar_trigger_class(true, ""));
  println!("dioxus-shadcn web demo menubar item class: {}", menubar_item_class(true, false, ""));
  println!(
    "dioxus-shadcn web demo menubar primitive open: {}",
    MenubarPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-shadcn web demo navigation menu class: {}", navigation_menu_class("w-full"));
  println!(
    "dioxus-shadcn web demo navigation menu trigger class: {}",
    navigation_menu_trigger_class(true, "")
  );
  println!(
    "dioxus-shadcn web demo navigation menu link class: {}",
    navigation_menu_link_class(true, "")
  );
  println!(
    "dioxus-shadcn web demo navigation menu primitive open: {}",
    NavigationMenuPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-shadcn web demo switch class: {}", switch_class(true, "mt-2"));
  println!("dioxus-shadcn web demo switch thumb class: {}", switch_thumb_class(true));
  println!("dioxus-shadcn web demo tabs list class: {}", tabs_list_class("mt-4"));
  println!("dioxus-shadcn web demo tabs trigger class: {}", tabs_trigger_class(true, "min-w-24"));
  println!("dioxus-shadcn web demo tabs content class: {}", tabs_content_class("p-4"));
  println!(
    "dioxus-shadcn web demo toggle class: {}",
    toggle_class(ToggleVariant::Default, ToggleSize::Md, true, "")
  );
  println!("dioxus-shadcn web demo accordion item class: {}", accordion_item_class(""));
  println!("dioxus-shadcn web demo accordion trigger class: {}", accordion_trigger_class(""));
  println!("dioxus-shadcn web demo accordion content class: {}", accordion_content_class("px-1"));
  println!("dioxus-shadcn web demo dialog overlay class: {}", dialog_overlay_class(""));
  println!("dioxus-shadcn web demo dialog content class: {}", dialog_content_class("max-w-xl"));
  println!(
    "dioxus-shadcn web demo dialog primitive open: {}",
    DialogPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-shadcn web demo drawer overlay class: {}", drawer_overlay_class(""));
  println!("dioxus-shadcn web demo drawer content class: {}", drawer_content_class("max-h-[70vh]"));
  println!(
    "dioxus-shadcn web demo drawer primitive open: {}",
    DrawerPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-shadcn web demo popover content class: {}", popover_content_class("w-80"));
  println!(
    "dioxus-shadcn web demo popover primitive open: {}",
    PopoverPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-shadcn web demo hover card content class: {}", hover_card_content_class("w-96"));
  println!(
    "dioxus-shadcn web demo hover card primitive open: {}",
    HoverCardPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-shadcn web demo tooltip content class: {}", tooltip_content_class(""));
  println!(
    "dioxus-shadcn web demo tooltip primitive delay: {}",
    TooltipPrimitiveConfig::controlled(true).delay_ms
  );
  println!("dioxus-shadcn web demo select trigger class: {}", select_trigger_class(false, "w-44"));
  println!("dioxus-shadcn web demo select item class: {}", select_item_class(true, ""));
  println!(
    "dioxus-shadcn web demo select primitive value: {:?}",
    SelectPrimitiveConfig::controlled(true, Some("system".to_string())).value
  );
  println!("dioxus-shadcn web demo native select class: {}", native_select_class(false, "w-48"));
  println!("dioxus-shadcn web demo native select group class: {}", native_select_group_class(""));
  println!("dioxus-shadcn web demo native select option class: {}", native_select_option_class(""));
  println!("dioxus-shadcn web demo sheet overlay class: {}", sheet_overlay_class(""));
  println!(
    "dioxus-shadcn web demo sheet content class: {}",
    sheet_content_class(SheetSide::Right, "w-80")
  );
  println!(
    "dioxus-shadcn web demo sheet primitive open: {}",
    SheetPrimitiveConfig::controlled(true).open
  );
  println!("dioxus-shadcn web demo dropdown content class: {}", dropdown_content_class(""));
  println!("dioxus-shadcn web demo dropdown item class: {}", dropdown_item_class(false, ""));
  println!(
    "dioxus-shadcn web demo dropdown primitive open: {}",
    DropdownPrimitiveConfig::controlled(true).open
  );
}
