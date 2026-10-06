use dioxus::prelude::*;
use dioxus_shadcn::{Input, Label, Tabs, TabsContent, TabsList, TabsTrigger};

#[component]
pub fn TabsAccountDemo() -> Element {
  rsx! {
    Tabs { class: "max-w-sm", default_value: "account",
      TabsList { "aria-label": "Settings",
        TabsTrigger { value: "account", "Account" }
        TabsTrigger { value: "password", "Password" }
        TabsTrigger { value: "billing", disabled: true, "Billing" }
      }
      TabsContent { value: "account",
        div { class: "grid gap-2",
          Label { r#for: "tabs-account-name", "Name" }
          Input { id: "tabs-account-name", value: "Ada Lovelace" }
        }
      }
      TabsContent { value: "password",
        div { class: "grid gap-2",
          Label { r#for: "tabs-account-password", "New password" }
          Input { id: "tabs-account-password", r#type: "password" }
        }
      }
    }
  }
}
