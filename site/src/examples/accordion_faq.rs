use dioxus::prelude::*;
use dioxus_shadcn::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

const QUESTIONS: [(&str, &str, &str); 3] = [
  ("copy", "Can I edit the components?", "Yes. dxui add copies the source into your app, and you own it."),
  ("styling", "How is it styled?", "With Tailwind CSS v4 classes and the shadcn/ui semantic tokens."),
  ("dark", "Is there a dark theme?", "Add the dark class to an ancestor to turn it on."),
];

#[component]
pub fn AccordionFaqDemo() -> Element {
  rsx! {
    Accordion { class: "max-w-md", default_value: "copy",
      for (value, question, answer) in QUESTIONS {
        AccordionItem { key: "{value}", value,
          AccordionTrigger { "{question}" }
          AccordionContent { "{answer}" }
        }
      }
    }
  }
}
