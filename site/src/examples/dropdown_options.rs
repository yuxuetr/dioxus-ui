use dioxus::prelude::*;
use dioxus_shadcn::{
  ButtonSize, ButtonVariant, Dropdown, DropdownAlign, DropdownCheckboxItem, DropdownContent,
  DropdownLabel, DropdownRadioGroup, DropdownRadioItem, DropdownSeparator, DropdownShortcut,
  DropdownTrigger, button_class, use_density,
};

#[component]
pub fn DropdownOptionsDemo() -> Element {
  let mut status_bar = use_signal(|| true);
  let mut minimap = use_signal(|| false);

  rsx! {
    Dropdown {
      DropdownTrigger {
        class: button_class(ButtonVariant::Outline, ButtonSize::Md, use_density(), ""),
        "View"
      }
      DropdownContent { align: DropdownAlign::Start, class: "w-56",
        DropdownLabel { class: "pl-8", "Appearance" }
        DropdownCheckboxItem {
          checked: status_bar(),
          onclick: move |_| status_bar.toggle(),
          "Status bar"
          DropdownShortcut { "Ctrl+/" }
        }
        DropdownCheckboxItem { checked: minimap(), onclick: move |_| minimap.toggle(), "Minimap" }
        DropdownSeparator {}
        DropdownLabel { class: "pl-8", "Panel position" }
        DropdownRadioGroup { default_value: "bottom",
          DropdownRadioItem { value: "bottom", "Bottom" }
          DropdownRadioItem { value: "right", "Right" }
        }
      }
    }
  }
}
