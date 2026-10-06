use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use dioxus_shadcn_core::RegistryComponent;

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
  let components = all_entries();

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

  for (_, component) in &components {
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
  let public_registry_names =
    components.iter().map(|(_, component)| component.name.as_str()).collect::<HashSet<_>>();
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
  let manifest_path = root.join("crates").join("dioxus-shadcn").join("Cargo.toml");
  let manifest =
    fs::read_to_string(&manifest_path).expect("dioxus-shadcn manifest should be readable");
  let feature_names = parse_manifest_features(&manifest);

  assert_eq!(
    public_registry_names, feature_names,
    "public registry component names should match dioxus-shadcn crate feature names"
  );
}

#[test]
fn registry_files_match_template_and_module_names() {
  let components = all_entries();

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
  let components = all_entries();
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

/// A template imports sibling modules through `super::`. Each one must be a
/// dependency, so `dxui add` copies it, and each helper dependency must be
/// imported, so `dxui add` copies no helper the template does not use.
/// Components may also depend on components they are composed with.
#[test]
fn template_imports_match_dependencies() {
  let entries = all_entries();
  let helpers =
    load_entries("helpers").into_iter().map(|(_, helper)| helper.name).collect::<BTreeSet<_>>();
  for (path, entry) in &entries {
    let mut imported = BTreeSet::new();
    for file in &entry.files {
      let source = fs::read_to_string(cli_root().join(&file.source)).expect("template exists");
      for (index, _) in source.match_indices("super::") {
        let rest = &source[index + "super::".len()..];
        let module = rest
          .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
          .next()
          .unwrap_or_default();
        imported.insert(module.replace('_', "-"));
      }
    }
    for module in &imported {
      assert!(entry.dependencies.contains(module), "{} imports {module}", path.display());
    }
    for dependency in entry.dependencies.iter().filter(|dependency| helpers.contains(*dependency)) {
      assert!(imported.contains(dependency), "{} never imports {dependency}", path.display());
    }
  }
}

#[test]
fn generated_templates_do_not_import_internal_crates() {
  for entry in fs::read_dir(cli_root().join("templates")).expect("templates directory should exist")
  {
    let entry = entry.expect("template entry should be readable");
    let path = entry.path();
    let source = fs::read_to_string(&path).expect("template should be readable");

    assert!(
      !source.contains("dioxus_shadcn_core") && !source.contains("dioxus_shadcn_primitives"),
      "{} should not import internal dioxus-shadcn crates",
      path.display()
    );
  }
}

/// Blocks (RFC 0073) live apart from the component registry: each copies
/// one source into `src/blocks/`, names only known components, and imports
/// no internal crate.
#[test]
fn block_entries_are_valid() {
  let components = public_component_names(&load_registry_components());
  let blocks_dir = cli_root().join("blocks");
  let mut registered = BTreeSet::new();
  for entry in fs::read_dir(&blocks_dir).expect("blocks directory should exist") {
    let path = entry.expect("block entry").path();
    if path.extension().is_none_or(|extension| extension != "json") {
      continue;
    }
    let json = fs::read_to_string(&path).expect("block json should be readable");
    let block: RegistryComponent = serde_json::from_str(&json).expect("block json should parse");
    let module = component_name_to_module(&block.name);
    assert!(!components.contains(&block.name), "{} shares a component name", block.name);
    assert_eq!(block.files.len(), 1, "{} copies one source", block.name);
    for file in &block.files {
      assert_eq!(file.source, format!("blocks/{module}.rs"), "{}", block.name);
      assert_eq!(file.target, format!("src/blocks/{module}.rs"), "{}", block.name);
      let source = fs::read_to_string(cli_root().join(&file.source)).expect("block source exists");
      assert!(!source.contains("dioxus_shadcn"), "{} imports an internal crate", block.name);
      let name = block.name.split('-').map(|part| {
        let mut chars = part.chars();
        chars
          .next()
          .map(|first| first.to_uppercase().chain(chars).collect::<String>())
          .unwrap_or_default()
      });
      let component = format!("pub fn {}Block(", name.collect::<String>());
      assert!(source.contains(&component), "{} defines {component}", block.name);
      registered.insert(file.source.clone());
    }
    for dependency in &block.dependencies {
      assert!(components.contains(dependency), "{} needs unknown {dependency}", block.name);
    }
  }
  let sources = fs::read_dir(&blocks_dir)
    .expect("blocks directory should exist")
    .filter_map(|entry| {
      let name = entry.ok()?.file_name().into_string().ok()?;
      name.ends_with(".rs").then(|| format!("blocks/{name}"))
    })
    .collect::<BTreeSet<_>>();
  assert_eq!(sources, registered, "every block source should be registered");
}

#[test]
fn template_overlay_scripts_match_crate_scripts() {
  for (crate_file, template_file, name) in [
    (
      "crates/dioxus-shadcn/src/modal_focus.rs",
      "templates/modal_focus.rs",
      "MODAL_FOCUS_SCOPE_SCRIPT",
    ),
    (
      "crates/dioxus-shadcn/src/anchored_overlay.rs",
      "templates/anchored_overlay.rs",
      "ANCHORED_OVERLAY_SCRIPT",
    ),
    (
      "crates/dioxus-shadcn/src/dismiss_timer.rs",
      "templates/dismiss_timer.rs",
      "DISMISS_TIMER_SCRIPT",
    ),
    ("crates/dioxus-shadcn/src/listbox.rs", "templates/listbox.rs", "LISTBOX_SCRIPT"),
    (
      "crates/dioxus-shadcn/src/roving_group.rs",
      "templates/roving_group.rs",
      "ROVING_GROUP_SCRIPT",
    ),
    ("crates/dioxus-shadcn/src/menubar.rs", "templates/menubar.rs", "MENUBAR_SCRIPT"),
    ("crates/dioxus-shadcn/src/hover_open.rs", "templates/hover_open.rs", "HOVER_OPEN_SCRIPT"),
    ("crates/dioxus-shadcn/src/slider.rs", "templates/slider.rs", "SLIDER_POINTER_SCRIPT"),
    ("crates/dioxus-shadcn/src/resizable.rs", "templates/resizable.rs", "RESIZABLE_HANDLE_SCRIPT"),
    (
      "crates/dioxus-shadcn/src/checkbox.rs",
      "templates/checkbox.rs",
      "CHECKBOX_INDETERMINATE_SCRIPT",
    ),
    ("crates/dioxus-shadcn/src/input_otp.rs", "templates/input_otp.rs", "INPUT_OTP_FILTER_SCRIPT"),
    (
      "crates/dioxus-shadcn/src/navigation_menu.rs",
      "templates/navigation_menu.rs",
      "NAVIGATION_MENU_SCRIPT",
    ),
  ] {
    let crate_source = fs::read_to_string(workspace_root().join(crate_file))
      .unwrap_or_else(|error| panic!("{crate_file} should be readable: {error}"));
    let template_source = fs::read_to_string(cli_root().join(template_file))
      .unwrap_or_else(|error| panic!("{template_file} should be readable: {error}"));
    assert_eq!(
      raw_string_const(&template_source, name),
      raw_string_const(&crate_source, name),
      "{template_file} {name} should match {crate_file}"
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

/// The public components.
fn load_registry_components() -> Vec<(PathBuf, RegistryComponent)> {
  load_entries("registry")
}

/// The components and the helpers they share (RFC 0074).
fn all_entries() -> Vec<(PathBuf, RegistryComponent)> {
  [load_entries("registry"), load_entries("helpers")].concat()
}

fn load_entries(dir: &str) -> Vec<(PathBuf, RegistryComponent)> {
  let registry_dir = cli_root().join(dir);
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
  components.iter().map(|(_, component)| component.name.clone()).collect()
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
