//! The generated-table merge candidate (RFC 0076): each utility is classified
//! into the longhand slots it sets, and a component utility is dropped when a
//! user utility under the same variants and importance sets all of them.
//! A token the table cannot classify removes nothing and is never removed.

#[rustfmt::skip]
mod table;

use table::{GROUPS, PROPERTIES, ROOTS, SETS, STATIC};

/// The type of an arbitrary value, as Tailwind infers it or a hint names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
  Color,
  Length,
  Percentage,
  Number,
  Url,
  Image,
  Shadow,
}

/// A value form a root accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Value {
  /// The root alone, such as `rounded`.
  Bare,
  /// `px-13`.
  Integer,
  /// `px-2.25`, in quarters.
  Decimal,
  /// `w-1/3`.
  Fraction,
  /// A named value from one of the keyword groups, such as `bg-primary`.
  Keyword(u16),
  /// `text-[13px]`, with the kind inferred from the value.
  Literal(Kind),
  /// `text-[length:var(--x)]` or `text-(length:--x)`.
  Hinted(Kind),
  /// `bg-[var(--x)]` or `bg-(--x)`.
  Var,
  /// Any arbitrary value, on a root with one slot set.
  Arbitrary,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Utility<'a> {
  variants: &'a str,
  important: bool,
  slots: &'static [u16],
}

pub(crate) fn classify(token: &str) -> Option<Utility<'_>> {
  let (variants, utility) = match last_top_level_colon(token) {
    Some(index) => (&token[..index], &token[index + 1..]),
    None => ("", token),
  };
  let (utility, important) = match utility.strip_suffix('!').or_else(|| utility.strip_prefix('!')) {
    Some(rest) => (rest, true),
    None => (utility, false),
  };
  Some(Utility { variants, important, slots: lookup(utility)? })
}

/// Joins the component classes and the user classes, without the component
/// utilities a user utility replaces.
pub(crate) fn merge(component: &str, user: &str) -> String {
  let users = user.split_whitespace().filter_map(classify).collect::<Vec<_>>();
  let mut merged = String::with_capacity(component.len() + user.len() + 1);
  let tokens = component
    .split_whitespace()
    .filter(|token| {
      !classify(token).is_some_and(|own| {
        users.iter().any(|other| {
          other.variants == own.variants
            && other.important == own.important
            && is_subset(own.slots, other.slots)
        })
      })
    })
    .chain(user.split_whitespace());
  for token in tokens {
    if !merged.is_empty() {
      merged.push(' ');
    }
    merged.push_str(token);
  }
  merged
}

fn is_subset(inner: &[u16], outer: &[u16]) -> bool {
  let mut outer = outer.iter();
  inner.iter().all(|slot| outer.any(|other| other == slot))
}

fn last_top_level_colon(token: &str) -> Option<usize> {
  let mut depth = 0i32;
  let mut last = None;
  for (index, byte) in token.bytes().enumerate() {
    match byte {
      b'[' | b'(' => depth += 1,
      b']' | b')' => depth -= 1,
      b':' if depth == 0 => last = Some(index),
      _ => {}
    }
  }
  last
}

fn lookup(utility: &str) -> Option<&'static [u16]> {
  if let Ok(index) = STATIC.binary_search_by(|(name, _)| name.cmp(&utility)) {
    return Some(SETS[STATIC[index].1 as usize]);
  }
  if let Some(inner) = utility.strip_prefix('[').and_then(|rest| rest.strip_suffix(']')) {
    let (property, _) = inner.split_once(':')?;
    let index = PROPERTIES.binary_search_by(|(name, _)| name.cmp(&property)).ok()?;
    return Some(SETS[PROPERTIES[index].1 as usize]);
  }

  // Roots from the longest: `border-t-2` is `border-t` with `2` before it is
  // `border` with `t-2`. Dashes inside an arbitrary value do not split.
  let named_end = utility.find(['[', '(']).unwrap_or(utility.len());
  let splits = std::iter::once(utility.len()).chain(
    utility[..named_end].rmatch_indices('-').map(|(index, _)| index).filter(|index| *index > 0),
  );
  for split in splits {
    let root = &utility[..split];
    let Ok(index) = ROOTS.binary_search_by(|(name, _)| name.cmp(&root)) else {
      continue;
    };
    let value = utility.get(split + 1..).unwrap_or("");
    if let Some(set) = match_value(ROOTS[index].1, value) {
      return Some(SETS[set as usize]);
    }
  }
  None
}

fn match_value(rules: &[(Value, u16)], value: &str) -> Option<u16> {
  let form = value_form(value)?;
  rules.iter().find_map(|(rule, set)| {
    let matches = match (rule, form) {
      (Value::Keyword(group), Form::Named(name)) => {
        GROUPS[*group as usize].binary_search(&name).is_ok()
      }
      (Value::Arbitrary, Form::Arbitrary(_)) => true,
      (rule, Form::Arbitrary(Some(value))) => *rule == value,
      (rule, Form::Plain(value)) => *rule == value,
      _ => false,
    };
    matches.then_some(*set)
  })
}

#[derive(Clone, Copy)]
enum Form<'a> {
  Plain(Value),
  Named(&'a str),
  /// An arbitrary value and its form, or `None` for one of no inferable kind.
  Arbitrary(Option<Value>),
}

fn value_form(value: &str) -> Option<Form<'_>> {
  if value.is_empty() {
    return Some(Form::Plain(Value::Bare));
  }
  if let Some(inner) = value.strip_prefix('[').and_then(|rest| rest.strip_suffix(']')) {
    return Some(Form::Arbitrary(arbitrary_form(inner, false)));
  }
  if let Some(inner) = value.strip_prefix('(').and_then(|rest| rest.strip_suffix(')')) {
    return Some(Form::Arbitrary(arbitrary_form(inner, true)));
  }
  let is_digits = |text: &str| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit());
  if is_digits(value) {
    return Some(Form::Plain(Value::Integer));
  }
  if let Some((whole, quarter)) = value.split_once('.') {
    return (is_digits(whole) && matches!(quarter, "25" | "5" | "75"))
      .then_some(Form::Plain(Value::Decimal));
  }
  if let Some((numerator, denominator)) = value.split_once('/')
    && is_digits(numerator)
    && is_digits(denominator)
  {
    return Some(Form::Plain(Value::Fraction));
  }
  // `primary/50`: the opacity modifier does not change the slots.
  let name = value.split_once('/').map_or(value, |(name, _)| name);
  Some(Form::Named(name))
}

/// `var(--x)` or a `(--x)` shorthand, a `kind:` hint, or a literal whose kind
/// the value shows. A value of no inferable kind is `None`.
fn arbitrary_form(inner: &str, shorthand: bool) -> Option<Value> {
  if let Some((hint, rest)) = inner.split_once(':')
    && !hint.is_empty()
    && hint.bytes().all(|byte| byte.is_ascii_lowercase() || byte == b'-')
  {
    let kind = match hint {
      "color" => Kind::Color,
      "length" => Kind::Length,
      "percentage" => Kind::Percentage,
      "number" => Kind::Number,
      "url" => Kind::Url,
      "image" => Kind::Image,
      "shadow" => Kind::Shadow,
      _ => return None,
    };
    let is_var = if shorthand { rest.starts_with("--") } else { rest.starts_with("var(") };
    return is_var.then_some(Value::Hinted(kind));
  }
  if shorthand {
    return inner.starts_with("--").then_some(Value::Var);
  }
  if inner.starts_with("var(") {
    return Some(Value::Var);
  }
  literal_kind(inner).map(Value::Literal)
}

fn literal_kind(value: &str) -> Option<Kind> {
  const COLOR_FUNCTIONS: &[&str] = &[
    "rgb(",
    "rgba(",
    "hsl(",
    "hsla(",
    "hwb(",
    "lab(",
    "lch(",
    "oklab(",
    "oklch(",
    "color(",
    "color-mix(",
  ];
  const LENGTH_UNITS: &[&str] = &[
    "px", "rem", "em", "ex", "ch", "lh", "rlh", "vw", "vh", "vmin", "vmax", "svw", "svh", "lvw",
    "lvh", "dvw", "dvh", "cqw", "cqh", "cqi", "cqb", "cqmin", "cqmax", "cm", "mm", "in", "pt",
    "pc", "q",
  ];
  let is_number = |text: &str| {
    let text = text.strip_prefix('-').unwrap_or(text);
    !text.is_empty()
      && text.bytes().all(|byte| byte.is_ascii_digit() || byte == b'.')
      && text.bytes().any(|b| b.is_ascii_digit())
  };
  if let Some(hex) = value.strip_prefix('#') {
    return (matches!(hex.len(), 3 | 4 | 6 | 8)
      && hex.bytes().all(|byte| byte.is_ascii_hexdigit()))
    .then_some(Kind::Color);
  }
  if COLOR_FUNCTIONS.iter().any(|function| value.starts_with(function)) {
    return Some(Kind::Color);
  }
  if value.starts_with("url(") {
    return Some(Kind::Url);
  }
  if ["linear-gradient(", "radial-gradient(", "conic-gradient(", "repeating-"]
    .iter()
    .any(|prefix| value.starts_with(prefix))
  {
    return Some(Kind::Image);
  }
  if value.strip_suffix('%').is_some_and(is_number) {
    return Some(Kind::Percentage);
  }
  if is_number(value) {
    return Some(Kind::Number);
  }
  let is_length = |text: &str| {
    let digits_end =
      text.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-')).unwrap_or(text.len());
    is_number(&text[..digits_end])
      && (LENGTH_UNITS.contains(&&text[digits_end..]) || text[digits_end..].is_empty())
  };
  if is_length(value) {
    return (!is_number(value)).then_some(Kind::Length);
  }
  // `0_0_0_1px_var(--x)`: each comma-separated shadow has an x and a y offset.
  let is_shadow =
    value.split(',').all(|shadow| shadow.split('_').filter(|part| is_length(part)).count() >= 2);
  is_shadow.then_some(Kind::Shadow)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn last_wins_and_partial_overlaps_keep_both() {
    assert_eq!(merge("h-10 px-4 py-2", "px-2"), "h-10 py-2 px-2");
    assert_eq!(merge("px-4", "p-2"), "p-2");
    assert_eq!(merge("p-4", "px-2"), "p-4 px-2");
    assert_eq!(
      merge("bg-primary hover:bg-primary/90", "bg-accent"),
      "hover:bg-primary/90 bg-accent"
    );
    assert_eq!(merge("text-sm text-muted-foreground", "text-primary"), "text-sm text-primary");
  }

  #[test]
  fn unknown_tokens_change_nothing() {
    assert_eq!(
      merge("group bg-primary", "group/item app-card bg-primry px-foo"),
      "group bg-primary group/item app-card bg-primry px-foo"
    );
    assert_eq!(merge("border-b", "border-b-primary"), "border-b border-b-primary");
  }
}
