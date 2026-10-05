use dioxus::prelude::*;
use dioxus_shadcn::{
  LayoutOrientation, ResizableHandle, ResizablePanel, ResizablePanelGroup, ResizablePanelState,
  resizable_resize_pair,
};

#[component]
pub fn Demo() -> Element {
  let mut panels =
    use_signal(|| (ResizablePanelState::new(35.0, 20.0, 80.0), ResizablePanelState::new(65.0, 20.0, 80.0)));

  rsx! {
    ResizablePanelGroup {
      class: "h-40 max-w-md rounded-md border border-border",
      orientation: LayoutOrientation::Horizontal,
      ResizablePanel { id: "resizable-panels-files", size: panels().0.size, min_size: 20.0, max_size: 80.0,
        div { class: "flex h-full items-center justify-center text-sm", "Files" }
      }
      ResizableHandle {
        orientation: LayoutOrientation::Horizontal,
        "aria-controls": "resizable-panels-files",
        "aria-label": "Resize the files panel",
        value: panels().0.size,
        min: 20.0,
        max: 80.0,
        on_resize: move |delta| {
          let (first, second) = panels();
          panels.set(resizable_resize_pair(first, second, delta));
        },
      }
      ResizablePanel { size: panels().1.size, min_size: 20.0, max_size: 80.0,
        div { class: "flex h-full items-center justify-center text-sm", "Editor" }
      }
    }
  }
}
