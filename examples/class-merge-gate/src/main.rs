//! Runs a class merge candidate over the RFC 0076 corpus for
//! `scripts/class-merge-gate.mjs`: every component utility merged with every
//! user utility, reporting which component utilities the merge removed.

use std::error::Error;
use std::time::Instant;
use std::{env, fs};

use serde_json::{Value, json};

type Merge = fn(&str, &str) -> String;
type Known = fn(&str) -> bool;

fn main() {
  if let Err(error) = run() {
    eprintln!("class-merge-gate: {error}");
    std::process::exit(1);
  }
}

fn run() -> Result<(), Box<dyn Error>> {
  let args = env::args().skip(1).collect::<Vec<_>>();
  let [candidate, corpus_path] = args.as_slice() else {
    return Err("usage: dioxus-ui-class-merge-gate <tw-merge|table> <corpus.json>".into());
  };
  // Each candidate merges a component class list with a user class list and
  // says whether it can classify a user token.
  let (merge, known): (Merge, Known) = match candidate.as_str() {
    "tw-merge" => (
      |component, user| tw_merge::merge::merge_classes(format!("{component} {user}")),
      // A merge that cannot classify a token leaves a repeated copy in place.
      |user| {
        tw_merge::merge::merge_classes(format!("{user} {user}")).split_whitespace().count() == 1
      },
    ),
    "table" => (
      |component, user| dioxus_shadcn_core::merge_classes(component.to_string(), user),
      |user| {
        dioxus_shadcn_core::merge_classes(user.to_string(), user).split_whitespace().count() == 1
      },
    ),
    other => return Err(format!("unknown candidate `{other}`").into()),
  };
  let corpus: Value = serde_json::from_str(&fs::read_to_string(corpus_path)?)?;
  let strings = |key: &str| -> Result<Vec<String>, Box<dyn Error>> {
    corpus[key]
      .as_array()
      .ok_or_else(|| format!("corpus has no `{key}` array"))?
      .iter()
      .map(|value| {
        value
          .as_str()
          .map(str::to_string)
          .ok_or_else(|| format!("`{key}` holds a non-string").into())
      })
      .collect()
  };
  let components = strings("components")?;
  let users = strings("users")?;
  let class_lists = strings("classLists")?;

  let mut removed = Vec::new();
  let mut dropped_users = Vec::new();
  for (component_index, component) in components.iter().enumerate() {
    for (user_index, user) in users.iter().enumerate() {
      if component == user {
        continue;
      }
      let merged = merge(component, user);
      let mut tokens = merged.split_whitespace();
      if !tokens.clone().any(|token| token == user) {
        dropped_users.push(json!([component_index, user_index]));
      }
      if !tokens.any(|token| token == component) {
        removed.push(json!([component_index, user_index]));
      }
    }
  }

  let unknown = users
    .iter()
    .enumerate()
    .filter(|(_, user)| !known(user))
    .map(|(index, _)| index)
    .collect::<Vec<_>>();

  // Cost per call on the shipped class lists: with no user class, and with
  // a typical one-token and three-token override.
  let mut cost = serde_json::Map::new();
  for user in ["", "mt-4", "px-2 bg-accent text-sm"] {
    let rounds = 200;
    let start = Instant::now();
    let mut checksum = 0;
    for _ in 0..rounds {
      for class_list in &class_lists {
        checksum += merge(class_list, user).len();
      }
    }
    let calls = (rounds * class_lists.len()) as f64;
    let nanos = start.elapsed().as_nanos() as f64 / calls;
    cost.insert(user.to_string(), json!({ "nanosPerCall": nanos.round(), "checksum": checksum }));
  }

  println!(
    "{}",
    json!({ "removed": removed, "droppedUsers": dropped_users, "unknown": unknown, "cost": cost })
  );
  Ok(())
}
