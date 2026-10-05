use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KbdSize {
  Sm,
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

pub const KBD_BASE_CLASS: &str = "inline-flex items-center justify-center rounded border border-border bg-muted font-mono font-medium text-muted-foreground shadow-sm";

pub fn kbd_class(size: KbdSize, class: &str) -> String {
  classes([Some(KBD_BASE_CLASS), Some(size.class()), Some(class)])
}

#[component]
pub fn Kbd(
  #[props(default = KbdSize::Md)] size: KbdSize,
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
