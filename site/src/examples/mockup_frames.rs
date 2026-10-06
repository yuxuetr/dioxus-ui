use dioxus::prelude::*;
use dioxus_shadcn::{MockupBrowser, MockupCode, MockupCodeLine, MockupPhone};

#[component]
pub fn Demo() -> Element {
  rsx! {
    div { class: "grid items-start gap-6 lg:grid-cols-[1fr_auto]",
      div { class: "grid gap-6",
        MockupBrowser { url: "https://acme.example/pricing", content_class: "grid place-items-center p-10",
          p { class: "text-sm text-muted-foreground", "Your page here" }
        }
        MockupCode {
          MockupCodeLine { prefix: "$", "cargo install dioxus-shadcn-cli" }
          MockupCodeLine { prefix: "$", "dxui add mockup" }
          MockupCodeLine { highlight: true, "Added mockup to src/components/ui" }
        }
      }
      MockupPhone {
        div { class: "grid h-full place-items-center p-4 text-center text-sm text-muted-foreground",
          "Mobile preview"
        }
      }
    }
  }
}
