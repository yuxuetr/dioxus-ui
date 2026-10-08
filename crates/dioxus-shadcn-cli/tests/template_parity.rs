//! Source-copy templates are hand-maintained copies of the crate modules
//! (RFC 0066). This test compares them item by item so a change made to one
//! copy fails until it is made to the other.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use proc_macro2::{Delimiter, Group, Literal, TokenStream, TokenTree};
use quote::ToTokens;

fn workspace_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .nth(2)
    .expect("cli crate should be nested under workspace/crates")
    .to_path_buf()
}

fn rust_files(dir: &Path) -> Vec<PathBuf> {
  let mut files = fs::read_dir(dir)
    .unwrap_or_else(|error| panic!("read {}: {error}", dir.display()))
    .map(|entry| entry.expect("directory entry").path())
    .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
    .collect::<Vec<_>>();
  files.sort();
  files
}

/// Item tokens with formatting, comments, and doc attributes removed, string
/// literals reduced to their values, and the differences templates make on
/// purpose dropped: visibility (templates keep inlined helpers private),
/// `#[cfg(feature = ...)]` gates (templates have no features), and trailing
/// commas left by line width.
fn normalize(tokens: TokenStream) -> TokenStream {
  let mut normalized = Vec::new();
  let mut tokens = tokens.into_iter().peekable();
  while let Some(token) = tokens.next() {
    match token {
      TokenTree::Punct(punct) if punct.as_char() == '#' => {
        let dropped = matches!(tokens.peek(), Some(TokenTree::Group(group))
        if group.delimiter() == Delimiter::Bracket && {
          let attribute = group.stream().to_string();
          attribute.starts_with("doc") || (attribute.starts_with("cfg") && attribute.contains("feature"))
        });
        if dropped {
          tokens.next();
        } else {
          normalized.push(TokenTree::Punct(punct));
        }
      }
      TokenTree::Ident(ident) if ident == "pub" => {
        if matches!(tokens.peek(), Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis)
        {
          tokens.next();
        }
      }
      TokenTree::Punct(punct) if punct.as_char() == ',' && tokens.peek().is_none() => {}
      TokenTree::Group(group) => {
        normalized.push(TokenTree::Group(Group::new(group.delimiter(), normalize(group.stream()))));
      }
      TokenTree::Literal(literal) => {
        let canonical = syn::parse_str::<syn::LitStr>(&literal.to_string())
          .map(|string| Literal::string(&string.value()))
          .unwrap_or(literal);
        normalized.push(TokenTree::Literal(canonical));
      }
      other => normalized.push(other),
    }
  }
  normalized.into_iter().collect()
}

fn is_test_module(item: &syn::Item) -> bool {
  matches!(item, syn::Item::Mod(module)
    if module.attrs.iter().any(|attr| attr.to_token_stream().to_string().contains("cfg (test)")))
}

/// Keyed, normalized items of one file. An `impl` block contributes one
/// entry per associated item, since a template inlines only the methods of a
/// primitive it uses.
fn items(path: &Path) -> BTreeMap<String, String> {
  let source =
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
  let file =
    syn::parse_file(&source).unwrap_or_else(|error| panic!("parse {}: {error}", path.display()));
  let mut items = BTreeMap::new();
  let mut insert = |key: String, tokens: TokenStream| {
    let previous = items.insert(key.clone(), normalize(tokens).to_string());
    assert!(previous.is_none(), "{} defines {key} twice", path.display());
  };
  for item in &file.items {
    let name = match item {
      syn::Item::Use(_) | syn::Item::ExternCrate(_) => continue,
      _ if is_test_module(item) => continue,
      syn::Item::Impl(block) => {
        let header = match &block.trait_ {
          Some((_, path, _)) => {
            format!("impl {} for {}", path.to_token_stream(), block.self_ty.to_token_stream())
          }
          None => format!("impl {}", block.self_ty.to_token_stream()),
        };
        for impl_item in &block.items {
          let member = match impl_item {
            syn::ImplItem::Const(member) => member.ident.to_string(),
            syn::ImplItem::Fn(member) => member.sig.ident.to_string(),
            syn::ImplItem::Type(member) => member.ident.to_string(),
            other => other.to_token_stream().to_string(),
          };
          insert(format!("{header} :: {member}"), impl_item.to_token_stream());
        }
        continue;
      }
      syn::Item::Const(item) => item.ident.to_string(),
      syn::Item::Enum(item) => item.ident.to_string(),
      syn::Item::Fn(item) => item.sig.ident.to_string(),
      // `macro_rules! name` by its name; an invocation such as
      // `component_script!(name = ...)` by the macro and its first token.
      syn::Item::Macro(item) => match &item.ident {
        Some(ident) => ident.to_string(),
        None => format!(
          "{}!({})",
          item.mac.path.to_token_stream(),
          item
            .mac
            .tokens
            .clone()
            .into_iter()
            .next()
            .map(|token| token.to_string())
            .unwrap_or_default()
        ),
      },
      syn::Item::Mod(item) => item.ident.to_string(),
      syn::Item::Static(item) => item.ident.to_string(),
      syn::Item::Struct(item) => item.ident.to_string(),
      syn::Item::Trait(item) => item.ident.to_string(),
      syn::Item::Type(item) => item.ident.to_string(),
      syn::Item::Union(item) => item.ident.to_string(),
      other => other.to_token_stream().to_string(),
    };
    insert(name, item.to_token_stream());
  }
  items
}

/// Where a template item may come from besides its own crate module: the
/// primitives and core crates, which templates cannot import, and the crate's
/// other modules, whose items a template may inline.
struct Sources {
  by_key: BTreeMap<String, Vec<(String, String)>>,
}

impl Sources {
  fn load(root: &Path) -> Self {
    let mut by_key = BTreeMap::<String, Vec<(String, String)>>::new();
    for dir in [
      "crates/dioxus-shadcn-primitives/src",
      "crates/dioxus-shadcn-core/src",
      "crates/dioxus-shadcn/src",
    ] {
      for path in rust_files(&root.join(dir)) {
        let label = path.strip_prefix(root).unwrap_or(&path).display().to_string();
        for (key, tokens) in items(&path) {
          by_key.entry(key).or_default().push((label.clone(), tokens));
        }
      }
    }
    Self { by_key }
  }

  /// `Ok` when some source defines the item identically; otherwise the
  /// sources that define it differently, empty when none does.
  fn find(&self, key: &str, tokens: &str) -> Result<(), Vec<(String, String)>> {
    let candidates = self.by_key.get(key).map(Vec::as_slice).unwrap_or_default();
    if candidates.iter().any(|(_, candidate)| candidate == tokens) {
      Ok(())
    } else {
      Err(candidates.to_vec())
    }
  }
}

/// Both sides around the first token where they diverge.
fn first_difference(template: &str, source: &str) -> String {
  let start = template
    .char_indices()
    .zip(source.chars())
    .find(|((_, left), right)| left != right)
    .map(|((index, _), _)| index)
    .unwrap_or_else(|| template.len().min(source.len()));
  let window = |text: &str| -> String {
    text
      .char_indices()
      .filter(|(index, _)| index + 40 >= start && *index < start + 60)
      .map(|(_, character)| character)
      .collect()
  };
  format!("\n    template: ...{}...\n    source:   ...{}...", window(template), window(source))
}

fn describe(key: &str, tokens: &str, mismatched: &[(String, String)]) -> String {
  match mismatched.first() {
    None => format!("{key}: only in the template"),
    Some((label, source)) => {
      format!("{key}: differs from {label}{}", first_difference(tokens, source))
    }
  }
}

/// Crate items templates leave out on purpose: (template, item, reason).
const CRATE_ONLY: &[(&str, &str, &str)] = &[(
  "chart.rs",
  "CHART_COLOR_CLASSES",
  "lists classes for Tailwind's crate scan; templates inline the primitive that spells them",
)];

#[test]
fn templates_match_crate_modules() {
  let root = workspace_root();
  let sources = Sources::load(&root);
  let templates = root.join("crates/dioxus-shadcn-cli/templates");
  let mut drift = Vec::new();

  for template_path in rust_files(&templates) {
    let file_name = template_path.file_name().and_then(|name| name.to_str()).unwrap_or_default();
    let module_path = root.join("crates/dioxus-shadcn/src").join(file_name);
    let template_items = items(&template_path);
    let module_items = if module_path.is_file() { items(&module_path) } else { BTreeMap::new() };

    for (key, tokens) in &template_items {
      let problem = match module_items.get(key) {
        Some(module_tokens) if module_tokens == tokens => None,
        Some(module_tokens) => Some(format!(
          "{key}: differs from crates/dioxus-shadcn/src/{file_name}{}",
          first_difference(tokens, module_tokens)
        )),
        None => {
          sources.find(key, tokens).err().map(|mismatched| describe(key, tokens, &mismatched))
        }
      };
      if let Some(problem) = problem {
        drift.push(format!("templates/{file_name}: {problem}"));
      }
    }
    for key in module_items.keys().filter(|key| !template_items.contains_key(*key)) {
      let allowed = CRATE_ONLY.iter().any(|(file, item, _)| *file == file_name && item == key);
      if !allowed {
        drift.push(format!("templates/{file_name}: {key}: only in the crate module"));
      }
    }
  }

  for (file, item, _) in CRATE_ONLY {
    let module_path = root.join("crates/dioxus-shadcn/src").join(file);
    let template_path = templates.join(file);
    let still_crate_only =
      items(&module_path).contains_key(*item) && !items(&template_path).contains_key(*item);
    if !still_crate_only {
      drift
        .push(format!("CRATE_ONLY lists templates/{file}: {item}, which is no longer crate only"));
    }
  }

  assert!(drift.is_empty(), "{} template items drifted:\n{}", drift.len(), drift.join("\n"));
}

/// The `class` parameters and props a file declares, by name: `class: &str`,
/// `track_class: String`, and the like.
fn user_class_names(tokens: TokenStream, names: &mut Vec<String>) {
  let tokens = tokens.into_iter().collect::<Vec<_>>();
  for (index, token) in tokens.iter().enumerate() {
    match token {
      TokenTree::Group(group) => user_class_names(group.stream(), names),
      TokenTree::Ident(ident) if ident.to_string().ends_with("class") => {
        let rest = tokens[index + 1..].iter().take(3).map(ToString::to_string).collect::<String>();
        if rest.starts_with(":&str") || rest.starts_with(":String") {
          names.push(ident.to_string());
        }
      }
      _ => {}
    }
  }
}

/// Elements of `classes([...])` calls that are a user class, which must go
/// through `merge_classes` instead (RFC 0076).
fn user_classes_in_classes_calls(tokens: TokenStream, names: &[String], found: &mut Vec<String>) {
  let tokens = tokens.into_iter().collect::<Vec<_>>();
  for (index, token) in tokens.iter().enumerate() {
    let TokenTree::Group(group) = token else {
      continue;
    };
    let is_classes_call = matches!(tokens.get(index.wrapping_sub(1)), Some(TokenTree::Ident(ident)) if ident == "classes")
      && group.delimiter() == Delimiter::Parenthesis;
    if is_classes_call {
      for element in group.stream().to_string().replace(' ', "").trim_matches(['[', ']']).split(',')
      {
        let inner = element.strip_prefix("Some(").and_then(|rest| rest.strip_suffix(')'));
        let name = inner.map(|inner| inner.trim_start_matches('&').trim_end_matches(".as_str()"));
        if name.is_some_and(|name| names.iter().any(|candidate| candidate == name)) {
          found.push(element.to_string());
        }
      }
    }
    user_classes_in_classes_calls(group.stream(), names, found);
  }
}

#[test]
fn class_functions_merge_the_user_class() {
  let root = workspace_root();
  let mut appended = Vec::new();
  for dir in ["crates/dioxus-shadcn/src", "crates/dioxus-shadcn-cli/templates"] {
    for path in rust_files(&root.join(dir)) {
      let source = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
      // The test module starts at a line's start; a `#[cfg(test)]` inside a
      // macro, such as `component_script!`'s, is indented.
      let source = source.split("\n#[cfg(test)]").next().unwrap_or_default();
      let tokens = source
        .parse::<TokenStream>()
        .unwrap_or_else(|error| panic!("lex {}: {error}", path.display()));
      let mut names = Vec::new();
      user_class_names(tokens.clone(), &mut names);
      let mut found = Vec::new();
      user_classes_in_classes_calls(tokens, &names, &mut found);
      appended.extend(found.into_iter().map(|element| {
        format!("{}: {element}", path.strip_prefix(&root).unwrap_or(&path).display())
      }));
    }
  }
  assert!(
    appended.is_empty(),
    "class functions append a user class without merge_classes:\n{}",
    appended.join("\n")
  );
}
