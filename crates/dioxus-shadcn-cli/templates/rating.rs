use super::element_id::next_element_id;
use super::utils::classes;
use dioxus::prelude::*;

pub const RATING_BASE_CLASS: &str = "inline-flex items-center gap-1";
// The mask that draws the star would clip a focus outline, so the wrapper
// shows the ring for the focused radio.
pub const RATING_STAR_WRAPPER_CLASS: &str = "inline-flex rounded-sm has-focus-visible:ring-2 has-focus-visible:ring-ring has-focus-visible:ring-offset-2 has-focus-visible:ring-offset-background";
pub const RATING_STAR_CLASS: &str = "size-6 cursor-pointer appearance-none bg-muted-foreground/40 [mask:url(data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20viewBox=%270%200%2024%2024%27%3E%3Cpath%20d=%27M12%202l3.09%206.26L22%209.27l-5%204.87%201.18%206.88L12%2017.77l-6.18%203.25L7%2014.14%202%209.27l6.91-1.01L12%202z%27/%3E%3C/svg%3E)_center/contain_no-repeat] focus-visible:outline-none data-[filled=true]:bg-warning disabled:cursor-not-allowed disabled:opacity-50";

pub fn rating_class(class: &str) -> String {
  classes([Some(RATING_BASE_CLASS), Some(class)])
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
  let generated_name = use_hook(|| format!("dxui-rating-{}", next_element_id()));
  let name = name.unwrap_or(generated_name);

  rsx! {
    div { class, role: "radiogroup", ..attributes,
      for star in 1..=max {
        span { key: "{star}", class: RATING_STAR_WRAPPER_CLASS,
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
