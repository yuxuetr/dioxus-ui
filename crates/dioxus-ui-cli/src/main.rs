use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

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
    Some("help") | Some("--help") | Some("-h") | None => {
      print_help();
      Ok(())
    }
    Some(command) => Err(format!("unknown command `{command}`").into()),
  }
}

fn init_command(args: &[OsString]) -> Result<(), Box<dyn Error>> {
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
      Some(flag) => return Err(format!("unknown init option `{flag}`").into()),
      None => return Err("init option is not valid UTF-8".into()),
    }
  }

  init_project(&root)?;
  println!("initialized dioxus-ui in {}", root.display());
  Ok(())
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

fn write_new_file(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
  if path.exists() {
    return Ok(());
  }

  fs::write(path, content)?;
  Ok(())
}

fn print_help() {
  println!(
    "dxui\n\nUsage:\n  dxui init [--root <path>]\n\nCommands:\n  init    Prepare a Dioxus project for dioxus-ui generated components"
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
}
