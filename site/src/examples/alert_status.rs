use dioxus::prelude::*;
use dioxus_shadcn::{Alert, AlertDescription, AlertTitle, AlertVariant};

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "grid max-w-lg gap-4",
      Alert { variant: AlertVariant::Success,
        AlertTitle { "Payment received" }
        AlertDescription { variant: AlertVariant::Success, "Invoice INV-001 is paid in full." }
      }
      Alert { variant: AlertVariant::Warning,
        AlertTitle { "Storage almost full" }
        AlertDescription { variant: AlertVariant::Warning, "You have used 90% of your 10 GB plan." }
      }
      Alert { variant: AlertVariant::Info,
        AlertTitle { "Scheduled maintenance" }
        AlertDescription { variant: AlertVariant::Info, "The service is read-only on Sunday from 02:00 to 03:00 UTC." }
      }
    }
  }
}
