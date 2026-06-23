#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

features=(
  accordion
  alert
  alert-dialog
  avatar
  badge
  button
  card
  checkbox
  dialog
  drawer
  dropdown
  hover-card
  input
  label
  pagination
  popover
  progress
  radio-group
  select
  separator
  sheet
  skeleton
  slider
  spinner
  switch
  table
  tabs
  textarea
  toggle
  toggle-group
  tooltip
)

for feature in "${features[@]}"; do
  echo "checking dioxus-ui feature: ${feature}"
  cargo check -q -p dioxus-ui --no-default-features --features "${feature}"
done

echo "checking static component feature set"
cargo check -q -p dioxus-ui --no-default-features --features "alert,alert-dialog,avatar,badge,card,pagination,progress,separator,skeleton,spinner,table"

echo "checking primitive-backed feature set"
cargo check -q -p dioxus-ui --no-default-features --features "dialog,drawer,dropdown,hover-card,popover,radio-group,select,sheet,slider,toggle-group,tooltip"

echo "checking all dioxus-ui features"
cargo check -q -p dioxus-ui --all-features

echo "feature checks passed"
