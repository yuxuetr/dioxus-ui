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
  let components = load_registry_components(&root);

  assert!(!components.is_empty(), "registry should contain components");

  let names =
    components.iter().map(|(_, component)| component.name.as_str()).collect::<HashSet<_>>();

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

#[test]
fn public_registry_components_have_docs_pages() {
  let root = workspace_root();
  let components = load_registry_components(&root);

  for (_, component) in public_components(&components) {
    let docs_page = root.join("docs").join("components").join(format!("{}.md", component.name));

    assert!(
      docs_page.is_file(),
      "public component {} should have docs page {}",
      component.name,
      docs_page.display()
    );
  }
}

#[test]
fn component_catalog_matches_public_registry() {
  let root = workspace_root();
  let components = load_registry_components(&root);
  let public_registry_names = public_components(&components)
    .into_iter()
    .map(|(_, component)| component.name.as_str())
    .collect::<HashSet<_>>();
  let catalog_path = root.join("docs").join("components").join("README.md");
  let catalog = fs::read_to_string(&catalog_path).expect("component catalog should be readable");
  let catalog_names = parse_catalog_component_names(&catalog);

  assert_eq!(
    public_registry_names, catalog_names,
    "component catalog should match public registry components"
  );
}

fn load_registry_components(root: &Path) -> Vec<(PathBuf, RegistryComponent)> {
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

  components
}

fn public_components(
  components: &[(PathBuf, RegistryComponent)],
) -> Vec<(&PathBuf, &RegistryComponent)> {
  components
    .iter()
    .filter(|(_, component)| component.name != "utils")
    .map(|(path, component)| (path, component))
    .collect()
}

fn parse_catalog_component_names(catalog: &str) -> HashSet<&str> {
  catalog
    .lines()
    .filter_map(|line| {
      let trimmed = line.trim();

      if !trimmed.starts_with("| [") {
        return None;
      }

      let link_start = trimmed.find("](")?;
      let link_end = trimmed[link_start + 2..].find(".md)")? + link_start + 2;

      Some(&trimmed[link_start + 2..link_end])
    })
    .collect()
}
