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
    ButtonVariant::Primary,
    ButtonSize::Md,
    UiDensity::Comfortable,
    "w-full",
  );

  println!("dioxus-ui web demo button class: {class}");
  println!("dioxus-ui web demo input class: {}", input_class(false, "mt-2"));
  println!("dioxus-ui web demo textarea class: {}", textarea_class(false, "mt-2"));
  println!("dioxus-ui web demo label class: {}", label_class("mb-2"));
  println!("dioxus-ui web demo checkbox class: {}", checkbox_class(true, "mt-2"));
  println!("dioxus-ui web demo switch class: {}", switch_class(true, "mt-2"));
  println!("dioxus-ui web demo switch thumb class: {}", switch_thumb_class(true));
  println!("dioxus-ui web demo tabs list class: {}", tabs_list_class("mt-4"));
  println!(
    "dioxus-ui web demo tabs trigger class: {}",
    tabs_trigger_class(true, "min-w-24")
  );
  println!("dioxus-ui web demo tabs content class: {}", tabs_content_class("p-4"));
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
