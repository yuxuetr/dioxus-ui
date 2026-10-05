use dioxus::prelude::*;
use dioxus_shadcn::{Stat, StatDescription, StatFigure, StatGroup, StatTitle, StatValue};

#[component]
pub fn Demo() -> Element {
  rsx! {
    StatGroup { class: "max-w-full",
      Stat {
        StatTitle { "Revenue" }
        StatValue { "$45,231" }
        StatDescription { "+20.1% from last month" }
        StatFigure { class: "text-2xl", span { "aria-hidden": "true", "$" } }
      }
      Stat {
        StatTitle { "Subscriptions" }
        StatValue { "2,350" }
        StatDescription { "+180 this week" }
      }
      Stat {
        StatTitle { "Churn" }
        StatValue { "1.2%" }
        StatDescription { "-0.4% from last month" }
      }
    }
  }
}
