use dioxus::prelude::*;
use dioxus_ui_core::classes;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum KbdSize {
  Sm,
  #[default]
  Md,
  Lg,
}

impl KbdSize {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "min-h-5 px-1 text-[11px]",
      Self::Md => "min-h-6 px-1.5 text-xs",
      Self::Lg => "min-h-7 px-2 text-sm",
    }
  }
}

pub const KBD_BASE_CLASS: &str = "inline-flex items-center justify-center rounded border border-zinc-200 bg-zinc-50 font-mono font-medium text-zinc-700 shadow-sm";

pub fn kbd_class(size: KbdSize, class: &str) -> String {
  classes([Some(KBD_BASE_CLASS), Some(size.class()), Some(class)])
}

#[component]
pub fn Kbd(
  #[props(default)] size: KbdSize,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = kbd_class(size, &class);

  rsx! {
    kbd {
      class,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn kbd_class_reflects_size() {
    let actual = kbd_class(KbdSize::Lg, "ml-1");

    assert!(actual.contains(KBD_BASE_CLASS));
    assert!(actual.contains("min-h-7 px-2 text-sm"));
    assert!(actual.ends_with("ml-1"));
  }
}
