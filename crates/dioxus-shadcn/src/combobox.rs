use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
pub use dioxus_shadcn_primitives::{
  ActiveDescendantState, DismissBehavior, OverlayAlign, OverlaySide, PopoverPrimitiveConfig,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::choice::{Choice, use_choice};
use crate::default_attribute::default_attribute;
use crate::density::{density_control_class, use_density, with_density};
use crate::element_id::next_element_id;
use crate::listbox::{ListboxMode, use_listbox};
use crate::root_state::{Controllable, use_controllable, use_root_context};

pub const COMBOBOX_TRIGGER_BASE_CLASS: &str = "flex h-10 w-full items-center justify-between rounded-md border bg-background px-3 py-2 text-sm text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
pub const COMBOBOX_INPUT_BASE_CLASS: &str = "flex h-10 w-full rounded-md bg-transparent px-3 py-2 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50";
pub const COMBOBOX_CONTENT_BASE_CLASS: &str = "z-50 max-h-96 min-w-[max(8rem,var(--dxui-anchor-width,0px))] overflow-hidden rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-md";
pub const COMBOBOX_LIST_BASE_CLASS: &str = "max-h-80 overflow-y-auto overflow-x-hidden";
pub const COMBOBOX_EMPTY_BASE_CLASS: &str = "py-6 text-center text-sm text-muted-foreground";
pub const COMBOBOX_STATUS_BASE_CLASS: &str = "sr-only";
pub const COMBOBOX_GROUP_BASE_CLASS: &str = "overflow-hidden p-1 text-foreground";
pub const COMBOBOX_VALUE_BASE_CLASS: &str = "truncate";
pub const COMBOBOX_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-[active=true]:bg-accent data-[active=true]:text-accent-foreground data-highlighted:bg-accent data-highlighted:text-accent-foreground data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 pe-8 after:absolute after:end-2 after:size-4 after:bg-current after:[mask:url(data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20viewBox=%270%200%2016%2016%27%20fill=%27none%27%20stroke=%27black%27%20stroke-width=%272%27%20stroke-linecap=%27round%27%20stroke-linejoin=%27round%27%3E%3Cpath%20d=%27M3.5%208.5l3%203%206-7%27/%3E%3C/svg%3E)_center/contain_no-repeat]";

pub fn combobox_trigger_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  merge_classes(classes([Some(COMBOBOX_TRIGGER_BASE_CLASS), Some(invalid_class)]), class)
}

pub fn combobox_input_class(class: &str) -> String {
  merge_classes(classes([Some(COMBOBOX_INPUT_BASE_CLASS)]), class)
}

pub fn combobox_content_class(class: &str) -> String {
  merge_classes(classes([Some(COMBOBOX_CONTENT_BASE_CLASS)]), class)
}

pub fn combobox_list_class(class: &str) -> String {
  merge_classes(classes([Some(COMBOBOX_LIST_BASE_CLASS)]), class)
}

pub fn combobox_empty_class(class: &str) -> String {
  merge_classes(classes([Some(COMBOBOX_EMPTY_BASE_CLASS)]), class)
}

pub fn combobox_status_class(class: &str) -> String {
  merge_classes(classes([Some(COMBOBOX_STATUS_BASE_CLASS)]), class)
}

pub fn combobox_group_class(class: &str) -> String {
  merge_classes(classes([Some(COMBOBOX_GROUP_BASE_CLASS)]), class)
}

pub fn combobox_value_class(class: &str) -> String {
  merge_classes(classes([Some(COMBOBOX_VALUE_BASE_CLASS)]), class)
}

pub fn combobox_item_class(active: bool, selected: bool, class: &str) -> String {
  let active_class = if active { "bg-accent text-accent-foreground" } else { "" };
  // A selected option shows a check mark, distinct from the highlight
  // (RFC 0062).
  let selected_class = if selected { "after:opacity-100" } else { "after:opacity-0" };

  merge_classes(
    classes([Some(COMBOBOX_ITEM_BASE_CLASS), Some(active_class), Some(selected_class)]),
    class,
  )
}

pub fn combobox_active_descendant_state(active_id: Option<String>) -> ActiveDescendantState {
  ActiveDescendantState::new(active_id)
}

/// What a `Combobox` shares with its parts (RFC 0077).
#[derive(Clone)]
struct ComboboxContext {
  input_id: String,
  choice: Choice,
  open: Controllable<bool>,
  set_open: Callback<bool>,
}

impl ComboboxContext {
  fn list_id(&self) -> String {
    format!("{}-list", self.input_id)
  }
}

fn use_combobox(part: &str) -> ComboboxContext {
  use_root_context::<ComboboxContext>(part, "Combobox")
}

/// The root of a combobox: it owns the chosen value, or values with
/// `multiple`, and whether the list is open, and links its parts, as
/// `Select` does. Pass `value` (`values`) or `open` to control them, or
/// `default_value` (`default_values`) and `default_open` to start them; the
/// change callbacks hear every change the user makes either way. The typed
/// text stays with the app, which filters the items it renders. A combobox
/// has one combobox element, its `ComboboxInput`, or else its
/// `ComboboxTrigger`; `id` names it, for a `Label` to point at, and the list
/// anchors to it. Without `id` the ids are generated.
#[component]
pub fn Combobox(
  #[props(default)] id: Option<String>,
  #[props(default)] value: ReadSignal<Option<String>>,
  #[props(default)] default_value: Option<String>,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] multiple: bool,
  #[props(default)] values: ReadSignal<Option<Vec<String>>>,
  #[props(default)] default_values: Vec<String>,
  #[props(default)] on_values_change: Option<EventHandler<Vec<String>>>,
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let generated = use_hook(|| format!("dxui-combobox-{}-input", next_element_id()));
  let input_id = id.unwrap_or(generated);
  let choice = use_choice(
    multiple,
    value,
    default_value,
    on_value_change,
    values,
    default_values,
    on_values_change,
  );
  let open = use_controllable(move || open.cloned(), move || default_open, on_open_change);
  let set_open = use_callback(move |next: bool| open.set(next));
  use_context_provider(|| ComboboxContext { input_id, choice, open, set_open });

  rsx! { {children} }
}

/// A button that toggles the list. Without a `ComboboxInput` it is the
/// combobox element and takes the root's `id`.
#[component]
pub fn ComboboxTrigger(
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let context = use_combobox("ComboboxTrigger");
  let class =
    combobox_trigger_class(invalid, &with_density(density_control_class(use_density()), &class));
  let open = context.open.get();
  let set_open = context.set_open;
  // A trigger may sit in a combobox without a list, so it names the list
  // only while open.
  let controls =
    default_attribute(&attributes, "aria-controls", context.list_id()).filter(|_| open);
  let id = default_attribute(&attributes, "id", context.input_id.clone());

  rsx! {
    button {
      r#type: "button",
      role: "combobox",
      id,
      class,
      disabled,
      "aria-controls": controls,
      "aria-expanded": open.to_string(),
      "aria-invalid": invalid.to_string(),
      "data-state": if open { "open" } else { "closed" },
      onclick: move |event| {
        set_open.call(!open);
        if let Some(handler) = onclick {
          handler.call(event);
        }
      },
      ..attributes,
      {children}
    }
  }
}

/// Typed text opens the list and reaches the app through `oninput`; the app
/// filters the items it renders. ArrowDown on a closed input opens the list.
#[component]
pub fn ComboboxInput(
  #[props(default)] value: String,
  #[props(default)] placeholder: String,
  #[props(default)] disabled: bool,
  #[props(default)] oninput: Option<EventHandler<FormEvent>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
  let context = use_combobox("ComboboxInput");
  let class = combobox_input_class(&with_density(density_control_class(use_density()), &class));
  let open = context.open.get();
  let set_open = context.set_open;
  let controls = default_attribute(&attributes, "aria-controls", context.list_id());

  rsx! {
    input {
      role: "combobox",
      id: context.input_id,
      class,
      value,
      placeholder,
      disabled,
      autocomplete: "off",
      "aria-autocomplete": "list",
      "aria-controls": controls,
      "aria-expanded": open.to_string(),
      oninput: move |event| {
        if !open {
          set_open.call(true);
        }
        if let Some(handler) = oninput {
          handler.call(event);
        }
      },
      onkeydown: move |event| {
        if !open && event.key() == Key::ArrowDown {
          event.prevent_default();
          set_open.call(true);
        }
      },
      ..attributes,
    }
  }
}

/// The content is placed next to the input while focus stays in it:
/// ArrowDown and ArrowUp move the highlighted option, and Enter or click
/// choose it, which closes the list unless the combobox takes `multiple`
/// values. Escape and outside interactions close it per `dismiss`.
#[component]
pub fn ComboboxContent(
  #[props(default)] class: String,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::Start)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let context = use_combobox("ComboboxContent");
  let class = combobox_content_class(&class);
  let open = context.open.get();
  let anchor_id = Some(context.input_id.clone());
  // With `multiple` a choice leaves the list open; Escape and outside
  // interactions still close it through the anchored overlay.
  let closes_on_choice = (!context.choice.multiple).then_some(context.set_open);
  let listbox = use_listbox(
    open,
    anchor_id.clone(),
    ListboxMode::Combobox,
    Some(context.choice.choose),
    closes_on_choice,
  );
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id, anchor_point: None, side, align, side_offset },
    dismiss,
    Some(context.set_open),
  );

  rsx! {
    div {
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-anchored": anchored,
      "data-dxui-listbox": listbox,
      {children}
    }
  }
}

/// The listbox, named by the input, whose `aria-controls` points at it.
#[component]
pub fn ComboboxList(#[props(default)] class: String, children: Element) -> Element {
  let context = use_combobox("ComboboxList");
  let class = combobox_list_class(&class);

  rsx! {
    div {
      role: "listbox",
      id: context.list_id(),
      class,
      "aria-labelledby": context.input_id,
      "aria-multiselectable": context.choice.multiple.then_some("true"),
      {children}
    }
  }
}

#[component]
pub fn ComboboxEmpty(#[props(default)] class: String, children: Element) -> Element {
  let class = combobox_empty_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// A visually hidden polite status region for result announcements. Keep it
/// mounted and change only its children; an empty text says nothing.
/// Place it outside `ComboboxContent`, which is hidden while closed.
#[component]
pub fn ComboboxStatus(#[props(default)] class: String, children: Element) -> Element {
  let class = combobox_status_class(&class);

  rsx! {
    div {
      role: "status",
      class,
      "aria-live": "polite",
      "aria-atomic": "true",
      {children}
    }
  }
}

#[component]
pub fn ComboboxGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = combobox_group_class(&class);

  rsx! {
    div {
      role: "group",
      class,
      {children}
    }
  }
}

#[component]
pub fn ComboboxValue(#[props(default)] class: String, children: Element) -> Element {
  let class = combobox_value_class(&class);

  rsx! {
    span {
      class,
      {children}
    }
  }
}

#[component]
pub fn ComboboxItem(
  value: String,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let context = use_combobox("ComboboxItem");
  let selected = context.choice.chosen().contains(&value);
  let class = combobox_item_class(
    false,
    selected,
    &with_density(density_control_class(use_density()), &class),
  );

  rsx! {
    div {
      role: "option",
      class,
      "aria-disabled": disabled.to_string(),
      "aria-selected": selected.to_string(),
      "data-disabled": disabled.to_string(),
      "data-selected": selected.to_string(),
      "data-value": value,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_trigger_renders_passed_attributes_and_keeps_its_state() {
    fn app() -> Element {
      rsx! {
        Combobox { id: "fruit",
          ComboboxTrigger { "Pick" }
        }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"role="combobox" id="fruit""#), "{html}");
    assert!(!html.contains("aria-controls"), "{html}");
    assert!(html.contains(r#"aria-expanded="false""#));
    assert!(html.contains(r#"role="combobox""#));
  }

  #[test]
  fn ssr_input_controls_the_list_that_takes_its_name() {
    fn app() -> Element {
      rsx! {
        Combobox { id: "fruit", default_open: true, default_value: "apple",
          ComboboxInput {}
          ComboboxContent {
            ComboboxList {
              ComboboxItem { value: "apple", "Apple" }
              ComboboxItem { value: "banana", "Banana" }
            }
          }
        }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"role="combobox" id="fruit""#), "{html}");
    assert!(html.contains(r#"aria-controls="fruit-list" aria-expanded="true""#), "{html}");
    assert!(html.contains(r#"id="fruit-list""#));
    assert!(html.contains(r#"aria-labelledby="fruit""#));
    assert!(!html.contains(" hidden"), "{html}");
    assert_eq!(html.matches(r#"aria-selected="true""#).count(), 1);
  }

  #[test]
  fn ssr_passed_aria_controls_replaces_the_derived_one() {
    fn app() -> Element {
      rsx! {
        Combobox { id: "fruit",
          ComboboxInput { "aria-controls": "custom-list" }
        }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"aria-controls="custom-list""#));
    assert!(!html.contains("fruit-list"));
  }

  #[test]
  fn ssr_multiple_list_is_multiselectable() {
    fn app() -> Element {
      rsx! {
        Combobox { multiple: true, default_values: vec!["a".to_string()],
          ComboboxContent {
            ComboboxList {
              ComboboxItem { value: "a", "A" }
              ComboboxItem { value: "b", "B" }
            }
          }
        }
      }
    }
    let html = render(app);

    assert!(html.contains("aria-multiselectable=\"true\""));
    assert_eq!(html.matches("after:opacity-100").count(), 1);
    assert!(html.contains(r#"id="dxui-combobox-0-input-list""#), "{html}");
  }

  #[test]
  fn ssr_combobox_part_outside_its_root_renders_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        ComboboxItem { value: "a", "Apple" }
      }
    }
    let html = render(app);

    assert!(html.contains("before"));
    assert!(!html.contains("Apple"), "{html}");
  }

  #[test]
  fn combobox_trigger_class_reflects_invalid_state() {
    let actual = combobox_trigger_class(true, "w-60");

    assert_eq!(
      actual,
      "flex h-10 items-center justify-between rounded-md border bg-background px-3 py-2 text-sm text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50 border-destructive focus-visible:ring-destructive w-60"
    );
    assert!(actual.contains("border-destructive focus-visible:ring-destructive"));
    assert!(actual.ends_with("w-60"));
  }

  #[test]
  fn combobox_item_class_reflects_active_and_selected_state() {
    let actual = combobox_item_class(true, true, "gap-2");

    assert!(actual.contains(COMBOBOX_ITEM_BASE_CLASS));
    assert!(actual.contains("bg-accent text-accent-foreground"));
    assert!(actual.ends_with("gap-2"));
  }

  #[test]
  fn combobox_primitive_config_is_reexported() {
    let config = PopoverPrimitiveConfig::controlled(true);

    assert!(config.open);
  }
}
