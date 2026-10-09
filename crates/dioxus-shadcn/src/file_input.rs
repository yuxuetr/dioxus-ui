//! File Input: a styled native file picker for uploads and imports.

use crate::density::{density_control_class, use_density, with_density};
use dioxus::html::{FileData, HasFileData};
use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

const FILE_INPUT_BASE_CLASS: &str = "flex h-10 w-full cursor-pointer items-center overflow-hidden rounded-md border bg-background pe-3 text-sm text-muted-foreground transition-colors file:me-3 file:h-full file:cursor-pointer file:border-0 file:border-e file:border-solid file:border-input file:bg-secondary file:px-3 file:text-sm file:font-medium file:text-secondary-foreground hover:file:bg-secondary/80 focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";

const FILE_DROPZONE_BASE_CLASS: &str = "rounded-md border-2 border-dashed border-input p-6 text-center text-sm text-muted-foreground transition-colors data-[dragging=true]:border-primary data-[dragging=true]:bg-accent/40 data-[dragging=true]:text-foreground data-[disabled=true]:cursor-not-allowed data-[disabled=true]:opacity-50";

/// Classes for the file input: base classes, the destructive border and ring when
/// `invalid` or the plain ones otherwise, then `class` merged over them.
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

/// Classes for the drop area: base classes, which follow its `data-dragging`
/// and `data-disabled`, with `class` merged over them.
pub fn file_dropzone_class(class: &str) -> String {
  merge_classes(classes([Some(FILE_DROPZONE_BASE_CLASS)]), class)
}

/// An area that takes files dragged onto it and passes them to `on_files`.
/// It sets `data-dragging="true"` while files are over it, counting entries
/// and exits so moving across its children does not flicker. Dropping needs
/// a pointer: put a `FileInput` or another picker inside or beside it for
/// keyboard users. Other attributes, such as `aria-label`, go to the area.
#[component]
pub fn FileDropzone(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] on_files: Option<EventHandler<Vec<FileData>>>,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = file_dropzone_class(&class);
  let mut depth = use_signal(|| 0_u32);
  let dragging = !disabled && depth() > 0;

  rsx! {
    div {
      class,
      "data-dragging": dragging.to_string(),
      "data-disabled": disabled.to_string(),
      ondragenter: move |event| {
        event.prevent_default();
        if !disabled {
          depth += 1;
        }
      },
      // A drop only fires where dragover was cancelled.
      ondragover: move |event| event.prevent_default(),
      ondragleave: move |_| depth.set(depth().saturating_sub(1)),
      ondrop: move |event| {
        event.prevent_default();
        depth.set(0);
        let files = event.files();
        if let Some(handler) = on_files.filter(|_| !disabled && !files.is_empty()) {
          handler.call(files);
        }
      },
      ..attributes,
      {children}
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

  #[test]
  fn ssr_dropzone_starts_without_dragging_and_passes_attributes() {
    fn app() -> Element {
      rsx! { FileDropzone { "aria-label": "Upload area", class: "p-2", "Drop files" } }
    }
    let html = render(app);

    assert!(html.contains(r#"data-dragging="false""#), "{html}");
    assert!(html.contains(r#"data-disabled="false""#));
    assert!(html.contains(r#"aria-label="Upload area""#));
    assert!(html.contains("p-2") && !html.contains("p-6"), "{html}");
    assert!(html.contains("Drop files"));
  }
}
