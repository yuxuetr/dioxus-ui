//! Renders each component's docs page, from its API Surface section on, to
//! HTML for the component site (M199.2), so the site shows the reference
//! instead of linking to GitHub.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::{Component, Path, PathBuf};

use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, html};

/// Replaced at runtime with the site's base path, such as `/dioxus-ui`.
const SITE_BASE: &str = "__SITE_BASE__";

fn main() {
  let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let root = manifest.parent().expect("the site sits in the workspace root");
  let docs = root.join("docs/components");
  let registry = root.join("crates/dioxus-shadcn-cli/registry");
  println!("cargo:rerun-if-changed={}", docs.display());
  println!("cargo:rerun-if-changed={}", registry.display());

  let slugs = fs::read_dir(&registry)
    .expect("read the CLI registry")
    .filter_map(|entry| entry.ok()?.path().file_stem()?.to_str().map(str::to_string))
    .filter(|name| name != "utils" && name != "schema")
    .collect::<BTreeSet<_>>();
  let repository = env!("CARGO_PKG_REPOSITORY");

  let mut out = String::from("pub const REFERENCES: &[(&str, &str)] = &[\n");
  for slug in &slugs {
    let path = docs.join(format!("{slug}.md"));
    let markdown = fs::read_to_string(&path).expect("read a component docs page");
    let start = markdown.find("## API Surface").unwrap_or(markdown.len());
    let parser =
      Parser::new_ext(&markdown[start..], Options::ENABLE_TABLES).map(|event| match event {
        Event::Start(Tag::Link { link_type, dest_url, title, id }) => Event::Start(Tag::Link {
          link_type,
          dest_url: rewrite_link(&dest_url, &slugs, repository),
          title,
          id,
        }),
        other => other,
      });
    let mut rendered = String::new();
    html::push_html(&mut rendered, parser);
    // A code block that scrolls sideways must take keyboard focus.
    let rendered = rendered.replace("<pre>", r#"<pre tabindex="0">"#);
    writeln!(out, "  ({slug:?}, {rendered:?}),").expect("write to a string");
  }
  out.push_str("];\n");

  let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("cargo sets OUT_DIR"));
  fs::write(out_dir.join("references.rs"), out).expect("write the rendered references");
}

/// Links to another component's page go to its site route; other relative
/// links go to the file on GitHub; absolute links and fragments stay.
fn rewrite_link(dest: &str, slugs: &BTreeSet<String>, repository: &str) -> CowStr<'static> {
  if dest.starts_with('#') || dest.contains("://") || dest.starts_with("mailto:") {
    return CowStr::from(dest.to_string());
  }
  let (path, fragment) =
    dest.split_once('#').map_or((dest, ""), |(path, fragment)| (path, fragment));
  let fragment = if fragment.is_empty() { String::new() } else { format!("#{fragment}") };
  if let Some(stem) = path.strip_suffix(".md").filter(|stem| slugs.contains(*stem)) {
    return CowStr::from(format!("{SITE_BASE}/components/{stem}{fragment}"));
  }
  let mut resolved = PathBuf::from("docs/components");
  for part in Path::new(path).components() {
    match part {
      Component::ParentDir => {
        resolved.pop();
      }
      Component::Normal(name) => resolved.push(name),
      _ => {}
    }
  }
  CowStr::from(format!("{repository}/blob/main/{}{fragment}", resolved.display()))
}
