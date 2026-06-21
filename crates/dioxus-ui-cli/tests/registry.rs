use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use dioxus_ui_core::RegistryComponent;

fn workspace_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .nth(2)
    .expect("cli crate should be nested under workspace/crates")
    .to_path_buf()
}

#[test]
fn registry_entries_are_valid() {
  let root = workspace_root();
  let registry_dir = root.join("registry");
  let entries = fs::read_dir(&registry_dir).expect("registry directory should exist");
  let mut components = Vec::new();

  for entry in entries {
    let entry = entry.expect("registry entry should be readable");
    let path = entry.path();

    if path.file_name().and_then(|name| name.to_str()) == Some("schema.json") {
      continue;
    }

    if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
      continue;
    }

    let json = fs::read_to_string(&path).expect("registry json should be readable");
    let component: RegistryComponent =
      serde_json::from_str(&json).expect("registry json should match component schema");

    components.push((path, component));
  }

  assert!(!components.is_empty(), "registry should contain components");

  let names = components
    .iter()
    .map(|(_, component)| component.name.as_str())
    .collect::<HashSet<_>>();

  assert_eq!(names.len(), components.len(), "component names must be unique");

  for (path, component) in &components {
    assert!(
      !component.files.is_empty(),
      "{} should declare at least one template file",
      path.display()
    );

    for file in &component.files {
      assert!(
        root.join(&file.source).is_file(),
        "{} references missing template {}",
        path.display(),
        file.source
      );
      assert!(
        Path::new(&file.target).is_relative(),
        "{} target must be relative: {}",
        path.display(),
        file.target
      );
    }

    for dependency in &component.dependencies {
      assert!(
        names.contains(dependency.as_str()),
        "{} references unknown dependency {}",
        path.display(),
        dependency
      );
    }
  }
}
