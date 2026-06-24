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
  calendar
  card
  checkbox
  command
  combobox
  context-menu
  data-table
  date-picker
  dialog
  drawer
  dropdown
  hover-card
  input
  label
  menubar
  native-select
  navigation-menu
  pagination
  popover
  progress
  radio-group
  resizable
  select
  scroll-area
  separator
  sheet
  sidebar
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
cargo check -q -p dioxus-ui --no-default-features --features "alert,alert-dialog,avatar,badge,card,native-select,pagination,progress,separator,sidebar,skeleton,spinner,table"

echo "checking primitive-backed feature set"
cargo check -q -p dioxus-ui --no-default-features --features "combobox,context-menu,date-picker,dialog,drawer,dropdown,hover-card,menubar,navigation-menu,popover,radio-group,select,sheet,slider,toggle-group,tooltip"

echo "checking all dioxus-ui features"
cargo check -q -p dioxus-ui --all-features

echo "feature checks passed"
