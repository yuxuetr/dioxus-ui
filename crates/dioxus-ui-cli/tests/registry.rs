use std::collections::{BTreeSet, HashSet};
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

/// Registry JSON and templates ship inside the CLI crate.
fn cli_root() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn registry_entries_are_valid() {
  let components = load_registry_components();

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
        cli_root().join(&file.source).is_file(),
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
  let components = load_registry_components();

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
  let components = load_registry_components();
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

#[test]
fn public_registry_components_match_crate_features() {
  let root = workspace_root();
  let components = load_registry_components();
  let public_registry_names = public_component_names(&components);
  let manifest_path = root.join("crates").join("dioxus-ui").join("Cargo.toml");
  let manifest = fs::read_to_string(&manifest_path).expect("dioxus-ui manifest should be readable");
  let feature_names = parse_manifest_features(&manifest);

  assert_eq!(
    public_registry_names, feature_names,
    "public registry component names should match dioxus-ui crate feature names"
  );
}

#[test]
fn registry_files_match_template_and_module_names() {
  let components = load_registry_components();

  for (path, component) in &components {
    let module_name = component_name_to_module(&component.name);
    let expected_source = format!("templates/{module_name}.rs");
    let expected_target = format!("src/components/ui/{module_name}.rs");

    assert_eq!(
      component.files.len(),
      1,
      "{} should copy exactly one component template",
      path.display()
    );
    assert_eq!(
      component.files[0].source,
      expected_source,
      "{} source should match component module name",
      path.display()
    );
    assert_eq!(
      component.files[0].target,
      expected_target,
      "{} target should match generated module path",
      path.display()
    );
  }
}

#[test]
fn every_template_file_is_registered() {
  let components = load_registry_components();
  let registered_sources = components
    .iter()
    .flat_map(|(_, component)| component.files.iter().map(|file| file.source.as_str()))
    .collect::<BTreeSet<_>>();
  let template_sources = fs::read_dir(cli_root().join("templates"))
    .expect("templates directory should exist")
    .map(|entry| {
      let entry = entry.expect("template entry should be readable");
      let file_name = entry.file_name().into_string().expect("template filename should be utf-8");

      format!("templates/{file_name}")
    })
    .collect::<BTreeSet<_>>();

  assert_eq!(
    registered_sources.into_iter().map(str::to_string).collect::<BTreeSet<_>>(),
    template_sources,
    "every template should be owned by exactly one registry entry"
  );
}

#[test]
fn generated_templates_do_not_import_internal_crates() {
  for entry in fs::read_dir(cli_root().join("templates")).expect("templates directory should exist")
  {
    let entry = entry.expect("template entry should be readable");
    let path = entry.path();
    let source = fs::read_to_string(&path).expect("template should be readable");

    assert!(
      !source.contains("dioxus_ui_core") && !source.contains("dioxus_ui_primitives"),
      "{} should not import internal dioxus-ui crates",
      path.display()
    );
  }
}

#[test]
fn template_overlay_scripts_match_crate_scripts() {
  let template_source = fs::read_to_string(cli_root().join("templates/utils.rs"))
    .expect("utils template should be readable");

  for (crate_file, name) in [
    ("crates/dioxus-ui/src/modal_focus.rs", "MODAL_FOCUS_SCOPE_SCRIPT"),
    ("crates/dioxus-ui/src/anchored_overlay.rs", "ANCHORED_OVERLAY_SCRIPT"),
  ] {
    let crate_source = fs::read_to_string(workspace_root().join(crate_file))
      .unwrap_or_else(|error| panic!("{crate_file} should be readable: {error}"));
    assert_eq!(
      raw_string_const(&template_source, name),
      raw_string_const(&crate_source, name),
      "templates/utils.rs {name} should match {crate_file}"
    );
  }
}

fn raw_string_const<'a>(source: &'a str, name: &str) -> &'a str {
  let marker = format!("{name}: &str = r#\"");
  let start = source.find(&marker).unwrap_or_else(|| panic!("missing {name}")) + marker.len();
  let length = source[start..].find("\"#;").unwrap_or_else(|| panic!("unterminated {name}"));
  &source[start..start + length]
}

#[test]
fn feature_check_script_covers_public_registry_features() {
  let root = workspace_root();
  let components = load_registry_components();
  let public_registry_names = public_component_names(&components);
  let script_path = root.join("scripts").join("feature-check.sh");
  let script = fs::read_to_string(&script_path).expect("feature-check script should be readable");
  let script_features = parse_feature_check_features(&script);

  assert_eq!(
    public_registry_names, script_features,
    "feature-check.sh should cover every public registry feature exactly once"
  );
}

fn load_registry_components() -> Vec<(PathBuf, RegistryComponent)> {
  let registry_dir = cli_root().join("registry");
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

fn public_component_names(components: &[(PathBuf, RegistryComponent)]) -> BTreeSet<String> {
  public_components(components).into_iter().map(|(_, component)| component.name.clone()).collect()
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

fn component_name_to_module(name: &str) -> String {
  name.replace('-', "_")
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

fn parse_manifest_features(manifest: &str) -> BTreeSet<String> {
  let mut features = BTreeSet::new();
  let mut in_features = false;

  for line in manifest.lines() {
    let trimmed = line.trim();

    if trimmed == "[features]" {
      in_features = true;
      continue;
    }

    if in_features && trimmed.starts_with('[') {
      break;
    }

    if !in_features || trimmed.is_empty() || trimmed.starts_with('#') {
      continue;
    }

    if let Some((name, _)) = trimmed.split_once('=') {
      let name = name.trim();

      if name != "default" {
        features.insert(name.to_string());
      }
    }
  }

  features
}

fn parse_feature_check_features(script: &str) -> BTreeSet<String> {
  let mut features = BTreeSet::new();
  let mut in_features = false;

  for line in script.lines() {
    let trimmed = line.trim();

    if trimmed == "features=(" {
      in_features = true;
      continue;
    }

    if in_features && trimmed == ")" {
      break;
    }

    if in_features && !trimmed.is_empty() && !trimmed.starts_with('#') {
      features.insert(trimmed.to_string());
    }
  }

  features
}
