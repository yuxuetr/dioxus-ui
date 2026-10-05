use dioxus::prelude::*;
use dioxus_ui::{
  TypographyBlockquote, TypographyH1, TypographyH2, TypographyInlineCode, TypographyLead,
  TypographyMuted, TypographyP, TypographyProse,
};

#[component]
pub fn Demo() -> Element {
  rsx! {
    TypographyProse {
      TypographyH1 { "The Joke Tax" }
      TypographyLead { "A king, a jester, and a very taxing decree." }
      TypographyH2 { class: "mt-8", "The decree" }
      TypographyP {
        "The king declared that every joke must pay a tax, recorded with "
        TypographyInlineCode { "jokes.tax()" }
        "."
      }
      TypographyBlockquote { "After all, everyone enjoys a good joke, so it's only fair that they pay for the privilege." }
      TypographyMuted { "Updated two days ago." }
    }
  }
}
