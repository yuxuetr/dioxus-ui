use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
  let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
  let workspace_root = manifest_dir
    .ancestors()
    .nth(2)
    .map(Path::to_path_buf)
    .ok_or("failed to resolve workspace root")?;
  let registry_dir = workspace_root.join("registry");
  let out_dir = PathBuf::from(env::var("OUT_DIR")?);
  let generated_path = out_dir.join("embedded_assets.rs");

  println!("cargo:rerun-if-changed={}", registry_dir.display());
  println!("cargo:rerun-if-changed={}", workspace_root.join("templates").display());

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

  let mut generated = String::new();
  generated.push_str("struct EmbeddedAsset {\n");
  generated.push_str("  source: &'static str,\n");
  generated.push_str("  content: &'static str,\n");
  generated.push_str("}\n\n");
  generated.push_str("const EMBEDDED_REGISTRY_JSON: &[&str] = &[\n");

  for path in registry_paths {
    generated.push_str("  include_str!(\"");
    generated.push_str(&escape_rust_string(&path));
    generated.push_str("\"),\n");
  }

  generated.push_str("];\n\n");
  generated.push_str("const EMBEDDED_ASSETS: &[EmbeddedAsset] = &[\n");

  for source in asset_sources {
    let path = workspace_root.join(&source);

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

  generated.push_str("];\n");

  fs::write(generated_path, generated)?;
  Ok(())
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
