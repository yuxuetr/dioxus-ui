use dioxus::prelude::*;
use dioxus_shadcn::{Checkbox, Label};

#[component]
pub fn CheckboxBasicDemo() -> Element {
  let mut terms = use_signal(|| true);
  let mut items = use_signal(|| [true, false]);
  let all = items().iter().all(|item| *item);
  let some = items().iter().any(|item| *item);

  rsx! {
    div { class: "grid gap-3",
      div { class: "flex items-center gap-2",
        Checkbox {
          id: "checkbox-basic-terms",
          checked: terms(),
          on_checked_change: move |checked| terms.set(checked),
        }
        Label { r#for: "checkbox-basic-terms", "Accept terms and conditions" }
      }
      div { class: "flex items-center gap-2",
        Checkbox { id: "checkbox-basic-disabled", disabled: true }
        Label { r#for: "checkbox-basic-disabled", "Disabled" }
      }
      div { class: "flex items-center gap-2",
        Checkbox {
          id: "checkbox-basic-all",
          checked: all,
          indeterminate: some && !all,
          on_checked_change: move |checked| items.set([checked, checked]),
        }
        Label { r#for: "checkbox-basic-all", "Select all" }
      }
      for (index, name) in ["Email", "Push"].into_iter().enumerate() {
        div { key: "{name}", class: "ml-6 flex items-center gap-2",
          Checkbox {
            id: "checkbox-basic-{name}",
            checked: items()[index],
            on_checked_change: move |checked| items.with_mut(|items| items[index] = checked),
          }
          Label { r#for: "checkbox-basic-{name}", "{name}" }
        }
      }
    }
  }
}
