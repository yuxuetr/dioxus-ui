use crate::density::{density_control_class, use_density, with_density};
use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

pub const FILE_INPUT_BASE_CLASS: &str = "flex h-10 w-full cursor-pointer items-center overflow-hidden rounded-md border bg-background pe-3 text-sm text-muted-foreground transition-colors file:me-3 file:h-full file:cursor-pointer file:border-0 file:border-e file:border-solid file:border-input file:bg-secondary file:px-3 file:text-sm file:font-medium file:text-secondary-foreground hover:file:bg-secondary/80 focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";

pub fn file_input_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  merge_classes(classes([Some(FILE_INPUT_BASE_CLASS), Some(invalid_class)]), class)
}

/// A native file input styled through `file:`. It binds no `value`, which a
/// file input cannot take; `onchange` passes the form event, whose `files()`
/// the app reads. Other attributes, such as `accept`, `multiple`, `id`, and
/// `name`, go to the input.
#[component]
pub fn FileInput(
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
  #[props(default)] class: String,
  #[props(default)] onchange: Option<EventHandler<FormEvent>>,
  #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
  let class =
    file_input_class(invalid, &with_density(density_control_class(use_density()), &class));

  rsx! {
    input {
      class,
      r#type: "file",
      disabled,
      "aria-invalid": invalid.then_some("true"),
      onchange: move |event| {
        if let Some(handler) = onchange {
          handler.call(event);
        }
      },
      ..attributes,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_file_input_is_a_native_file_input_without_a_value() {
    fn app() -> Element {
      rsx! { FileInput { accept: "image/*", multiple: true, invalid: true } }
    }
    let html = render(app);

    assert!(html.contains("type=\"file\""));
    assert!(html.contains("accept=\"image/*\""));
    assert!(html.contains("multiple"));
    assert!(html.contains("aria-invalid=\"true\""));
    assert!(!html.contains("value="));
    assert!(html.contains("border-destructive"));
  }
}
