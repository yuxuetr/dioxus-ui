//! Input OTP: visual slots for a one-time code, backed by a native input for
//! forms, mobile keyboards, and assistive technology.
use super::element_id::next_element_id;
use super::root_state::{Controllable, use_controllable, use_root_context};
use super::utils::{classes, merge_classes};
use super::density::{density_control_class, use_density, with_density};
use super::script::{Script, component_script};
use dioxus::prelude::*;

// Keep in sync with `input_otp_filter_script` in the `dioxus-shadcn` crate.
// Runs for the input's lifetime. Its listener on the input itself runs before
// the delegated Dioxus handler, so it can drop rejected characters from the
// native value first. Without it the input would keep characters the app
// never saw, and Backspace would remove those instead of the last digit.
component_script!(input_otp_filter_script = r#"
export async function run(dioxus) {
  const scopeId = await dioxus.recv();
  const input = document.querySelector(`[data-dxui-otp-input="${scopeId}"]`);
  if (!input) return;
  // Mirrors `InputOtpInputMode::allows`.
  const allowed = { numeric: /[0-9]/, text: /[\p{Alphabetic}\p{N}]/u };
  const onInput = () => {
    const pattern = allowed[input.inputMode] || allowed.text;
    const length = Number(input.dataset.length) || 0;
    const next = Array.from(input.value)
      .filter((ch) => pattern.test(ch))
      .slice(0, length)
      .join("");
    if (next !== input.value) input.value = next;
  };
  let finish;
  const ended = new Promise((resolve) => {
    finish = resolve;
  });
  const observer = new MutationObserver(() => {
    if (!input.isConnected) finish();
  });
  observer.observe(document.documentElement, { subtree: true, childList: true });
  input.addEventListener("input", onInput);
  await ended;
  observer.disconnect();
}
"#);

/// The render state of one one-time-code input slot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OtpSlotState {
  pub index: usize,
  /// The character in the slot, or `None` when empty.
  pub value: Option<char>,
  pub active: bool,
  pub disabled: bool,
}

/// Which characters a code accepts, and which keyboard phones show.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum InputOtpInputMode {
  /// Digits only, with a numeric keyboard.
  #[default]
  Numeric,
  /// Letters and digits.
  Text,
}

const INPUT_OTP_BASE_CLASS: &str = "relative flex items-center gap-2";
const INPUT_OTP_DISABLED_CLASS: &str = "opacity-50";
const INPUT_OTP_GROUP_BASE_CLASS: &str = "flex items-center gap-1";
const INPUT_OTP_SLOT_BASE_CLASS: &str = "relative flex h-10 w-10 items-center justify-center rounded-md border text-sm font-medium transition-colors";
const INPUT_OTP_SLOT_ACTIVE_CLASS: &str = "border-ring ring-2 ring-ring";
const INPUT_OTP_SLOT_INVALID_CLASS: &str = "border-destructive ring-2 ring-destructive";
const INPUT_OTP_SLOT_DISABLED_CLASS: &str = "cursor-not-allowed bg-muted text-muted-foreground";
const INPUT_OTP_SEPARATOR_BASE_CLASS: &str = "flex items-center px-1 text-muted-foreground";
const INPUT_OTP_HIDDEN_INPUT_BASE_CLASS: &str =
  "absolute inset-0 h-full w-full cursor-text opacity-0 disabled:cursor-not-allowed";

impl InputOtpInputMode {
  /// The input's `inputmode` value: `numeric` or `text`.
  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Numeric => "numeric",
      Self::Text => "text",
    }
  }

  /// Numeric codes keep ASCII digits; text codes keep letters and digits.
  pub fn allows(self, ch: char) -> bool {
    match self {
      Self::Numeric => ch.is_ascii_digit(),
      Self::Text => ch.is_alphanumeric(),
    }
  }
}

/// Keeps the characters `input_mode` allows, up to `length` of them, so typed
/// or pasted text such as `"123-456"` becomes a code.
pub fn input_otp_sanitize(value: &str, length: usize, input_mode: InputOtpInputMode) -> String {
  value.chars().filter(|ch| input_mode.allows(*ch)).take(length).collect()
}

/// Slot states for `length` slots holding `value`, with the caret at
/// `active_index`; characters past `length` are ignored.
pub fn otp_slots(value: &str, length: usize, active_index: usize) -> Vec<OtpSlotState> {
  otp_slots_with_disabled(value, length, active_index, &[])
}

/// Like [`otp_slots`], but marks the slots at `disabled_indices` disabled and
/// keeps the caret off them.
pub fn otp_slots_with_disabled(
  value: &str,
  length: usize,
  active_index: usize,
  disabled_indices: &[usize],
) -> Vec<OtpSlotState> {
  let chars = otp_value_chars(value, length);

  (0..length)
    .map(|index| {
      let disabled = disabled_indices.contains(&index);

      OtpSlotState {
        index,
        value: chars.get(index).copied(),
        active: index == active_index && !disabled,
        disabled,
      }
    })
    .collect()
}

/// Whether `value` has at least `length` characters.
pub fn otp_is_complete(value: &str, length: usize) -> bool {
  value.chars().take(length.saturating_add(1)).count() >= length
}

/// Writes `ch` at slot `index`, replacing what is there; see
/// [`otp_insert_char_filtered`].
pub fn otp_insert_char(value: &str, index: usize, ch: char, length: usize) -> String {
  otp_insert_char_filtered(value, index, ch, length, |_| true)
}

/// Writes `ch` at slot `index`, replacing what is there.
///
/// An index past the end appends, so the code never has holes. Returns the
/// value cut to `length` characters, unchanged otherwise when `allow_char`
/// rejects `ch` or `length` is 0.
pub fn otp_insert_char_filtered<F>(
  value: &str,
  index: usize,
  ch: char,
  length: usize,
  allow_char: F,
) -> String
where
  F: Fn(char) -> bool,
{
  if length == 0 || !allow_char(ch) {
    return otp_clamp_value(value, length);
  }

  let mut chars = otp_value_chars(value, length);
  let index = insertion_index(index, chars.len(), length);

  if index < chars.len() {
    chars[index] = ch;
  } else {
    chars.push(ch);
  }

  chars.into_iter().take(length).collect()
}

/// Removes the character at `index`, shifting later ones left; an index past
/// the end changes nothing.
pub fn otp_delete_char(value: &str, index: usize) -> String {
  let mut chars: Vec<char> = value.chars().collect();

  if index < chars.len() {
    chars.remove(index);
  }

  chars.into_iter().collect()
}

/// Writes pasted text starting at slot `index`; see
/// [`otp_apply_paste_filtered`].
pub fn otp_apply_paste(value: &str, index: usize, paste: &str, length: usize) -> String {
  otp_apply_paste_filtered(value, index, paste, length, |_| true)
}

/// Writes the characters of `paste` that `allow_char` accepts into
/// consecutive slots from `index`, overwriting existing ones.
///
/// Characters that do not fit in `length` slots are dropped. When nothing in
/// `paste` is accepted, returns the value cut to `length`; when `length` is 0,
/// returns an empty string.
pub fn otp_apply_paste_filtered<F>(
  value: &str,
  index: usize,
  paste: &str,
  length: usize,
  allow_char: F,
) -> String
where
  F: Fn(char) -> bool,
{
  if length == 0 {
    return String::new();
  }

  let paste_chars: Vec<char> = paste.chars().filter(|ch| allow_char(*ch)).collect();

  if paste_chars.is_empty() {
    return otp_clamp_value(value, length);
  }

  let mut chars = otp_value_chars(value, length);
  let start = insertion_index(index, chars.len(), length);
  let mut paste_index = 0;
  let mut current_index = start;

  while current_index < length && paste_index < paste_chars.len() {
    if current_index < chars.len() {
      chars[current_index] = paste_chars[paste_index];
    } else {
      chars.push(paste_chars[paste_index]);
    }

    paste_index += 1;
    current_index += 1;
  }

  chars.into_iter().take(length).collect()
}

/// The slot the caret moves to after typing at `index`: the first empty slot
/// or the next one, whichever is later, capped at the last slot. Returns 0
/// when `length` is 0.
pub fn otp_next_index(value: &str, index: usize, length: usize) -> usize {
  if length == 0 {
    return 0;
  }

  let value_len = value.chars().take(length).count();
  let next = if value_len >= length {
    index.saturating_add(1)
  } else {
    value_len.max(index.saturating_add(1))
  };

  next.min(length - 1)
}

/// The slot before `index`, stopping at 0.
pub const fn otp_previous_index(index: usize) -> usize {
  index.saturating_sub(1)
}

/// `value` cut to its first `length` characters.
pub fn otp_clamp_value(value: &str, length: usize) -> String {
  value.chars().take(length).collect()
}

/// Classes for the root: base classes, dimmed when `disabled`, then `class`
/// merged over them.
pub fn input_otp_class(disabled: bool, class: &str) -> String {
  merge_classes(classes([Some(INPUT_OTP_BASE_CLASS), disabled.then_some(INPUT_OTP_DISABLED_CLASS)]), class)
}

/// Classes for a group of slots, with `class` merged over them.
pub fn input_otp_group_class(class: &str) -> String {
  merge_classes(classes([Some(INPUT_OTP_GROUP_BASE_CLASS)]), class)
}

/// Classes for one slot: base classes, a ring when `invalid` (destructive) or
/// `active`, muted colors when `disabled`, then `class` merged over them.
pub fn input_otp_slot_class(active: bool, invalid: bool, disabled: bool, class: &str) -> String {
  merge_classes(classes([Some(INPUT_OTP_SLOT_BASE_CLASS), Some(if invalid {
      INPUT_OTP_SLOT_INVALID_CLASS
    } else if active {
      INPUT_OTP_SLOT_ACTIVE_CLASS
    } else {
      "border-input"
    }), Some(if disabled { INPUT_OTP_SLOT_DISABLED_CLASS } else { "bg-background text-foreground" })]), class)
}

fn input_otp_slot_display(value: Option<char>) -> String {
  value.map(|ch| ch.to_string()).unwrap_or_default()
}

/// Classes for the separator between groups, with `class` merged over them.
pub fn input_otp_separator_class(class: &str) -> String {
  merge_classes(classes([Some(INPUT_OTP_SEPARATOR_BASE_CLASS)]), class)
}

/// Classes for the transparent native input laid over the slots, with `class`
/// merged over them.
pub fn input_otp_hidden_input_class(class: &str) -> String {
  merge_classes(classes([Some(INPUT_OTP_HIDDEN_INPUT_BASE_CLASS)]), class)
}

fn otp_value_chars(value: &str, length: usize) -> Vec<char> {
  value.chars().take(length).collect()
}

fn insertion_index(index: usize, value_len: usize, length: usize) -> usize {
  index.min(value_len).min(length)
}

#[derive(Clone, Copy)]
struct InputOtpContext {
  value: Controllable<String>,
  length: usize,
  disabled: bool,
  invalid: bool,
}

fn use_input_otp(part: &str) -> InputOtpContext {
  use_root_context::<Signal<InputOtpContext>>(part, "InputOtp").cloned()
}

/// The root of a one-time code of `length` characters: it owns the code
/// (RFC 0077). Pass `value` to control it, or `default_value` to start it;
/// `on_value_change` hears every change the user makes either way. The slots
/// show it and `InputOtpHiddenInput` takes the typing.
#[component]
pub fn InputOtp(
  length: usize,
  #[props(default)] value: ReadSignal<Option<String>>,
  #[props(default)] default_value: String,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let value = use_controllable(move || value.cloned(), move || default_value, on_value_change);
  let context = InputOtpContext { value, length, disabled, invalid };
  // `length`, `disabled`, and `invalid` may change, so the context is
  // refreshed on every render.
  let mut shared = use_context_provider(|| Signal::new(context));
  let stale = *shared.peek();
  if (stale.length, stale.disabled, stale.invalid) != (length, disabled, invalid) {
    shared.set(context);
  }
  let class = input_otp_class(disabled, &class);

  rsx! {
    div {
      class,
      role: "group",
      "aria-disabled": disabled.to_string(),
      "aria-invalid": invalid.to_string(),
      "data-disabled": disabled.to_string(),
      "data-invalid": invalid.to_string(),
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn InputOtpGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = input_otp_group_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// Shows the code's character at `index`, counting from zero. The slot the
/// next character goes to is active; `disabled` disables this slot alone.
#[component]
pub fn InputOtpSlot(
  index: usize,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
) -> Element {
  let otp = use_input_otp("InputOtpSlot");
  let code = otp.value.get();
  let filled = code.chars().count();
  let slot = otp_slots(&code, otp.length, filled.min(otp.length.saturating_sub(1)))
    .into_iter()
    .find(|slot| slot.index == index);
  let value = slot.as_ref().and_then(|slot| slot.value);
  let disabled = disabled || otp.disabled;
  let active = slot.is_some_and(|slot| slot.active) && !disabled;
  let invalid = otp.invalid;
  let class = input_otp_slot_class(active, invalid, disabled, &with_density(density_control_class(use_density()), &class));
  let display = input_otp_slot_display(value);

  rsx! {
    div {
      class,
      role: "presentation",
      "data-index": index.to_string(),
      "data-active": active.to_string(),
      "data-disabled": disabled.to_string(),
      "data-invalid": invalid.to_string(),
      span {
        "aria-hidden": "true",
        {display}
      }
    }
  }
}

#[component]
pub fn InputOtpSeparator(
  #[props(default)] class: String,
  #[props(default = String::from("-"))] content: String,
) -> Element {
  let class = input_otp_separator_class(&class);

  rsx! {
    div {
      class,
      role: "presentation",
      "aria-hidden": "true",
      {content}
    }
  }
}

/// The transparent input over the slots that takes the typing, a paste, or
/// an autofill, keeping the characters `input_mode` allows. Name it with
/// `aria-label` or `aria-labelledby`.
#[component]
pub fn InputOtpHiddenInput(
  #[props(default)] name: Option<String>,
  #[props(default)] input_mode: InputOtpInputMode,
  #[props(default = Some(String::from("one-time-code")))] autocomplete: Option<String>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
  let otp = use_input_otp("InputOtpHiddenInput");
  let class = input_otp_hidden_input_class(&class);
  let inputmode = input_mode.attribute();
  let value = otp.value.get();
  let current = value.clone();
  let length = otp.length;
  let scope_id = use_input_otp_filter();

  rsx! {
    input {
      r#type: "text",
      class,
      value,
      name,
      inputmode,
      autocomplete,
      disabled: otp.disabled,
      "aria-invalid": otp.invalid.to_string(),
      "data-length": length.to_string(),
      "data-dxui-otp-input": scope_id,
      oninput: move |event: FormEvent| {
        let next = input_otp_sanitize(&event.value(), length, input_mode);
        if next != current {
          otp.value.set(next);
        }
      },
      ..attributes,
    }
  }
}

/// Runs the filter script for the input's lifetime and returns the value for
/// its `data-dxui-otp-input` attribute.
fn use_input_otp_filter() -> String {
  let scope_id = use_hook(|| format!("dxui-otp-{}", next_element_id()));
  let effect_scope_id = scope_id.clone();

  use_effect(move || {
    let script = input_otp_filter_script::start();
    // A send error means the page already finished the script; nothing to track.
    let _ = script.send(effect_scope_id.as_str());
  });

  scope_id
}
