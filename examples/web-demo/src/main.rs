use dioxus_ui::{
  accordion_content_class, accordion_item_class, accordion_trigger_class,
  alert_dialog_action_class, alert_dialog_content_class, alert_dialog_overlay_class, alert_class,
  alert_description_class, alert_title_class, avatar_class, avatar_fallback_class,
  avatar_image_class, badge_class, button_class, card_class, card_content_class,
  card_description_class, card_footer_class, card_header_class, card_title_class, checkbox_class,
  dialog_content_class, dialog_overlay_class, drawer_content_class, drawer_overlay_class,
  dropdown_content_class, dropdown_item_class, dropdown_label_class, dropdown_separator_class,
  input_class, label_class, pagination_class, pagination_link_class, popover_content_class,
  popover_description_class, popover_header_class, popover_title_class, progress_class,
  progress_indicator_class, progress_percent, radio_group_class, radio_group_item_class,
  radio_group_move_value, select_content_class, select_item_class, select_label_class,
  select_separator_class, select_trigger_class, select_value_class, separator_class,
  sheet_content_class, sheet_overlay_class, skeleton_class, slider_percent, slider_range_style,
  slider_root_class, slider_thumb_style, slider_track_class, switch_class, spinner_class,
  switch_thumb_class, table_class, table_row_class, tabs_content_class, tabs_list_class,
  tabs_trigger_class, textarea_class, toggle_class, toggle_group_class, toggle_group_item_class,
  toggle_group_move_value,
  toggle_group_multiple_selection, tooltip_content_class,
  AlertDialogActionVariant, AlertDialogPrimitiveConfig, AlertVariant, BadgeVariant, ButtonSize,
  ButtonVariant, DialogPrimitiveConfig, DrawerPrimitiveConfig, DropdownPrimitiveConfig,
  FocusMove, NavigationOrientation, PopoverPrimitiveConfig, RovingFocusItem,
  SelectPrimitiveConfig, SeparatorOrientation, SheetPrimitiveConfig, SheetSide, SpinnerSize,
  ToggleSize, ToggleVariant, TooltipPrimitiveConfig, UiDensity,
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
