use dioxus::prelude::*;
use dioxus_shadcn::{Input, Label, Tabs, TabsContent, TabsList, TabsTrigger};

#[component]
pub fn Demo() -> Element {
  let mut tab = use_signal(|| "account".to_string());
  let active = move |value: &str| tab() == value;

  rsx! {
    Tabs { class: "max-w-sm", on_value_change: move |value: String| tab.set(value),
      TabsList { "aria-label": "Settings",
        TabsTrigger { value: "account", active: active("account"), "Account" }
        TabsTrigger { value: "password", active: active("password"), "Password" }
        TabsTrigger { value: "billing", active: active("billing"), disabled: true, "Billing" }
      }
      TabsContent { value: "account", active: active("account"),
        div { class: "grid gap-2",
          Label { r#for: "tabs-account-name", "Name" }
          Input { id: "tabs-account-name", value: "Ada Lovelace" }
        }
      }
      TabsContent { value: "password", active: active("password"),
        div { class: "grid gap-2",
          Label { r#for: "tabs-account-password", "New password" }
          Input { id: "tabs-account-password", r#type: "password" }
        }
      }
    }
  }
}
