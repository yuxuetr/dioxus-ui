use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
  // Assets live inside the crate so `cargo package` ships them with the CLI.
  let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
  let registry_dir = manifest_dir.join("registry");
  let out_dir = PathBuf::from(env::var("OUT_DIR")?);
  let generated_path = out_dir.join("embedded_assets.rs");

  println!("cargo:rerun-if-changed={}", registry_dir.display());
  println!("cargo:rerun-if-changed={}", manifest_dir.join("templates").display());

  let mut registry_paths = Vec::new();
  let mut asset_sources = BTreeSet::new();

  for entry in fs::read_dir(&registry_dir)? {
    let path = entry?.path();

    if path.file_name().and_then(|name| name.to_str()) == Some("schema.json") {
      continue;
    }

    if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
      continue;
    }

    let json = fs::read_to_string(&path)?;

    for source in registry_sources(&json)? {
      asset_sources.insert(source);
    }

    registry_paths.push(path);
  }

  registry_paths.sort();

  // Blocks (RFC 0073) and helpers (RFC 0074), in the registry format, kept
  // apart from the component registry.
  let block_paths = entry_paths(&manifest_dir.join("blocks"), &mut asset_sources)?;
  let helper_paths = entry_paths(&manifest_dir.join("helpers"), &mut asset_sources)?;

  let mut generated = String::new();
  generated.push_str("struct EmbeddedAsset {\n");
  generated.push_str("  source: &'static str,\n");
  generated.push_str("  content: &'static str,\n");
  generated.push_str("}\n\n");
  push_json_list(&mut generated, "EMBEDDED_REGISTRY_JSON", &registry_paths);
  push_json_list(&mut generated, "EMBEDDED_BLOCK_JSON", &block_paths);
  push_json_list(&mut generated, "EMBEDDED_HELPER_JSON", &helper_paths);
  generated.push_str("const EMBEDDED_ASSETS: &[EmbeddedAsset] = &[\n");

  for source in asset_sources {
    let path = manifest_dir.join(&source);

    println!("cargo:rerun-if-changed={}", path.display());

    generated.push_str("  EmbeddedAsset {\n");
    generated.push_str("    source: \"");
    generated.push_str(&escape_rust_string(&source));
    generated.push_str("\",\n");
    generated.push_str("    content: include_str!(\"");
    generated.push_str(&escape_rust_string(&path));
    generated.push_str("\"),\n");
    generated.push_str("  },\n");
  }

  generated.push_str("];\n\n");

  // Theme presets (RFC 0057), one file per preset, named by its file stem.
  let themes_dir = manifest_dir.join("themes");
  println!("cargo:rerun-if-changed={}", themes_dir.display());

  let mut theme_paths = Vec::new();
  for entry in fs::read_dir(&themes_dir)? {
    let path = entry?.path();
    if path.extension().and_then(|extension| extension.to_str()) == Some("css") {
      theme_paths.push(path);
    }
  }
  theme_paths.sort();

  generated.push_str("const EMBEDDED_THEMES: &[(&str, &str)] = &[\n");
  for path in theme_paths {
    let name = path
      .file_stem()
      .and_then(|stem| stem.to_str())
      .ok_or("theme preset file name is not valid UTF-8")?;
    generated.push_str("  (\"");
    generated.push_str(name);
    generated.push_str("\", include_str!(\"");
    generated.push_str(&escape_rust_string(&path));
    generated.push_str("\")),\n");
  }
  generated.push_str("];\n");

  fs::write(generated_path, generated)?;
  Ok(())
}

/// The registry-format entries in `dir`, sorted, adding their sources to
/// `asset_sources`.
fn entry_paths(
  dir: &Path,
  asset_sources: &mut BTreeSet<String>,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
  println!("cargo:rerun-if-changed={}", dir.display());
  let mut paths = Vec::new();
  for entry in fs::read_dir(dir)? {
    let path = entry?.path();
    if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
      continue;
    }
    asset_sources.extend(registry_sources(&fs::read_to_string(&path)?)?);
    paths.push(path);
  }
  paths.sort();
  Ok(paths)
}

fn push_json_list(generated: &mut String, name: &str, paths: &[PathBuf]) {
  generated.push_str("const ");
  generated.push_str(name);
  generated.push_str(": &[&str] = &[\n");
  for path in paths {
    generated.push_str("  include_str!(\"");
    generated.push_str(&escape_rust_string(path));
    generated.push_str("\"),\n");
  }
  generated.push_str("];\n\n");
}

fn registry_sources(json: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let value = serde_json::from_str::<Value>(json)?;
  let mut sources = Vec::new();

  for key in ["files", "assets"] {
    let Some(items) = value.get(key).and_then(Value::as_array) else {
      continue;
    };

    for item in items {
      let source = item
        .get("source")
        .and_then(Value::as_str)
        .ok_or("registry path mapping is missing a string source")?;

      sources.push(source.to_string());
    }
  }

  Ok(sources)
}

fn escape_rust_string(value: impl AsRef<Path>) -> String {
  value.as_ref().display().to_string().replace('\\', "\\\\").replace('"', "\\\"")
}
