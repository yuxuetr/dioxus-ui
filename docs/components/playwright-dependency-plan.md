# Playwright Dependency Plan

This document defines the M43 plan for introducing Playwright as an optional
browser automation dependency.

Status: Planned in M43.1. Playwright dev dependency and npm lockfile added in
M43.2. Opt-in mobile browser smoke script added in M44.2.

## Decision

Use npm and commit `package-lock.json` if Playwright is added. The repository
already has minimal `package.json` metadata, so npm is the lowest-churn package
manager choice for this milestone.

The preferred package is `@playwright/test` as a development dependency because
it provides:

- the Playwright browser API
- a stable CLI for future test scripts
- standard browser installation commands
- conventional lockfile behavior under npm

Playwright browser binaries must not be installed as a package install side
effect. Browser installation remains an explicit command:

```bash
npx playwright install chromium
```

## Dependency Boundary

M43 may add:

- `devDependencies["@playwright/test"]`
- `package-lock.json`
- documentation for dependency and browser installation

M43 should not add:

- browser-rendered smoke scripts as default release gates
- generated browser binaries
- screenshots or temporary browser profiles
- native Mobile support claims
- postinstall hooks that download browsers automatically

M43.2 added `@playwright/test` as a development dependency and committed
`package-lock.json`. It did not run `npx playwright install chromium`, so browser
binaries remain an explicit opt-in installation step.

## Offline Behavior

After dependencies are installed, these commands should still work without
Playwright browser binaries:

```bash
npm run verify:smoke
node scripts/web-preview-verify.mjs
node scripts/mobile-web-profile-verify.mjs
node scripts/desktop-preview-verify.mjs
```

Future browser-rendered scripts should fail with clear instructions when browser
binaries are missing, rather than triggering an implicit download.

## Contributor Install

Install JavaScript dependencies:

```bash
npm install
```

This restores `node_modules` from `package-lock.json`. It does not install
browser binaries.

Install the Chromium browser binary only when working on browser-rendered smoke
tests:

```bash
npx playwright install chromium
```

This command requires network access and writes to the local Playwright browser
cache. The downloaded browser is not a repository artifact.

## Future Browser Smoke Shape

The first future browser-rendered smoke should be opt-in, for example:

```bash
npm run verify:mobile-browser
```

That command should:

1. start `dx serve` on a fixed localhost port
2. wait for the preview URL
3. launch Playwright Chromium with a mobile viewport
4. assert the M41 selectors
5. close the browser
6. stop the server

It should remain outside default release gates until local and CI stability are
proven.

## M44.1 Script Contract

M44 should add an opt-in script:

```bash
npm run verify:mobile-browser
```

The script should:

- use a fixed localhost URL, initially `http://127.0.0.1:45237`
- start `dx serve --web --package dioxus-ui-web-demo --bin preview --port
  45237 --addr 127.0.0.1 --open false --hot-reload false --watch false
  --interactive false`
- wait for the preview URL to return `200 OK`
- launch Playwright Chromium with a `390x844` mobile viewport
- assert the page title, root, `mobile-profile`, form, message, chart, and
  overlay-open selectors
- verify the page is nonblank and the chart SVG has a non-empty bounding box
- close the browser and stop the `dx serve` process in cleanup paths

If Playwright Chromium is not installed, the script should fail with actionable
guidance:

```bash
npx playwright install chromium
```

The command remains opt-in and should not be added to default release gates in
M44.

M44.2 added:

```bash
npm run verify:mobile-browser
```

The command owns `dx serve` startup and cleanup. If Chromium is missing, it
prints the install command and exits without leaving the preview server running.

## Native Mobile Boundary

Playwright mobile browser profiles do not verify native Dioxus Mobile behavior,
software keyboard changes, safe-area inset measurement, mobile assistive
technology, or native gesture arbitration.

## M43.4 Validation Result

M43 completed with a reviewed Playwright development dependency and npm
lockfile. The milestone validation ran:

```bash
npm run verify:smoke
npm ls --depth=0
cargo test --workspace --all-features -q
git diff --check
```

All commands passed. `npm ls --depth=0` reports `@playwright/test@1.61.1`.
No browser binary was installed, and no browser-rendered smoke script was added.

## Next Milestone Seed

The next browser automation milestone can add the first opt-in browser smoke
script. It should:

- fail clearly when Chromium is not installed
- document `npx playwright install chromium`
- start and stop `dx serve` itself
- use a mobile viewport against the Web preview
- assert the M41 mobile browser selectors
- stay outside default release gates until stability is proven

## M44.4 Validation Result

M44 completed with an opt-in mobile browser smoke command:

```bash
npm run verify:mobile-browser
```

The milestone validation ran:

```bash
npm run verify:smoke
npm run verify:mobile-browser
cargo test --workspace --all-features -q
git diff --check
```

`npm run verify:smoke`, workspace tests, and diff checks passed. The browser
smoke command failed as expected in the current environment because Playwright
Chromium is not installed, and it printed:

```text
Playwright Chromium is not installed. Run `npx playwright install chromium` before npm run verify:mobile-browser.
```

The failure path cleaned up the preview server, so no localhost listener was
left behind.

## Next Milestone Seed

The next browser milestone should install Chromium in an opt-in environment and
run `npm run verify:mobile-browser` end to end. If it passes, the follow-through
can add optional screenshot capture and CI documentation while keeping the
command outside default release gates.

## M45.1 Chromium Install And E2E Plan

M45 is a local opt-in milestone. It may run:

```bash
npx playwright install chromium
npm run verify:mobile-browser
```

Expected artifact behavior:

- Playwright downloads Chromium into the local Playwright cache, not the
  repository
- `node_modules/` remains ignored
- screenshot files, if added later, must match ignored preview artifact patterns
- no browser binaries or generated profiles are committed

Expected success path:

1. Chromium installs successfully.
2. `npm run verify:mobile-browser` starts `dx serve`.
3. The script opens the Web preview at a mobile viewport.
4. It verifies the Mobile Web profile markers, preview panels, nonblank body,
   and chart SVG bounding box.
5. It exits with `mobile browser smoke passed`.
6. It leaves no listener on port `45237`.

Expected failure modes:

- network download failure during `npx playwright install chromium`
- platform-specific browser launch failure
- Dioxus preview server timeout
- selector or rendering assertion failure

Any failure should be documented without promoting the browser smoke to a
default release gate.

## M45.2 Chromium Install Result

The M45.2 install attempt ran:

```bash
npx playwright install chromium
```

The command produced no output for several minutes and was interrupted. The
repository worktree remained clean, and no browser binary was committed.

The local Playwright cache did not contain a usable Chromium browser after the
interrupted run:

```text
/Users/hal/Library/Caches/ms-playwright
/Users/hal/Library/Caches/ms-playwright/b
```

`npm run verify:mobile-browser` still fails with the expected actionable
message:

```text
Playwright Chromium is not installed. Run `npx playwright install chromium` before npm run verify:mobile-browser.
```

M45 should not claim end-to-end browser smoke success unless Chromium installs
cleanly and the smoke command passes.

## M45.3 Browser Smoke Result

The M45.3 browser smoke attempt ran:

```bash
npm run verify:mobile-browser
```

It did not run end to end because Chromium was not installed. The command
failed with the expected install guidance and left no listener on port `45237`.

The deterministic gates still passed:

```bash
npm run verify:smoke
```

M45 should finish as a documented blocked end-to-end run unless Chromium is
installed successfully in a later opt-in environment.

## M45.4 Final Result

M45 completed as a documented blocked end-to-end run. The deterministic gates
passed:

```bash
npm run verify:smoke
cargo test --workspace --all-features -q
git diff --check
```

The opt-in browser smoke command exists and has a validated missing-browser
failure path, but end-to-end browser rendering is not claimed because Chromium
did not install successfully in this environment.

## Next Milestone Seed

The next browser milestone should focus on reliable browser installation rather
than selector work. Options:

- retry `npx playwright install chromium` with a more verbose or mirrored
  download configuration
- document a supported preinstalled Chromium path if Playwright-managed browser
  downloads are unreliable
- add CI notes only after one browser installation path is proven repeatable

Until then, keep `npm run verify:mobile-browser` opt-in and out of default
release gates.

## M46.1 External Chrome Plan

M46 adds a second opt-in path for environments where Playwright-managed Chromium
downloads are unreliable. The mobile browser smoke command may use an external
browser executable when this environment variable is set:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:mobile-browser
```

This differs from Playwright-managed Chromium:

- the browser version is controlled by the local machine, not the lockfile
- launch behavior may vary by operating system and installed Chrome version
- it can avoid browser downloads for local smoke checks
- it is not portable enough for default release gates

Expected behavior:

- if `DIOXUS_UI_BROWSER_EXECUTABLE` is unset, use Playwright-managed Chromium
- if it is set to a missing path, fail before starting the preview server
- if it is set to an executable that cannot launch, report the launch failure
  and clean up the preview server
- do not claim native Mobile support or release-gate stability from this path

## M46.3 External Chrome Probe Result

The local system Chrome executable exists:

```text
/Applications/Google Chrome.app/Contents/MacOS/Google Chrome
```

Running the smoke in the default sandbox first failed while binding the preview
server:

```text
Failed to bind server to: 127.0.0.1:45237
Operation not permitted (os error 1)
```

Running the same command with local dev-server permissions passed:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:mobile-browser
```

Result:

```text
mobile browser smoke passed
```

The command cleaned up the preview server and left no listener on port `45237`.
This proves a local opt-in external Chrome path, but it remains outside default
release gates because the executable path and launch behavior are
machine-specific.
