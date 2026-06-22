use dioxus_ui::{
  accordion_content_class, accordion_item_class, accordion_trigger_class, alert_class,
  alert_description_class, alert_title_class, avatar_class, avatar_fallback_class,
  avatar_image_class, badge_class, button_class, card_class, card_content_class,
  card_description_class, card_footer_class, card_header_class, card_title_class, checkbox_class,
  dialog_content_class, dialog_overlay_class, dropdown_content_class, dropdown_item_class,
  dropdown_label_class, dropdown_separator_class, input_class, label_class, pagination_class,
  pagination_link_class, popover_content_class, popover_description_class, popover_header_class,
  popover_title_class, progress_class, progress_indicator_class, progress_percent,
  select_content_class, select_item_class, select_label_class, select_separator_class,
  select_trigger_class, select_value_class, separator_class, skeleton_class, switch_class,
  spinner_class, switch_thumb_class, table_class, table_row_class, tabs_content_class,
  tabs_list_class, tabs_trigger_class, textarea_class, toggle_class, tooltip_content_class,
  AlertVariant, BadgeVariant, ButtonSize, ButtonVariant, DialogPrimitiveConfig,
  DropdownPrimitiveConfig, PopoverPrimitiveConfig, SelectPrimitiveConfig, SeparatorOrientation,
  SpinnerSize, ToggleSize, ToggleVariant, TooltipPrimitiveConfig, UiDensity,
};

fn main() {
  let class = button_class(
    ButtonVariant::Secondary,
    ButtonSize::Sm,
    UiDensity::Compact,
    "justify-start",
  );

  println!("dioxus-ui desktop demo button class: {class}");
  println!("dioxus-ui desktop demo input class: {}", input_class(false, "h-8"));
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
