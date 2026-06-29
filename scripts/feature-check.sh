#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

features=(
  accordion
  alert
  alert-dialog
  aspect-ratio
  avatar
  badge
  breadcrumb
  button
  button-group
  calendar
  carousel
  card
  checkbox
  collapsible
  command
  combobox
  context-menu
  data-table
  date-picker
  dialog
  direction
  drawer
  dropdown
  empty
  field
  hover-card
  input
  input-group
  input-otp
  item
  kbd
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
  sonner
  spinner
  switch
  table
  tabs
  textarea
  toggle
  toggle-group
  toast
  tooltip
  typography
)

for feature in "${features[@]}"; do
  echo "checking dioxus-ui feature: ${feature}"
  cargo check -q -p dioxus-ui --no-default-features --features "${feature}"
done

echo "checking static component feature set"
cargo check -q -p dioxus-ui --no-default-features --features "alert,alert-dialog,aspect-ratio,avatar,badge,breadcrumb,button-group,card,carousel,collapsible,direction,empty,field,input-group,input-otp,item,kbd,native-select,pagination,progress,separator,sidebar,skeleton,sonner,spinner,table,toast,typography"

echo "checking primitive-backed feature set"
cargo check -q -p dioxus-ui --no-default-features --features "combobox,context-menu,date-picker,dialog,drawer,dropdown,hover-card,menubar,navigation-menu,popover,radio-group,select,sheet,slider,toggle-group,tooltip"

echo "checking all dioxus-ui features"
cargo check -q -p dioxus-ui --all-features

echo "feature checks passed"
