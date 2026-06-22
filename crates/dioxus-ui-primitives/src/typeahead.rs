/// Item metadata used by typeahead matching.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeaheadItem {
  pub id: String,
  pub label: String,
  pub disabled: bool,
}

impl TypeaheadItem {
  pub fn enabled(id: impl Into<String>, label: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      label: label.into(),
      disabled: false,
    }
  }

  pub fn disabled(id: impl Into<String>, label: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      label: label.into(),
      disabled: true,
    }
  }
}

/// Pure state for typeahead search buffers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeaheadState {
  pub buffer: String,
  pub timeout_ms: u16,
  pub last_input_ms: Option<u64>,
}

impl TypeaheadState {
  pub const fn new(timeout_ms: u16) -> Self {
    Self {
      buffer: String::new(),
      timeout_ms,
      last_input_ms: None,
    }
  }

  pub fn input(&self, character: char, now_ms: u64) -> Self {
    if character.is_control() {
      return self.clone();
    }

    let mut buffer = if self.should_reset(now_ms) {
      String::new()
    } else {
      self.buffer.clone()
    };

    buffer.push(character.to_ascii_lowercase());

    Self {
      buffer,
      timeout_ms: self.timeout_ms,
      last_input_ms: Some(now_ms),
    }
  }

  pub fn clear(&self) -> Self {
    Self {
      buffer: String::new(),
      timeout_ms: self.timeout_ms,
      last_input_ms: None,
    }
  }

  pub fn match_item<'a>(
    &self,
    items: &'a [TypeaheadItem],
    active_id: Option<&str>,
  ) -> Option<&'a str> {
    match_typeahead(items, &self.buffer, active_id)
  }

  fn should_reset(&self, now_ms: u64) -> bool {
    self.last_input_ms
      .map(|last_input_ms| now_ms.saturating_sub(last_input_ms) > u64::from(self.timeout_ms))
      .unwrap_or(true)
  }
}

impl Default for TypeaheadState {
  fn default() -> Self {
    Self::new(700)
  }
}

pub fn match_typeahead<'a>(
  items: &'a [TypeaheadItem],
  query: &str,
  active_id: Option<&str>,
) -> Option<&'a str> {
  let query = query.trim().to_ascii_lowercase();

  if query.is_empty() || items.is_empty() {
    return None;
  }

  let start = active_id
    .and_then(|id| items.iter().position(|item| item.id == id))
    .map(|index| index + 1)
    .unwrap_or(0);

  for offset in 0..items.len() {
    let index = (start + offset) % items.len();
    let item = &items[index];

    if item.disabled {
      continue;
    }

    if item.label.to_ascii_lowercase().starts_with(&query) {
      return Some(item.id.as_str());
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;

  fn items() -> Vec<TypeaheadItem> {
    vec![
      TypeaheadItem::enabled("apple", "Apple"),
      TypeaheadItem::disabled("apricot", "Apricot"),
      TypeaheadItem::enabled("banana", "Banana"),
      TypeaheadItem::enabled("blueberry", "Blueberry"),
    ]
  }

  #[test]
  fn input_appends_to_buffer_before_timeout() {
    let state = TypeaheadState::default().input('b', 100).input('l', 200);

    assert_eq!(state.buffer, "bl");
    assert_eq!(state.last_input_ms, Some(200));
  }

  #[test]
  fn input_resets_buffer_after_timeout() {
    let state = TypeaheadState::default().input('b', 100).input('a', 900);

    assert_eq!(state.buffer, "a");
  }

  #[test]
  fn match_skips_disabled_items() {
    assert_eq!(match_typeahead(&items(), "ap", None), Some("apple"));
  }

  #[test]
  fn match_starts_after_active_item() {
    assert_eq!(match_typeahead(&items(), "b", Some("banana")), Some("blueberry"));
  }

  #[test]
  fn state_matches_current_buffer() {
    let state = TypeaheadState::default().input('b', 100).input('l', 200);

    assert_eq!(state.match_item(&items(), None), Some("blueberry"));
  }

  #[test]
  fn clear_resets_buffer_and_timestamp() {
    let state = TypeaheadState::default().input('b', 100).clear();

    assert!(state.buffer.is_empty());
    assert_eq!(state.last_input_ms, None);
  }
}
