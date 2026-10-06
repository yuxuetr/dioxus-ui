use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

pub const DIFF_BASE_CLASS: &str = "relative isolate grid overflow-hidden rounded-md select-none";
pub const DIFF_LAYER_CLASS: &str = "col-start-1 row-start-1 [&>img]:size-full [&>img]:object-cover";
// The after layer shows right of the divider: the frame sets
// `--diff-position`, so the parts need no props.
pub const DIFF_AFTER_CLASS: &str = "[clip-path:inset(0_0_0_var(--diff-position))]";
// The range input covers the frame, invisible, so dragging, clicking, and the
// arrow keys all move the divider; the handle after it shows its focus.
pub const DIFF_INPUT_CLASS: &str =
  "peer absolute inset-0 z-20 size-full cursor-ew-resize appearance-none opacity-0";
pub const DIFF_DIVIDER_CLASS: &str = "pointer-events-none absolute inset-y-0 left-(--diff-position) z-10 w-0.5 -translate-x-1/2 bg-background shadow-[0_0_0_1px_var(--color-border)]";
pub const DIFF_HANDLE_CLASS: &str = "pointer-events-none absolute top-1/2 left-(--diff-position) z-10 grid size-8 -translate-x-1/2 -translate-y-1/2 place-items-center rounded-full border border-border bg-background text-xs text-muted-foreground shadow-sm peer-focus-visible:ring-2 peer-focus-visible:ring-ring";

pub fn diff_class(class: &str) -> String {
  merge_classes(classes([Some(DIFF_BASE_CLASS)]), class)
}

pub fn diff_layer_class(after: bool, class: &str) -> String {
  merge_classes(classes([Some(DIFF_LAYER_CLASS), after.then_some(DIFF_AFTER_CLASS)]), class)
}

/// The divider position clamped to 0 through 100; anything that is not a
/// number is the middle.
pub fn diff_position(position: f64) -> f64 {
  if position.is_nan() { 50.0 } else { position.clamp(0.0, 100.0) }
}

/// A before-and-after comparison. `DiffBefore` shows left of the divider and
/// `DiffAfter` right of it, at `position` percent. A native range input
/// moves the divider by drag, click, or arrow keys and calls
/// `on_position_change`; `label` names it.
#[component]
pub fn Diff(
  #[props(default = 50.0)] position: f64,
  #[props(default)] on_position_change: Option<EventHandler<f64>>,
  #[props(default = "Comparison position".to_string())] label: String,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = diff_class(&class);
  let position = diff_position(position);

  rsx! {
    div { class, style: "--diff-position: {position}%", ..attributes,
      {children}
      // Range inputs run right to left in an RTL page; the clip does not.
      input {
        class: DIFF_INPUT_CLASS,
        r#type: "range",
        dir: "ltr",
        min: "0",
        max: "100",
        step: "1",
        value: "{position}",
        "aria-label": label,
        oninput: move |event| {
          if let (Some(handler), Ok(value)) = (on_position_change, event.value().parse::<f64>()) {
            handler.call(diff_position(value));
          }
        },
      }
      div { class: DIFF_DIVIDER_CLASS, "aria-hidden": "true" }
      div { class: DIFF_HANDLE_CLASS, "aria-hidden": "true", "↔" }
    }
  }
}

#[component]
pub fn DiffBefore(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = diff_layer_class(false, &class);

  rsx! {
    div { class, ..attributes, {children} }
  }
}

#[component]
pub fn DiffAfter(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = diff_layer_class(true, &class);

  rsx! {
    div { class, ..attributes, {children} }
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
  fn ssr_diff_sets_the_position_and_a_named_range() {
    fn app() -> Element {
      rsx! {
        Diff { position: 30.0,
          DiffBefore { "Before" }
          DiffAfter { "After" }
        }
      }
    }
    let html = render(app);

    assert!(html.contains("style=\"--diff-position: 30%\""));
    assert!(html.contains("type=\"range\""));
    assert!(html.contains("value=\"30\""));
    assert!(html.contains("aria-label=\"Comparison position\""));
    assert!(html.contains("dir=\"ltr\""));
    assert!(html.contains("[clip-path:inset(0_0_0_var(--diff-position))]\">After</div>"));
    assert!(!html.contains("clip-path:inset(0_0_0_var(--diff-position))]\">Before"));
  }

  #[test]
  fn position_is_clamped() {
    assert_eq!(diff_position(-1.0), 0.0);
    assert_eq!(diff_position(250.0), 100.0);
    assert_eq!(diff_position(f64::NAN), 50.0);
  }
}
