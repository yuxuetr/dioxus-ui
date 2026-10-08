//! Safe URL: keeps a link's `href` only when following it cannot run script.

/// The schemes a link may use. Others, such as `javascript:` and `data:`,
/// can run script when the link is followed.
const SAFE_SCHEMES: [&str; 4] = ["http", "https", "mailto", "tel"];

/// `href` when it is relative, a fragment, or uses a scheme in
/// `SAFE_SCHEMES`; otherwise `None`, which renders no `href`. The scheme is
/// read as a browser reads it: without surrounding spaces and control
/// characters, and without tabs and line breaks anywhere.
pub(crate) fn safe_href(href: String) -> Option<String> {
  let cleaned = href
    .trim_matches(|character: char| character <= ' ')
    .chars()
    .filter(|character| !matches!(character, '\t' | '\n' | '\r'))
    .collect::<String>();
  match cleaned.split_once(':') {
    Some((scheme, _)) if is_scheme(scheme) => {
      SAFE_SCHEMES.iter().any(|safe| scheme.eq_ignore_ascii_case(safe)).then_some(href)
    }
    _ => Some(href),
  }
}

/// Whether `text` is a URL scheme: a letter, then letters, digits, `+`, `-`,
/// or `.`. Text before a colon that is not one, such as `/a` in `/a:b`, makes
/// the URL relative.
fn is_scheme(text: &str) -> bool {
  let mut characters = text.chars();
  characters.next().is_some_and(|character| character.is_ascii_alphabetic())
    && characters
      .all(|character| character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.'))
}
