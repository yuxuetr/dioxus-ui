//! A server-rendered page that the browser hydrates, for
//! `scripts/fullstack-hydration-verify.mjs`: Tabs and Select take their element
//! ids from the render (RFC 0075), so each request writes the same ids and the
//! hydrated page finds its parts by them.

use dioxus::prelude::*;
use dioxus_shadcn::{
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Tabs, TabsContent, TabsList,
  TabsTrigger,
};

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    Tabs { default_value: "one",
      TabsList {
        for tab in ["one", "two", "three"] {
          TabsTrigger { key: "{tab}", value: tab, "{tab}" }
        }
      }
      for tab in ["one", "two", "three"] {
        TabsContent { key: "{tab}", value: tab, "panel {tab}" }
      }
    }
    Select { default_value: "system",
      SelectTrigger { SelectValue { placeholder: "Theme" } }
      SelectContent {
        for theme in ["light", "dark", "system"] {
          SelectItem { key: "{theme}", value: theme, "{theme}" }
        }
      }
    }
  }
}
