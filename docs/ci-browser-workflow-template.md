# CI Browser Workflow Template

This document provides a copyable, non-blocking GitHub Actions workflow template
for the mobile browser smoke. It is documentation only. It is not active until a
maintainer copies the YAML into `.github/workflows/`.

Before copying this template, review
[RFC 0009: CI Browser Workflow Activation](rfcs/0009-ci-browser-workflow-activation.md).

## Recommended First Template

The first workflow should be manually triggered and non-blocking. It should not
run on every pull request until browser installation, localhost serving, and
artifact upload are proven stable on the selected runner.

Copy this into `.github/workflows/browser-smoke.yml` only after review:

```yaml
name: Browser Smoke

on:
  workflow_dispatch:
  # Enable only after manual runs are stable.
  # schedule:
  #   - cron: "0 8 * * 1"

permissions:
  contents: read

jobs:
  mobile-browser-smoke:
    name: Mobile browser smoke
    runs-on: ubuntu-latest
    continue-on-error: true
    timeout-minutes: 30

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Set up Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Set up Node
        uses: actions/setup-node@v4
        with:
          node-version: "22"
          cache: npm

      - name: Install Node dependencies
        run: npm ci

      - name: Install Dioxus CLI
        run: cargo install dioxus-cli --locked

      - name: Run deterministic local gates
        run: npm run verify

      - name: Run Rust workspace tests
        run: cargo test --workspace --all-features -q

      - name: Install Playwright Chromium
        run: npx playwright install chromium

      - name: Run mobile browser smoke
        env:
          DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT: "1"
        run: npm run verify:mobile-browser

      - name: Upload mobile browser screenshots
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: mobile-browser-screenshots
          path: dioxus-ui-mobile-browser-preview-*.png
          if-no-files-found: ignore
```

## External Chrome Variant

For runners that already provide Chrome or Chromium, replace the Playwright
install step and browser smoke step with:

```yaml
      - name: Run mobile browser smoke with external Chrome
        env:
          CHROME_BIN: /usr/bin/google-chrome
          DIOXUS_UI_BROWSER_EXECUTABLE: /usr/bin/google-chrome
          DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT: "1"
        run: npm run verify:mobile-browser
```

The executable must exist on the runner image. The script fails before starting
the preview server if `DIOXUS_UI_BROWSER_EXECUTABLE` points to a missing file.

## Review Checklist

Before making this workflow active:

- review RFC 0009 and choose the current rollout phase
- verify `cargo install dioxus-cli --locked` is acceptable for CI runtime
- decide whether `continue-on-error: true` should stay enabled
- decide whether the job should stay `workflow_dispatch` only
- confirm Chromium downloads are allowed on the CI network
- confirm screenshot artifacts are useful and retention policy is acceptable
- keep the job out of required merge checks until it is stable

## Scope Limits

This workflow verifies rendered Web preview behavior in a mobile browser
viewport. It does not provide native Dioxus Mobile coverage, mobile simulator
coverage, software keyboard coverage, pixel-level visual regression, or Desktop
WebView screenshot coverage.
