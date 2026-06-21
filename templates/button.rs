use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonVariant {
  Primary,
  Secondary,
  Destructive,
  Outline,
  Ghost,
  Link,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonSize {
  Sm,
  Md,
  Lg,
  Icon,
}

#[component]
pub fn Button(
  #[props(default = ButtonVariant::Primary)] variant: ButtonVariant,
  #[props(default = ButtonSize::Md)] size: ButtonSize,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  children: Element,
) -> Element {
  let base = "inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:pointer-events-none disabled:opacity-50";

  let variant_class = match variant {
    ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
    ButtonVariant::Secondary => "bg-zinc-100 text-zinc-900 hover:bg-zinc-200",
    ButtonVariant::Destructive => "bg-red-600 text-white hover:bg-red-700",
    ButtonVariant::Outline => "border border-zinc-200 bg-white hover:bg-zinc-100",
    ButtonVariant::Ghost => "bg-transparent hover:bg-zinc-100",
    ButtonVariant::Link => "bg-transparent text-blue-600 underline-offset-4 hover:underline",
  };

  let size_class = match size {
    ButtonSize::Sm => "h-8 px-3 text-sm",
    ButtonSize::Md => "h-10 px-4 text-sm",
    ButtonSize::Lg => "h-12 px-6 text-base",
    ButtonSize::Icon => "h-10 w-10",
  };

  let class = format!("{base} {variant_class} {size_class} {class}");

  rsx! {
    button {
      class,
      disabled,
      {children}
    }
  }
}
