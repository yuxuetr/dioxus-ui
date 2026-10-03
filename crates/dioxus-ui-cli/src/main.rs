use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use dioxus_ui_core::RegistryComponent;

include!(concat!(env!("OUT_DIR"), "/embedded_assets.rs"));

const DEFAULT_CSS: &str = r#"@import "tailwindcss";

@theme {
  --color-background: var(--dxui-background);
  --color-foreground: var(--dxui-foreground);
}

:root {
  --dxui-background: #ffffff;
  --dxui-foreground: #09090b;
}
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
struct AddOptions {
  root: PathBuf,
  overwrite: bool,
}

fn main() {
  if let Err(error) = run(env::args_os().skip(1)) {
    eprintln!("dxui: {error}");
    std::process::exit(1);
  }
}

fn run<I>(args: I) -> Result<(), Box<dyn Error>>
where
  I: IntoIterator<Item = OsString>,
{
  let args = args.into_iter().collect::<Vec<_>>();

  match args.first().and_then(|arg| arg.to_str()) {
    Some("init") => init_command(&args[1..]),
    Some("add") => add_command(&args[1..]),
    Some("list") => list_command(),
    Some("help") | Some("--help") | Some("-h") | None => {
      print_help();
      Ok(())
    }
    Some(command) => Err(format!("unknown command `{command}`").into()),
  }
}

fn init_command(args: &[OsString]) -> Result<(), Box<dyn Error>> {
  let root = parse_root(args, "init")?;

  init_project(&root)?;
  println!("initialized dioxus-ui in {}", root.display());
  Ok(())
}

fn add_command(args: &[OsString]) -> Result<(), Box<dyn Error>> {
  let (component_name, options) = parse_add_options(args)?;

  add_component_with_options(&options.root, &component_name, options.overwrite)?;
  println!("added {component_name} to {}", options.root.display());
  Ok(())
}

fn list_command() -> Result<(), Box<dyn Error>> {
  for component in load_registry()? {
    if component.name == "utils" {
      continue;
    }

    println!("{}", component.name);
  }

  Ok(())
}

fn parse_root(args: &[OsString], command: &str) -> Result<PathBuf, Box<dyn Error>> {
  let mut root = env::current_dir()?;
  let mut index = 0;

  while index < args.len() {
    match args[index].to_str() {
      Some("--root") => {
        let value = args.get(index + 1).ok_or("missing value for --root")?;
        root = PathBuf::from(value);
        index += 2;
      }
      Some(flag) => return Err(format!("unknown {command} option `{flag}`").into()),
      None => return Err(format!("{command} option is not valid UTF-8").into()),
    }
  }

  Ok(root)
}

fn parse_add_options(args: &[OsString]) -> Result<(String, AddOptions), Box<dyn Error>> {
  let mut component_name = None;
  let mut root = env::current_dir()?;
  let mut overwrite = false;
  let mut index = 0;

  while index < args.len() {
    match args[index].to_str() {
      Some("--root") => {
        let value = args.get(index + 1).ok_or("missing value for --root")?;
        root = PathBuf::from(value);
        index += 2;
      }
      Some("--overwrite") => {
        overwrite = true;
        index += 1;
      }
      Some(flag) if flag.starts_with('-') => {
        return Err(format!("unknown add option `{flag}`").into());
      }
      Some(value) => {
        if component_name.is_some() {
          return Err(format!("unexpected add argument `{value}`").into());
        }

        component_name = Some(value.to_string());
        index += 1;
      }
      None => return Err("add argument is not valid UTF-8".into()),
    }
  }

  let component_name = component_name.ok_or("missing component name")?;
  Ok((component_name, AddOptions { root, overwrite }))
}

fn init_project(root: &Path) -> Result<(), Box<dyn Error>> {
  let assets_dir = root.join("assets");
  let ui_dir = root.join("src").join("components").join("ui");

  fs::create_dir_all(&assets_dir)?;
  fs::create_dir_all(&ui_dir)?;

  write_new_file(&assets_dir.join("dioxus-ui.css"), DEFAULT_CSS)?;
  write_new_file(&ui_dir.join("mod.rs"), "")?;

  Ok(())
}

#[cfg(test)]
fn add_component(root: &Path, component_name: &str) -> Result<(), Box<dyn Error>> {
  add_component_with_options(root, component_name, false)
}

fn add_component_with_options(
  root: &Path,
  component_name: &str,
  overwrite: bool,
) -> Result<(), Box<dyn Error>> {
  init_project(root)?;

  let registry = load_registry()?;
  let mut added = Vec::new();

  add_component_recursive(root, component_name, &registry, overwrite, &mut added)?;
  update_ui_mod(root, &added)?;

  Ok(())
}

fn add_component_recursive(
  root: &Path,
  component_name: &str,
  registry: &[RegistryComponent],
  overwrite: bool,
  added: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
  if added.iter().any(|name| name == component_name) {
    return Ok(());
  }

  let component = registry
    .iter()
    .find(|component| component.name == component_name)
    .ok_or_else(|| unknown_component_error(component_name, registry))?;

  for dependency in &component.dependencies {
    add_component_recursive(root, dependency, registry, overwrite, added)?;
  }

  for file in &component.files {
    let target = root.join(&file.target);
    let content = embedded_asset_content(&file.source)?;

    if let Some(parent) = target.parent() {
      fs::create_dir_all(parent)?;
    }

    write_component_file(&target, content, overwrite)?;
  }

  for asset in &component.assets {
    let target = root.join(&asset.target);
    let content = embedded_asset_content(&asset.source)?;

    if let Some(parent) = target.parent() {
      fs::create_dir_all(parent)?;
    }

    write_component_file(&target, content, overwrite)?;
  }

  added.push(component.name.clone());
  Ok(())
}

fn unknown_component_error(component_name: &str, registry: &[RegistryComponent]) -> String {
  let available = registry
    .iter()
    .filter(|component| component.name != "utils")
    .map(|component| component.name.as_str())
    .collect::<Vec<_>>()
    .join(", ");

  format!("unknown component `{component_name}`. available components: {available}")
}

fn update_ui_mod(root: &Path, component_names: &[String]) -> Result<(), Box<dyn Error>> {
  let mod_path = root.join("src").join("components").join("ui").join("mod.rs");
  let existing = if mod_path.exists() { fs::read_to_string(&mod_path)? } else { String::new() };
  let mut modules = existing.lines().filter_map(parse_mod_line).collect::<Vec<_>>();

  for component_name in component_names {
    let module = component_name.replace('-', "_");

    if !modules.iter().any(|existing| existing == &module) {
      modules.push(module);
    }
  }

  modules.sort();

  let content = modules.iter().map(|module| format!("pub mod {module};\n")).collect::<String>();

  if let Some(parent) = mod_path.parent() {
    fs::create_dir_all(parent)?;
  }

  fs::write(mod_path, content)?;
  Ok(())
}

fn parse_mod_line(line: &str) -> Option<String> {
  let line = line.trim();
  let name = line.strip_prefix("pub mod ")?.strip_suffix(';')?.trim();

  if name.is_empty() { None } else { Some(name.to_string()) }
}

fn load_registry() -> Result<Vec<RegistryComponent>, Box<dyn Error>> {
  let mut components = Vec::new();

  for json in EMBEDDED_REGISTRY_JSON {
    let component = serde_json::from_str::<RegistryComponent>(json)?;

    components.push(component);
  }

  components.sort_by(|left, right| left.name.cmp(&right.name));
  Ok(components)
}

fn embedded_asset_content(source: &str) -> Result<&'static str, Box<dyn Error>> {
  EMBEDDED_ASSETS
    .iter()
    .find(|asset| asset.source == source)
    .map(|asset| asset.content)
    .ok_or_else(|| format!("embedded asset `{source}` was not found").into())
}

fn write_new_file(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
  if path.exists() {
    return Ok(());
  }

  fs::write(path, content)?;
  Ok(())
}

fn write_component_file(path: &Path, content: &str, overwrite: bool) -> Result<(), Box<dyn Error>> {
  if path.exists() && !overwrite {
    return Ok(());
  }

  fs::write(path, content)?;
  Ok(())
}

fn print_help() {
  println!(
    "dxui\n\nUsage:\n  dxui init [--root <path>]\n  dxui add <component> [--root <path>] [--overwrite]\n  dxui list\n\nCommands:\n  init    Prepare a Dioxus project for dioxus-ui generated components\n  add     Copy a component template into a project\n  list    List available registry components"
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::atomic::{AtomicU64, Ordering};
  use std::time::{SystemTime, UNIX_EPOCH};

  static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

  fn temp_project() -> PathBuf {
    let nanos = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("system clock should be after unix epoch")
      .as_nanos();
    let counter = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);

    env::temp_dir().join(format!("dxui-init-test-{}-{nanos}-{counter}", std::process::id()))
  }

  #[test]
  fn init_project_creates_css_and_ui_module() {
    let root = temp_project();

    init_project(&root).expect("init should succeed");

    let css = fs::read_to_string(root.join("assets").join("dioxus-ui.css"))
      .expect("css should be readable");

    assert!(css.contains("@import \"tailwindcss\";"));
    assert!(root.join("src").join("components").join("ui").join("mod.rs").is_file());
  }

  #[test]
  fn init_project_does_not_overwrite_existing_css() {
    let root = temp_project();
    let assets = root.join("assets");

    fs::create_dir_all(&assets).expect("assets directory should be created");
    fs::write(assets.join("dioxus-ui.css"), "custom").expect("css should be written");

    init_project(&root).expect("init should succeed");

    let css = fs::read_to_string(assets.join("dioxus-ui.css")).expect("css should be readable");

    assert_eq!(css, "custom");
  }

  #[test]
  fn add_component_copies_template_and_updates_mod() {
    let root = temp_project();

    add_component(&root, "button").expect("add should succeed");

    assert!(root.join("src").join("components").join("ui").join("button.rs").is_file());

    let modules = fs::read_to_string(root.join("src").join("components").join("ui").join("mod.rs"))
      .expect("mod file should be readable");

    assert_eq!(modules, "pub mod button;\npub mod utils;\n");
  }

  #[test]
  fn add_component_does_not_overwrite_existing_template() {
    let root = temp_project();
    let ui_dir = root.join("src").join("components").join("ui");

    fs::create_dir_all(&ui_dir).expect("ui dir should be created");
    fs::write(ui_dir.join("button.rs"), "custom").expect("button should be written");

    add_component(&root, "button").expect("add should succeed");

    let button = fs::read_to_string(ui_dir.join("button.rs")).expect("button should be readable");

    assert_eq!(button, "custom");
  }

  #[test]
  fn add_component_overwrites_existing_template_when_requested() {
    let root = temp_project();
    let ui_dir = root.join("src").join("components").join("ui");

    fs::create_dir_all(&ui_dir).expect("ui dir should be created");
    fs::write(ui_dir.join("button.rs"), "custom").expect("button should be written");

    add_component_with_options(&root, "button", true).expect("add should succeed");

    let button = fs::read_to_string(ui_dir.join("button.rs")).expect("button should be readable");

    assert!(button.contains("pub enum ButtonVariant"));
    assert_ne!(button, "custom");
  }

  #[test]
  fn add_component_repeatedly_keeps_modules_unique() {
    let root = temp_project();

    add_component(&root, "button").expect("first add should succeed");
    add_component(&root, "button").expect("second add should succeed");

    let modules = fs::read_to_string(root.join("src").join("components").join("ui").join("mod.rs"))
      .expect("mod file should be readable");

    assert_eq!(modules, "pub mod button;\npub mod utils;\n");
  }

  #[test]
  fn add_component_unknown_name_lists_available_components() {
    let root = temp_project();
    let error = add_component(&root, "missing").expect_err("add should fail");
    let message = error.to_string();

    assert!(message.contains("unknown component `missing`"));
    assert!(message.contains("available components:"));
    assert!(message.contains("button"));
    assert!(!message.contains("utils"));
  }

  #[test]
  fn registry_sources_are_embedded() {
    let registry = load_registry().expect("registry should load");

    for component in registry {
      for file in component.files {
        assert!(
          embedded_asset_content(&file.source).is_ok(),
          "{} should embed {}",
          component.name,
          file.source
        );
      }

      for asset in component.assets {
        assert!(
          embedded_asset_content(&asset.source).is_ok(),
          "{} should embed {}",
          component.name,
          asset.source
        );
      }
    }
  }

  #[test]
  fn parse_add_options_accepts_overwrite_and_root() {
    let root = temp_project();
    let args = vec![
      OsString::from("button"),
      OsString::from("--overwrite"),
      OsString::from("--root"),
      root.clone().into_os_string(),
    ];

    let (component_name, options) = parse_add_options(&args).expect("options should parse");

    assert_eq!(component_name, "button");
    assert_eq!(options.root, root);
    assert!(options.overwrite);
  }
}
