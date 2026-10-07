//! Rating: lets the user pick a score from one to five stars, or to another
//! maximum.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

use crate::density::{density_hit_area_class, use_density};
use crate::element_id::next_element_id;

const RATING_BASE_CLASS: &str = "inline-flex items-center gap-1";
// The mask that draws the star would clip a focus outline, so the wrapper
// shows the ring for the focused radio.
const RATING_STAR_WRAPPER_CLASS: &str = "inline-flex rounded-sm has-focus-visible:ring-2 has-focus-visible:ring-ring has-focus-visible:ring-offset-2 has-focus-visible:ring-offset-background";
const RATING_STAR_CLASS: &str = "size-6 cursor-pointer appearance-none bg-muted-foreground/40 [mask:url(data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20viewBox=%270%200%2024%2024%27%3E%3Cpath%20d=%27M12%202l3.09%206.26L22%209.27l-5%204.87%201.18%206.88L12%2017.77l-6.18%203.25L7%2014.14%202%209.27l6.91-1.01L12%202z%27/%3E%3C/svg%3E)_center/contain_no-repeat] focus-visible:outline-none data-[filled=true]:bg-warning disabled:cursor-not-allowed disabled:opacity-50";

/// Classes for the row of stars, with `class` merged over them.
pub fn rating_class(class: &str) -> String {
  merge_classes(classes([Some(RATING_BASE_CLASS)]), class)
}

/// A star rating from 0 (none) to `max`, as a radio group: the arrow keys
/// move between stars, and a change calls `on_value_change` with the chosen
/// star. Name the group with `aria-label`; each star is named "{n} of {max}".
#[component]
pub fn Rating(
  #[props(default)] value: u8,
  #[props(default = 5)] max: u8,
  #[props(default)] name: Option<String>,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] on_value_change: Option<EventHandler<u8>>,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
  let class = rating_class(&class);
  // A label passes a press on the star's hit area to its radio (RFC 0078).
  let star_class = merge_classes(
    classes([Some(RATING_STAR_WRAPPER_CLASS)]),
    density_hit_area_class(use_density()),
  );
  let generated_name = use_hook(|| format!("dxui-rating-{}", next_element_id()));
  let name = name.unwrap_or(generated_name);

  rsx! {
    div { class, role: "radiogroup", ..attributes,
      for star in 1..=max {
        label { key: "{star}", class: star_class.clone(),
          input {
            class: RATING_STAR_CLASS,
            r#type: "radio",
            name: name.clone(),
            value: "{star}",
            checked: star == value,
            disabled,
            "aria-label": "{star} of {max}",
            "data-filled": star <= value,
            onchange: move |_| {
              if let Some(handler) = on_value_change {
                handler.call(star);
              }
            },
          }
        }
      }
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
  fn ssr_rating_is_a_group_of_named_radios() {
    fn app() -> Element {
      rsx! { Rating { value: 3, name: "product", "aria-label": "Product rating" } }
    }
    let html = render(app);

    assert!(html.contains("role=\"radiogroup\""));
    assert!(html.contains("aria-label=\"Product rating\""));
    assert_eq!(html.matches("type=\"radio\"").count(), 5);
    assert_eq!(html.matches("name=\"product\"").count(), 5);
    assert!(html.contains("aria-label=\"3 of 5\""));
    assert_eq!(
      html.matches("data-filled=true").count() + html.matches("data-filled=\"true\"").count(),
      3
    );
    assert_eq!(html.matches("checked").count(), 1);
  }

  #[test]
  fn ssr_ratings_get_distinct_names() {
    fn app() -> Element {
      rsx! {
        Rating { max: 1 }
        Rating { max: 1 }
      }
    }
    let html = render(app);
    let names =
      html.split("name=\"").skip(1).filter_map(|rest| rest.split('"').next()).collect::<Vec<_>>();

    assert_eq!(names.len(), 2);
    assert_ne!(names[0], names[1]);
  }
}
