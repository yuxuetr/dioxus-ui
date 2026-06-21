use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use dioxus_ui_core::RegistryComponent;

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
  let component_name = args
    .first()
    .and_then(|arg| arg.to_str())
    .ok_or("missing component name")?;
  let root = parse_root(&args[1..], "add")?;

  add_component(&root, component_name)?;
  println!("added {component_name} to {}", root.display());
  Ok(())
}

fn list_command() -> Result<(), Box<dyn Error>> {
  for component in load_registry()? {
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
        let value = args
          .get(index + 1)
          .ok_or("missing value for --root")?;
        root = PathBuf::from(value);
        index += 2;
      }
      Some(flag) => return Err(format!("unknown {command} option `{flag}`").into()),
      None => return Err(format!("{command} option is not valid UTF-8").into()),
    }
  }

  Ok(root)
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

fn add_component(root: &Path, component_name: &str) -> Result<(), Box<dyn Error>> {
  init_project(root)?;

  let registry = load_registry()?;
  let mut added = Vec::new();

  add_component_recursive(root, component_name, &registry, &mut added)?;
  update_ui_mod(root, &added)?;

  Ok(())
}

fn add_component_recursive(
  root: &Path,
  component_name: &str,
  registry: &[RegistryComponent],
  added: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
  if added.iter().any(|name| name == component_name) {
    return Ok(());
  }

  let component = registry
    .iter()
    .find(|component| component.name == component_name)
    .ok_or_else(|| format!("unknown component `{component_name}`"))?;

  for dependency in &component.dependencies {
    add_component_recursive(root, dependency, registry, added)?;
  }

  let workspace = workspace_root();

  for file in &component.files {
    let source = workspace.join(&file.source);
    let target = root.join(&file.target);
    let content = fs::read_to_string(&source)?;

    if let Some(parent) = target.parent() {
      fs::create_dir_all(parent)?;
    }

    write_new_file(&target, &content)?;
  }

  for asset in &component.assets {
    let source = workspace.join(&asset.source);
    let target = root.join(&asset.target);
    let content = fs::read_to_string(&source)?;

    if let Some(parent) = target.parent() {
      fs::create_dir_all(parent)?;
    }

    write_new_file(&target, &content)?;
  }

  added.push(component.name.clone());
  Ok(())
}

fn update_ui_mod(root: &Path, component_names: &[String]) -> Result<(), Box<dyn Error>> {
  let mod_path = root.join("src").join("components").join("ui").join("mod.rs");
  let existing = if mod_path.exists() {
    fs::read_to_string(&mod_path)?
  } else {
    String::new()
  };
  let mut modules = existing
    .lines()
    .filter_map(parse_mod_line)
    .collect::<Vec<_>>();

  for component_name in component_names {
    let module = component_name.replace('-', "_");

    if !modules.iter().any(|existing| existing == &module) {
      modules.push(module);
    }
  }

  modules.sort();

  let content = modules
    .iter()
    .map(|module| format!("pub mod {module};\n"))
    .collect::<String>();

  if let Some(parent) = mod_path.parent() {
    fs::create_dir_all(parent)?;
  }

  fs::write(mod_path, content)?;
  Ok(())
}

fn parse_mod_line(line: &str) -> Option<String> {
  let line = line.trim();
  let name = line
    .strip_prefix("pub mod ")?
    .strip_suffix(';')?
    .trim();

  if name.is_empty() {
    None
  } else {
    Some(name.to_string())
  }
}

fn load_registry() -> Result<Vec<RegistryComponent>, Box<dyn Error>> {
  let registry_dir = workspace_root().join("registry");
  let mut components = Vec::new();

  for entry in fs::read_dir(registry_dir)? {
    let path = entry?.path();

    if path.file_name().and_then(|name| name.to_str()) == Some("schema.json") {
      continue;
    }

    if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
      continue;
    }

    let json = fs::read_to_string(path)?;
    let component = serde_json::from_str::<RegistryComponent>(&json)?;

    components.push(component);
  }

  components.sort_by(|left, right| left.name.cmp(&right.name));
  Ok(components)
}

fn workspace_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .nth(2)
    .map(Path::to_path_buf)
    .unwrap_or_else(|| PathBuf::from("."))
}

fn write_new_file(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
  if path.exists() {
    return Ok(());
  }

  fs::write(path, content)?;
  Ok(())
}

fn print_help() {
  println!(
    "dxui\n\nUsage:\n  dxui init [--root <path>]\n  dxui add <component> [--root <path>]\n  dxui list\n\nCommands:\n  init    Prepare a Dioxus project for dioxus-ui generated components\n  add     Copy a component template into a project\n  list    List available registry components"
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

    assert_eq!(modules, "pub mod button;\n");
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
}
