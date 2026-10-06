use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use dioxus_shadcn_core::RegistryComponent;

include!(concat!(env!("OUT_DIR"), "/embedded_assets.rs"));

const DEFAULT_CSS: &str = r#"@import "tailwindcss";

/* Semantic color tokens (RFC 0051), in the shadcn/ui v4 layout: `:root` and
   `.dark` hold the values and `@theme inline` maps them to Tailwind colors,
   so `bg-primary` reads `var(--primary)`. Redefine a token to rebrand. The
   dark theme is opt-in: add the `dark` class to an ancestor, such as the
   root element. */
@custom-variant dark (&:is(.dark *));

:root {
  --radius: 0.5rem;
  --background: oklch(1 0 0);
  --foreground: oklch(0.141 0.005 285.823);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.141 0.005 285.823);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.141 0.005 285.823);
  --primary: oklch(0.21 0.006 285.885);
  --primary-foreground: oklch(0.985 0 0);
  --secondary: oklch(0.967 0.001 286.375);
  --secondary-foreground: oklch(0.21 0.006 285.885);
  --muted: oklch(0.967 0.001 286.375);
  --muted-foreground: oklch(0.442 0.017 285.786);
  --accent: oklch(0.967 0.001 286.375);
  --accent-foreground: oklch(0.21 0.006 285.885);
  --destructive: oklch(0.532 0.245 27.325);
  --destructive-foreground: oklch(0.985 0 0);
  --success: oklch(0.627 0.194 149.214);
  --success-foreground: oklch(0.141 0.005 285.823);
  --warning: oklch(0.666 0.179 58.318);
  --warning-foreground: oklch(0.141 0.005 285.823);
  --info: oklch(0.546 0.245 262.881);
  --info-foreground: oklch(0.985 0 0);
  --border: oklch(0.92 0.004 286.32);
  --input: oklch(0.92 0.004 286.32);
  --ring: oklch(0.552 0.016 285.938);
  --chart-1: oklch(0.646 0.222 41.116);
  --chart-2: oklch(0.6 0.118 184.704);
  --chart-3: oklch(0.398 0.07 227.392);
  --chart-4: oklch(0.828 0.189 84.429);
  --chart-5: oklch(0.769 0.188 70.08);
  --sidebar: oklch(0.985 0 0);
  --sidebar-foreground: oklch(0.141 0.005 285.823);
  --sidebar-primary: oklch(0.21 0.006 285.885);
  --sidebar-primary-foreground: oklch(0.985 0 0);
  --sidebar-accent: oklch(0.967 0.001 286.375);
  --sidebar-accent-foreground: oklch(0.21 0.006 285.885);
  --sidebar-border: oklch(0.92 0.004 286.32);
  --sidebar-ring: oklch(0.552 0.016 285.938);
}

.dark {
  color-scheme: dark;
  --background: oklch(0.141 0.005 285.823);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.21 0.006 285.885);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.21 0.006 285.885);
  --popover-foreground: oklch(0.985 0 0);
  --primary: oklch(0.92 0.004 286.32);
  --primary-foreground: oklch(0.21 0.006 285.885);
  --secondary: oklch(0.274 0.006 286.033);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.274 0.006 286.033);
  --muted-foreground: oklch(0.705 0.015 286.067);
  --accent: oklch(0.274 0.006 286.033);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.704 0.191 22.216);
  --destructive-foreground: oklch(0.141 0.005 285.823);
  --success: oklch(0.723 0.219 149.579);
  --success-foreground: oklch(0.141 0.005 285.823);
  --warning: oklch(0.769 0.188 70.08);
  --warning-foreground: oklch(0.141 0.005 285.823);
  --info: oklch(0.623 0.214 259.815);
  --info-foreground: oklch(0.141 0.005 285.823);
  --border: oklch(1 0 0 / 10%);
  --input: oklch(1 0 0 / 15%);
  --ring: oklch(0.552 0.016 285.938);
  --chart-1: oklch(0.488 0.243 264.376);
  --chart-2: oklch(0.696 0.17 162.48);
  --chart-3: oklch(0.769 0.188 70.08);
  --chart-4: oklch(0.627 0.265 303.9);
  --chart-5: oklch(0.645 0.246 16.439);
  --sidebar: oklch(0.21 0.006 285.885);
  --sidebar-foreground: oklch(0.985 0 0);
  --sidebar-primary: oklch(0.488 0.243 264.376);
  --sidebar-primary-foreground: oklch(0.985 0 0);
  --sidebar-accent: oklch(0.274 0.006 286.033);
  --sidebar-accent-foreground: oklch(0.985 0 0);
  --sidebar-border: oklch(1 0 0 / 10%);
  --sidebar-ring: oklch(0.552 0.016 285.938);
}

@theme inline {
  --radius-sm: calc(var(--radius) - 4px);
  --radius-md: calc(var(--radius) - 2px);
  --radius-lg: var(--radius);
  --radius-xl: calc(var(--radius) + 4px);
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --color-card: var(--card);
  --color-card-foreground: var(--card-foreground);
  --color-popover: var(--popover);
  --color-popover-foreground: var(--popover-foreground);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-foreground);
  --color-secondary: var(--secondary);
  --color-secondary-foreground: var(--secondary-foreground);
  --color-muted: var(--muted);
  --color-muted-foreground: var(--muted-foreground);
  --color-accent: var(--accent);
  --color-accent-foreground: var(--accent-foreground);
  --color-destructive: var(--destructive);
  --color-destructive-foreground: var(--destructive-foreground);
  --color-success: var(--success);
  --color-success-foreground: var(--success-foreground);
  --color-warning: var(--warning);
  --color-warning-foreground: var(--warning-foreground);
  --color-info: var(--info);
  --color-info-foreground: var(--info-foreground);
  --color-border: var(--border);
  --color-input: var(--input);
  --color-ring: var(--ring);
  --color-chart-1: var(--chart-1);
  --color-chart-2: var(--chart-2);
  --color-chart-3: var(--chart-3);
  --color-chart-4: var(--chart-4);
  --color-chart-5: var(--chart-5);
  --color-sidebar: var(--sidebar);
  --color-sidebar-foreground: var(--sidebar-foreground);
  --color-sidebar-primary: var(--sidebar-primary);
  --color-sidebar-primary-foreground: var(--sidebar-primary-foreground);
  --color-sidebar-accent: var(--sidebar-accent);
  --color-sidebar-accent-foreground: var(--sidebar-accent-foreground);
  --color-sidebar-border: var(--sidebar-border);
  --color-sidebar-ring: var(--sidebar-ring);
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
    Some("list") => list_command(&args[1..]),
    Some("theme") => theme_command(&args[1..]),
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
  println!("initialized dioxus-shadcn in {}", root.display());
  Ok(())
}

fn add_command(args: &[OsString]) -> Result<(), Box<dyn Error>> {
  let (component_name, options) = parse_add_options(args)?;

  let block = add_component_with_options(&options.root, &component_name, options.overwrite)?;
  println!("added {component_name} to {}", options.root.display());
  if block {
    println!("declare `mod blocks;` in src/main.rs to use it");
  }
  if has_legacy_utils(&options.root) {
    println!("{LEGACY_UTILS_NOTE}");
  }
  Ok(())
}

const LEGACY_UTILS_NOTE: &str = "note: src/components/ui/utils.rs comes from dxui 0.3 or earlier and \
still holds the shared helpers, which now have their own files. Components copied before and after \
this keep separate copies and can pick the same element ids; re-copy the older ones with \
`dxui add <name> --overwrite`, which also replaces utils.rs.";

/// Whether the app's `utils.rs` still defines the helper hooks that 0.3 and
/// earlier kept there (RFC 0074).
fn has_legacy_utils(root: &Path) -> bool {
  fs::read_to_string(root.join("src").join("components").join("ui").join("utils.rs"))
    .is_ok_and(|utils| utils.contains("fn use_"))
}

/// Prints one component per line, which scripts read, or with `blocks`, one
/// block per line.
fn list_command(args: &[OsString]) -> Result<(), Box<dyn Error>> {
  let entries = match args.first().and_then(|arg| arg.to_str()) {
    None => load_registry()?,
    Some("blocks") => load_blocks()?,
    Some(other) => {
      return Err(format!("unknown list `{other}`; use `dxui list` or `dxui list blocks`").into());
    }
  };
  for entry in &entries {
    println!("{}", entry.name);
  }

  Ok(())
}

fn theme_command(args: &[OsString]) -> Result<(), Box<dyn Error>> {
  match args.first().and_then(|arg| arg.to_str()) {
    Some("list") => {
      for (name, css) in EMBEDDED_THEMES {
        println!("{name} ({})", theme_scheme(css));
      }
      Ok(())
    }
    Some("add") => {
      let (names, root) = parse_theme_add_options(&args[1..])?;
      let added = add_themes(&root, &names)?;
      if added.is_empty() {
        println!("themes already present in {}", stylesheet_path(&root).display());
      } else {
        println!("added themes {} to {}", added.join(", "), stylesheet_path(&root).display());
      }
      Ok(())
    }
    Some(subcommand) => {
      Err(format!("unknown theme subcommand `{subcommand}`; use `list` or `add`").into())
    }
    None => Err("missing theme subcommand; use `list` or `add`".into()),
  }
}

fn parse_theme_add_options(args: &[OsString]) -> Result<(Vec<String>, PathBuf), Box<dyn Error>> {
  let mut names = Vec::new();
  let mut root = env::current_dir()?;
  let mut index = 0;

  while index < args.len() {
    match args[index].to_str() {
      Some("--root") => {
        let value = args.get(index + 1).ok_or("missing value for --root")?;
        root = PathBuf::from(value);
        index += 2;
      }
      Some(flag) if flag.starts_with('-') => {
        return Err(format!("unknown theme add option `{flag}`").into());
      }
      Some(value) => {
        names.push(value.to_string());
        index += 1;
      }
      None => return Err("theme add argument is not valid UTF-8".into()),
    }
  }

  if names.is_empty() {
    return Err("missing theme name; run `dxui theme list` for the presets".into());
  }

  Ok((names, root))
}

fn theme_scheme(css: &str) -> &'static str {
  if css.contains("color-scheme: dark;") { "dark" } else { "light" }
}

fn stylesheet_path(root: &Path) -> PathBuf {
  root.join("assets").join("dioxus-shadcn.css")
}

/// Appends each preset not already in the stylesheet, after its marker check,
/// and returns the names it added. Unknown names fail before any write.
fn add_themes(root: &Path, names: &[String]) -> Result<Vec<String>, Box<dyn Error>> {
  let mut presets = Vec::new();
  for name in names {
    let preset = EMBEDDED_THEMES
      .iter()
      .find(|(preset_name, _)| preset_name == name)
      .map(|(_, css)| *css)
      .ok_or_else(|| unknown_theme_error(name))?;
    presets.push((name, preset));
  }

  let path = stylesheet_path(root);
  let mut css = fs::read_to_string(&path)
    .map_err(|error| format!("cannot read {}: {error}; run `dxui init` first", path.display()))?;
  let mut added = Vec::new();

  for (name, preset) in presets {
    if css.contains(&format!("/* dxui theme: {name} */")) {
      continue;
    }
    if !css.is_empty() && !css.ends_with('\n') {
      css.push('\n');
    }
    css.push('\n');
    css.push_str(preset);
    added.push(name.clone());
  }

  if !added.is_empty() {
    fs::write(&path, css)?;
  }

  Ok(added)
}

fn unknown_theme_error(name: &str) -> String {
  let available = EMBEDDED_THEMES.iter().map(|(name, _)| *name).collect::<Vec<_>>().join(", ");

  format!("unknown theme `{name}`. available themes: {available}")
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

  write_new_file(&assets_dir.join("dioxus-shadcn.css"), DEFAULT_CSS)?;
  write_new_file(&ui_dir.join("mod.rs"), "")?;

  Ok(())
}

#[cfg(test)]
fn add_component(root: &Path, component_name: &str) -> Result<(), Box<dyn Error>> {
  add_component_with_options(root, component_name, false).map(|_| ())
}

/// Adds a component, or a block with its components (RFC 0073). Returns
/// whether `name` was a block.
fn add_component_with_options(
  root: &Path,
  name: &str,
  overwrite: bool,
) -> Result<bool, Box<dyn Error>> {
  init_project(root)?;

  let registry = load_registry()?;
  // Dependencies name components or helpers (RFC 0074).
  let known = [registry.as_slice(), load_helpers()?.as_slice()].concat();
  let mut added = Vec::new();
  let blocks = load_blocks()?;
  let block = blocks.iter().find(|block| block.name == name);

  match block {
    Some(block) => {
      for dependency in &block.dependencies {
        add_component_recursive(root, dependency, &known, overwrite, &mut added)?;
      }
      copy_entry_files(root, block, overwrite)?;
      update_mod_file(
        &root.join("src").join("blocks").join("mod.rs"),
        std::slice::from_ref(&block.name),
      )?;
    }
    None if registry.iter().any(|component| component.name == name) => {
      add_component_recursive(root, name, &known, overwrite, &mut added)?;
    }
    None => {
      let blocks = blocks.iter().map(|block| block.name.as_str()).collect::<Vec<_>>().join(", ");
      return Err(
        format!("{}. available blocks: {blocks}", unknown_component_error(name, &registry)).into(),
      );
    }
  }
  update_ui_mod(root, &added)?;

  Ok(block.is_some())
}

fn add_component_recursive(
  root: &Path,
  component_name: &str,
  known: &[RegistryComponent],
  overwrite: bool,
  added: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
  if added.iter().any(|name| name == component_name) {
    return Ok(());
  }

  let component = known
    .iter()
    .find(|component| component.name == component_name)
    .ok_or_else(|| format!("registry entry `{component_name}` was not found"))?;

  for dependency in &component.dependencies {
    add_component_recursive(root, dependency, known, overwrite, added)?;
  }

  copy_entry_files(root, component, overwrite)?;
  added.push(component.name.clone());
  Ok(())
}

/// Copies an entry's files and assets to their targets under `root`.
fn copy_entry_files(
  root: &Path,
  entry: &RegistryComponent,
  overwrite: bool,
) -> Result<(), Box<dyn Error>> {
  let files = entry.files.iter().map(|file| (&file.source, &file.target));
  let assets = entry.assets.iter().map(|asset| (&asset.source, &asset.target));
  for (source, target) in files.chain(assets) {
    let target = root.join(target);
    let content = embedded_asset_content(source)?;

    if let Some(parent) = target.parent() {
      fs::create_dir_all(parent)?;
    }

    write_component_file(&target, content, overwrite)?;
  }
  Ok(())
}

fn unknown_component_error(component_name: &str, registry: &[RegistryComponent]) -> String {
  let available =
    registry.iter().map(|component| component.name.as_str()).collect::<Vec<_>>().join(", ");

  format!("unknown component `{component_name}`. available components: {available}")
}

fn update_ui_mod(root: &Path, component_names: &[String]) -> Result<(), Box<dyn Error>> {
  update_mod_file(&root.join("src").join("components").join("ui").join("mod.rs"), component_names)
}

/// Declares each name's module in the `mod.rs` at `mod_path`, keeping the
/// existing declarations, sorted and unique.
fn update_mod_file(mod_path: &Path, component_names: &[String]) -> Result<(), Box<dyn Error>> {
  let mod_path = mod_path.to_path_buf();
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

fn load_blocks() -> Result<Vec<RegistryComponent>, Box<dyn Error>> {
  load_entries(EMBEDDED_BLOCK_JSON)
}

fn load_helpers() -> Result<Vec<RegistryComponent>, Box<dyn Error>> {
  load_entries(EMBEDDED_HELPER_JSON)
}

fn load_registry() -> Result<Vec<RegistryComponent>, Box<dyn Error>> {
  load_entries(EMBEDDED_REGISTRY_JSON)
}

/// Parses embedded registry-format entries, sorted by name.
fn load_entries(jsons: &[&str]) -> Result<Vec<RegistryComponent>, Box<dyn Error>> {
  let mut entries = jsons
    .iter()
    .map(|json| serde_json::from_str::<RegistryComponent>(json))
    .collect::<Result<Vec<_>, _>>()?;
  entries.sort_by(|left, right| left.name.cmp(&right.name));
  Ok(entries)
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
    "dxui\n\nUsage:\n  dxui init [--root <path>]\n  dxui add <component|block> [--root <path>] [--overwrite]\n  dxui list [blocks]\n  dxui theme list\n  dxui theme add <theme>... [--root <path>]\n\nCommands:\n  init    Prepare a Dioxus project for dioxus-shadcn generated components\n  add     Copy a component, or a block with its components, into a project\n  list    List the components, or the blocks\n  theme   List theme presets, or add them to assets/dioxus-shadcn.css"
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

    let css = fs::read_to_string(root.join("assets").join("dioxus-shadcn.css"))
      .expect("css should be readable");

    assert!(css.contains("@import \"tailwindcss\";"));
    assert!(css.contains(".dark {\n  color-scheme: dark;"));
    assert!(css.contains("@theme inline {\n"));
    assert!(css.contains("--color-primary: var(--primary);"));
    assert!(root.join("src").join("components").join("ui").join("mod.rs").is_file());
  }

  #[test]
  fn add_themes_appends_each_preset_once() {
    let root = temp_project();
    init_project(&root).expect("init should succeed");

    let added =
      add_themes(&root, &["cupcake".to_string(), "dracula".to_string(), "cupcake".to_string()])
        .expect("adding themes should succeed");
    assert_eq!(added, ["cupcake", "dracula"]);

    let css = fs::read_to_string(stylesheet_path(&root)).expect("css should be readable");
    assert!(css.starts_with(DEFAULT_CSS));
    assert_eq!(css.matches("/* dxui theme: cupcake */").count(), 1);
    assert!(css.contains("[data-theme=\"dracula\"] {\n  color-scheme: dark;"));

    let added =
      add_themes(&root, &["cupcake".to_string()]).expect("repeating a theme should succeed");
    assert!(added.is_empty());
    assert_eq!(fs::read_to_string(stylesheet_path(&root)).expect("css should be readable"), css);
  }

  #[test]
  fn add_themes_rejects_unknown_names_without_writing() {
    let root = temp_project();
    init_project(&root).expect("init should succeed");

    let error = add_themes(&root, &["cupcake".to_string(), "no-such-theme".to_string()])
      .expect_err("an unknown theme should fail");
    assert!(error.to_string().contains("unknown theme `no-such-theme`"));
    assert!(error.to_string().contains("cupcake"));
    assert_eq!(
      fs::read_to_string(stylesheet_path(&root)).expect("css should be readable"),
      DEFAULT_CSS
    );
  }

  #[test]
  fn add_themes_needs_the_stylesheet() {
    let error = add_themes(&temp_project(), &["cupcake".to_string()])
      .expect_err("a missing stylesheet should fail");
    assert!(error.to_string().contains("run `dxui init` first"));
  }

  #[test]
  fn embedded_themes_report_their_scheme() {
    assert_eq!(EMBEDDED_THEMES.len(), 33);
    let scheme = |name: &str| {
      EMBEDDED_THEMES.iter().find(|(preset, _)| *preset == name).map(|(_, css)| theme_scheme(css))
    };
    assert_eq!(scheme("cupcake"), Some("light"));
    assert_eq!(scheme("dracula"), Some("dark"));
  }

  #[test]
  fn init_project_does_not_overwrite_existing_css() {
    let root = temp_project();
    let assets = root.join("assets");

    fs::create_dir_all(&assets).expect("assets directory should be created");
    fs::write(assets.join("dioxus-shadcn.css"), "custom").expect("css should be written");

    init_project(&root).expect("init should succeed");

    let css = fs::read_to_string(assets.join("dioxus-shadcn.css")).expect("css should be readable");

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

  fn ui_files(root: &Path) -> Vec<String> {
    let mut files = fs::read_dir(root.join("src").join("components").join("ui"))
      .expect("ui dir should be readable")
      .map(|entry| entry.expect("ui entry").file_name().to_string_lossy().into_owned())
      .collect::<Vec<_>>();
    files.sort();
    files
  }

  #[test]
  fn add_component_copies_only_the_helpers_it_uses() {
    let root = temp_project();
    add_component(&root, "button").expect("add should succeed");
    assert_eq!(ui_files(&root), ["button.rs", "mod.rs", "utils.rs"]);

    let root = temp_project();
    add_component(&root, "dropdown").expect("add should succeed");
    assert_eq!(
      ui_files(&root),
      [
        "anchored_overlay.rs",
        "dropdown.rs",
        "listbox.rs",
        "menu_marks.rs",
        "menu_sub.rs",
        "mod.rs",
        "overlay.rs",
        "utils.rs"
      ]
    );
  }

  #[test]
  fn helpers_are_not_components() {
    let root = temp_project();
    let error = add_component(&root, "listbox").expect_err("a helper is not a component");
    assert!(error.to_string().contains("unknown component `listbox`"));
    assert!(!error.to_string().contains("modal-focus"));
    let registry = load_registry().expect("registry should load");
    for helper in load_helpers().expect("helpers should load") {
      assert!(!registry.iter().any(|component| component.name == helper.name), "{}", helper.name);
    }
  }

  #[test]
  fn legacy_utils_is_detected() {
    let root = temp_project();
    add_component(&root, "button").expect("add should succeed");
    assert!(!has_legacy_utils(&root));

    let utils = root.join("src").join("components").join("ui").join("utils.rs");
    fs::write(&utils, "pub fn classes() {}\npub fn use_listbox() {}\n")
      .expect("utils should be written");
    assert!(has_legacy_utils(&root));
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
    assert!(message.contains("available blocks: dashboard, login, settings"));
  }

  #[test]
  fn add_block_copies_it_with_its_components() {
    let root = temp_project();

    let block = add_component_with_options(&root, "login", false).expect("add should succeed");

    assert!(block);
    let source = fs::read_to_string(root.join("src").join("blocks").join("login.rs"))
      .expect("block file should exist");
    assert!(source.contains("pub fn LoginBlock("));
    let blocks = fs::read_to_string(root.join("src").join("blocks").join("mod.rs"))
      .expect("blocks module should exist");
    assert_eq!(blocks, "pub mod login;\n");
    let ui = fs::read_to_string(root.join("src").join("components").join("ui").join("mod.rs"))
      .expect("ui module should exist");
    for module in ["button", "card", "checkbox", "field", "input", "label", "separator", "utils"] {
      assert!(ui.contains(&format!("pub mod {module};")), "{module}");
    }
    // A component name is not a block.
    assert!(!add_component_with_options(&root, "card", false).expect("add should succeed"));
  }

  #[test]
  fn blocks_have_names_no_component_uses() {
    let registry = load_registry().expect("registry should load");
    let blocks = load_blocks().expect("blocks should load");

    assert!(!blocks.is_empty());
    for block in &blocks {
      assert!(!registry.iter().any(|component| component.name == block.name), "{}", block.name);
      for file in &block.files {
        assert!(embedded_asset_content(&file.source).is_ok(), "{}", file.source);
      }
    }
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
