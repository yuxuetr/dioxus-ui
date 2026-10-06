use super::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use super::default_attribute::default_attribute;
use super::element_id::next_element_id;
use super::listbox::{ListboxMode, use_listbox};
pub use super::overlay::{DismissBehavior, OverlayAlign, OverlaySide, SelectPrimitiveConfig};
use super::root_state::{Controllable, use_controllable, use_root_context};
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

pub const SELECT_TRIGGER_BASE_CLASS: &str = "flex h-10 w-full items-center justify-between rounded-md border bg-background px-3 py-2 text-sm text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
pub const SELECT_VALUE_BASE_CLASS: &str = "truncate";
pub const SELECT_CONTENT_BASE_CLASS: &str = "z-50 max-h-96 min-w-[max(8rem,var(--dxui-anchor-width,0px))] overflow-hidden rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-md";
pub const SELECT_GROUP_BASE_CLASS: &str = "p-1";
pub const SELECT_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-muted-foreground";
// A selected option shows a check mark at the inline end, so it stays
// distinct from the highlighted one, which takes the accent background
// (RFC 0062).
pub const SELECT_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm text-foreground outline-none transition-colors focus:bg-accent data-highlighted:bg-accent data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 pe-8 after:absolute after:end-2 after:size-4 after:bg-current after:[mask:url(data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20viewBox=%270%200%2016%2016%27%20fill=%27none%27%20stroke=%27black%27%20stroke-width=%272%27%20stroke-linecap=%27round%27%20stroke-linejoin=%27round%27%3E%3Cpath%20d=%27M3.5%208.5l3%203%206-7%27/%3E%3C/svg%3E)_center/contain_no-repeat]";
pub const SELECT_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-border";

pub fn select_trigger_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  merge_classes(classes([Some(SELECT_TRIGGER_BASE_CLASS), Some(invalid_class)]), class)
}

pub fn select_value_class(class: &str) -> String {
  merge_classes(classes([Some(SELECT_VALUE_BASE_CLASS)]), class)
}

pub fn select_content_class(class: &str) -> String {
  merge_classes(classes([Some(SELECT_CONTENT_BASE_CLASS)]), class)
}

pub fn select_group_class(class: &str) -> String {
  merge_classes(classes([Some(SELECT_GROUP_BASE_CLASS)]), class)
}

pub fn select_label_class(class: &str) -> String {
  merge_classes(classes([Some(SELECT_LABEL_BASE_CLASS)]), class)
}

pub fn select_item_class(selected: bool, class: &str) -> String {
  let selected_class = if selected { "after:opacity-100" } else { "after:opacity-0" };

  merge_classes(classes([Some(SELECT_ITEM_BASE_CLASS), Some(selected_class)]), class)
}

pub fn select_separator_class(class: &str) -> String {
  merge_classes(classes([Some(SELECT_SEPARATOR_BASE_CLASS)]), class)
}

/// Click requests `!open` through `on_open_change`, and ArrowDown or ArrowUp
/// on a closed trigger requests open. Pass `id` as the content's `anchor_id`.
/// What a `Select` shares with its parts (RFC 0077).
#[derive(Clone)]
struct SelectContext {
  trigger_id: String,
  multiple: bool,
  single: Controllable<Option<String>>,
  many: Controllable<Vec<String>>,
  open: Controllable<bool>,
  choose: Callback<String>,
  set_open: Callback<bool>,
}

impl SelectContext {
  fn trigger_id(&self) -> String {
    self.trigger_id.clone()
  }

  fn content_id(&self) -> String {
    format!("{}-content", self.trigger_id)
  }

  fn chosen(&self) -> Vec<String> {
    if self.multiple { self.many.get() } else { self.single.get().into_iter().collect() }
  }
}

/// The root of a select: it owns the chosen value, or values with
/// `multiple`, and whether the list is open, and links its parts. Pass
/// `value` (`values`) or `open` to control them, or `default_value`
/// (`default_values`) and `default_open` to start them; the change callbacks
/// hear every change the user makes either way. `id` names the trigger, for
/// a `Label` to point at; without it the ids are generated.
#[component]
pub fn Select(
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
  let generated = use_hook(|| format!("dxui-select-{}-trigger", next_element_id()));
  let trigger_id = id.unwrap_or(generated);
  let single_change = use_callback(move |next: Option<String>| {
    if let (Some(handler), Some(next)) = (on_value_change, next) {
      handler.call(next);
    }
  });
  let single =
    use_controllable(move || value().map(Some), move || default_value, Some(single_change));
  let many = use_controllable(move || values.cloned(), move || default_values, on_values_change);
  let open = use_controllable(move || open.cloned(), move || default_open, on_open_change);
  let choose = use_callback(move |chosen: String| {
    if multiple {
      let mut next = many.get();
      match next.iter().position(|value| *value == chosen) {
        Some(index) => {
          next.remove(index);
        }
        None => next.push(chosen),
      }
      many.set(next);
    } else {
      single.set(Some(chosen));
    }
  });
  let set_open = use_callback(move |next: bool| open.set(next));
  use_context_provider(|| SelectContext {
    trigger_id,
    multiple,
    single,
    many,
    open,
    choose,
    set_open,
  });

  rsx! { {children} }
}

#[component]
pub fn SelectTrigger(
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let context = use_root_context::<SelectContext>("SelectTrigger", "Select");
  let class = select_trigger_class(invalid, &class);
  let open = context.open.get();
  let set_open = context.set_open;
  let controls = default_attribute(&attributes, "aria-controls", context.content_id());

  rsx! {
    button {
      r#type: "button",
      role: "combobox",
      id: context.trigger_id(),
      class,
      disabled,
      "aria-controls": controls,
      "aria-expanded": open.to_string(),
      "aria-haspopup": "listbox",
      "aria-invalid": invalid.to_string(),
      onclick: move |_| set_open.call(!open),
      onkeydown: move |event| {
        if !open && matches!(event.key(), Key::ArrowDown | Key::ArrowUp) {
          event.prevent_default();
          set_open.call(true);
        }
      },
      ..attributes,
      {children}
    }
  }
}

/// The chosen value, the chosen values joined with commas, or the
/// placeholder. To show labels other than the values, put them in the
/// `SelectTrigger` instead.
#[component]
pub fn SelectValue(
  #[props(default)] placeholder: String,
  #[props(default)] class: String,
) -> Element {
  let context = use_root_context::<SelectContext>("SelectValue", "Select");
  let class = select_value_class(&class);
  let chosen = context.chosen().join(", ");
  let empty = chosen.is_empty();
  let text = if empty { placeholder } else { chosen };

  rsx! {
    span {
      class,
      "data-placeholder": empty.then_some("true"),
      "{text}"
    }
  }
}

#[component]
pub fn SelectContent(
  #[props(default)] class: String,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::Start)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let context = use_root_context::<SelectContext>("SelectContent", "Select");
  let class = select_content_class(&class);
  let open = context.open.get();
  let anchor_id = Some(context.trigger_id());
  let closes_on_choice = (!context.multiple).then_some(context.set_open);
  let listbox = use_listbox(
    open,
    anchor_id.clone(),
    ListboxMode::Select,
    Some(context.choose),
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
      role: "listbox",
      id: context.content_id(),
      class,
      "aria-labelledby": context.trigger_id(),
      "aria-multiselectable": context.multiple.then_some("true"),
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-anchored": anchored,
      "data-dxui-listbox": listbox,
      {children}
    }
  }
}

#[component]
pub fn SelectGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = select_group_class(&class);

  rsx! {
    div {
      role: "group",
      class,
      {children}
    }
  }
}

#[component]
pub fn SelectLabel(#[props(default)] class: String, children: Element) -> Element {
  let class = select_label_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn SelectItem(
  value: String,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let context = use_root_context::<SelectContext>("SelectItem", "Select");
  let selected = context.chosen().contains(&value);
  let class = select_item_class(selected, &class);

  rsx! {
    div {
      role: "option",
      class,
      "aria-selected": selected.to_string(),
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      "data-value": value,
      {children}
    }
  }
}

#[component]
pub fn SelectSeparator(#[props(default)] class: String) -> Element {
  let class = select_separator_class(&class);

  rsx! {
    div {
      role: "separator",
      class,
    }
  }
}
