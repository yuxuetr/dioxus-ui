use dioxus::prelude::*;

use super::element_id::next_element_id;

/// Links dialog content to the title and description mounted inside it. Each
/// part sets its flag while mounted, so the content never points at a missing
/// id.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct DialogLabels {
  id: usize,
  has_title: Signal<bool>,
  has_description: Signal<bool>,
}

#[derive(Clone, Copy)]
pub(crate) enum DialogLabelPart {
  Title,
  Description,
}

impl DialogLabels {
  fn part_id(self, part: DialogLabelPart) -> String {
    match part {
      DialogLabelPart::Title => format!("dxui-dialog-{}-title", self.id),
      DialogLabelPart::Description => format!("dxui-dialog-{}-description", self.id),
    }
  }

  fn flag(self, part: DialogLabelPart) -> Signal<bool> {
    match part {
      DialogLabelPart::Title => self.has_title,
      DialogLabelPart::Description => self.has_description,
    }
  }

  /// The content's `aria-labelledby` and `aria-describedby`. A passed
  /// `aria-labelledby` or `aria-label` replaces the first and a passed
  /// `aria-describedby` the second, so SSR writes a single value.
  pub(crate) fn content_attributes(
    self,
    attributes: &[Attribute],
  ) -> (Option<String>, Option<String>) {
    let passed =
      |names: &[&str]| attributes.iter().any(|attribute| names.contains(&attribute.name));
    let labelledby = (!passed(&["aria-labelledby", "aria-label"]) && (self.has_title)())
      .then(|| self.part_id(DialogLabelPart::Title));
    let describedby = (!passed(&["aria-describedby"]) && (self.has_description)())
      .then(|| self.part_id(DialogLabelPart::Description));
    (labelledby, describedby)
  }
}

/// Called by a dialog content part; provides the ids to its title and
/// description.
pub(crate) fn use_dialog_labels() -> DialogLabels {
  let labels = use_hook(|| DialogLabels {
    id: next_element_id(),
    has_title: Signal::new(false),
    has_description: Signal::new(false),
  });
  use_context_provider(|| labels)
}

/// Called by a title or description part; returns its `id`, or `None`
/// outside dialog content.
pub(crate) fn use_dialog_label_part(part: DialogLabelPart) -> Option<String> {
  let labels = try_use_context::<DialogLabels>();
  use_effect(move || {
    if let Some(labels) = labels {
      labels.flag(part).set(true);
    }
  });
  use_drop(move || {
    // The content may already be gone when the whole dialog unmounts.
    if let Some(labels) = labels {
      if let Ok(mut flag) = labels.flag(part).try_write() {
        *flag = false;
      }
    }
  });
  labels.map(|labels| labels.part_id(part))
}
