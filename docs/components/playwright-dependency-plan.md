# Playwright Dependency Plan

This document defines the M43 plan for introducing Playwright as an optional
browser automation dependency.

Status: Planned in M43.1. Playwright dev dependency and npm lockfile added in
M43.2.

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
