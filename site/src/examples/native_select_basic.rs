use dioxus::prelude::*;
use dioxus_ui::{Label, NativeSelect, NativeSelectGroup, NativeSelectOption};

#[component]
pub fn Demo() -> Element {
  let mut timezone = use_signal(|| "utc".to_string());

  rsx! {
    div { class: "grid max-w-xs gap-2",
      Label { r#for: "native-select-basic", "Time zone" }
      NativeSelect {
        id: "native-select-basic",
        on_value_change: move |value| timezone.set(value),
        NativeSelectOption { value: "utc", selected: timezone() == "utc", "UTC" }
        NativeSelectGroup { label: "Americas",
          NativeSelectOption { value: "new-york", selected: timezone() == "new-york", "New York" }
          NativeSelectOption { value: "sao-paulo", selected: timezone() == "sao-paulo", "São Paulo" }
        }
        NativeSelectGroup { label: "Asia",
          NativeSelectOption { value: "tokyo", selected: timezone() == "tokyo", "Tokyo" }
          NativeSelectOption { value: "shanghai", selected: timezone() == "shanghai", disabled: true, "Shanghai" }
        }
      }
      p { class: "text-sm text-muted-foreground", "Value: {timezone}" }
    }
  }
}
