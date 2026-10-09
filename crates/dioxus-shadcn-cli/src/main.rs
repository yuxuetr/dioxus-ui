use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    Some("diff") => diff_command(&args[1..]),
    Some("list") => list_command(&args[1..]),
    Some("theme") => theme_command(&args[1..]),
    Some("--version") | Some("-V") => {
      println!("dxui {}", env!("CARGO_PKG_VERSION"));
      Ok(())
    }
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

  let sources = crate_sources(&root)?;
  let path = app_path(&root, Path::new("assets/dioxus-shadcn.css"))?;
  let css = fs::read_to_string(&path)
    .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
  if let Some(refreshed) = refresh_crate_sources(&css, &sources) {
    fs::write(&path, refreshed)?;
    let previous = crate_source_paths(&css);
    for stale in previous.iter().filter(|stale| !sources.contains(stale)) {
      println!("removed @source \"{stale}\" from {}", path.display());
    }
    for source in sources.iter().filter(|source| !previous.contains(source)) {
      println!("added @source \"{source}\" to {}", path.display());
    }
  }
  Ok(())
}

/// Returns the `src` directory of every `dioxus-shadcn` package in the
/// dependency graph of the app at `root`, as `cargo metadata` resolves it.
/// A directory without `Cargo.toml` has none; a manifest Cargo cannot read is
/// an error, so a stale `@source` line is never kept without a word.
fn crate_sources(root: &Path) -> Result<Vec<String>, Box<dyn Error>> {
  let manifest = root.join("Cargo.toml");
  if !manifest.is_file() {
    return Ok(Vec::new());
  }

  let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
  let output = Command::new(cargo)
    .args(["metadata", "--format-version", "1", "--manifest-path"])
    .arg(&manifest)
    .output()
    .map_err(|error| {
      format!("cannot run cargo metadata: {error}; the @source line was not updated")
    })?;
  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(
      format!(
        "cargo metadata failed for {}; the @source line was not updated:\n{}",
        manifest.display(),
        stderr.trim()
      )
      .into(),
    );
  }

  let root = fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
  crate_sources_from_metadata(&String::from_utf8_lossy(&output.stdout), &root)
}

/// The `@source` path of each `dioxus-shadcn` package in `json`: relative to
/// the stylesheet in `assets/` when the package is inside the app at `root`,
/// such as a vendored or path dependency, so it holds wherever the app is
/// checked out; absolute otherwise, as for a crate in Cargo's registry,
/// whose place differs between machines.
fn crate_sources_from_metadata(json: &str, root: &Path) -> Result<Vec<String>, Box<dyn Error>> {
  let metadata: serde_json::Value = serde_json::from_str(json)?;
  let packages = metadata["packages"].as_array().ok_or("cargo metadata has no packages")?;
  let mut sources = Vec::new();

  for package in packages.iter().filter(|package| package["name"] == "dioxus-shadcn") {
    let manifest =
      package["manifest_path"].as_str().ok_or("cargo metadata package has no manifest_path")?;
    let dir =
      Path::new(manifest).parent().ok_or("cargo metadata manifest_path has no directory")?;
    let dir = fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let source = match dir.strip_prefix(root) {
      Ok(inside) => Path::new("..").join(inside).join("src"),
      Err(_) => dir.join("src"),
    };
    // CSS strings treat `\` as an escape; Tailwind reads `/` on every platform.
    let source = source.to_string_lossy().replace('\\', "/");
    if source.contains('"') {
      return Err(
        format!("cannot write an @source line for `{source}`, which contains a quote").into(),
      );
    }
    sources.push(source);
  }

  sources.sort();
  sources.dedup();
  Ok(sources)
}

/// The path of an `@source` line that names a `dioxus-shadcn` source
/// directory: `.../dioxus-shadcn/src` for path and git dependencies, or
/// `.../dioxus-shadcn-<version>/src` for registry ones.
fn crate_source_path(line: &str) -> Option<&str> {
  let rest = line.trim().strip_prefix("@source")?.trim_start();
  let quote = rest.chars().next().filter(|quote| *quote == '"' || *quote == '\'')?;
  let path = rest[1..].strip_suffix(';')?.trim_end().strip_suffix(quote)?;
  let mut components = path.trim_end_matches('/').rsplit('/');
  let (Some("src"), Some(dir)) = (components.next(), components.next()) else {
    return None;
  };
  let names_crate = dir == "dioxus-shadcn"
    || dir
      .strip_prefix("dioxus-shadcn-")
      .is_some_and(|version| version.starts_with(|c: char| c.is_ascii_digit()));
  names_crate.then_some(path)
}

fn crate_source_paths(css: &str) -> Vec<String> {
  css.lines().filter_map(crate_source_path).map(str::to_string).collect()
}

/// Returns the stylesheet with its crate `@source` lines naming exactly
/// `sources`, in place of the first such line or after the Tailwind import,
/// or `None` when it already does. With no sources the stylesheet is left as
/// it is: an app without the crate keeps whatever lines its author wrote.
fn refresh_crate_sources(css: &str, sources: &[String]) -> Option<String> {
  if sources.is_empty() || crate_source_paths(css) == sources {
    return None;
  }

  let wanted = sources.iter().map(|source| format!("@source \"{source}\";\n")).collect::<String>();
  let lines = css.split_inclusive('\n').collect::<Vec<_>>();
  let anchor = lines.iter().position(|line| crate_source_path(line).is_some());
  let after_import = lines.iter().position(|line| {
    let line = line.trim();
    line.starts_with("@import \"tailwindcss\"") || line.starts_with("@import 'tailwindcss'")
  });
  let mut refreshed = String::with_capacity(css.len() + wanted.len());

  if anchor.is_none() && after_import.is_none() {
    refreshed.push_str(&wanted);
  }
  for (index, line) in lines.iter().enumerate() {
    if crate_source_path(line).is_some() {
      if Some(index) == anchor {
        refreshed.push_str(&wanted);
      }
      continue;
    }
    refreshed.push_str(line);
    if anchor.is_none() && Some(index) == after_import {
      if !line.ends_with('\n') {
        refreshed.push('\n');
      }
      refreshed.push_str(&wanted);
    }
  }

  Some(refreshed)
}

/// Where `dxui add` copies the `script` helper.
const SCRIPT_TARGET: &str = "src/components/ui/script.rs";

/// The crates the `script` helper uses (RFC 0080): its line in `Cargo.toml`,
/// and whether only `wasm32` needs it.
const SCRIPT_CRATES: &[(&str, &str, bool)] = &[
  ("serde", "serde = \"1\"", false),
  ("serde_json", "serde_json = \"1\"", true),
  ("wasm-bindgen", "wasm-bindgen = \"0.2\"", true),
];

/// Names the `script` helper's crates that the app does not declare, with
/// the lines to add.
fn print_missing_script_crates(declared: &[String]) {
  let missing = |wasm_only: bool| {
    SCRIPT_CRATES
      .iter()
      .filter(|(name, _, wasm)| {
        *wasm == wasm_only && !declared.iter().any(|declared| declared == name)
      })
      .map(|(_, line, _)| *line)
      .collect::<Vec<_>>()
  };
  let (everywhere, wasm) = (missing(false), missing(true));
  if everywhere.is_empty() && wasm.is_empty() {
    return;
  }
  println!("the copied page scripts need these crates; add them to Cargo.toml:");
  if !everywhere.is_empty() {
    println!("  [dependencies]");
    everywhere.iter().for_each(|line| println!("  {line}"));
  }
  if !wasm.is_empty() {
    println!("  [target.'cfg(target_arch = \"wasm32\")'.dependencies]");
    wasm.iter().for_each(|line| println!("  {line}"));
  }
}

/// The dependencies the app at `root` declares, on any target, by crate
/// name. Without a manifest Cargo can read, it is none, so every needed
/// crate is named.
fn declared_dependencies(root: &Path) -> Vec<String> {
  let manifest = root.join("Cargo.toml");
  let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
  let output = Command::new(cargo)
    .args(["metadata", "--format-version", "1", "--no-deps", "--manifest-path"])
    .arg(&manifest)
    .output();
  match output {
    Ok(output) if output.status.success() => {
      let manifest = fs::canonicalize(&manifest).unwrap_or(manifest);
      declared_dependencies_from_metadata(&String::from_utf8_lossy(&output.stdout), &manifest)
    }
    _ => Vec::new(),
  }
}

fn declared_dependencies_from_metadata(json: &str, manifest: &Path) -> Vec<String> {
  let Ok(metadata) = serde_json::from_str::<serde_json::Value>(json) else {
    return Vec::new();
  };
  let packages = metadata["packages"].as_array().map(Vec::as_slice).unwrap_or_default();
  packages
    .iter()
    // Cargo may name the manifest through a symlink, such as macOS's `/var`.
    .filter(|package| {
      package["manifest_path"].as_str().is_some_and(|path| {
        fs::canonicalize(path).unwrap_or_else(|_| PathBuf::from(path)) == manifest
      })
    })
    .flat_map(|package| package["dependencies"].as_array().map(Vec::as_slice).unwrap_or_default())
    .filter_map(|dependency| dependency["name"].as_str().map(str::to_string))
    .collect()
}

fn add_command(args: &[OsString]) -> Result<(), Box<dyn Error>> {
  let (names, options) = parse_add_options(args)?;
  if names.is_empty() {
    return Err("missing component name".into());
  }

  let added = add_entries(&options.root, &names, options.overwrite)?;
  println!("added {} to {}", names.join(", "), options.root.display());
  for (target, status) in &added.files {
    let status = match status {
      FileStatus::Written => "written",
      FileStatus::Unchanged => "unchanged",
      FileStatus::Kept => "kept, differs from the template",
    };
    println!("  {target}: {status}");
  }
  if added.files.iter().any(|(_, status)| *status == FileStatus::Kept) {
    println!(
      "see the differences with `dxui diff {}`, or replace kept files with --overwrite",
      names.join(" ")
    );
  }
  if added.files.iter().any(|(target, _)| target == SCRIPT_TARGET) {
    print_missing_script_crates(&declared_dependencies(&options.root));
  }
  if added.block {
    println!("declare `mod blocks;` in src/main.rs to use it");
  }
  if has_legacy_utils(&options.root) {
    println!("{LEGACY_UTILS_NOTE}");
  }
  Ok(())
}

/// Prints a unified diff from each of the app's copies to the template
/// `dxui add` would write, and fails when any copy differs or is missing, so
/// CI can check that copies match the CLI.
fn diff_command(args: &[OsString]) -> Result<(), Box<dyn Error>> {
  let (names, options) = parse_add_options(args)?;
  if options.overwrite {
    return Err("unknown diff option `--overwrite`".into());
  }
  let names = if names.is_empty() { copied_entries(&options.root)? } else { names };
  if names.is_empty() {
    return Err(format!("no components or blocks copied into {}", options.root.display()).into());
  }

  let mut differing = 0;
  for (source, target) in entry_files(&resolve_entries(&names)?) {
    let template = embedded_asset_content(source)?;
    match fs::read_to_string(options.root.join(target)) {
      Ok(copy) if copy == template => {}
      Ok(copy) => {
        differing += 1;
        print!("{}", unified_diff(&copy, template, target));
      }
      Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
        differing += 1;
        println!("{target}: not in the app; `dxui add` would write it");
      }
      Err(error) => return Err(format!("cannot read {target}: {error}").into()),
    }
  }

  match differing {
    0 => {
      println!("the copies of {} match the templates", names.join(", "));
      Ok(())
    }
    1 => Err("1 file differs from the templates".into()),
    count => Err(format!("{count} files differ from the templates").into()),
  }
}

/// The changes from the app's copy to the template, with three lines of
/// context.
/// The components and blocks declared in the app's `ui/mod.rs` and
/// `blocks/mod.rs`. Helpers come back as their dependents' dependencies, and
/// the app's own modules are not entries.
fn copied_entries(root: &Path) -> Result<Vec<String>, Box<dyn Error>> {
  let entries =
    load_registry()?.into_iter().chain(load_blocks()?).map(|entry| entry.name).collect::<Vec<_>>();
  let mut copied = Vec::new();
  for mod_path in [root.join("src/components/ui/mod.rs"), root.join("src/blocks/mod.rs")] {
    let declared = match fs::read_to_string(&mod_path) {
      Ok(declared) => declared,
      Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
      Err(error) => return Err(format!("cannot read {}: {error}", mod_path.display()).into()),
    };
    copied.extend(
      declared
        .lines()
        .filter_map(parse_mod_line)
        .map(|module| module.replace('_', "-"))
        .filter(|name| entries.contains(name)),
    );
  }
  Ok(copied)
}

fn unified_diff(copy: &str, template: &str, target: &str) -> String {
  similar::TextDiff::from_lines(copy, template)
    .unified_diff()
    .context_radius(3)
    .header(&format!("a/{target}"), &format!("b/{target}"))
    .to_string()
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

/// `relative` under the app's `root`, refused when a part of it below `root`
/// is a symbolic link: a write through one, such as a linked
/// `src/components`, would land outside the app.
fn app_path(root: &Path, relative: &Path) -> Result<PathBuf, Box<dyn Error>> {
  let mut path = root.to_path_buf();
  for component in relative.components() {
    path.push(component);
    if fs::symlink_metadata(&path).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
      return Err(
        format!(
          "{} is a symbolic link; dxui writes only inside {}",
          path.display(),
          root.display()
        )
        .into(),
      );
    }
  }
  Ok(path)
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

  let path = app_path(root, Path::new("assets/dioxus-shadcn.css"))?;
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

fn parse_add_options(args: &[OsString]) -> Result<(Vec<String>, AddOptions), Box<dyn Error>> {
  let mut names = Vec::new();
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
        names.push(value.to_string());
        index += 1;
      }
      None => return Err("add argument is not valid UTF-8".into()),
    }
  }

  Ok((names, AddOptions { root, overwrite }))
}

/// Starts `src/components/ui/mod.rs`. Copied components are a library inside
/// the app: a binary crate warns about every variant, prop, and re-export it
/// does not use yet, which is not a mistake in the app.
const UI_MOD_HEADER: &str =
  "// Copied components keep the variants, props, and re-exports this app does
// not use yet, as a library would.
#![allow(dead_code, unused_imports)]
";

fn init_project(root: &Path) -> Result<(), Box<dyn Error>> {
  let assets_dir = app_path(root, Path::new("assets"))?;
  let ui_dir = app_path(root, Path::new("src/components/ui"))?;

  fs::create_dir_all(&assets_dir)?;
  fs::create_dir_all(&ui_dir)?;

  write_new_file(&app_path(root, Path::new("assets/dioxus-shadcn.css"))?, DEFAULT_CSS)?;
  write_new_file(&app_path(root, Path::new("src/components/ui/mod.rs"))?, UI_MOD_HEADER)?;

  Ok(())
}

#[cfg(test)]
fn add_component(root: &Path, component_name: &str) -> Result<(), Box<dyn Error>> {
  add_entries(root, &[component_name.to_string()], false).map(|_| ())
}

/// The registry entries that `names`, components or blocks, stand for.
struct Resolved {
  /// Components and helpers, each after its dependencies, once each.
  components: Vec<RegistryComponent>,
  /// The named blocks (RFC 0073).
  blocks: Vec<RegistryComponent>,
}

/// Resolves components and blocks with their dependencies, failing on the
/// first name that is neither.
fn resolve_entries(names: &[String]) -> Result<Resolved, Box<dyn Error>> {
  let registry = load_registry()?;
  let blocks = load_blocks()?;
  // Dependencies name components or helpers (RFC 0074).
  let known = [registry.as_slice(), load_helpers()?.as_slice()].concat();
  let mut resolved = Resolved { components: Vec::new(), blocks: Vec::new() };

  for name in names {
    if let Some(block) = blocks.iter().find(|block| &block.name == name) {
      for dependency in &block.dependencies {
        collect_component(dependency, &known, &mut resolved.components)?;
      }
      if !resolved.blocks.iter().any(|added| added.name == block.name) {
        resolved.blocks.push(block.clone());
      }
    } else if registry.iter().any(|component| &component.name == name) {
      collect_component(name, &known, &mut resolved.components)?;
    } else {
      let blocks = blocks.iter().map(|block| block.name.as_str()).collect::<Vec<_>>().join(", ");
      return Err(
        format!("{}. available blocks: {blocks}", unknown_component_error(name, &registry)).into(),
      );
    }
  }
  Ok(resolved)
}

/// Appends `name`'s entry to `collected` after its dependencies, unless it is
/// already there.
fn collect_component(
  name: &str,
  known: &[RegistryComponent],
  collected: &mut Vec<RegistryComponent>,
) -> Result<(), Box<dyn Error>> {
  if collected.iter().any(|component| component.name == name) {
    return Ok(());
  }
  let component = known
    .iter()
    .find(|component| component.name == name)
    .ok_or_else(|| format!("registry entry `{name}` was not found"))?;
  for dependency in &component.dependencies {
    collect_component(dependency, known, collected)?;
  }
  collected.push(component.clone());
  Ok(())
}

/// Each file and asset of the resolved entries, as (embedded source, target
/// relative to the app root).
fn entry_files(resolved: &Resolved) -> Vec<(&str, &str)> {
  resolved
    .components
    .iter()
    .chain(&resolved.blocks)
    .flat_map(|entry| {
      let files = entry.files.iter().map(|file| (file.source.as_str(), file.target.as_str()));
      let assets = entry.assets.iter().map(|asset| (asset.source.as_str(), asset.target.as_str()));
      files.chain(assets)
    })
    .collect()
}

/// What `dxui add` did with one file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FileStatus {
  Written,
  /// The app's copy already matched the template.
  Unchanged,
  /// The app's copy differs from the template and `--overwrite` was not set.
  Kept,
}

/// What `dxui add` did.
#[derive(Debug)]
struct Added {
  /// Each target relative to the app root, in copy order.
  files: Vec<(String, FileStatus)>,
  /// Whether any name was a block.
  block: bool,
}

/// Adds components, and blocks with their components, checking every name
/// before anything is written.
fn add_entries(root: &Path, names: &[String], overwrite: bool) -> Result<Added, Box<dyn Error>> {
  let resolved = resolve_entries(names)?;
  // Every target is checked before anything is written.
  let targets = entry_files(&resolved)
    .into_iter()
    .map(|(source, target)| Ok((source, target, app_path(root, Path::new(target))?)))
    .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

  init_project(root)?;
  let mut files = Vec::new();
  for (source, target, path) in targets {
    let status = write_component_file(&path, embedded_asset_content(source)?, overwrite)?;
    files.push((target.to_string(), status));
  }
  let names_of = |entries: &[RegistryComponent]| {
    entries.iter().map(|entry| entry.name.clone()).collect::<Vec<_>>()
  };
  update_ui_mod(root, &names_of(&resolved.components))?;
  if !resolved.blocks.is_empty() {
    update_mod_file(&app_path(root, Path::new("src/blocks/mod.rs"))?, &names_of(&resolved.blocks))?;
  }

  Ok(Added { files, block: !resolved.blocks.is_empty() })
}

fn unknown_component_error(component_name: &str, registry: &[RegistryComponent]) -> String {
  let available =
    registry.iter().map(|component| component.name.as_str()).collect::<Vec<_>>().join(", ");

  format!("unknown component `{component_name}`. available components: {available}")
}

fn update_ui_mod(root: &Path, component_names: &[String]) -> Result<(), Box<dyn Error>> {
  update_mod_file(&app_path(root, Path::new("src/components/ui/mod.rs"))?, component_names)
}

/// Declares each name's module in the `mod.rs` at `mod_path`, keeping the
/// existing declarations, sorted and unique, after the file's other lines.
fn update_mod_file(mod_path: &Path, component_names: &[String]) -> Result<(), Box<dyn Error>> {
  let mod_path = mod_path.to_path_buf();
  let existing = if mod_path.exists() { fs::read_to_string(&mod_path)? } else { String::new() };
  let mut modules = existing.lines().filter_map(parse_mod_line).collect::<Vec<_>>();
  let other_lines =
    existing.lines().filter(|line| parse_mod_line(line).is_none()).collect::<Vec<_>>().join("\n");
  let other_lines = other_lines.trim_end();

  for component_name in component_names {
    let module = component_name.replace('-', "_");

    if !modules.iter().any(|existing| existing == &module) {
      modules.push(module);
    }
  }

  modules.sort();

  let mut content =
    if other_lines.is_empty() { String::new() } else { format!("{other_lines}\n\n") };
  content.extend(modules.iter().map(|module| format!("pub mod {module};\n")));

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

/// Writes `content` to `path` unless the file already holds it, or holds
/// something else and `overwrite` is not set.
fn write_component_file(
  path: &Path,
  content: &str,
  overwrite: bool,
) -> Result<FileStatus, Box<dyn Error>> {
  match fs::read_to_string(path) {
    Ok(existing) if existing == content => return Ok(FileStatus::Unchanged),
    Ok(_) if !overwrite => return Ok(FileStatus::Kept),
    Ok(_) => {}
    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
    Err(error) => return Err(format!("cannot read {}: {error}", path.display()).into()),
  }

  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?;
  }
  fs::write(path, content)?;
  Ok(FileStatus::Written)
}

fn print_help() {
  println!(
    "dxui\n\nUsage:\n  dxui init [--root <path>]\n  dxui add <component|block>... [--root <path>] [--overwrite]\n  dxui diff [<component|block>...] [--root <path>]\n  dxui list [blocks]\n  dxui theme list\n  dxui theme add <theme>... [--root <path>]\n  dxui --version\n\nCommands:\n  init    Prepare a Dioxus project for dioxus-shadcn, and keep the crate's @source line current\n  add     Copy components, or blocks with their components, into a project\n  diff    Show how the project's copies, or the named ones, differ from the templates\n  list    List the components, or the blocks\n  theme   List theme presets, or add them to assets/dioxus-shadcn.css"
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn declared_dependencies_come_from_the_app_package() {
    let json = r#"{"packages": [
      {"manifest_path": "/app/Cargo.toml", "dependencies": [{"name": "dioxus"}, {"name": "serde"}, {"name": "wasm-bindgen"}]},
      {"manifest_path": "/app/other/Cargo.toml", "dependencies": [{"name": "serde_json"}]}
    ]}"#;

    assert_eq!(
      declared_dependencies_from_metadata(json, Path::new("/app/Cargo.toml")),
      ["dioxus", "serde", "wasm-bindgen"]
    );
    assert!(
      declared_dependencies_from_metadata("not json", Path::new("/app/Cargo.toml")).is_empty()
    );
  }
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
  #[cfg(unix)]
  fn add_refuses_a_symlinked_directory_and_writes_nothing() {
    let root = temp_project();
    let outside = temp_project();
    fs::create_dir_all(root.join("src")).expect("src should be created");
    fs::create_dir_all(&outside).expect("outside dir should be created");
    std::os::unix::fs::symlink(&outside, root.join("src").join("components"))
      .expect("symlink should be created");

    let error = add_component(&root, "button").expect_err("a symlinked directory is refused");

    assert!(error.to_string().contains("symbolic link"), "{error}");
    assert!(error.to_string().contains("components"), "{error}");
    assert_eq!(fs::read_dir(&outside).expect("outside dir").count(), 0);
    assert!(!root.join("assets").exists(), "nothing is written");
  }

  #[test]
  #[cfg(unix)]
  fn add_refuses_a_symlinked_file_and_writes_nothing() {
    let root = temp_project();
    let outside = temp_project();
    let ui = root.join("src").join("components").join("ui");
    fs::create_dir_all(&ui).expect("ui dir should be created");
    fs::create_dir_all(&outside).expect("outside dir should be created");
    fs::write(outside.join("secret.rs"), "kept").expect("outside file should be written");
    std::os::unix::fs::symlink(outside.join("secret.rs"), ui.join("button.rs"))
      .expect("symlink should be created");

    let error =
      add_entries(&root, &["button".to_string()], true).expect_err("a symlinked file is refused");

    assert!(error.to_string().contains("button.rs"), "{error}");
    assert_eq!(fs::read_to_string(outside.join("secret.rs")).expect("outside file"), "kept");
    assert!(!ui.join("utils.rs").exists(), "nothing is written");
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

  const REGISTRY_SOURCE: &str =
    "/home/me/.cargo/registry/src/index.crates.io-1/dioxus-shadcn-0.4.3/src";

  #[test]
  fn refresh_adds_the_crate_source_after_the_import() {
    let css = "@import \"tailwindcss\";\n\n:root {}\n";
    let refreshed =
      refresh_crate_sources(css, &[REGISTRY_SOURCE.to_string()]).expect("line should be added");

    assert_eq!(
      refreshed,
      format!("@import \"tailwindcss\";\n@source \"{REGISTRY_SOURCE}\";\n\n:root {{}}\n")
    );
  }

  #[test]
  fn refresh_replaces_a_stale_crate_source_in_place() {
    let css = "@import \"tailwindcss\";\n@source \"../src\";\n@source \"/x/dioxus-shadcn-0.4.2/src\";\n@source '/y/dioxus-shadcn/src/';\n:root {}\n";
    let refreshed =
      refresh_crate_sources(css, &[REGISTRY_SOURCE.to_string()]).expect("line should be replaced");

    assert_eq!(
      refreshed,
      format!(
        "@import \"tailwindcss\";\n@source \"../src\";\n@source \"{REGISTRY_SOURCE}\";\n:root {{}}\n"
      )
    );
  }

  #[test]
  fn refresh_leaves_a_current_crate_source_alone() {
    let css = format!("@import \"tailwindcss\";\n@source \"{REGISTRY_SOURCE}\";\n");

    assert_eq!(refresh_crate_sources(&css, &[REGISTRY_SOURCE.to_string()]), None);
  }

  #[test]
  fn refresh_without_the_crate_changes_nothing() {
    let css = "@import \"tailwindcss\";\n@source \"/x/dioxus-shadcn-0.4.2/src\";\n";

    assert_eq!(refresh_crate_sources(css, &[]), None);
  }

  #[test]
  fn crate_source_lines_name_only_the_styled_crate() {
    for line in [
      "@source \"/x/dioxus-shadcn-0.4.2/src\";",
      "  @source '/x/crates/dioxus-shadcn/src' ;",
      "@source \"C:/x/dioxus-shadcn-1.0.0-rc.1/src/\";",
    ] {
      assert!(crate_source_path(line).is_some(), "{line}");
    }
    for line in [
      "@source \"/x/dioxus-shadcn-primitives-0.4.2/src\";",
      "@source \"/x/dioxus-shadcn-cli/src\";",
      "@source \"/x/dioxus-shadcn-0.4.2/templates\";",
      "@source not \"/x/dioxus-shadcn-0.4.2/src\";",
      "@source \"../src\";",
    ] {
      assert!(crate_source_path(line).is_none(), "{line}");
    }
  }

  #[test]
  fn metadata_yields_each_styled_crate_source() {
    let json = r#"{"packages": [
      {"name": "dioxus-shadcn-core", "manifest_path": "/r/dioxus-shadcn-core-0.4.3/Cargo.toml"},
      {"name": "dioxus-shadcn", "manifest_path": "/r/dioxus-shadcn-0.4.3/Cargo.toml"},
      {"name": "dioxus-shadcn", "manifest_path": "/r/dioxus-shadcn-0.4.2/Cargo.toml"}
    ]}"#;

    let sources =
      crate_sources_from_metadata(json, Path::new("/app")).expect("metadata should parse");

    assert_eq!(sources, ["/r/dioxus-shadcn-0.4.2/src", "/r/dioxus-shadcn-0.4.3/src"]);
  }

  #[test]
  fn a_crate_inside_the_app_gets_a_relative_source() {
    let json = r#"{"packages": [
      {"name": "dioxus-shadcn", "manifest_path": "/app/vendor/dioxus-shadcn/Cargo.toml"}
    ]}"#;

    let sources =
      crate_sources_from_metadata(json, Path::new("/app")).expect("metadata should parse");

    assert_eq!(sources, ["../vendor/dioxus-shadcn/src"]);
    assert_eq!(
      crate_source_path(r#"@source "../vendor/dioxus-shadcn/src";"#),
      Some("../vendor/dioxus-shadcn/src")
    );
  }

  #[test]
  fn crate_sources_reads_this_workspace() {
    let cli = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace = cli.parent().and_then(Path::parent).expect("cli crate should sit in crates/");

    let sources = crate_sources(workspace).expect("cargo metadata should succeed");

    // The workspace's own crate is inside it, so the path is relative.
    assert_eq!(sources, ["../crates/dioxus-shadcn/src"]);
    assert_eq!(
      crate_sources(&temp_project()).expect("no manifest is no error"),
      Vec::<String>::new()
    );
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

    assert_eq!(
      modules,
      format!(
        "{UI_MOD_HEADER}\npub mod button;\npub mod class_merge;\npub mod class_merge_table;\npub mod density;\npub mod utils;\n"
      )
    );
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
    assert_eq!(
      ui_files(&root),
      ["button.rs", "class_merge.rs", "class_merge_table.rs", "density.rs", "mod.rs", "utils.rs"]
    );

    let root = temp_project();
    add_component(&root, "dropdown").expect("add should succeed");
    assert_eq!(
      ui_files(&root),
      [
        "anchored_overlay.rs",
        "class_merge.rs",
        "class_merge_table.rs",
        "default_attribute.rs",
        "density.rs",
        "dropdown.rs",
        "element_id.rs",
        "layer.rs",
        "listbox.rs",
        "menu_marks.rs",
        "menu_radio.rs",
        "menu_sub.rs",
        "mod.rs",
        "overlay.rs",
        "overlay_root.rs",
        "root_state.rs",
        "script.rs",
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
  fn add_component_keeps_other_lines_of_the_ui_module() {
    let root = temp_project();
    let ui_dir = root.join("src").join("components").join("ui");
    fs::create_dir_all(&ui_dir).expect("ui dir should be created");
    fs::write(ui_dir.join("mod.rs"), "pub mod card;\npub use card::Card;\n")
      .expect("mod file should be written");

    add_component(&root, "button").expect("add should succeed");

    let modules = fs::read_to_string(ui_dir.join("mod.rs")).expect("mod file should be readable");
    assert_eq!(
      modules,
      "pub use card::Card;\n\npub mod button;\npub mod card;\npub mod class_merge;\npub mod class_merge_table;\npub mod density;\npub mod utils;\n"
    );
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

    add_entries(&root, &["button".to_string()], true).expect("add should succeed");

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

    assert_eq!(
      modules,
      format!(
        "{UI_MOD_HEADER}\npub mod button;\npub mod class_merge;\npub mod class_merge_table;\npub mod density;\npub mod utils;\n"
      )
    );
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
    assert!(message.contains(
      "available blocks: chat, dashboard, files, inbox, landing, login, pricing, schedule, settings, signup"
    ));
  }

  #[test]
  fn add_block_copies_it_with_its_components() {
    let root = temp_project();

    let block =
      add_entries(&root, &["login".to_string()], false).expect("add should succeed").block;

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
    assert!(!add_entries(&root, &["card".to_string()], false).expect("add should succeed").block);
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

    let (names, options) = parse_add_options(&args).expect("options should parse");

    assert_eq!(names, ["button"]);
    assert_eq!(options.root, root);
    assert!(options.overwrite);
  }

  #[test]
  fn add_takes_several_names() {
    let args = ["button", "--overwrite", "dialog", "login"].map(OsString::from);
    let (names, options) = parse_add_options(&args).expect("options should parse");
    assert_eq!(names, ["button", "dialog", "login"]);
    assert!(options.overwrite);
    assert!(add_command(&[]).is_err(), "add needs a name");

    let root = temp_project();
    let block = add_entries(&root, &names, false).expect("add should succeed").block;
    assert!(block);
    let ui = ui_files(&root);
    for file in ["button.rs", "dialog.rs", "card.rs", "modal_focus.rs"] {
      assert!(ui.iter().any(|name| name == file), "{file}");
    }
    let blocks = fs::read_to_string(root.join("src").join("blocks").join("mod.rs"))
      .expect("blocks module should exist");
    assert_eq!(blocks, "pub mod login;\n");
  }

  #[test]
  fn add_checks_every_name_before_writing() {
    let root = temp_project();
    let names = ["button", "missing", "dialog"].map(String::from);
    let error = add_entries(&root, &names, false).expect_err("an unknown name should fail");
    assert!(error.to_string().contains("unknown component `missing`"));
    assert!(!root.exists(), "nothing should be written");
  }

  #[test]
  fn add_reports_what_it_did_with_each_file() {
    let root = temp_project();
    let button = ["button".to_string()];
    let statuses = |added: Added| added.files;
    let written = statuses(add_entries(&root, &button, false).expect("add should succeed"));
    assert_eq!(
      written,
      [
        ("src/components/ui/class_merge_table.rs".to_string(), FileStatus::Written),
        ("src/components/ui/class_merge.rs".to_string(), FileStatus::Written),
        ("src/components/ui/utils.rs".to_string(), FileStatus::Written),
        ("src/components/ui/density.rs".to_string(), FileStatus::Written),
        ("src/components/ui/button.rs".to_string(), FileStatus::Written)
      ]
    );

    let again = statuses(add_entries(&root, &button, false).expect("add should succeed"));
    assert!(again.iter().all(|(_, status)| *status == FileStatus::Unchanged), "{again:?}");

    let button_path = root.join("src/components/ui/button.rs");
    fs::write(&button_path, "custom").expect("button should be written");
    let kept = statuses(add_entries(&root, &button, false).expect("add should succeed"));
    assert_eq!(kept[3].1, FileStatus::Unchanged);
    assert_eq!(kept[4].1, FileStatus::Kept);
    assert_eq!(fs::read_to_string(&button_path).expect("button should be readable"), "custom");

    let replaced = statuses(add_entries(&root, &button, true).expect("add should succeed"));
    assert_eq!(replaced[4].1, FileStatus::Written);
  }

  #[test]
  fn diff_fails_while_copies_differ_from_the_templates() {
    let root = temp_project();
    let args = |names: &[&str]| {
      let mut args = names.iter().map(OsString::from).collect::<Vec<_>>();
      args.extend([OsString::from("--root"), root.clone().into_os_string()]);
      args
    };
    add_component(&root, "dialog").expect("add should succeed");
    diff_command(&args(&["dialog"])).expect("fresh copies should match");

    let dialog = root.join("src/components/ui/dialog.rs");
    let template = fs::read_to_string(&dialog).expect("dialog should be readable");
    fs::write(&dialog, template.replacen("pub fn Dialog", "pub fn MyDialog", 1))
      .expect("dialog should be written");
    fs::remove_file(root.join("src/components/ui/overlay.rs")).expect("overlay should be removed");
    let error = diff_command(&args(&["dialog"])).expect_err("an edited copy should differ");
    assert_eq!(error.to_string(), "2 files differ from the templates");
    // utils.rs matches; button.rs was never added.
    let error = diff_command(&args(&["button"])).expect_err("a missing copy should differ");
    assert_eq!(error.to_string(), "1 file differs from the templates");
    assert!(diff_command(&args(&["missing"])).is_err());

    let diff =
      unified_diff(&template.replacen("pub fn Dialog", "pub fn MyDialog", 1), &template, "x.rs");
    assert!(diff.starts_with("--- a/x.rs\n+++ b/x.rs\n@@ "), "{diff}");
    assert!(diff.contains("\n-pub fn MyDialog") && diff.contains("\n+pub fn Dialog"), "{diff}");
  }

  #[test]
  fn diff_without_names_checks_every_copied_entry() {
    let root = temp_project();
    let args = vec![OsString::from("--root"), root.clone().into_os_string()];
    let error = diff_command(&args).expect_err("an app without copies has nothing to check");
    assert!(error.to_string().starts_with("no components or blocks"), "{error}");

    add_entries(&root, &["button".to_string(), "dashboard".to_string()], false)
      .expect("add should succeed");
    // The app's own module is not an entry.
    let ui_mod = root.join("src/components/ui/mod.rs");
    let declared = fs::read_to_string(&ui_mod).expect("mod.rs should be readable");
    fs::write(&ui_mod, format!("{declared}pub mod my_widget;\n"))
      .expect("mod.rs should be written");
    let copied = copied_entries(&root).expect("entries should be read");
    assert!(copied.contains(&"button".to_string()) && copied.contains(&"dashboard".to_string()));
    assert!(!copied.iter().any(|name| name == "utils" || name == "my-widget"), "{copied:?}");
    diff_command(&args).expect("fresh copies should match");

    let block = root.join("src/blocks/dashboard.rs");
    let source = fs::read_to_string(&block).expect("block should be readable");
    fs::write(&block, format!("// edited\n{source}")).expect("block should be written");
    let error = diff_command(&args).expect_err("an edited block should differ");
    assert_eq!(error.to_string(), "1 file differs from the templates");
  }

  #[test]
  fn version_flags_are_commands() {
    assert!(run(["--version"].map(OsString::from)).is_ok());
    assert!(run(["-V"].map(OsString::from)).is_ok());
  }
}
