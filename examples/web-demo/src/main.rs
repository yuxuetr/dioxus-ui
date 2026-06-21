use dioxus_ui::{button_class, ButtonSize, ButtonVariant, UiDensity};

fn main() {
  let class = button_class(
    ButtonVariant::Primary,
    ButtonSize::Md,
    UiDensity::Comfortable,
    "w-full",
  );

  println!("dioxus-ui web demo button class: {class}");
}
