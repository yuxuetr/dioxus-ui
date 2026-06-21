use dioxus_ui::{
  button_class, input_class, label_class, textarea_class, ButtonSize, ButtonVariant, UiDensity,
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
}
