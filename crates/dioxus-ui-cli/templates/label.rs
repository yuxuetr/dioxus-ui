use dioxus::prelude::*;
use super::utils::classes;

pub const LABEL_BASE_CLASS: &str = "text-sm font-medium leading-none text-zinc-900 peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

pub fn label_class(class: &str) -> String {
  classes([Some(LABEL_BASE_CLASS), Some(class)])
}

#[component]
pub fn Label(
  #[props(default)] r#for: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = label_class(&class);

  rsx! {
    label {
      class,
      r#for,
      {children}
    }
  }
}
