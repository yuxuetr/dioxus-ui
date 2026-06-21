/// Compose class fragments into a deterministic class string.
///
/// This helper does not construct Tailwind tokens. It only joins complete class
/// fragments that already exist in source.
pub fn classes<I, S>(parts: I) -> String
where
  I: IntoIterator<Item = Option<S>>,
  S: AsRef<str>,
{
  let mut output = String::new();

  for part in parts.into_iter().flatten() {
    for token in part.as_ref().split_whitespace() {
      if !output.is_empty() {
        output.push(' ');
      }

      output.push_str(token);
    }
  }

  output
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn joins_class_fragments_in_order() {
    let actual = classes([
      Some("inline-flex items-center"),
      Some("h-10 px-4"),
      Some("bg-blue-600 text-white"),
    ]);

    assert_eq!(
      actual,
      "inline-flex items-center h-10 px-4 bg-blue-600 text-white"
    );
  }

  #[test]
  fn skips_none_and_blank_fragments() {
    let actual = classes([Some("  inline-flex  "), None, Some(""), Some("px-4")]);

    assert_eq!(actual, "inline-flex px-4");
  }

  #[test]
  fn preserves_duplicate_tokens_for_tailwind_ordering() {
    let actual = classes([Some("px-3 bg-blue-600"), Some("px-4")]);

    assert_eq!(actual, "px-3 bg-blue-600 px-4");
  }
}
