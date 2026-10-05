use dioxus::prelude::*;
use dioxus_shadcn::{Alert, AlertDescription, AlertTitle, AlertVariant};

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "grid max-w-lg gap-4",
      Alert {
        AlertTitle { "Heads up!" }
        AlertDescription { "You can add components to your app with the CLI." }
      }
      Alert { variant: AlertVariant::Destructive,
        AlertTitle { "Error" }
        AlertDescription { variant: AlertVariant::Destructive, "Your session has expired. Please sign in again." }
      }
    }
  }
}
