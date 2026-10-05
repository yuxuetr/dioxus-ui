#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OtpSlotState {
  pub index: usize,
  pub value: Option<char>,
  pub active: bool,
  pub disabled: bool,
}

pub fn otp_slots(value: &str, length: usize, active_index: usize) -> Vec<OtpSlotState> {
  otp_slots_with_disabled(value, length, active_index, &[])
}

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

pub fn otp_is_complete(value: &str, length: usize) -> bool {
  value.chars().take(length.saturating_add(1)).count() >= length
}

pub fn otp_insert_char(value: &str, index: usize, ch: char, length: usize) -> String {
  otp_insert_char_filtered(value, index, ch, length, |_| true)
}

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

pub fn otp_delete_char(value: &str, index: usize) -> String {
  let mut chars: Vec<char> = value.chars().collect();

  if index < chars.len() {
    chars.remove(index);
  }

  chars.into_iter().collect()
}

pub fn otp_apply_paste(value: &str, index: usize, paste: &str, length: usize) -> String {
  otp_apply_paste_filtered(value, index, paste, length, |_| true)
}

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

pub const fn otp_previous_index(index: usize) -> usize {
  index.saturating_sub(1)
}

pub fn otp_clamp_value(value: &str, length: usize) -> String {
  value.chars().take(length).collect()
}

fn otp_value_chars(value: &str, length: usize) -> Vec<char> {
  value.chars().take(length).collect()
}

fn insertion_index(index: usize, value_len: usize, length: usize) -> usize {
  index.min(value_len).min(length)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn builds_slot_state_with_active_and_disabled_slots() {
    let slots = otp_slots_with_disabled("12", 4, 2, &[1, 3]);

    assert_eq!(
      slots,
      vec![
        OtpSlotState { index: 0, value: Some('1'), active: false, disabled: false },
        OtpSlotState { index: 1, value: Some('2'), active: false, disabled: true },
        OtpSlotState { index: 2, value: None, active: true, disabled: false },
        OtpSlotState { index: 3, value: None, active: false, disabled: true },
      ]
    );
  }

  #[test]
  fn reports_completion_against_requested_length() {
    assert!(otp_is_complete("", 0));
    assert!(!otp_is_complete("12", 4));
    assert!(otp_is_complete("1234", 4));
    assert!(otp_is_complete("12345", 4));
  }

  #[test]
  fn inserts_char_at_slot_and_clamps_to_length() {
    assert_eq!(otp_insert_char("12", 2, '3', 4), "123");
    assert_eq!(otp_insert_char("1234", 1, '9', 4), "1934");
    assert_eq!(otp_insert_char("12", 99, '3', 4), "123");
    assert_eq!(otp_insert_char("1234", 4, '5', 4), "1234");
  }

  #[test]
  fn filters_invalid_insert_characters() {
    let numeric = |ch: char| ch.is_ascii_digit();

    assert_eq!(otp_insert_char_filtered("12", 2, 'a', 4, numeric), "12");
    assert_eq!(otp_insert_char_filtered("12", 2, '3', 4, numeric), "123");
  }

  #[test]
  fn deletes_char_at_boundaries_without_panicking() {
    assert_eq!(otp_delete_char("1234", 0), "234");
    assert_eq!(otp_delete_char("1234", 2), "124");
    assert_eq!(otp_delete_char("1234", 99), "1234");
    assert_eq!(otp_delete_char("", 0), "");
  }

  #[test]
  fn applies_paste_from_index_and_truncates_overflow() {
    assert_eq!(otp_apply_paste("1234", 1, "ab", 4), "1ab4");
    assert_eq!(otp_apply_paste("1", 1, "23456", 4), "1234");
    assert_eq!(otp_apply_paste("12", 99, "34", 4), "1234");
  }

  #[test]
  fn ignores_empty_or_invalid_paste() {
    let numeric = |ch: char| ch.is_ascii_digit();

    assert_eq!(otp_apply_paste("12", 1, "", 4), "12");
    assert_eq!(otp_apply_paste_filtered("12", 1, "ab", 4, numeric), "12");
    assert_eq!(otp_apply_paste_filtered("12", 1, "a34", 4, numeric), "134");
  }

  #[test]
  fn calculates_next_and_previous_indices() {
    assert_eq!(otp_next_index("", 0, 0), 0);
    assert_eq!(otp_next_index("", 0, 4), 1);
    assert_eq!(otp_next_index("12", 1, 4), 2);
    assert_eq!(otp_next_index("1234", 3, 4), 3);
    assert_eq!(otp_previous_index(0), 0);
    assert_eq!(otp_previous_index(3), 2);
  }

  #[test]
  fn clamps_value_to_slot_length() {
    assert_eq!(otp_clamp_value("12345", 4), "1234");
    assert_eq!(otp_clamp_value("12345", 0), "");
  }
}
