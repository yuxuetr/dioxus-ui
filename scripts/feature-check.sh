#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

features=(
  accordion
  alert
  alert-dialog
  aspect-ratio
  attachment
  avatar
  badge
  breadcrumb
  bubble
  button
  button-group
  calendar
  carousel
  card
  chart
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
  marker
  menubar
  message
  message-scroller
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
  tree
  typography
  stat
  timeline
  steps
  indicator
  status
  radial-progress
  countdown
  diff
  rating
  number-input
  tags-input
  file-input
  swap
  dock
  fab
  theme-controller
  menu
  mockup
)

for feature in "${features[@]}"; do
  echo "checking dioxus-shadcn feature: ${feature}"
  cargo check -q -p dioxus-shadcn --no-default-features --features "${feature}"
done

echo "checking static component feature set"
cargo check -q -p dioxus-shadcn --no-default-features --features "alert,alert-dialog,aspect-ratio,attachment,avatar,badge,breadcrumb,bubble,button-group,card,carousel,chart,collapsible,direction,empty,field,input-group,input-otp,item,kbd,marker,message,message-scroller,native-select,pagination,progress,separator,sidebar,skeleton,sonner,spinner,table,toast,typography,stat,timeline,steps,indicator,status,radial-progress,countdown,diff,swap,dock,theme-controller,menu,mockup"

echo "checking primitive-backed feature set"
cargo check -q -p dioxus-shadcn --no-default-features --features "combobox,context-menu,date-picker,dialog,drawer,dropdown,hover-card,menubar,navigation-menu,popover,radio-group,select,sheet,slider,toggle-group,tooltip,tree"

echo "checking all dioxus-shadcn features"
cargo check -q -p dioxus-shadcn --all-features

# The Mobile demo depends on the preview states alone, so the workspace
# build's feature unification would hide a feature they use but do not enable.
echo "checking preview states with their own features"
cargo check -q -p dioxus-ui-preview-states

echo "feature checks passed"
