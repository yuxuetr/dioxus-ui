# CI Browser Smoke

This guide documents how CI can run the opt-in mobile browser smoke for the
rendered Web preview. It does not add a repository workflow by itself.

For a copyable non-blocking GitHub Actions example, see the
[CI Browser Workflow Template](ci-browser-workflow-template.md).
For activation policy and promotion criteria, see
[RFC 0009: CI Browser Workflow Activation](rfcs/0009-ci-browser-workflow-activation.md).

## Status

Recommended first mode: manual or non-blocking CI job.

Do not make this a required merge gate until the CI runner has a proven browser
installation strategy and stable localhost serving behavior.

## Scope

This smoke verifies:

- the Dioxus Web preview can be served through `dx serve`
- the preview renders in a mobile browser viewport
- the Mobile Web profile selectors are present
- representative form, message, chart, and overlay panels are queryable
- optional screenshot artifacts are valid PNG files and meet the mobile
  viewport size lower bound

This smoke does not verify:

- native Dioxus Mobile rendering
- iOS or Android simulator behavior
- software keyboard behavior
- pixel-level visual regression
- Desktop WebView screenshot capture

## Prerequisites

CI needs:

- Rust toolchain
- Dioxus CLI available as `dx`
- Node.js and npm
- committed `package-lock.json`
- permission to bind `127.0.0.1:45237`
- either Playwright-managed Chromium or an external Chrome/Chromium executable

Install Node dependencies with:

```bash
npm ci
```

Run deterministic gates before the browser smoke:

```bash
npm run verify
cargo test --workspace --all-features -q
```

Before publishing or promoting browser smoke toward a required gate, run the
full local release aggregate separately:

```bash
npm run verify:release
```

The release aggregate is intentionally separate from browser smoke because it
does not install browsers, launch Playwright, capture screenshots, or claim
native Mobile/Desktop runtime coverage. The browser smoke is not part of the release gate.

Verify the committed browser smoke contract without launching a browser:

```bash
npm run verify:mobile-browser-metadata
```

This read-only check validates script and documentation wiring for the opt-in
smoke. It does not install browsers, start `dx serve`, write screenshots, or
validate rendered output.

## Playwright-managed Chromium

Use this path when CI is allowed to download and cache Playwright browser
binaries:

```bash
npm ci
npx playwright install chromium
npm run verify:mobile-browser
```

With screenshot metadata validation:

```bash
npm ci
npx playwright install chromium
DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

Expected screenshot-enabled output includes:

```text
mobile browser screenshot saved: /path/to/dioxus-ui-mobile-browser-preview-*.png
mobile browser screenshot metadata: WIDTHxHEIGHT, BYTES bytes
mobile browser smoke passed
```

The script fails if Chromium is missing and no external executable is provided.
The smoke uses a `390x844` mobile viewport.

## External Chrome

Use this path when the CI image already provides Chrome or Chromium:

```bash
npm ci
DIOXUS_UI_BROWSER_EXECUTABLE="$CHROME_BIN" npm run verify:mobile-browser
```

With screenshot metadata validation:

```bash
npm ci
DIOXUS_UI_BROWSER_EXECUTABLE="$CHROME_BIN" DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

The executable path must exist before the preview server starts. The script
fails early when `DIOXUS_UI_BROWSER_EXECUTABLE` points to a missing file.

## Artifacts

Screenshot files use this ignored pattern:

```text
dioxus-ui-mobile-browser-preview-*.png
```

If a CI job enables screenshots, upload those PNG files as job artifacts. Do
not commit them.

Browser profiles, Playwright caches, screenshots, and temporary preview outputs
must remain outside Git.

## Failure Modes

Common failures:

- `Playwright Chromium is not installed`: run `npx playwright install chromium`
  or provide `DIOXUS_UI_BROWSER_EXECUTABLE`
- `DIOXUS_UI_BROWSER_EXECUTABLE does not exist`: fix the CI image path or
  exported environment variable
- localhost bind failure on `127.0.0.1:45237`: check runner permissions and
  port conflicts
- selector failure: inspect Web preview changes before updating the smoke
- screenshot metadata failure: inspect the generated PNG, viewport settings,
  and browser launch mode

## Workflow Policy

The repository should keep browser smoke opt-in until a reviewed workflow is
added. A first workflow should be manual, scheduled, or non-blocking. Required
merge gates should continue to rely on deterministic Rust and structural preview
checks until browser installation and local serving are stable in CI.

The workflow template in `docs/ci-browser-workflow-template.md` is documentation
until copied into `.github/workflows/`.
Activation should follow
`docs/rfcs/0009-ci-browser-workflow-activation.md`.
