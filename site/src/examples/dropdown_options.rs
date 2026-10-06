use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonVariant, DropdownCheckboxItem, DropdownContent, DropdownLabel, DropdownRadioGroup,
  DropdownRadioItem, DropdownSeparator, DropdownShortcut,
};

#[component]
pub fn DropdownOptionsDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut status_bar = use_signal(|| true);
  let mut minimap = use_signal(|| false);
  let mut panel = use_signal(|| "bottom");

  rsx! {
    Button {
      id: "dropdown-options-trigger",
      variant: ButtonVariant::Outline,
      "aria-haspopup": "menu",
      "aria-expanded": "{open}",
      onclick: move |_| open.toggle(),
      "View"
    }
    DropdownContent {
      open: open(),
      anchor_id: "dropdown-options-trigger",
      align: dioxus_shadcn::DropdownAlign::Start,
      on_open_change: move |next| open.set(next),
      class: "w-56",
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
      DropdownRadioGroup { value: panel(),
        DropdownRadioItem {
          checked: panel() == "bottom",
          onclick: move |_| panel.set("bottom"),
          "Bottom"
        }
        DropdownRadioItem {
          checked: panel() == "right",
          onclick: move |_| panel.set("right"),
          "Right"
        }
      }
    }
  }
}
