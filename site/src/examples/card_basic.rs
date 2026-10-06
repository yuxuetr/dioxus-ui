use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonVariant, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Input, Label};

#[component]
pub fn CardBasicDemo() -> Element {
  rsx! {
    Card { class: "max-w-sm",
      CardHeader {
        CardTitle { "Create project" }
        CardDescription { "Deploy your new project in one click." }
      }
      CardContent {
        div { class: "grid gap-2",
          Label { r#for: "card-basic-name", "Name" }
          Input { id: "card-basic-name", placeholder: "Name of your project" }
        }
      }
      CardFooter { class: "justify-between",
        Button { variant: ButtonVariant::Outline, "Cancel" }
        Button { "Deploy" }
      }
    }
  }
}
