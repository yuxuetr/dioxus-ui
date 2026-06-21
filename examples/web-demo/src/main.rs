use dioxus_ui::{
  button_class, input_class, label_class, textarea_class, ButtonSize, ButtonVariant, UiDensity,
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
}
