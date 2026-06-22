#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

features=(
  accordion
  alert
  avatar
  badge
  button
  card
  checkbox
  dialog
  dropdown
  input
  label
  pagination
  popover
  progress
  radio-group
  select
  separator
  skeleton
  spinner
  switch
  table
  tabs
  textarea
  toggle
  tooltip
)

for feature in "${features[@]}"; do
  echo "checking dioxus-ui feature: ${feature}"
  cargo check -q -p dioxus-ui --no-default-features --features "${feature}"
done

echo "checking static component feature set"
cargo check -q -p dioxus-ui --no-default-features --features "alert,avatar,badge,card,pagination,progress,separator,skeleton,spinner,table"

echo "checking primitive-backed feature set"
cargo check -q -p dioxus-ui --no-default-features --features "dialog,dropdown,popover,radio-group,select,tooltip"

echo "checking all dioxus-ui features"
cargo check -q -p dioxus-ui --all-features

echo "feature checks passed"
