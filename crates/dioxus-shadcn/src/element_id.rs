use std::cell::Cell;
use std::rc::Rc;

use dioxus::core::provide_root_context;
use dioxus::prelude::*;

/// The next number of the current virtual DOM (RFC 0075).
#[derive(Clone, Default)]
struct ElementIds(Rc<Cell<usize>>);

/// The next element id number of the current virtual DOM, for a hook to keep.
/// A server renders each request in a new virtual DOM and the browser builds
/// the same tree to hydrate it, so both hand out the same numbers, which a
/// process-wide counter does not.
pub(crate) fn next_element_id() -> usize {
  let ids = try_consume_context::<ElementIds>()
    .unwrap_or_else(|| provide_root_context(ElementIds::default()));
  let id = ids.0.get();
  ids.0.set(id + 1);
  id
}

#[cfg(test)]
#[cfg(all(
  feature = "accordion",
  feature = "checkbox",
  feature = "dialog",
  feature = "slider",
  feature = "tabs"
))]
mod tests {
  use crate::accordion::{Accordion, AccordionItem, AccordionTrigger};
  use crate::checkbox::Checkbox;
  use crate::dialog::{DialogContent, DialogTitle};
  use crate::slider::Slider;
  use crate::tabs::{Tabs, TabsList, TabsTrigger};
  use dioxus::prelude::*;

  fn page() -> Element {
    rsx! {
      Tabs {
        TabsList { TabsTrigger { value: "one", active: true, "One" } }
      }
      Accordion {
        AccordionItem { value: "a", AccordionTrigger { "A" } }
      }
      Checkbox { indeterminate: true }
      Slider { value: 40.0, "aria-label": "Volume" }
      DialogContent { open: true, DialogTitle { "Title" } }
    }
  }

  fn render() -> String {
    let mut dom = VirtualDom::new(page);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn each_render_numbers_ids_from_zero() {
    let first = render();
    assert!(first.contains("dxui-tabs-0-trigger-one"), "{first}");
    assert!(first.contains("\"dxui-roving-group-"), "{first}");
    // A server renders each request in a new virtual DOM, as a test does.
    assert_eq!(render(), first);
  }
}
