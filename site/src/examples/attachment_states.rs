use dioxus::prelude::*;
use dioxus_ui::{
  Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription,
  AttachmentGroup, AttachmentMedia, AttachmentState, AttachmentTitle, AttachmentTrigger,
};

const FILES: [(&str, &str, AttachmentState); 4] = [
  ("report.pdf", "2.4 MB, uploaded", AttachmentState::Done),
  ("photo.png", "Uploading, 60%", AttachmentState::Uploading),
  ("notes.txt", "Scanning", AttachmentState::Processing),
  ("archive.zip", "Upload failed", AttachmentState::Error),
];

#[component]
pub fn Demo() -> Element {
  rsx! {
    AttachmentGroup { class: "flex-col",
      for (name, detail, state) in FILES {
        Attachment { key: "{name}", state,
          AttachmentMedia {
          svg { class: "h-5 w-5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", "aria-hidden": "true",
            path { d: "M14 3H7a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8z" }
            path { d: "M14 3v5h5" }
          }
        }
          AttachmentContent {
            AttachmentTitle { "{name}" }
            AttachmentDescription { "{detail}" }
          }
          AttachmentActions { AttachmentAction { "Remove" } }
        }
      }
    }
    AttachmentTrigger { class: "mt-3", "Attach a file" }
  }
}
