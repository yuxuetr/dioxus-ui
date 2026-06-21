use dioxus_ui::{
  accordion_content_class, accordion_item_class, accordion_trigger_class, button_class,
  checkbox_class, dialog_content_class, dialog_overlay_class, dropdown_content_class,
  dropdown_item_class, dropdown_label_class, dropdown_separator_class, input_class, label_class,
  popover_content_class, popover_description_class, popover_header_class, popover_title_class,
  select_content_class, select_item_class, select_label_class, select_separator_class,
  select_trigger_class, select_value_class, switch_class, switch_thumb_class, tabs_content_class,
  tabs_list_class, tabs_trigger_class, textarea_class, tooltip_content_class, ButtonSize,
  ButtonVariant, DialogPrimitiveConfig, DropdownPrimitiveConfig, PopoverPrimitiveConfig,
  SelectPrimitiveConfig, TooltipPrimitiveConfig, UiDensity,
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
