use dioxus_ui::{button_class, ButtonSize, ButtonVariant, UiDensity};

fn main() {
  let class = button_class(
    ButtonVariant::Secondary,
    ButtonSize::Sm,
    UiDensity::Compact,
    "justify-start",
  );

  println!("dioxus-ui desktop demo button class: {class}");
}
