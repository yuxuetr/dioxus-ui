use dioxus::prelude::*;
use dioxus_ui::{Accordion, AccordionContent, AccordionItem, AccordionTrigger, accordion_single_open};

const QUESTIONS: [(&str, &str, &str); 3] = [
  ("copy", "Can I edit the components?", "Yes. dxui add copies the source into your app, and you own it."),
  ("styling", "How is it styled?", "With Tailwind CSS v4 classes and the shadcn/ui semantic tokens."),
  ("dark", "Is there a dark theme?", "Add the dark class to an ancestor to turn it on."),
];

#[component]
pub fn Demo() -> Element {
  let mut open = use_signal(|| Some("copy".to_string()));
  let is_open = move |value: &str| open().as_deref() == Some(value);

  rsx! {
    Accordion {
      class: "max-w-md",
      on_toggle: move |value: String| open.set(accordion_single_open(open().as_deref(), &value)),
      for (value, question, answer) in QUESTIONS {
        AccordionItem { key: "{value}", value,
          AccordionTrigger { open: is_open(value), "{question}" }
          AccordionContent { open: is_open(value), "{answer}" }
        }
      }
    }
  }
}
